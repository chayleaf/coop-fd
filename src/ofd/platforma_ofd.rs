use std::{collections::HashMap, ffi::CStr};

use crate::{copy_ref, digits, Config, Item, Receipt};
use tokio::sync::mpsc;

fn parse(data: &str, ret: &mut Receipt) -> Option<()> {
    // TODO: parse older formats? just for fun
    let data = data.split("fido_cheque_container\">").nth(1)?;
    let data = data.split('<').next()?;
    let data = data.trim();
    if let Some(data) = data
        .split("&lt;!-- Название --&gt;")
        .nth(1)
        .and_then(|x| x.split("&lt;!-- /Название --&gt;").next())
    {
        if let Some(x) = data
            .split("&lt;b&gt;")
            .nth(1)
            .and_then(|x| x.split("&lt;/b&gt;").next())
        {
            ret.company.name = html_escape::decode_html_entities(x).into_owned();
        }
        if let Some(x) = data
            .split("&gt;ИНН")
            .nth(1)
            .and_then(|x| x.split("&lt;").next())
        {
            ret.company.inn = x.trim().to_owned();
        }
    }
    if let Some(data) = data
        .split("&lt;!-- Предоплата --&gt;")
        .nth(1)
        .and_then(|x| x.split("&lt;!-- /Предоплата --&gt;").next())
    {
        // items
        for data in data.split("&lt;!-- Fragment - field --&gt;") {
            let mut item = Item::default();
            if let Some(x) = data
                .split("&lt;b&gt;")
                .nth(1)
                .and_then(|x| x.split("&lt;/b&gt;").next())
                .and_then(|x| x.split_once(' '))
                .map(|x| x.1)
            {
                item.name = html_escape::decode_html_entities(x).into_owned();
            }
            if let Some(data) = data
                .split("&lt;!-- Цена --&gt;")
                .nth(1)
                .and_then(|x| x.split("&lt;b&gt;").nth(1))
                .and_then(|x| x.split("&lt;/b&gt;").next())
            {
                if let Some(x) = data
                    .split("&lt;span&gt;")
                    .nth(1)
                    .and_then(|x| x.split_whitespace().next())
                    .and_then(|x| x.parse::<f64>().ok())
                {
                    item.count = x;
                }
                if let Some(x) = data
                    .split('x')
                    .next()
                    .and_then(|x| x.split("&lt;/span&gt;").nth(1))
                    .and_then(|x| x.split("&lt;span&gt;").last())
                {
                    if x != "&lt;!-- --&gt;" {
                        item.unit = html_escape::decode_html_entities(x).into_owned();
                    }
                }
                if let Some(x) = data
                    .split('x')
                    .nth(1)
                    .and_then(|x| x.split("&lt;span&gt;").nth(1))
                    .and_then(|x| x.split("&lt;/span&gt;").next())
                    .and_then(|x| digits(x).parse::<u64>().ok())
                {
                    item.per_item = x;
                }
            }
            if let Some(data) = data
                .split("&lt;!-- Общая стоимость позиции --&gt;")
                .nth(1)
                .and_then(|x| x.split("&lt;!-- /Общая стоимость позиции --&gt;").next())
            {
                if let Some(x) = data
                    .split("&lt;span")
                    .nth(2)
                    .and_then(|x| x.split("&quot;&gt;").nth(1))
                    .and_then(|x| x.split("&lt;/span&gt;").next())
                    .and_then(|x| digits(x).parse::<u64>().ok())
                {
                    item.total = x;
                }
            }
            if let Some(data) = data
                .split("&lt;!-- Сумма НДС за предмет расчета --&gt;")
                .nth(1)
                .and_then(|x| {
                    x.split("&lt;!-- /Сумма НДС за предмет расчета --&gt;")
                        .next()
                })
            {
                if let Some(x) = data
                    .split("&lt;span")
                    .nth(2)
                    .and_then(|x| x.split("&quot;&gt;").nth(1))
                    .and_then(|x| x.split("&lt;/span&gt;").next())
                    .and_then(|x| digits(x).parse::<u64>().ok())
                {
                    if item.total != x {
                        item.tax = x;
                    }
                }
            }
            ret.items.push(item);
        }
        ret.items.pop();
    }
    if let Some(data) = data
        .split("&lt;!-- ИТОГ --&gt;")
        .nth(1)
        .and_then(|x| x.split("&lt;!-- /ИТОГ --&gt;").next())
    {
        if let Some(x) = data
            .split("&lt;span&gt;")
            .nth(1)
            .and_then(|x| x.split("&lt;/span&gt;").next())
            .and_then(|x| digits(x).parse::<u64>().ok())
        {
            ret.total = x;
        }
    }
    if let Some(data) = data
        .split("&lt;!-- ИТОГ - Тело таблицы --&gt;")
        .nth(1)
        .and_then(|x| x.split("&lt;!-- /ИТОГ - Тело таблицы --&gt;").next())
    {
        let mut it = data
            .split("block&quot;&gt;")
            .skip(1)
            .map(|x| x.split("&lt;/span&gt;").next());
        while let Some(k) = it.next() {
            let v = it.next();
            let (Some(k), Some(Some(v))) = (k, v) else {
                continue;
            };
            let Ok(v) = digits(v).parse::<u64>() else {
                continue;
            };
            match k {
                "НАЛИЧНЫМИ" => ret.total_cash = v,
                "БЕЗНАЛИЧНЫМИ" => ret.total_card = v,
                x if x.starts_with("СУММА НДС ЧЕКА ПО СТАВКЕ") => {
                    ret.total_tax += v;
                }
                _ => {}
            }
        }
    }
    Some(())
}

async fn fetch2(config: &Config, rec: &mut Receipt) -> reqwest::Result<bool> {
    let client = reqwest::Client::builder()
        .user_agent("Mozilla/5.0 (Windows NT 10.0; rv:109.0) Gecko/20100101 Firefox/118.0")
        .cookie_store(true)
        .build()?;
    let url = format!(
        "https://lk.platformaofd.ru/web/noauth/cheque/search?fn={}&fp={}&i={}",
        rec.r#fn, rec.fp, rec.i
    );
    let req = client.get(&url).build()?;
    let form_res = client.execute(req).await?;
    let form = form_res.text().await?;
    let mut succ = false;
    if let Some(captcha) = form
        .split("class=\"form-captcha-image\" src=\"")
        .nth(1)
        .and_then(|x| x.split('"').next())
    {
        let req = client
            .get(format!("https://lk.platformaofd.ru{captcha}"))
            .build()?;
        let captcha_res = client.execute(req).await?;
        let captcha_img = captcha_res.bytes().await?;
        let mut api = leptess::tesseract::TessApi::new(None, "fin").unwrap();
        api.raw
            .set_variable(
                leptess::Variable::TesseditCharWhitelist.as_cstr(),
                CStr::from_bytes_with_nul(b"0123456789\0").unwrap(),
            )
            .unwrap();
        if let Ok(pix) = leptess::leptonica::pix_read_mem(&captcha_img) {
            api.set_image(&pix);
        }
        let captcha = api
            .get_utf8_text()
            .unwrap_or_default()
            .as_str()
            .chars()
            .filter(char::is_ascii_digit)
            .collect::<String>();
        log::info!("captcha: {captcha}");
        if let Some(csrf) = form
            .split("type=\"hidden\" name=\"_csrf\" value=\"")
            .nth(1)
            .and_then(|x| x.split('"').next())
        {
            let req = client
                .post("https://lk.platformaofd.ru/web/noauth/cheque/search")
                .form(&{
                    let mut form = HashMap::new();
                    form.insert("fn", rec.r#fn.clone());
                    form.insert("fp", rec.fp.clone());
                    form.insert("i", rec.i.clone());
                    form.insert("captcha", captcha);
                    form.insert("_csrf", csrf.to_owned());
                    form
                })
                .header("Referer", url)
                .header("Origin", "https://lk.platformaofd.ru")
                .build()?;
            let res = client.execute(req).await?;
            if res.url().path().ends_with("/id") {
                rec.id = res
                    .url()
                    .query_pairs()
                    .find_map(|(k, v)| (k == "id").then_some(v))
                    .unwrap_or_default()
                    .into_owned();
                let text = res.text().await?;
                let mut path = config.data_path.clone();
                path.push("raw");
                path.push("platforma-ofd");
                path.push(rec.id.clone() + ".html");
                if let Err(err) = tokio::fs::write(path, &text).await {
                    log::error!("failed to write raw receipt: {err:?}");
                }
                parse(&text, rec);
                if let Some(fnifp) = rec.fnifp() {
                    let mut path = config.data_path.clone();
                    path.push("parsed");
                    path.push(fnifp + ".json");
                    match serde_json::to_vec(rec) {
                        Ok(rec) => {
                            if let Err(err) = tokio::fs::write(path, &rec).await {
                                log::error!("failed to write receipt cache: {err:?}");
                            }
                        }
                        Err(err) => {
                            log::error!("failed to serialize receipt: {err:?}");
                        }
                    }
                }
                succ = true;
            }
        }
    }
    Ok(succ)
}

pub(crate) async fn fetch(config: &'static Config, rec: Receipt) -> Option<Receipt> {
    let (tx, mut rx) = mpsc::channel(1);
    for _ in 0..8 {
        let mut rec = rec.clone();
        let tx = tx.clone();
        let config = copy_ref(config);
        tokio::spawn(async move {
            for _ in 0..8 {
                if matches!(fetch2(config, &mut rec).await, Ok(true)) {
                    let _ = tx.try_send(rec);
                    break;
                }
                if tx.is_closed() {
                    break;
                }
            }
        });
    }
    drop(tx);
    rx.recv().await
}
