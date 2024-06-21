use async_trait::async_trait;
use fiscal_data::{enums, fields, Document, Object, TlvType};
use serde::Deserialize;

use crate::Config;

use super::{custom, fill_missing_fields, Error, Provider};

#[derive(Copy, Clone, Debug, Default, Deserialize, Hash, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
struct SentMessage {
    fd_id: u64,
}

#[derive(Copy, Clone, Debug, Default, Deserialize, Hash, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
struct FnsStatus {
    #[serde(rename = "sentTimeStamp")]
    sent_timestamp: chrono::DateTime<chrono::Utc>,
    #[serde(rename = "sentResult")]
    sent_result: bool,
    #[serde(rename = "codeResult")]
    code_result: u8,
    #[serde(rename = "sentMessage")]
    sent_message: SentMessage,
}

#[derive(Clone, Debug, Default, Deserialize, Hash, Eq, PartialEq)]
struct Header {
    #[serde(rename = "Signature")]
    signature: Vec<u8>,
    #[serde(rename = "VersionS")]
    version_s: Vec<u8>,
    #[serde(rename = "VersionP")]
    version_p: Vec<u8>,
    #[serde(rename = "NumberFN")]
    number_fn: String,
    #[serde(rename = "BodySize")]
    body_size: u64,
    #[serde(rename = "Flags")]
    flags: u32,
    #[serde(rename = "VerificationCode")]
    verification_code: u32,
}

#[derive(Clone, Debug, Default, Deserialize, Hash, Eq, PartialEq)]
#[serde(rename_all = "PascalCase")]
struct Container {
    #[serde(rename = "MessageType")]
    message_type: u16,
    #[serde(rename = "Priority")]
    priority: u32,
    #[serde(rename = "Functionality")]
    functionality: u32,
    #[serde(rename = "FNnumber")]
    fn_number: String,
    #[serde(rename = "DocumentNumber")]
    document_number: u32,
    // message header
    #[serde(rename = "Header21", with = "fiscal_data::json::base64_vec")]
    header_21: Vec<u8>,
    // #[serde(rename = "ClearMessage")]
    // clear_message: Option<serde_json::Value>,
    // message fiscal sign
    #[serde(rename = "Fs21", with = "fiscal_data::json::base64_array")]
    fs_21: [u8; 8],
}

#[derive(Clone, Debug, Deserialize)]
#[allow(unused)]
struct Doc {
    #[serde(rename = "$id")]
    id: String,
    #[serde(rename = "$sessionID")]
    session_id: String,
    #[serde(rename = "$code")]
    code: u16,
    #[serde(rename = "$header")]
    header: Header,
    #[serde(rename = "$container")]
    container: Container,
    #[serde(rename = "$serverTimeStamp")]
    #[serde(with = "chrono::serde::ts_seconds")]
    server_timestamp: chrono::DateTime<chrono::Utc>,
    // ffd version
    version: String,
    #[serde(rename = "$sentTimeStamp")]
    #[serde(with = "chrono::serde::ts_seconds")]
    sent_timestamp: chrono::DateTime<chrono::Utc>,
    #[serde(rename = "$regRetailAddress")]
    reg_retail_address: String,
    #[serde(rename = "$regRetailPlace")]
    reg_retail_place: String,
    #[serde(rename = "$regUser")]
    reg_user: String,
    #[serde(rename = "$regUserINN")]
    reg_user_inn: String,
    #[serde(rename = "$doubles")]
    doubles: u64,
    #[serde(rename = "$final")]
    r#final: bool,
    #[serde(rename = "$addedClickhouseResult")]
    added_clickhouse_result: bool,
    document: fiscal_data::json::Document,
}

#[derive(Clone, Debug, Default, Deserialize)]
#[allow(unused)]
struct Res {
    #[serde(rename = "giftLink")]
    gift_link: Option<String>,
    #[serde(rename = "giftLink2")]
    gift_link_2: Option<String>,
    #[serde(rename = "fnsStatus")]
    fns_status: Option<FnsStatus>,
    #[serde(rename = "Doc")]
    doc: Option<Doc>,
}

#[derive(Clone, Debug, Default, Deserialize)]
struct Response {
    #[allow(unused)]
    ok: bool,
    result: Option<Res>,
}

pub struct Astral;
#[async_trait]
impl Provider for Astral {
    fn id(&self) -> &'static str {
        "astral"
    }
    fn name(&self) -> &'static str {
        "ЗАО \"КАЛУГА АСТРАЛ\""
    }
    fn url(&self) -> &'static str {
        "ofd.astralnalog.ru"
    }
    fn exts(&self) -> &'static [&'static str] {
        &["json"]
    }
    fn inn(&self) -> &'static str {
        "4029017981"
    }
    async fn fetch_raw_data(&self, config: &Config, rec: &mut Object) -> Result<Vec<u8>, Error> {
        let drive_num = rec
            .get::<fields::DriveNum>()?
            .ok_or(Error::MissingData("fn"))?;
        let [_, _, a, b, c, d] = rec
            .get::<fields::DocFiscalSign>()?
            .ok_or(Error::MissingData("fp"))?;
        let fiscal_sign = u32::from_be_bytes([a, b, c, d]);
        let doc_num = rec
            .get::<fields::DocNum>()?
            .ok_or(Error::MissingData("fd"))?;
        let client = reqwest::Client::builder()
            .user_agent("Mozilla/5.0 (Windows NT 10.0; rv:109.0) Gecko/20100101 Firefox/118.0")
            .build()?;
        let ret = client
            .execute(
                client
                    .post("https://ofd.astralnalog.ru/api/v4.2/landing.getReceipt")
                    .multipart(
                        reqwest::multipart::Form::new()
                            .text("fiscalDriveNumber", drive_num.to_string())
                            .text("fiscalDocumentNumber", doc_num.to_string())
                            .text("fiscalSign", fiscal_sign.to_string())
                            .text("recaptcha_response", "a"),
                    )
                    .build()?,
            )
            .await?
            .bytes()
            .await?
            .to_vec();
        #[cfg(not(debug_assertions))]
        {
            let Ok(res) = serde_json::from_slice::<Response>(&ret) else {
                return Err(Error::ParseError);
            };
            if !res.ok || res.result.is_none() {
                return Err(Error::MissingData("result"));
            }
        }
        if let Some(provider) = {
            let x = super::registry()
                .await
                .by_id("", rec)
                .find(|x| x.id() != self.id());
            x
        } {
            let _ = super::fetch_raw(config, provider, rec, false).await;
        }
        Ok(ret)
    }
    async fn parse(&self, config: &Config, data: &[u8], rec: Object) -> Result<Document, Error> {
        let res = serde_json::from_slice::<Response>(data)?;
        let res = res.result.ok_or(Error::MissingData("result"))?;
        let doc = res.doc.ok_or(Error::MissingData("doc"))?;
        let parsed = if let Some(provider) = {
            let x = super::registry()
                .await
                .by_id("", &rec)
                .find(|x| x.id() != self.id());
            x
        } {
            super::fetch2(config, provider, rec).await.ok()
        } else {
            None
        };
        let mut ret = match if let Some(data) = doc.document.raw_data() {
            Document::from_bytes(data.to_vec()).or_else(|_| doc.document.clone().try_into())
        } else {
            doc.document.clone().try_into()
        } {
            Ok(x) => x,
            Err(err) => {
                return parsed.ok_or_else(|| err.into());
            }
        };
        let data = ret.data_mut();
        if let Some(parsed) = parsed {
            fill_missing_fields(data, parsed.data());
        }
        data.set::<custom::Id>(doc.id)?;
        data.set::<custom::SessionId>(doc.session_id)?;
        if let Some(fns_status) = res.fns_status {
            data.set::<custom::FdId>(fns_status.sent_message.fd_id)?;
        }
        let mut fallback = Object::new();
        if let Some(ver) = match doc.version.as_str() {
            "1" | "1.0" => Some(enums::FfdVersion::V1),
            "1.05" => Some(enums::FfdVersion::V1_05),
            "1.1" => Some(enums::FfdVersion::V1_1),
            "1.2" => Some(enums::FfdVersion::V1_2),
            _ => None,
        } {
            fallback.set::<fields::KktFfdVer>(ver)?;
        }
        fallback.set::<fields::RetailPlaceAddress>(doc.reg_retail_address)?;
        fallback.set::<fields::RetailPlace>(doc.reg_retail_place)?;
        fallback.set::<fields::User>(doc.reg_user)?;
        fallback.set::<fields::UserInn>(doc.reg_user_inn)?;
        fill_missing_fields(data, &fallback);
        if ret.container_header().is_none() {
            ret.set_container_header(doc.container.header_21)?;
        }
        if ret.message_fiscal_sign().is_none() {
            ret.set_message_fiscal_sign(doc.container.fs_21);
        }
        Ok(ret)
    }
}

#[cfg(test)]
mod test {
    use fiscal_data::{Document, TlvType};

    use super::Response;
    #[test]
    fn test() {
        let test_data = include_bytes!("../../test_data/astral1.json");
        let fns = include_bytes!("../../test_data/astral1_fns.json");
        let res = serde_json::from_slice::<Response>(test_data).unwrap();
        let fiscal_data::json::Document::Receipt(doc) = res.result.unwrap().doc.unwrap().document
        else {
            panic!()
        };
        let fns = serde_json::from_slice::<fiscal_data::json::Document>(fns).unwrap();
        let mut fns = Document::try_from(fns).unwrap();
        let mut raw = Document::from_bytes(doc.raw_data.clone().unwrap()).unwrap();
        let mut doc = Document::try_from(doc).unwrap();
        for doc in [&mut raw, &mut fns, &mut doc] {
            for field in [1008, 1044, 1048, 1077, 1117, 1187, 1203, 1209] {
                doc.data_mut().set_raw(field, &[]);
            }
        }
        assert_eq!(raw.data(), fns.data());
        assert_eq!(doc.data(), raw.data());
    }
}
