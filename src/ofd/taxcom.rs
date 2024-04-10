use crate::{digits, Config, Item, Receipt};

async fn stage1(rec: &mut Receipt) -> reqwest::Result<String> {
    let client = reqwest::Client::builder()
        .user_agent("Mozilla/5.0 (Windows NT 10.0; rv:109.0) Gecko/20100101 Firefox/118.0")
        .cookie_store(true)
        .build()?;
    if rec.id.is_empty() {
        let url = "https://receipt.taxcom.ru/";
        let res = client
            .execute(
                client
                    .post(url)
                    .header("Content-Type", "application/x-www-form-urlencoded")
                    .body(format!(
                        "FiscalSign={}&Summ={}.{:02}",
                        rec.fp,
                        rec.total / 100,
                        rec.total % 100
                    ))
                    .build()?,
            )
            .await?;

        if res.url().path().ends_with("/show") {
            if let Some(id) = res.url().query_pairs().find(|x| x.0 == "id") {
                rec.id = id.1.into_owned();
            }
        }
        if rec.id.is_empty() {
            log::error!("invalid url: {}", res.url());
            return Ok(String::new());
        }
    }
    let url = format!("https://receipt.taxcom.ru/v01/show?id={}", rec.id);
    let res = client
        .execute(client.get(url).build()?)
        .await?
        .text()
        .await?;
    Ok(res)
}

fn stage2(res: &str, rec: &mut Receipt) {
    if let Some(s) = res
        .split("<span class=\"receipt-subtitle\">")
        .nth(1)
        .and_then(|s| s.split("</span>").next())
        .and_then(|s| s.split('>').nth(1))
        .and_then(|s| s.split('<').next())
    {
        rec.company.name = html_escape::decode_html_entities(s.trim()).into_owned();
    }
    if let Some(s) = res
        .split("receipt-company-name\"")
        .nth(1)
        .and_then(|s| s.split("receipt-row-1\">").nth(1))
        .and_then(|s| s.split('>').nth(1))
        .and_then(|s| s.split('<').next())
        .and_then(|s| s.strip_prefix("ИНН"))
    {
        rec.company.inn = s.trim().to_owned();
    }
    if let Some((items, s)) = res
        .split("<div class=\"items\">")
        .nth(1)
        .and_then(|s| s.split_once(">ИТОГО:<"))
    {
        for s in items.split("<div class=\"item\">").skip(1) {
            let mut item = Item::default();
            if let Some(s) = s
                .split("receipt-row-1\">")
                .nth(1)
                .and_then(|s| Some(s.split_once("<span")?.1))
            {
                if let Some(s) = s.split('>').nth(1).and_then(|s| s.split('<').next()) {
                    item.name = html_escape::decode_html_entities(
                        s.trim_end().trim_end_matches(';').trim(),
                    )
                    .into_owned();
                }
                if let Some(s) = s
                    .split("<span")
                    .nth(1)
                    .and_then(|s| s.split('>').nth(1))
                    .and_then(|s| s.split('<').next())
                {
                    item.unit = html_escape::decode_html_entities(s.trim()).into_owned();
                }
            }
            let mut tables = s.split("<table class=\"receipt-row-2\">").skip(1);
            if let Some(s) = tables.next() {
                // 1023
                if let Some(s) = s
                    .split("<span")
                    .nth(1)
                    .and_then(|s| s.split('>').nth(1))
                    .and_then(|s| s.split('<').next())
                    .and_then(|s| s.trim().parse::<f64>().ok())
                {
                    item.count = s;
                }
                // 1079
                if let Some(s) = s
                    .split("<span")
                    .nth(2)
                    .and_then(|s| s.split('>').nth(1))
                    .and_then(|s| s.split('<').next())
                    .and_then(|s| digits(s).parse::<u64>().ok())
                {
                    item.per_item = s;
                }
                // 1043
                if let Some(s) = s
                    .split("<span")
                    .nth(3)
                    .and_then(|s| s.split('>').nth(1))
                    .and_then(|s| s.split('<').next())
                    .and_then(|s| digits(s).parse::<u64>().ok())
                {
                    item.total = s;
                }
            }
            if let Some(s) = tables.next() {
                // 1200
                if let Some(s) = s
                    .split("<span")
                    .nth(2)
                    .and_then(|s| s.split('>').nth(1))
                    .and_then(|s| s.split('<').next())
                    .and_then(|s| digits(s).parse::<u64>().ok())
                {
                    item.tax = s;
                }
            }
            rec.items.push(item);
        }
        if let Some(s) = s
            .split("<span")
            .nth(1)
            .and_then(|s| s.split('>').nth(1))
            .and_then(|s| s.split('<').next())
            .and_then(|s| digits(s).parse::<u64>().ok())
        {
            rec.total = s;
        }
        if let Some(s) = s
            .split(">БЕЗНАЛИЧНЫМИ:<")
            .nth(1)
            .and_then(|s| s.split("<span").nth(1))
            .and_then(|s| s.split('>').nth(1))
            .and_then(|s| s.split('<').next())
            .and_then(|s| digits(s).parse::<u64>().ok())
        {
            rec.total_card = s;
        }
        if let Some(s) = s
            .split(">НАЛИЧНЫМИ:<")
            .nth(1)
            .and_then(|s| s.split("<span").nth(1))
            .and_then(|s| s.split('>').nth(1))
            .and_then(|s| s.split('<').next())
            .and_then(|s| digits(s).parse::<u64>().ok())
        {
            rec.total_cash = s;
        }
        if let Some(s) = s
            .split(">НДС 20%:<")
            .nth(1)
            .and_then(|s| s.split("<span").nth(1))
            .and_then(|s| s.split('>').nth(1))
            .and_then(|s| s.split('<').next())
            .and_then(|s| digits(s).parse::<u64>().ok())
        {
            rec.total_tax += s;
        }
        if let Some(s) = s
            .split(">НДС 10%:<")
            .nth(1)
            .and_then(|s| s.split("<span").nth(1))
            .and_then(|s| s.split('>').nth(1))
            .and_then(|s| s.split('<').next())
            .and_then(|s| digits(s).parse::<u64>().ok())
        {
            rec.total_tax += s;
        }
        if let Some(s) = s
            .split(">НДС 18%:<")
            .nth(1)
            .and_then(|s| s.split("<span").nth(1))
            .and_then(|s| s.split('>').nth(1))
            .and_then(|s| s.split('<').next())
            .and_then(|s| digits(s).parse::<u64>().ok())
        {
            rec.total_tax += s;
        }
    }
}

pub(crate) async fn fetch(_config: &'static Config, mut rec: Receipt) -> Option<Receipt> {
    let res = match stage1(&mut rec).await {
        Ok(x) => {
            if x.is_empty() {
                return None;
            }
            x
        }
        Err(err) => {
            log::error!("taxcom error: {err}");
            return None;
        }
    };
    stage2(&res, &mut rec);
    Some(rec)
}

#[cfg(test)]
mod test {
    #[test]
    fn test() {
        let test_data = include_str!("../../test_data/taxcom1.html");
        let mut rec = crate::Receipt::default();
        super::stage2(test_data, &mut rec);
        assert_eq!(rec.items[0].per_item, 10999);
        assert_eq!(rec.items[0].total, 10999);
        assert_eq!(rec.items[0].count, 1.0);
        assert_eq!(rec.items.len(), 1);
    }
}
