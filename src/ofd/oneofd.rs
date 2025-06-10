use async_trait::async_trait;
use fiscal_data::{enums::FfdVersion, fields, Document, Object};
use serde::Deserialize;

use crate::server::State;

use super::{custom, fill_missing_fields, Error, Provider};

/*
#[derive(Clone, Debug, Default, Deserialize, Hash, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
struct Agent {
    operator_phone_to_transfer: Vec<String>,
    payment_agent_phone: Vec<String>,
    operator_phone_to_receive: Vec<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Hash, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
struct ItemOptions {
    quantity: String,
    name: String,
    sum: String,
    nds_rate: String,
    calculation_subject_sign: String,
    calculation_type_sign: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
struct TaxLayout {
    r#type: String,
    printed_name: String,
    rate: f64,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
struct ItemTax {
    layout: TaxLayout,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
struct ReceiptTax {
    layout: TaxLayout,
    sum: f64,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
struct TaxInfo {
    rate: f64,
    name: String,
    sum: f64,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
struct Item {
    options: ItemOptions,
    price: u64,
    quantity: u32,
    nds_rate: u32,
    taxes: Vec<ItemTax>,
    calculation_subject_sign: u32,
    calculation_type_sign: u32,
    name: String,
    sum: u64,
    industry_receipt_requisite: Vec<serde_json::Value>,
}*/

#[derive(Clone, Debug, Default, Deserialize, Hash, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
struct FnsStatus {
    //status: String,
    confirmation: String,
    /*confirmation_date: String,
    receiver: String,*/
}

/*#[derive(Clone, Debug, Default, Deserialize, Hash, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
struct Aggregation {
    sum: String,
    cash_sum: String,
    ecash_sum: String,
    prepayment_sum: String,
    postpayment_sum: String,
    countersubmission_sum: String,
    prepayment_100: String,
}

#[derive(Clone, Debug, Default, Deserialize, Hash, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
struct BuyerNotificationStatus {
    to: String,
    from: String,
    sent_at: String,
    carrier: String,
}*/

#[derive(Clone, Debug, Default, Deserialize, Hash, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
struct ReceiptExtras {
    fns_status: FnsStatus,
    /*aggregation: Aggregation,
    buyer_notification_status: BuyerNotificationStatus,*/
}

/*#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
struct Payment {
    sum: f64,
    payment_type: String,
}*/

#[derive(Clone, Debug, Default, Deserialize, Hash, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
struct ReceiptOptions {
    /*buyer_address: String,
    retail_place_address: String,
    date_time: String,
    user_inn: String,
    total_sum: String,
    cash_total_sum: String,
    machine_number: String,
    kkt_reg_id: String,
    shift_number: String,
    fiscal_document_number: String,
    fiscal_drive_number: String,
    request_number: String,
    user: String,
    operation_type: String,
    taxation_type: String,
    fns_url: String,
    fiscal_sign: String,
    ecash_total_sum: String,
    nds18: String,
    nds0: String,
    retail_place: String,*/
    protocol_version: String,
    /*prepayment_sum: String,
    postpayment_sum: String,
    counter_submission_sum: String,*/
    extras: ReceiptExtras,
    /*sender_address: String,
    as_of: String,
    kkt_number: String,
    org_id: String,*/
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
struct Ticket {
    /*request_payload: Option<serde_json::Value>,
    response_payload: Option<serde_json::Value>,
    accepted_at: chrono::DateTime<chrono::Utc>,
    /// UTC time of arrival to OFD servers?
    inserted_at: chrono::DateTime<chrono::Utc>,*/
    #[serde(with = "fiscal_data::json::base64_array")]
    fs_21: [u8; 8],
    #[serde(with = "fiscal_data::json::base64_vec")]
    header_21: Vec<u8>,
    /*protocol_version: u32,
    user_inn: String,
    /// Receipt number in the shift
    request_number: u32,
    provider_phone: Vec<serde_json::Value>,
    agent: Agent,
    operator_phone_to_transfer: Vec<String>,
    payment_agent_phone: Vec<String>,
    operator_phone_to_receive: Vec<String>,
    buyer_information: serde_json::Value,
    industry_receipt_requisite: Vec<serde_json::Value>,
    items: Vec<Item>,
    operation_type: u32,
    taxation_type: u32,
    user: String,
    retail_place_address: String,
    machine_number: String,
    retail_place: String,
    fns_url: String,
    buyer_address: String,
    qr_code: String,
    eligible_for_nds20: bool,
    kkt_reg_id: String,
    header_21_bytes: Option<serde_json::Value>,
    fs_21_bytes: Option<serde_json::Value>,
    shift_number: u32,
    field_1209: u32,
    fiscal_id: String,
    /// FN
    fiscal_document_number: u32,
    /// FD
    fiscal_drive_number: String,
    /// KKT id
    kkm_id: String,
    /// Local time
    //#[serde(with = "fiscal_data::json::as_localtime")]
    transaction_date: chrono::NaiveDateTime,
    transaction_id: String,*/
    options: ReceiptOptions,
    /*r#type: String,
    counter_submission_sum: f64,
    cash_total_sum: f64,
    ecash_total_sum: f64,
    prepayment_sum: f64,
    postpayment_sum: f64,
    payments: Vec<Payment>,
    total_sum: f64,
    taxes: Vec<ReceiptTax>,
    nds20: f64,
    nds0: f64,
    total_tax: f64,*/
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
struct Container {
    ticket: Ticket,
    /*ticket_description: String,
    org_title: String,
    org_id: String,
    retail_place_address: String,
    kkm_fns_id: String,
    found_date: String,
    taxes: Vec<TaxInfo>,
    kpp: String,
    owned: bool,*/
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
struct FnsRes {
    fd_id: String,
}

pub struct OneOfd;
#[async_trait]
impl Provider for OneOfd {
    fn id(&self) -> &'static str {
        "1ofd"
    }
    fn name(&self) -> &'static str {
        "АО \"Энергетические системы и коммуникации\""
    }
    fn url(&self) -> &'static str {
        "1-ofd.ru"
    }
    fn exts(&self) -> &'static [&'static str] {
        &["json"]
    }
    fn inn(&self) -> &'static str {
        "7709364346"
    }
    async fn fetch_raw_data(&self, state: &State, rec: &mut Object) -> Result<Vec<u8>, Error> {
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
        let operation_type = rec
            .get::<fields::PaymentType>()?
            .ok_or(Error::MissingData("operationType"))?;
        let sum = rec
            .get::<fields::TotalSum>()?
            .ok_or(Error::MissingData("sum"))?;
        let date = rec
            .get::<fields::DateTime>()?
            .ok_or(Error::MissingData("date"))?;
        let client = reqwest::Client::builder().build()?;
        let ret = client
            .execute(
                client
                    .get(format!(
                        "https://consumer.1-ofd.ru/api/tickets/ticket/t={}&s={}.{}&fn={}&i={}&fp={}&n={}",
                        date.format("%Y%m%dT%H%M"),
                        sum / 100, sum % 100,
                        drive_num,
                        doc_num,
                        fiscal_sign,
                        operation_type as u32,
                    ))
                    .build()?,
            )
            .await?
            .bytes()
            .await?
            .to_vec();
        #[cfg(not(debug_assertions))]
        {
            let Ok(_res) = serde_json::from_slice::<Container>(&ret) else {
                return Err(Error::ParseError);
            };
        }
        if let Some(provider) = {
            let x = super::registry()
                .await
                .default(rec)
                .find(|x| x.id() != self.id());
            x
        } {
            let _ = super::fetch_raw(state, &*provider, rec, false).await;
        }
        Ok(ret)
    }
    async fn parse(&self, state: &State, data: &[u8], rec: Object) -> Result<Document, Error> {
        let res = serde_json::from_slice::<Container>(data)?;
        let mut ret = if let Some(provider) = {
            let x = super::registry()
                .await
                .default(&rec)
                .find(|x| x.id() != self.id());
            x
        } {
            super::fetch2(state, &*provider, rec).await?
        } else {
            return Err(Error::MissingData("fallback data"));
        };
        let data = ret.data_mut();
        if let Ok(res) =
            serde_json::from_str::<FnsRes>(&res.ticket.options.extras.fns_status.confirmation)
        {
            if let Ok(x) = res.fd_id.parse() {
                data.set::<custom::FdId>(x)?;
            }
        }
        let mut fallback = Object::new();
        if let Ok(ver) = res.ticket.options.protocol_version.parse::<u8>() {
            let ver = FfdVersion::from(ver);
            if ver != FfdVersion::Unknown {
                fallback.set::<fields::KktFfdVer>(ver)?;
            }
        }
        fill_missing_fields(data, &fallback);
        ret.set_container_header(res.ticket.header_21)?;
        ret.set_message_fiscal_sign(res.ticket.fs_21);
        Ok(ret)
    }
}

#[cfg(test)]
mod test {
    //use fiscal_data::{fields, Document, TlvType};

    use super::Container;
    #[test]
    fn test() {
        let test_data = include_bytes!("../../test_data/1ofd1.json");
        //let fns = include_bytes!("../../test_data/1ofd1_fns.json");
        let _res = serde_json::from_slice::<Container>(test_data).unwrap();
        // let rec = serde_json::from_slice::<fiscal_data::json::Document>(fns).unwrap();
        // let rec = Document::try_from(rec).unwrap();
    }
}
