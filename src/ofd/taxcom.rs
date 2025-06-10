use async_trait::async_trait;
use fiscal_data::{fields, FieldInternal, Object};
use serde::Serialize;

use crate::{ofd::custom, parse_sum, server::State};

use super::{Error, Provider};

fn parse(data: &[u8], rec: &mut Object) -> Result<(), Error> {
    let s = std::str::from_utf8(data).map_err(|_| Error::MissingData("response"))?;
    let get_str = |tag| {
        s.split(&format!("receipt-value-{tag}\">"))
            .nth(1)?
            .split('<')
            .next()
    };
    rec.set::<fields::DocFiscalSign>({
        let fp: u64 = get_str(fields::DocFiscalSign::TAG)
            .ok_or(Error::MissingData("fp"))?
            .parse()
            .map_err(|_| Error::MissingData("fp"))?;
        let [_, _, a, b, c, d, e, f] = fp.to_be_bytes();
        [a, b, c, d, e, f]
    })?;
    rec.set::<fields::DriveNum>(
        get_str(fields::DriveNum::TAG)
            .ok_or(Error::MissingData("fn"))?
            .to_owned(),
    )?;
    rec.set::<fields::DocNum>(
        get_str(fields::DocNum::TAG)
            .ok_or(Error::MissingData("i"))?
            .parse()
            .map_err(|_| Error::MissingData("i"))?,
    )?;
    rec.set::<fields::TotalSum>(
        parse_sum(get_str(fields::TotalSum::TAG).ok_or(Error::MissingData("s"))?)
            .ok_or(Error::MissingData("s"))?,
    )?;
    rec.set::<fields::DateTime>(
        chrono::NaiveDateTime::parse_from_str(
            get_str(fields::DateTime::TAG).ok_or(Error::MissingData("t"))?,
            "%d.%m.%y %H:%M",
        )
        .map_err(|_| Error::MissingData("t"))?,
    )?;
    Ok(())
}

pub struct Taxcom;
#[async_trait]
impl Provider for Taxcom {
    fn id(&self) -> &'static str {
        "taxcom"
    }
    fn name(&self) -> &'static str {
        "ООО \"Такском\""
    }
    fn url(&self) -> &'static str {
        "www.taxcom.ru"
    }
    fn exts(&self) -> &'static [&'static str] {
        &["html"]
    }
    fn inn(&self) -> &'static str {
        "7704211201"
    }
    fn cache_id(&self, rec: &Object) -> Result<String, Error> {
        let [_, _, a, b, c, d] = rec
            .get::<fields::DocFiscalSign>()?
            .ok_or(Error::MissingData("fp"))?;
        let fiscal_sign = u32::from_be_bytes([a, b, c, d]);
        let sum = rec
            .get::<fields::TotalSum>()?
            .ok_or(Error::MissingData("s"))?;
        Ok(format!("taxcom_{fiscal_sign}_{sum}"))
    }
    async fn fetch_raw_data(&self, state: &State, rec: &mut Object) -> Result<Vec<u8>, Error> {
        let [_, _, a, b, c, d] = rec
            .get::<fields::DocFiscalSign>()?
            .ok_or(Error::MissingData("fp"))?;
        let fiscal_sign = u32::from_be_bytes([a, b, c, d]);
        let sum = rec
            .get::<fields::TotalSum>()?
            .ok_or(Error::MissingData("s"))?;
        let client = reqwest::Client::builder()
            .user_agent("Mozilla/5.0 (Windows NT 10.0; rv:109.0) Gecko/20100101 Firefox/118.0")
            .cookie_store(true)
            .build()?;
        if !rec.contains::<custom::Id>() {
            #[derive(Serialize)]
            #[serde(rename_all = "PascalCase")]
            struct Form {
                fiscal_sign: u32,
                summ: String,
                #[serde(skip_serializing_if = "Option::is_none")]
                receipt_date_filter: Option<chrono::NaiveDate>,
            }
            let res = client
                .execute(
                    client
                        .post("https://receipt.taxcom.ru/")
                        .header("Content-Type", "application/x-www-form-urlencoded")
                        .form(&Form {
                            fiscal_sign,
                            summ: format!("{}.{:02}", sum / 100, sum % 100),
                            receipt_date_filter: rec
                                .get::<fields::DateTime>()
                                .ok()
                                .flatten()
                                .map(|x| x.date()),
                        })
                        .build()?,
                )
                .await?;
            log::info!("taxcom res: {}", res.url());
            if res.url().path().ends_with("/show") {
                if let Some(id) = res.url().query_pairs().find(|x| x.0 == "id") {
                    rec.set::<custom::Id>(id.1.into_owned())?;
                }
            }
        }
        let id = rec.get::<custom::Id>()?.ok_or(Error::MissingData("id"))?;
        let res = client
            .execute(
                client
                    .get("https://receipt.taxcom.ru/v01/show")
                    .query(&[("id", id)])
                    .build()?,
            )
            .await?;
        if !res.status().is_success() {
            return Err(Error::NoResponse);
        }
        let data = res.bytes().await?;
        parse(&data, rec)?;
        if let Some(provider) = {
            let x = super::registry()
                .await
                .default(rec)
                .find(|x| x.id() != self.id());
            x
        } {
            super::fetch_raw(state, provider, rec, false).await?;
        }
        Ok(data.to_vec())
    }
    async fn parse(
        &self,
        state: &State,
        data: &[u8],
        mut rec: Object,
    ) -> Result<fiscal_data::Document, Error> {
        parse(data, &mut rec)?;
        if let Some(provider) = {
            let x = super::registry()
                .await
                .default(&rec)
                .find(|x| x.id() != self.id());
            x
        } {
            super::fetch2(state, provider, rec).await
        } else {
            Err(Error::MissingData("provider"))
        }
    }
}

#[cfg(test)]
mod test {
    use fiscal_data::Object;

    use super::parse;

    #[test]
    fn test() {
        let data = include_bytes!("../../test_data/taxcom1.html");
        parse(data, &mut Object::new()).unwrap();
    }
}
