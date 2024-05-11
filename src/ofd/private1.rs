//! Private provider 1

use async_trait::async_trait;

use fiscal_data::{
    fields,
    json::{
        Bso, BsoCorrection, CloseArchive, CloseShift, CurrentStateReport, FiscalReport,
        FiscalReportCorrection, OpenShift, Receipt, ReceiptCorrection,
    },
};
use serde::Deserialize;

use super::{Error, Provider};
use crate::Config;

pub struct Private1 {
    endpoint: String,
}

impl Private1 {
    pub fn new(endpoint: &str) -> Self {
        Self {
            endpoint: endpoint.to_owned(),
        }
    }
}

#[derive(Deserialize)]
struct Res<T> {
    status: bool,
    data: T,
}

#[async_trait]
impl Provider for Private1 {
    fn id(&self) -> &'static str {
        "private1"
    }
    fn url(&self) -> &'static str {
        ""
    }
    fn inn(&self) -> &'static str {
        ""
    }
    fn name(&self) -> &'static str {
        ""
    }
    fn exts(&self) -> &'static [&'static str] {
        &["json"]
    }
    async fn fetch_raw_data(&self, rec: &mut fiscal_data::Object) -> Result<Vec<u8>, Error> {
        let drive_num = rec
            .get::<fields::DriveNum>()?
            .ok_or(Error::MissingData("fn"))?;
        let [_, _, a, b, c, d] = rec
            .get::<fields::DocFiscalSign>()?
            .ok_or(Error::MissingData("fp"))?;
        let fiscal_sign = u32::from_be_bytes([a, b, c, d]);
        let date = rec
            .get::<fields::DateTime>()?
            .ok_or(Error::MissingData("date"))?;
        let sum = rec
            .get::<fields::TotalSum>()?
            .ok_or(Error::MissingData("sum"))?;
        let doc_num = rec
            .get::<fields::DocNum>()?
            .ok_or(Error::MissingData("fd"))?;
        let date = date.format("%Y-%m-%dT%H:%M:%S");
        let url = format!(
            "{}?fn={drive_num}&i={doc_num}&fiscalSign={fiscal_sign}&date={date}&sum={sum}",
            self.endpoint,
        );
        let ret = reqwest::get(url).await?.bytes().await?.to_vec();
        #[cfg(not(debug_assertions))]
        {
            let Ok(res) = serde_json::from_slice::<Res<serde_json::Value>>(&ret) else {
                return Err(Error::ParseError);
            };
            if !res.status {
                return Err(Error::NoResponse);
            }
        }
        Ok(ret)
    }
    async fn parse(
        &self,
        _config: &Config,
        data: &[u8],
        _rec: fiscal_data::Object,
    ) -> Result<fiscal_data::Object, Error> {
        log::info!("trying {data:?}");
        let res = serde_json::from_slice::<Res<serde_json::Value>>(data)?;
        log::info!("data ok");
        if !res.status {
            return Err(Error::ParseError);
        }
        let serde_json::Value::Object(code) = res.data else {
            return Err(Error::ParseError);
        };
        let Some(serde_json::Value::Number(code)) = code.get("code") else {
            return Err(Error::ParseError);
        };
        let code = code.as_u64().ok_or(Error::ParseError)?;
        Ok(match code {
            1 => serde_json::from_slice::<Res<FiscalReport>>(data)?
                .data
                .try_into()?,
            11 => serde_json::from_slice::<Res<FiscalReportCorrection>>(data)?
                .data
                .try_into()?,
            2 => serde_json::from_slice::<Res<OpenShift>>(data)?
                .data
                .try_into()?,
            21 => serde_json::from_slice::<Res<CurrentStateReport>>(data)?
                .data
                .try_into()?,
            3 => serde_json::from_slice::<Res<Receipt>>(data)?
                .data
                .try_into()?,
            31 => serde_json::from_slice::<Res<ReceiptCorrection>>(data)?
                .data
                .try_into()?,
            4 => serde_json::from_slice::<Res<Bso>>(data)?.data.try_into()?,
            41 => serde_json::from_slice::<Res<BsoCorrection>>(data)?
                .data
                .try_into()?,
            5 => serde_json::from_slice::<Res<CloseShift>>(data)?
                .data
                .try_into()?,
            6 => serde_json::from_slice::<Res<CloseArchive>>(data)?
                .data
                .try_into()?,
            _ => return Err(Error::ParseError),
        })
    }
}

#[cfg(test)]
mod test {
    use fiscal_data::Object;

    use crate::{ofd::Provider, Config};

    #[test]
    fn test() {
        for s in [&include_bytes!("../../test_data/private1_1.json")[..], &include_bytes!("../../test_data/private1_2.json")[..]] {
            tokio_test::block_on(async {
                super::Private1::new("")
                    .parse(&Config::default(), s, Object::new())
                    .as_mut()
                    .await
                    .unwrap();
            });
        }
    }
}
