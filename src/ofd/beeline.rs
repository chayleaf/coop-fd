use crate::{Company, Config, Item, Operation, Receipt};
use serde::{de::Visitor, Deserialize};
use super::{NdsType, Provider, PaymentType, ProductType};

#[allow(dead_code)]
#[repr(u8)]
enum Format {
    Png = 1,
    Pdf = 2,
    Json = 4,
}

/// Used by the seller to store some data (?)
#[derive(Clone, Debug, Default, Deserialize)]
#[serde(default, rename_all = "camelCase")]
struct Property {
    property_name: String,
    property_value: String,
}

#[repr(u8)]
#[derive(Copy, Clone, Debug, Default, Deserialize, Hash, Eq, PartialEq)]
#[serde(from = "u8")]
enum AgentType {
    #[default]
    Unknown = 255,
    // БАНК. ПЛ. АГЕНТ
    BankPaymentAgent = 0,
    // БАНК. ПЛ. СУБАГЕНТ
    BankPaymentSubagent = 1,
    // ПЛ. АГЕНТ
    PaymentAgent = 2,
    // ПЛ. СУБАГЕНТ
    PaymentSubagent = 3,
    // ПОВЕРЕННЫЙ
    Attorney = 4,
    // КОМИССИОНЕР (see: договор комиссии)
    Commissioner = 5,
    // АГЕНТ
    Agent = 6,
}

impl From<u8> for AgentType {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::BankPaymentAgent,
            1 => Self::BankPaymentSubagent,
            2 => Self::PaymentAgent,
            3 => Self::PaymentSubagent,
            4 => Self::Attorney,
            5 => Self::Commissioner,
            6 => Self::Agent,
            _ => Self::Unknown,
        }
    }
}

#[repr(u8)]
#[derive(Copy, Clone, Debug, Default, Deserialize, Hash, Eq, PartialEq)]
#[serde(from = "u8")]
enum TaxationType {
    #[default]
    Unknown = 255,
    // ОСН
    Osn = 0,
    // УСН доход
    UsnIncome = 1,
    // УСН доход - расход
    UsnIncomeExpense = 2,
    // ЕНВД
    Envd = 3,
    // ЕСХН
    Eshn = 4,
    // Патент
    Patent = 5,
}

impl From<u8> for TaxationType {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::Osn,
            1 => Self::UsnIncome,
            2 => Self::UsnIncomeExpense,
            3 => Self::Envd,
            4 => Self::Eshn,
            5 => Self::Patent,
            _ => Self::Unknown,
        }
    }
}

#[repr(u8)]
#[derive(Copy, Clone, Debug, Default, Deserialize, Hash, Eq, PartialEq)]
#[serde(from = "u8")]
enum FnsStatusCode {
    #[default]
    Unknown = 0,
    // В процессе
    InProgress = 1,
    // Отправлен
    Sent = 2,
    // Предупреждение
    Warning = 3,
    // Ошибка
    Error = 4,
}

impl From<u8> for FnsStatusCode {
    fn from(value: u8) -> Self {
        match value {
            1 => Self::InProgress,
            2 => Self::Sent,
            3 => Self::Warning,
            4 => Self::Error,
            _ => Self::Unknown,
        }
    }
}

#[repr(u8)]
#[derive(Copy, Clone, Debug, Default, Deserialize, Hash, Eq, PartialEq)]
#[serde(from = "u8")]
enum CrptStatus {
    #[default]
    Unknown = 0,
    // Отправлено в ЦРПТ
    Success = 1,
    // Ошибка ЦРПТ
    Error = 2,
}

impl From<u8> for CrptStatus {
    fn from(value: u8) -> Self {
        match value {
            1 => Self::Success,
            2 => Self::Error,
            _ => Self::Unknown,
        }
    }
}

#[derive(Clone, Debug, Default, Deserialize)]
#[serde(default)]
struct ProductCodeNew {
    // КТ Ф.3
    #[serde(rename = "markedCodeF3")]
    marked_code_f3: Option<String>,
    // КТ Ф.6
    #[serde(rename = "markedCodeF6")]
    marked_code_f6: Option<String>,
    // ГС1?
    #[serde(rename = "markedCodeGs1m")]
    marked_code_gs1m: Option<String>,
}

#[derive(Clone, Debug, Default)]
struct Phone(pub Vec<String>);

struct PhoneVis;
impl<'de> Visitor<'de> for PhoneVis {
    type Value = Phone;
    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a phone number")
    }
    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(Phone(vec![v.to_owned()]))
    }
    fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(Phone(vec![v]))
    }
    fn visit_borrowed_str<E>(self, v: &'_ str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(Phone(vec![v.to_owned()]))
    }
    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::SeqAccess<'de>,
    {
        let mut v = seq.size_hint().map(Vec::with_capacity).unwrap_or_default();
        while let Some(item) = seq.next_element()? {
            v.push(item);
        }
        Ok(Phone(v))
    }
}

impl<'de> Deserialize<'de> for Phone {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_any(PhoneVis)
    }
}

#[derive(Clone, Debug, Default, Deserialize)]
#[serde(default, rename_all = "camelCase")]
struct ProviderData {
    // Телефон поставщика
    provider_phone: Option<Phone>,
    // Наименование поставщика
    provider_name: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize)]
#[serde(default, rename_all = "camelCase")]
struct PaymentAgentData {
    // Телефон оп.перевода
    agent_phone: Option<Phone>,
    // Оп. агента
    bank_agent_operation: Option<String>,
    // Телефон пл. агента
    bank_agent_phone: Option<Phone>,
    // Телефон оп. пр. платежа
    payment_agent_phone: Option<Phone>,
    // operator
    operator_transfer_name: Option<String>,
    // Адр. оператора
    payment_provider_address: Option<String>,
    // ИНН оп. перевода
    operator_transfer_inn: Option<String>,
}

/// Single item
#[derive(Clone, Debug, Default, Deserialize)]
#[serde(default, rename_all = "camelCase")]
struct BlItem {
    // checking for milk products/etc (e.g. [М+])
    checking_result: Option<String>,
    name: String,
    // Цена
    price: u64,
    // Количество
    quantity: f64,
    // кг, шт., etc
    unit: String,
    // Сумма
    sum: u64,
    nds: Option<NdsType>,
    nds_sum: Option<u64>,
    // Код товара
    product_code: Option<String>,
    // Код товара (but better)
    product_code_new: Vec<ProductCodeNew>,
    // Контр. код КМ
    control_code: Option<String>,
    // Дробь (never seen this in use)
    fractional_quantity: Option<u64>,
    fractional_part: Option<u64>,
    // Код страны
    origin_country_code: Option<String>,
    // Декларация
    custom_entry_num: Option<String>,
    // Акциз
    excise_duty: Option<String>,
    payment_type: Option<PaymentType>,
    product_type: Option<ProductType>,
    // Признак агента по предмету расчета (comma-separated)
    payment_agent_by_product_type: Option<String>,
    provider_data: Option<ProviderData>,
    payment_agent_data: Option<PaymentAgentData>,
    provider_inn: Option<String>,
    // Итоговая сумма предоплатами (авансами)
    prepaid_sum: Option<u64>,
    // Итоговая сумма постоплатами (кредитами)
    credit_sum: Option<u64>,
    // Итоговая сумма встречными предоставлениями
    provision_sum: Option<u64>,
}

// this is non-exhaustive and only maid for the buyer receipt check API /shrug
// this has mixed snake and camel (mostly camel) case, so I can't rename_all
#[derive(Clone, Debug, Default, Deserialize)]
#[serde(default)]
#[allow(non_snake_case)]
struct Rec {
    // Код формы ФД
    code: u8,
    // Stringified taxationType
    taxationTypeName: String,
    // Версия ФФД; 2 = 1.05, 3 = 1.1, 4 = 1.2
    fiscalDocumentFormatVer: Option<u8>,
    // № ФН / fn
    fiscalDriveNumber: String,
    // Регистрационный номер ККТ / kkt, has trailing ws
    kktRegId: String,
    // ИНН / seller inn, has trailing ws
    userInn: Option<String>,
    // № ФД / fd
    fiscalDocumentNumber: u32,
    // Дата | Время / unix ts (utc)
    #[serde(with = "chrono::serde::ts_seconds")]
    dateTime: chrono::DateTime<chrono::Utc>,
    // ФП (fp)
    fiscalSign: u64,
    // № смены
    shiftNumber: u32,
    // №
    requestNumber: u32,
    // t
    operationType: Option<Operation>,
    // ККТ для Интернет (see below comment on signs)
    internetSign: Option<u8>,
    items: Vec<BlItem>,
    buyer: Option<String>,
    buyerInn: Option<String>,
    // Адрес покупателя
    buyerAddress: Option<String>,
    // Телефон или эл. адрес покупателя
    buyerPhoneOrAddress: Option<String>,
    // Адрес покупателя
    buyerInformation: Option<String>,
    // used to store misc info associated with a receipt
    properties: Vec<Property>,
    // Эл. адрес отправителя
    sellerAddress: Option<String>,
    // № АВТ
    machineNumber: Option<String>,
    // Итоговая сумма наличными
    cashTotalSum: Option<u64>,
    // Итоговая сумма безналичными
    ecashTotalSum: Option<u64>,
    // Итоговая сумма предоплатами (авансами)
    prepaidSum: Option<u64>,
    // Итоговая сумма постоплатами (кредитами)
    creditSum: Option<u64>,
    // Итоговая сумма встречными предоставлениями
    provisionSum: Option<u64>,
    // Общая итоговая сумма
    totalSum: u64,
    // Сайт ФНС
    fnsURL: Option<String>,
    // Место расчетов (in case of online purchases it's a URL to the seller)
    retailPlace: Option<String>,
    // Тип коррекции: 0 = Самостоятельная, 1 = По предписанию
    correctionType: Option<u8>,
    // // Дата коррекции; Номер предписания налог.органа
    // correctionBase: Vec<{correctionDocumentDate: u64, correctionDocumentNumber: u32}>
    // taxes (in kopeyka)
    // НДС 10%
    nds10: Option<u64>,
    // НДС 18%
    nds18: Option<u64>,
    // НДС 20%
    nds20: Option<u64>,
    // НДС 10/110
    ndsCalculated10: Option<u64>,
    // НДС 18/118
    ndsCalculated18: Option<u64>,
    // НДС 20/120
    ndsCalculated20: Option<u64>,
    // ИТОГО c НДС 0%
    nds0: Option<u64>,
    // ИТОГО без НДС
    ndsNo: Option<u64>,
    // НДС 10/110
    nds10110: Option<u64>,
    // НДС 20/120
    nds20120: Option<u64>,
    // client = seller (company name)
    client_name: String,
    // shown between client_name and retailPlaceAddress if present
    user: Option<String>,
    // Адрес расчетов
    retailPlaceAddress: String,
    // Система налогооблажения (only taxationTypeName is used, which doesn't exist, maybe it's typo)
    taxationType: TaxationType,
    // КМ? (sic) if 1, whatever that is
    checkingLabeledResult: Option<u8>,
    // Кассир
    operator: String,
    // Телефон оп. перевода
    operatorPhoneToTransfer: Option<Phone>,
    // Операция агента
    bankAgentOperation: Option<String>,
    // Телефон пл. агента
    bankAgentPhone: Option<Phone>,
    // Телефон оп. пр. платежа
    paymentAgentPhone: Option<Phone>,
    // Оператор
    operatorTransferName: Option<String>,
    // Адрес оператора
    paymentProviderAddress: Option<String>,
    // ИНН оператора перевода
    operatorTransferInn: Option<String>,
    // Телефон поставщика
    providerPhone: Option<Phone>,
    // unused
    kktType: Option<String>,
    // unused
    ffd: Option<String>,
    // Заводской номер ККТ
    kktNumber: Option<String>,
    // Признак агента (actually, only paymentAgentTypeName is used, which doesn't exist, maybe it's a typo)
    paymentAgentType: Option<String>,
    // ИНН кассира
    operatorInn: Option<String>,
    // Signs = null or 1 boolean
    // Принтер в автомате
    printInMachineSign: Option<u8>,
    // АС БСО
    bsoSign: Option<u8>,
    // Подакцизные товары
    exciseDutyProductSign: Option<u8>,
    // ТМТ
    labeledProductSign: Option<u8>,
    // ККТ для услуг
    serviceSign: Option<u8>,
    // Проведение азартной игры
    gamblingSign: Option<u8>,
    // Проведение лотереи
    lotterySign: Option<u8>,
    // Ломбард
    pawnshopSign: Option<u8>,
    // Страхование
    insuranceSign: Option<u8>,
}

#[derive(Clone, Debug, Default, Deserialize)]
#[serde(default, rename_all = "camelCase")]
struct Json {
    receipt: Rec,
}

#[derive(Clone, Debug, Default, Deserialize)]
#[serde(default, rename_all = "camelCase")]
struct Res {
    has_result: bool,
    // // base64 png
    // png: Option<String>,
    // // base64 pdf
    // pdf: Option<String>,
    json: Option<Json>,
    // base64 png
    qr_code: Option<String>,
    // // Unused?
    // tlv: Option<()>,
    // // Unused?
    // fns_transfer: Option<()>,
    // // Unused?
    // fns_error: Option<()>,
    // Сообщение ФНС
    fns_error_message: Option<String>,
    // Ошибка ФЛК
    out_error_message: Option<String>,
    // Статус ЦРПТ
    crpt_transfer: Option<CrptStatus>,
    // Link to flocktory (for promos)
    flocktory_link: String,
    // Статус ФНС
    fns_status_display: Option<String>,
    fns_status_display_code: Option<FnsStatusCode>,
    // Получен ОФД (unix ts)
    #[serde(with = "chrono::serde::ts_seconds_option")]
    ofd_date: Option<chrono::DateTime<chrono::Utc>>,
    // Получен ФНС (unix ts)
    #[serde(with = "chrono::serde::ts_seconds_option")]
    fns_date: Option<chrono::DateTime<chrono::Utc>>,
}

async fn stage1(config: &Config, rec: &Receipt) -> reqwest::Result<Res> {
    let client = reqwest::Client::builder()
        // .user_agent("Mozilla/5.0 (Windows NT 10.0; rv:109.0) Gecko/20100101 Firefox/118.0")
        .cookie_store(true)
        // TODO: remove, I have no idea why this is needed
        .danger_accept_invalid_certs(true)
        .build()?;
    let url = format!(
        "https://ofd.beeline.ru/api/ofdcheck/checkf?fn={}&fp={}&fd={}&format=4",
        rec.r#fn, rec.fp, rec.i
    );
    log::info!("beeline: fetching from {url}");
    let res_text = client
        .execute(client.get(url).build()?)
        .await?
        .text()
        .await?;
    let Ok(res): Result<Res, _> = serde_json::from_str(&res_text) else {
        return Ok(Default::default());
    };
    if res.has_result {
        let mut path = config.data_path("raw/beeline");
        path.push(rec.fnifp().unwrap_or_default() + ".json");
        if let Err(err) = tokio::fs::write(path, &res_text).await {
            log::error!("failed to write raw receipt: {err:?}");
        }
    } else {
        log::warn!("no result");
    }
    Ok(res)
}

fn stage2(res: Res, rec: &mut Receipt) -> Option<()> {
    let res = res.json?.receipt;
    rec.company = Company {
        name: res.client_name,
        inn: res.userInn.unwrap_or_default(),
    };
    rec.total = res.totalSum;
    rec.total_cash = res.cashTotalSum.unwrap_or_default();
    rec.total_card = res.ecashTotalSum.unwrap_or_default();
    rec.total_tax = res
        .items
        .iter()
        .map(|x| x.nds_sum.unwrap_or_default())
        .sum();
    rec.r#fn = res.fiscalDriveNumber;
    rec.fp = res.fiscalSign.to_string();
    rec.i = res.fiscalDocumentNumber.to_string();
    // what is rec.n? i forgor :skull:
    rec.date = res
        .dateTime
        .with_timezone(crate::TZ.get().unwrap_or(&chrono_tz::UTC))
        .format("%Y%m%dT%H%M")
        .to_string();
    rec.items = res
        .items
        .into_iter()
        .map(|item| Item {
            name: item.name,
            id: item
                .product_code_new
                .into_iter()
                .next()
                .and_then(|c| c.marked_code_gs1m.or(c.marked_code_f6).or(c.marked_code_f3))
                .or(item.product_code)
                .unwrap_or_default(),
            count: item.quantity,
            unit: item.unit,
            per_item: item.price,
            total: item.sum,
            tax: item.nds_sum.unwrap_or_default(),
        })
        .collect();
    Some(())
}

pub(crate) async fn fetch(config: &'static Config, mut rec: Receipt) -> Option<Receipt> {
    let res = match stage1(config, &rec).await {
        Ok(x) => x,
        Err(err) => {
            log::error!("beeline error: {err}");
            return None;
        }
    };
    stage2(res, &mut rec)?;
    Some(rec)
}

pub struct Beeline;
impl Provider for Beeline {
    fn id(&self) -> &'static str {
        "beeline"
    }
    fn name(&self) -> &'static str {
        "ПАО \"Вымпел-Коммуникации\""
    }
    fn url(&self) -> &'static str {
        "ofd.beeline.ru"
    }
    fn exts(&self) -> &'static [&'static str] {
        &["json"]
    }
    fn inn(&self) -> &'static str {
        "7713076301"
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test() {
        let test_data = include_str!("../../test_data/beeline1.json");
        let res: super::Res = serde_json::from_str(test_data).unwrap();
        assert!(res.has_result);
        let mut rec = crate::Receipt::default();
        super::stage2(res, &mut rec).unwrap();
        assert_eq!(rec.items.len(), 18);
    }
}
