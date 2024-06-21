use async_trait::async_trait;
use chrono::NaiveDateTime;
use fiscal_data::{enums, fields, Ffd, FfdDoc, Object};
use serde::Deserialize;

use super::{Error, Provider};
use crate::Config;

#[repr(u8)]
#[derive(Copy, Clone, Debug, Default, Deserialize, Hash, Eq, PartialEq)]
#[serde(from = "u8")]
enum SchemeWarnings {
    #[default]
    Unknown = 255,
    /// Нет нарушения
    NoWarnings = 0,
    /// Предупреждение о максимальной длине документа
    MaxDocLengthWarning = 1,
    /// Предупреждение о минимальной длине документа
    MinDocLengthWarning = 2,
    /// Предупреждение о фиксированной длине документа
    FixedDocLengthWarning = 3,
}

impl From<u8> for SchemeWarnings {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::NoWarnings,
            1 => Self::MaxDocLengthWarning,
            2 => Self::MinDocLengthWarning,
            3 => Self::FixedDocLengthWarning,
            _ => Self::Unknown,
        }
    }
}

#[derive(Clone, Debug, Default, Ffd, Deserialize)]
#[serde(default)]
// some fields have underscores, so we can't use rename_all
#[allow(non_snake_case)]
struct ProductCode {
    #[ffd(tag = fields::KtN)]
    Code_Undefined: Option<String>,
    #[ffd(tag = fields::KtEan8)]
    Code_EAN_8: Option<String>,
    #[ffd(tag = fields::KtEan13)]
    Code_EAN_13: Option<String>,
    #[ffd(tag = fields::KtItf14)]
    Code_ITF_14: Option<String>,
    #[ffd(tag = fields::KtGs1_0)]
    Code_GS_1: Option<String>,
    #[ffd(tag = fields::KtGs1M)]
    Code_GS_1M: Option<String>,
    #[ffd(tag = fields::KtKmk)]
    Code_KMK: Option<String>,
    #[ffd(tag = fields::KtMi)]
    Code_MI: Option<String>,
    #[ffd(tag = fields::KtEgais2_0)]
    Code_EGAIS_2: Option<String>,
    #[ffd(tag = fields::KtEgais3_0)]
    Code_EGAIS_3: Option<String>,
    #[ffd(tag = fields::KtF1)]
    Code_F_1: Option<String>,
    #[ffd(tag = fields::KtF2)]
    Code_F_2: Option<String>,
    #[ffd(tag = fields::KtF3)]
    Code_F_3: Option<String>,
    #[ffd(tag = fields::KtF4)]
    Code_F_4: Option<String>,
    #[ffd(tag = fields::KtF5)]
    Code_F_5: Option<String>,
    #[ffd(tag = fields::KtF6)]
    Code_F_6: Option<String>,
}

/// Extra user property
#[derive(Clone, Debug, Default, Deserialize, Ffd)]
#[serde(default, rename_all = "PascalCase")]
#[allow(non_snake_case)]
struct ExtraProp {
    #[ffd(tag = fields::AdditionalUserPropName)]
    name: String,
    #[ffd(tag = fields::AdditionalUserPropValue)]
    value: String,
}

/// Single item
#[derive(Clone, Debug, Default, Deserialize, Ffd)]
#[serde(default)]
// some fields have underscores, so we can't use rename_all
#[allow(non_snake_case)]
struct OfdItem {
    /// Наименование позиции
    #[ffd(tag = fields::ItemName)]
    Name: Option<String>,
    /// Цена позиции (в копейках)
    #[ffd(tag = fields::ItemUnitPrice)]
    Price: u64,
    /// Количество единиц товарной позиции
    #[ffd(tag = fields::ItemQuantity)]
    Quantity: f64,
    // this is legacy bullshit, but why not support it i guess...
    /// Сумма НДС по предмету расчета со ставкой 20% в копейках 7) / 1102
    #[ffd(tag = fields::TotalVat20Sum)]
    Nds18_TotalSumm: Option<u64>,
    /// Сумма удерживаемого налога на добавленную стоимость (НДС) по ставку 10%, начисленная за смену (в копейках) / 1103
    #[ffd(tag = fields::TotalVat10Sum)]
    Nds10_TotalSumm: Option<u64>,
    /// Сумма по операциям, облагаемая НДС по ставке 0%, накопленная за смену (в копейках) / 1104
    #[ffd(tag = fields::TotalSumWithVat0)]
    Nds00_TotalSumm: Option<u64>,
    /// Сумма по операциям, не облагаемая НДС, накопленная за смену (в копейках) / 1105
    #[ffd(tag = fields::TotalSumWithNoVat)]
    NdsNA_TotalSumm: Option<u64>,
    /// Сумма удерживаемого налога на добавленную стоимость (НДС) по ставке в 20/120 8), начисленная за смену, в копейках / 1106
    #[ffd(tag = fields::TotalVat20_120Sum)]
    Nds18_CalculatedTotalSumm: Option<u64>,
    /// Сумма удерживаемого налога на добавленную стоимость (НДС) по ставке 10/110, начисленная за смену, в копейках / 1107
    #[ffd(tag = fields::TotalVat10_110Sum)]
    Nds10_CalculatedTotalSumm: Option<u64>,
    /// Стоимость по позиции (в копейках)
    #[ffd(tag = fields::ItemTotalPrice)]
    Total: u64,
    /// Cкидка/наценка / 1112
    // this is NOT actually 1112, at least not the documented 1112
    // for more info on how this OFD handles it, I'd need more data, as of now
    // many fields are only mentioned in the changelog
    // #[ffd(tag = fields::Modifiers)]
    DiscountMarkup: Option<i64>,
    /// Дополнительные реквизиты пользователя
    Extra: Option<String>,
    // 1224? 1225? idk
    Supplier: Option<serde_json::Value>,
    /// Признак способа расчета / 1214
    #[ffd(tag = fields::PaymentMethod)]
    CalculationMethod: Option<enums::PaymentMethod>,
    /// Признак предмета расчета / 1212
    #[ffd(tag = fields::ItemType)]
    SubjectType: Option<enums::ItemType>,
    /// Единица измерения предмета расчета / 1197
    #[ffd(tag = fields::Unit)]
    UnitOfMeasure: Option<String>,
    /// Код товарной номенклатуры / 1162
    // is this base64?
    // #[ffd(tag = fields::ProductCode)]
    ProductNomenclature: Option<String>,
    /// Код маркировки
    #[ffd(tag = fields::ProductCodeNew)]
    ProductCode: Option<ProductCode>,
    /// Размер НДС за единицу предмета расчета / 1198
    #[ffd(tag = fields::ItemUnitVat)]
    NDS_PieceSumm: Option<u64>,
    /// Ставка НДС / 1199
    #[ffd(tag = fields::VatRate)]
    NDS_Rate: Option<enums::VatType>,
    /// Сумма НДС за предмет расчета / 1200
    #[ffd(tag = fields::ItemTotalVat)]
    NDS_Summ: Option<u64>,
    /// Дополнительные реквизиты
    AdditionalRequisite: Option<String>,
    /// Цифровой код страны происхождения товара в соответствии с Общероссийским классификатором стран мира / 1230
    #[ffd(tag = fields::OriginCountry)]
    OriginCountryCode: Option<String>,
    /// Номер таможенной декларации в соответствии с форматом, установленным решением Комиссии
    /// Таможенного союза от 20.05.2010 N 257 (в ред. 17.12.2019 N 223) «О форме декларации на
    /// товары и порядке ее заполнения» / 1231
    #[ffd(tag = fields::CustomsDeclarationNum)]
    CustomDeclarationNumber: Option<String>,
    /// 1291, unknown property names
    // #[ffd(tag = fields::MarkedProductFractionalQuantity)]
    ProductFractionalNumber: Option<serde_json::Value>,
    /// Результаты проверки товара с обязательной маркировкой / 2106
    #[ffd(tag = fields::ProductInfoCheckResult)]
    ProductCheckResultDetails: Option<enums::MarkingCheckResult>,
    /// Результаты проверки маркированных товаров / 2107
    #[ffd(tag = fields::MarkedProductCheckResults)]
    #[serde(with = "fiscal_data::json::bool_num_opt")]
    ProductCheckResult: Option<bool>,
    /// Единицы измерения количества предмета расчета / 2108
    #[ffd(tag = fields::ItemQuantityUnit)]
    ProductUnitOfMeasure: Option<enums::Unit>,
    /// 1226
    #[ffd(tag = fields::SupplierInn)]
    Supplier_INN: Option<String>,
    /// 1262
    #[ffd(tag = fields::FoivId)]
    IndustryPropertyFOIV: Option<String>,
    /// 1263
    #[ffd(tag = fields::FoundationDocDateTime)]
    IndustryPropertyDocDate: Option<String>,
    /// 1264
    #[ffd(tag = fields::FoundationDocNum)]
    IndustryPropertyDocNumber: Option<String>,
    /// 1265
    #[ffd(tag = fields::IndustryPropValue)]
    IndustryPropertyValue: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, FfdDoc)]
#[serde(default)]
// some fields have underscores, so we can't use rename_all
#[allow(non_snake_case)]
struct Document {
    /// Наименование документа / 1000
    #[ffd(tag = fields::DocName)]
    DocumentName: Option<String>,
    /// Признак вида документа
    #[ffd(special = "tag")]
    Tag: enums::FormCode,
    /// Наименование пользователя (компании) / 1048
    #[ffd(tag = fields::User)]
    User: Option<String>,
    /// ИНН пользователя (владельца, ответственного лица) / 1018
    #[ffd(tag = fields::UserInn)]
    UserInn: Option<String>,
    /// Номер документа в смене / 1042
    #[ffd(tag = fields::ReceiptNum)]
    Number: u32,
    /// Дата и время последнего обновления информации о чеке (по данным кассы, yyyy-MM-ddThh:mm:ss) / 1012
    // 2024-04-20T09:50:00
    #[ffd(tag = fields::DateTime)]
    DateTime: chrono::NaiveDateTime,
    /// Номер смены (по данным кассы), в которую был сформирован документ / 1038
    #[ffd(tag = fields::ShiftNum)]
    ShiftNumber: u32,
    /// Признак типа операции / 1054
    #[ffd(tag = fields::PaymentType)]
    OperationType: enums::PaymentType,
    /// Тип налогообложения / 1055
    #[ffd(tag = fields::TaxType)]
    TaxationType: enums::TaxationTypes,
    /// Фамилия, имя, отчество оператора / 1021
    #[ffd(tag = fields::Operator)]
    Operator: Option<String>,
    /// Регистрационный номер кассы / 1037
    #[ffd(tag = fields::KktRegNum)]
    KKT_RegNumber: String,
    /// Номер фискального накопителя, установленного в кассу (серийный, заводской) / 1041
    #[ffd(tag = fields::DriveNum)]
    FN_FactoryNumber: String,
    /// Товарные позиции, перечисленные списком
    #[ffd(tag = fields::ReceiptItem)]
    Items: Vec<OfdItem>,
    /// Сторно товара
    // why is this a string???????
    StornoItems: Option<String>,
    /// Адрес расположения кассового аппарата / 1009
    #[ffd(tag = fields::RetailPlaceAddress)]
    RetailPlaceAddress: Option<String>,
    // Абонентский номер и (или) адрес электронной почты покупателя (клиента) в случае передачи ему кассового чека (БСО) в электронной форме
    #[ffd(tag = fields::BuyerPhoneOrEmail)]
    Buyer_Address: Option<String>,
    // Адрес электронной почты отправителя чека (e.g. noreply@ofd.ru) / 1117
    #[ffd(tag = fields::ReceiptSenderEmail)]
    Sender_Address: Option<String>,
    /// Номер телефона платежного агента / 1073
    #[ffd(tag = fields::PaymentAgentPhone)]
    #[serde(with = "fiscal_data::json::one_or_many")]
    PaymentAgent_Phone: Vec<String>,
    /// Номер телефона оператора по переводу денежных средств / 1075
    #[ffd(tag = fields::TransferOperatorPhone)]
    #[serde(with = "fiscal_data::json::one_or_many")]
    MoneyOperator_Phone: Vec<String>,
    /// Номер телефона банковского платежного агента / 1082?
    BankAgent_Phone: Option<String>,
    /// Операция банковского агента / 1044?
    BankAgent_Operation: Option<u32>,
    /// Размер комиссии банковского платежного агента / 1010?
    BankAgent_Comission: Option<u64>,
    /// Наименование оператора по переводу денежных средств / 1026
    #[ffd(tag = fields::TransferOperatorName)]
    MoneyOperator_Name: Option<String>,
    /// Адрес оператора по переводу денежных средств / 1005
    #[ffd(tag = fields::TransferOperatorAddress)]
    MoneyOperator_Address: Option<String>,
    /// ИНН оператора по переводу денежных средств / 1016
    #[ffd(tag = fields::TransferOperatorInn)]
    MoneyOperator_INN: Option<String>,
    /// Сумма удерживаемого налога на добавленную стоимость (НДС) по ставку 18%, начисленная за смену (в копейках) / 1102
    #[ffd(tag = fields::TotalVat20Sum)]
    Nds18_TotalSumm: Option<u64>,
    /// Сумма удерживаемого налога на добавленную стоимость (НДС) по ставку 10%, начисленная за смену (в копейках) / 1103
    #[ffd(tag = fields::TotalVat10Sum)]
    Nds10_TotalSumm: Option<u64>,
    /// Сумма по операциям, облагаемая НДС по ставке 0%, накопленная за смену (в копейках) / 1104
    #[ffd(tag = fields::TotalSumWithVat0)]
    Nds00_TotalSumm: Option<u64>,
    /// Сумма по операциям, не облагаемая НДС, накопленная за смену (в копейках) / 1105
    #[ffd(tag = fields::TotalSumWithVat0)]
    NdsNA_TotalSumm: Option<u64>,
    /// Сумма удерживаемого налога на добавленную стоимость (НДС) по ставке в 18/118, начисленная за смену, в копейках / 1106
    #[ffd(tag = fields::TotalVat20_120Sum)]
    Nds18_CalculatedTotalSumm: Option<u64>,
    /// Сумма удерживаемого налога на добавленную стоимость (НДС) по ставке в 10/110, начисленная за смену, в копейках / 1107
    #[ffd(tag = fields::TotalVat10_110Sum)]
    Nds10_CalculatedTotalSumm: Option<u64>,
    /// Общая сумма по чеку (в копейках) / 1020
    #[ffd(tag = fields::TotalSum)]
    Amount_Total: u64,
    /// Сумма наличными по чеку (в копейках) / 1031
    #[ffd(tag = fields::TotalCashSum)]
    Amount_Cash: u64,
    /// Сумма электронного платежа по чеку (в копейках) / 1081
    #[ffd(tag = fields::TotalEcashSum)]
    Amount_ECash: u64,
    /// Фискальный номер документа / 1040
    #[ffd(tag = fields::DocNum)]
    Document_Number: u64,
    /// Фискальный признак документа / 1077
    #[ffd(tag = fields::DocFiscalSign)]
    #[serde(with = "fiscal_data::json::base64_array")]
    FiscalSign: [u8; 6],
    /// Фискальный признак документа
    DecimalFiscalSign: String,
    /// Количество кассовых чеков за смену / 1118
    #[ffd(tag = fields::ReceiptCountPerShift)]
    ReceiptsCount: Option<u32>,
    /// Количество фискальных документов за смену / 1111
    #[ffd(tag = fields::DocCountPerShift)]
    DocumentsCount: Option<u32>,
    /// Количество ФД, не переданных ОФД (по которым не было получено подтверждения оператора) / 1097
    #[ffd(tag = fields::UntransmittedDocCount)]
    BadStateCount: Option<u32>,
    /// Дата первого ФД из числа не переданных ОФД / 1098
    #[ffd(tag = fields::UntransmittedDocDateTime)]
    BadStateDateTime: Option<NaiveDateTime>,
    /// Подтверждение оператора для переданного фискального документа отсутствует более двух дней / 1053
    // #[ffd(tag = fields::OfdResponseTimeoutFlag)]
    OFD_ResponseTimeout: Option<String>,
    /// До истечения срока действия ключей фискального признака в фискальном накопителе осталось менее 30 дней / 1050
    // #[ffd(tag = fields::DriveResourceExhaustionFlag)]
    FiscalDrive_Exhaustion: Option<String>,
    /// Применения ККТ в режиме, не предусматривающем обязательной передачи ФД в налоговые органы в электронной форме через ОФД / 1002
    // #[ffd(tag = fields::OfflineModeFlag)]
    OfflineMode: Option<String>,
    /// Номер первого ФД из числа не переданных ОФД / 1116
    #[ffd(tag = fields::UntransmittedDocNum)]
    BadStateNumber: Option<u64>,
    /// Признак ККТ, являющейся автоматизированной системой для БСО (может формировать только БСО и
    /// применяться для осуществления расчетов только при оказании услуг) / 1110
    // #[ffd(tag = fields::BsoFlag)]
    StrictFormSign: Option<String>,
    /// Признак работы в сфере услуг / 1109
    // #[ffd(tag = fields::ServiceFlag)]
    ServiceSectorSign: Option<String>,
    /// Признак передачи фискальных документов в зашифрованном виде оператору фискальных данных / 1056
    // #[ffd(tag = fields::EncryptionFlag)]
    EncryptionSign: Option<String>,
    /// Признак применения ККТ в составе автоматического устройства для расчетов / 1001
    // #[ffd(tag = fields::AutoModeFlag)]
    AutoMode: Option<String>,
    /// Заводской номер автоматического устройства для расчетов / 1036
    #[ffd(tag = fields::MachineNumber)]
    KKT_MachineNumber: Option<String>,
    /// Осуществления расчетов только в сети «Интернет», в которой отсутствует устройство для
    /// печати фискальных документов в составе ККТ / 1108
    #[ffd(tag = fields::OnlineKktFlag)]
    #[serde(with = "fiscal_data::json::bool_num_opt")]
    InternetSign: Option<bool>,
    /// ИНН оператора фискальных данных / 1017
    #[ffd(tag = fields::OfdInn)]
    OfdInn: Option<String>,
    /// Заводской номер KKT / 1013
    #[ffd(tag = fields::KktSerial)]
    KKT_FactoryNumber: Option<String>,
    /// Признак того, что до истечения срока действия ключей фискального признака в фискальном
    /// накопителе осталось менее 3 дней / 1051
    // #[ffd(tag = fields::DriveReplacementRequiredFlag)]
    // why is this a string????????
    FiscalDrive_ReplaceRequired: Option<String>,
    /// Признак того, что память фискального накопителя заполнена более чем на 99% / 1052
    // #[ffd(tag = fields::DriveMemoryFullFlag)]
    // why is this a string????????
    FiscalDrive_MemoryExceeded: Option<String>,
    /// Причина изменения сведений о ККТ / 1101
    #[ffd(tag = fields::ReregistrationReason)]
    ReRegReasons: Option<enums::ReregistrationReason>,
    /// Адрес сайта для проверки ФП / 1115
    // #[ffd(tag = fields::FiscalSignCheckUrl)]
    CheckFP_Site: Option<String>,
    /// Номер телефона платежного субагента / 1083
    // #[ffd(tag = fields::FiscalSignCheckUrl)]
    PaymentSubAgent_Phone: Option<String>,
    /// Номер телефона оператора по приему платежей / 1074
    #[ffd(tag = fields::PaymentOperatorPhone)]
    PaymentOperator_Phone: Option<String>,
    /// Размер комиссии платежного агента / 1011
    // #[ffd(tag = fields::PaymentAgentReward)]
    PaymentAgent_Comission: Option<u64>,
    /// Номер телефона банковского платежного субагента / 1082? / 1083? / 1119?
    BankSubAgent_Phone: Option<String>,
    /// Операция банковского платежного субагента / 1045
    // #[ffd(tag = fields::PaymentSubagentOperation)]
    BankSubAgent_Operation: Option<String>,
    /// Текст сообщения / 1066?
    // #[ffd(tag = fields::Message)]
    Messages: Option<String>,
    /// Скидка/наценка / 1112
    // #[ffd(tag = fields::Modifiers)]
    DiscountMarkup: Option<u64>,
    /// Дополнительные реквизиты пользователя
    #[ffd(tag = fields::AdditionalUserProp)]
    #[serde(deserialize_with = "fiscal_data::json::one_or_singleton_opt::deserialize")]
    Extra: Option<ExtraProp>,
    /// Номер версии формата фискальных документов / 1209
    #[ffd(tag = fields::FfdVer)]
    Format_Version: enums::FfdVersion,
    /// Версия форматов фискальных документов, реализованная в ККТ / 1189
    #[ffd(tag = fields::KktFfdVer)]
    Format_VersionKKT: Option<enums::FfdVersion>,
    /// Версия форматов фискальных документов, реализованная в ФН / 1190
    #[ffd(tag = fields::DriveFfdVer)]
    Format_VersionFN: Option<enums::FfdVersion>,
    /// Тип коррекции: Самостоятельно – 0; По предписанию – 1. / 1173
    #[ffd(tag = fields::CorrectionType)]
    Correction_Type: Option<enums::CorrectionType>,
    /// Основание для коррекции / 1174
    // #[ffd(tag = fields::CorrectionBasis)]
    // why is this a string????
    Correction: Option<String>,
    /// Сумма предоплаты / 1215
    #[ffd(tag = fields::TotalPrepaidSum)]
    Amount_Advance: u64,
    /// Сумма постоплаты / 1216
    #[ffd(tag = fields::TotalCreditSum)]
    Amount_Loan: u64,
    /// Сумма встречным предоставлением / 1217
    #[ffd(tag = fields::TotalProvisionSum)]
    Amount_Granting: u64,
    /// Номер телефон поставщика / 1171
    #[ffd(tag = fields::SupplierPhone)]
    Supplier_Phone: Option<String>,
    /// Сайт налогового органа / 1060
    #[ffd(tag = fields::FnsUrl)]
    TaxAuthority_Site: Option<String>,
    /// Дополнительные реквизиты
    AdditionalRequisite: Option<String>,
    /// 1191
    #[ffd(tag = fields::AdditionalItemProp)]
    ProductAdditionalRequisite: Option<String>,
    /// 1192
    #[ffd(tag = fields::AdditionalReceiptProp)]
    ReceiptAdditionalRequisite: Option<String>,
    /// Итоговая суммы расчетов, указанная в чеке / 1194
    // ?? this is not 1194 bruh
    // #[ffd(tag = fields::ShiftStats)]
    ShiftTotals: Option<u64>,
    /// Итоговые количества и итоговые суммы расчетов фискальных данных / 1157
    // #[ffd(tag = fields::DriveStats)]
    DeliveredTotals: Option<u64>,
    /// Итоговые количества и итоговые суммы расчетов непереданных фискальных данных / 1158
    // #[ffd(tag = fields::DriveUntransmittedStats)]
    UndeliveredTotals: Option<u64>,
    /// Место осуществления расчетов между пользователем и покупателем (клиентом) / 1187
    #[ffd(tag = fields::RetailPlace)]
    Calculation_Place: Option<String>,
    /// ИНН кассира / 1203
    #[ffd(tag = fields::OperatorInn)]
    Operator_INN: Option<String>,
    /// Признак проведения азартных игр / 1193
    #[ffd(tag = fields::GamblingFlag)]
    #[serde(with = "fiscal_data::json::bool_num_opt")]
    GamblingMode: Option<bool>,
    /// Проведение расчётов платежным агентом / 1057
    #[ffd(tag = fields::PaymentAgentTypes)]
    PaymentAgentMode: Option<enums::AgentTypes>,
    /// Признак проведения лотереи / 1126
    #[ffd(tag = fields::LotteryFlag)]
    #[serde(with = "fiscal_data::json::bool_num_opt")]
    LotteryMode: Option<bool>,
    /// Признак ККТ, предназначенной для применения только в составе автоматического устройства для расчетов / 1221
    // WHAT? this is NOT 1221
    Sign_KKT_Machine: Option<u8>,
    /// Продажа подакцизного товара / 1207
    #[ffd(tag = fields::ExciseFlag)]
    #[serde(with = "fiscal_data::json::bool_num_opt")]
    Sign_Excise: Option<bool>,
    /// Адрес сайта, на котором покупатель может бесплатно получить чек / 1208
    #[ffd(tag = fields::ReceiptRetrievalWebsite)]
    RecipeSite: Option<String>,
    /// Срок действия ключей фискального признака (величина учитывается в днях до момента истечения срока действия ключей) (yyyy-MM-ddThh:mm:ss) / 1213
    // #[ffd(tag = fields::FiscalSignValidityPeriod)]
    ValidityPeriod: Option<NaiveDateTime>,
    /// 1225
    #[ffd(tag = fields::SupplierName)]
    Supplier_Name: Option<String>,
    /// 1226
    #[ffd(tag = fields::SupplierInn)]
    Supplier_INN: Option<String>,
    /// 1262
    #[ffd(tag = fields::FoivId)]
    IndustryPropertyFOIV: Option<String>,
    /// 1263
    #[ffd(tag = fields::FoundationDocDateTime)]
    IndustryPropertyDocDate: Option<String>,
    /// 1264
    #[ffd(tag = fields::FoundationDocNum)]
    IndustryPropertyDocNumber: Option<String>,
    /// 1265
    #[ffd(tag = fields::IndustryPropValue)]
    IndustryPropertyValue: Option<String>,
    /// 1228
    #[ffd(tag = fields::BuyerInn)]
    BuyerInn: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize)]
#[serde(default, rename_all = "PascalCase")]
struct Coordinates {
    /// Широта
    latitude: f64,
    /// Долгота
    longitude: f64,
}

#[derive(Clone, Debug, Default, Deserialize)]
#[serde(default, rename_all = "PascalCase")]
struct Res {
    /// Версия сериализации документа
    version: u32,
    /// Версия формата фискальных данных
    document_format: String,
    /// Данные фискального документа
    document: Document,
    /// Признак вида документа
    tag: enums::FormCode,
    /// ИНН пользователя (владельца, ответственного лица) / 1018
    user_inn: String,
    /// Регистрационный номер кассы / 1037
    kkt_reg_number: String,
    /// Номер фискального накопителя, установленного в кассу (серийный, заводской) / 1041
    fn_number: String,
    /// Фискальный номер документа / 1040
    doc_number: u64,
    /// Дата и время формирования документа по данным кассы (yyyy-MM-ddThh:mm:ss)
    doc_date_time: chrono::NaiveDateTime,
    /// Фискальный признак документа / 1077
    #[serde(with = "fiscal_data::json::base64_array")]
    doc_fiscal_sign: [u8; 6],
    decimal_fiscal_sign: String,
    /// Дата и время приема документа в информационную систему (UTC, yyyy-MM-ddThh:mm:ss)
    c_date_utc: chrono::NaiveDateTime,
    /// Схема предупреждения документа
    scheme_warnings: Option<SchemeWarnings>,
    /// Ошибки валидации документа
    validation_errors: Option<String>,
    /// Адрес установки кассы
    reg_address: Option<String>,
    /// Идентификатор ФИАС
    fias_id: Option<String>,
    /// Географические координаты адреса установки кассы
    geo_point: Coordinates,
}

pub struct OfdRu;
#[async_trait]
impl Provider for OfdRu {
    fn id(&self) -> &'static str {
        "ofd-ru"
    }
    fn name(&self) -> &'static str {
        "ООО \"ПЕТЕР-СЕРВИС Спецтехнологии\""
    }
    fn url(&self) -> &'static str {
        "www.ofd.ru"
    }
    fn exts(&self) -> &'static [&'static str] {
        &["json"]
    }
    fn inn(&self) -> &'static str {
        "7841465198"
    }
    async fn fetch_raw_data(&self, config: &Config, rec: &mut Object) -> Result<Vec<u8>, Error> {
        let client = reqwest::Client::builder()
            // .user_agent("Mozilla/5.0 (Windows NT 10.0; rv:109.0) Gecko/20100101 Firefox/118.0")
            .cookie_store(true)
            .build()?;
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
        let url = format!(
            "https://check.ofd.ru/rec/{}/{}/{}",
            drive_num, doc_num, fiscal_sign
        );
        log::info!("ofd.ru: fetching from {url}");
        let res = client
            .execute(client.get(url).build()?)
            .await?
            .text()
            .await?;
        let Some(id) = (if let Some((_, s)) = res.split_once("/Document/RenderDoc?RawId=") {
            s
        } else if let Some((_, s)) = res.split_once("/Document/ReceiptJsonDownload?DocId=") {
            s
        } else {
            return Err(Error::MissingData("id"));
        })
        .split('"')
        .next()
        .and_then(|x| x.split('&').next()) else {
            return Err(Error::MissingData("id"));
        };
        log::info!("ofd.ru: id = {id}");
        let ret = client
            .execute(
                client
                    .get("https://ofd.ru/Document/ReceiptJsonDownload")
                    .query(&[("DocId", id)])
                    .build()?,
            )
            .await?
            .bytes()
            .await?
            .to_vec();
        #[cfg(not(debug_assertions))]
        {
            let Ok(_) = serde_json::from_slice::<Res>(&ret) else {
                return Err(Error::ParseError);
            };
        }
        rec.set::<super::custom::Id>(id.to_owned())?;
        if let Some(provider) = super::registry().await.by_id("", rec) {
            if provider.id() != self.id() {
                let _ = super::fetch_raw(config, provider, rec, false).await;
            }
        }
        Ok(ret)
    }
    async fn parse(
        &self,
        config: &Config,
        data: &[u8],
        rec: Object,
    ) -> Result<fiscal_data::Document, Error> {
        let res = serde_json::from_slice::<Res>(data)?;
        if let Some(provider) = super::registry().await.by_id("", &rec) {
            if let Ok(mut doc) = super::fetch2(config, provider, rec).await {
                if let Ok(fiscal_sign) = <[u8; 6]>::try_from(&res.document.FiscalSign[..])
                    .or_else(|_| <[u8; 6]>::try_from(&res.doc_fiscal_sign[..]))
                {
                    doc.data_mut()
                        .set::<fields::DocFiscalSign>(fiscal_sign)
                        .unwrap();
                }
                return Ok(doc);
            }
        }
        Ok(fiscal_data::Document::try_from(res.document)?)
    }
}

#[cfg(test)]
mod test {
    use super::Res;

    #[test]
    fn test() {
        let test_data = include_bytes!("../../test_data/ofdru1.json");
        let res = serde_json::from_slice::<Res>(test_data).unwrap();
        assert_eq!(res.document.Items.len(), 23);
        let test_data = include_bytes!("../../test_data/ofdru2.json");
        let res = serde_json::from_slice::<Res>(test_data).unwrap();
        assert_eq!(res.document.Items.len(), 3);
    }
}
