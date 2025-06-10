use async_trait::async_trait;
use fiscal_data::{fields, Object};
use serde::Deserialize;

use crate::{ofd::custom, parse_qr, server::State};

use super::{fill_missing_fields, Error, Provider};

#[derive(Deserialize)]
struct Res {
    url: String,
}

pub struct Icom24;
#[async_trait]
impl Provider for Icom24 {
    fn id(&self) -> &'static str {
        "icom24"
    }
    fn name(&self) -> &'static str {
        "ООО \"Инфоком\""
    }
    fn url(&self) -> &'static str {
        "check-bus.icom24.ru"
    }
    fn exts(&self) -> &'static [&'static str] {
        &["json"]
    }
    fn inn(&self) -> &'static str {
        "2452033608"
    }
    fn cache_id(&self, rec: &Object) -> Result<String, Error> {
        let code = rec
            .get::<custom::IcomCode>()?
            .ok_or(Error::MissingData("code"))?;
        let date = rec
            .get::<fields::DateTime>()?
            .ok_or(Error::MissingData("date"))?
            .date();
        Ok(
            date.format("icom24_%Y-%m-%d_").to_string()
                + &code.map(|x| format!("{x:02X}")).join(""),
        )
    }
    async fn fetch_raw_data(&self, state: &State, rec: &mut Object) -> Result<Vec<u8>, Error> {
        let code = rec
            .get::<custom::IcomCode>()?
            .ok_or(Error::MissingData("code"))?;
        let date = rec
            .get::<fields::DateTime>()?
            .ok_or(Error::MissingData("date"))?;
        let client = reqwest::Client::builder().build()?;
        let ret = client
            .execute(
                client
                    .get("https://fiscal-api-citybus.icom24.ru/receipt/url")
                    .query(&[("number", code.map(|x| format!("{x:02X}")).join(""))])
                    .query(&[("date", date)])
                    .build()?,
            )
            .await?
            .bytes()
            .await?
            .to_vec();
        log::info!("icom24 response: {ret:?}");
        let res: Res = serde_json::from_slice(&ret)?;
        if !res.url.starts_with("https://receipt.taxcom.ru/v01/show?") {
            return Err(Error::ParseError);
        }
        let (_, params) = res.url.split_once('?').ok_or(Error::ParseError)?;
        fill_missing_fields(rec, &parse_qr(params).await);
        if let Some(provider) = {
            let x = super::registry()
                .await
                .by_id("taxcom", rec)
                .find(|x| x.id() != self.id());
            x
        } {
            super::fetch_raw(state, &*provider, rec, false).await?;
        }
        Ok(ret)
    }
    async fn parse(
        &self,
        state: &State,
        data: &[u8],
        mut rec: Object,
    ) -> Result<fiscal_data::Document, Error> {
        let res: Res = serde_json::from_slice(data)?;
        if !res.url.starts_with("https://receipt.taxcom.ru/v01/show?") {
            return Err(Error::ParseError);
        }
        let (_, params) = res.url.split_once('?').ok_or(Error::ParseError)?;
        fill_missing_fields(&mut rec, &parse_qr(params).await);
        super::registry().await.fill("taxcom", &mut rec)?;
        if let Some(provider) = {
            let x = super::registry()
                .await
                .by_id("taxcom", &rec)
                .find(|x| x.id() != self.id());
            x
        } {
            super::fetch2(state, &*provider, rec).await
        } else {
            Err(Error::MissingData("provider"))
        }
    }
}
