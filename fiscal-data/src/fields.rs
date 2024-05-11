use std::{collections::BTreeMap, sync::OnceLock};

use crate::{
    enums,
    internal::{JsonName, Repr},
    FieldInternal, MultiField, Object, Padding, TlvType, VarFloat,
};

const fn json_name(s: &str) -> Option<JsonName<'_>> {
    Some(JsonName {
        name: s,
        enclosure_tag_overrides: &[],
    })
}

/// (1) Отчет о регистрации
pub enum RegistrationReport {}
impl FieldInternal for RegistrationReport {
    const PADDING: Padding = Padding::None { length: Some(6144) };
    const TAG: u16 = 1;
    type Type = Object;
}

/// (2) Отчет об открытии смены
pub enum ShiftStartReport {}
impl FieldInternal for ShiftStartReport {
    const PADDING: Padding = Padding::None { length: Some(4096) };
    const TAG: u16 = 2;
    type Type = Object;
}

/// (3) Кассовый чек
pub enum Receipt {}
impl FieldInternal for Receipt {
    const PADDING: Padding = Padding::None {
        length: Some(32768),
    };
    const TAG: u16 = 3;
    type Type = Object;
}

/// (4) Бланк строгой отчетности
pub enum Bso {}
impl FieldInternal for Bso {
    const PADDING: Padding = Padding::None {
        length: Some(32768),
    };
    const TAG: u16 = 4;
    type Type = Object;
}

/// (5) Отчет о закрытии смены
pub enum ShiftEndReport {}
impl FieldInternal for ShiftEndReport {
    const PADDING: Padding = Padding::None { length: Some(4096) };
    const TAG: u16 = 5;
    type Type = Object;
}

/// (6) Отчет о закрытии фискального накопителя
pub enum FnCloseReport {}
impl FieldInternal for FnCloseReport {
    const PADDING: Padding = Padding::None { length: Some(4096) };
    const TAG: u16 = 6;
    type Type = Object;
}

/// (7) Подтверждение оператора
pub enum OperatorConfirmation {}
impl FieldInternal for OperatorConfirmation {
    const PADDING: Padding = Padding::None { length: Some(512) };
    const TAG: u16 = 7;
    type Type = Object;
}

/// (11) Отчет об изменении параметров регистрации
pub enum RegistrationParamUpdateReport {}
impl FieldInternal for RegistrationParamUpdateReport {
    const PADDING: Padding = Padding::None { length: Some(6144) };
    const TAG: u16 = 11;
    type Type = Object;
}

/// (21) Отчет о текущем состоянии расчетов
pub enum PaymentStateReport {}
impl FieldInternal for PaymentStateReport {
    const PADDING: Padding = Padding::None {
        length: Some(32768),
    };
    const TAG: u16 = 21;
    type Type = Object;
}

/// (31) Кассовый чек коррекции
pub enum CorrectionReceipt {}
impl FieldInternal for CorrectionReceipt {
    const PADDING: Padding = Padding::None {
        length: Some(32768),
    };
    const TAG: u16 = 31;
    type Type = Object;
}

/// (41) Бланк строгой отчетности коррекции
pub enum CorrectionBso {}
impl FieldInternal for CorrectionBso {
    const PADDING: Padding = Padding::None {
        length: Some(32768),
    };
    const TAG: u16 = 41;
    type Type = Object;
}

/// (81) Запрос о коде маркировки
pub enum MarkingCodeRequest {}
impl FieldInternal for MarkingCodeRequest {
    const PADDING: Padding = Padding::None { length: Some(4096) };
    const TAG: u16 = 81;
    type Type = Object;
}

/// (82) Уведомление о реализации маркированного товара
pub enum MarkedProductSaleNotification {}
impl FieldInternal for MarkedProductSaleNotification {
    const PADDING: Padding = Padding::None {
        length: Some(32768),
    };
    const TAG: u16 = 82;
    type Type = Object;
}

/// (83) Ответ на запрос
pub enum MarkingResponse {}
impl FieldInternal for MarkingResponse {
    const PADDING: Padding = Padding::None { length: Some(512) };
    const TAG: u16 = 83;
    type Type = Object;
}

/// (84) Квитанция на уведомление
pub enum NotificationReceipt {}
impl FieldInternal for NotificationReceipt {
    const PADDING: Padding = Padding::None { length: Some(512) };
    const TAG: u16 = 84;
    type Type = Object;
}

/// (1000) Наименование документа
///
/// Наименование ФД
pub enum DocName {}
impl FieldInternal for DocName {
    const TAG: u16 = 1000;
    type Type = String;
}

/// (1001) Признак автоматического режима
///
/// Признак применения ККТ с автоматическим устройством для расчетов
pub enum AutoModeFlag {}
impl FieldInternal for AutoModeFlag {
    const PADDING: Padding = Padding::Right {
        length: 1,
        padding: b'\0',
    };
    const TAG: u16 = 1001;
    const JSON_NAME: Option<JsonName<'static>> = json_name("autoMode");
    type Type = bool;
}

/// (1002) Признак автономного режима
///
/// Признак применения ККТ в режиме, не предусматривающем обязательной передачи ФД в налоговые органы в электронной форме через ОФД
pub enum OfflineModeFlag {}
impl FieldInternal for OfflineModeFlag {
    const PADDING: Padding = Padding::Right {
        length: 1,
        padding: b'\0',
    };
    const TAG: u16 = 1002;
    const JSON_NAME: Option<JsonName<'static>> = json_name("offlineMode");
    type Type = bool;
}

/// (1003) Адрес банковского агента
#[deprecated]
pub enum BankAgentAddress {}
#[allow(deprecated)]
impl FieldInternal for BankAgentAddress {
    const PADDING: Padding = Padding::None { length: Some(256) };
    const TAG: u16 = 1003;
    type Type = String;
}
#[allow(deprecated)]
impl MultiField for BankAgentAddress {}

/// (1004) Адрес банковского субагента
#[deprecated]
pub enum BankSubagentAddress {}
#[allow(deprecated)]
impl FieldInternal for BankSubagentAddress {
    const PADDING: Padding = Padding::None { length: Some(256) };
    const TAG: u16 = 1004;
    type Type = String;
}
#[allow(deprecated)]
impl MultiField for BankSubagentAddress {}

/// (1005) Адрес оператора перевода
///
/// Место нахождения оператора по переводу денежных средств
pub enum TransferOperatorAddress {}
impl FieldInternal for TransferOperatorAddress {
    const PADDING: Padding = Padding::None { length: Some(256) };
    const TAG: u16 = 1005;
    const JSON_NAME: Option<JsonName<'static>> = json_name("transferOperatorAddress");
    type Type = String;
}
impl MultiField for TransferOperatorAddress {}

/// (1006) Адрес платежного агента
#[deprecated]
pub enum PaymentAgentAddress {}
#[allow(deprecated)]
impl FieldInternal for PaymentAgentAddress {
    const PADDING: Padding = Padding::None { length: Some(256) };
    const TAG: u16 = 1006;
    type Type = String;
}

/// (1007) Адрес платежного субагента
#[deprecated]
pub enum PaymentSubagentAddress {}
#[allow(deprecated)]
impl FieldInternal for PaymentSubagentAddress {
    const PADDING: Padding = Padding::None { length: Some(256) };
    const TAG: u16 = 1007;
    type Type = String;
}

/// (1008) Телефон или электронный адрес покупателя
///
/// Абонентский номер и (или) адрес электронной почты покупателя (клиента) в случае передачи ему кассового чека (БСО), кассового чека коррекции (БСО коррекции) в электронной форме
pub enum BuyerPhoneOrEmail {}
impl FieldInternal for BuyerPhoneOrEmail {
    const PADDING: Padding = Padding::None { length: Some(64) };
    const TAG: u16 = 1008;
    const JSON_NAME: Option<JsonName<'static>> = json_name("buyerPhoneOrAddress");
    type Type = String;
}

/// (1009) Адрес расчетов
///
/// Адрес осуществления расчетов между пользователем и покупателем (клиентом). В случае применения ККТ с автоматическим устройством для расчетов адрес установки этого автоматического устройства для расчетов
pub enum RetailPlaceAddress {}
impl FieldInternal for RetailPlaceAddress {
    const PADDING: Padding = Padding::None { length: Some(256) };
    const TAG: u16 = 1009;
    const JSON_NAME: Option<JsonName<'static>> = json_name("retailPlaceAddress");
    type Type = String;
}

/// (1010) Размер вознаграждения банковского агента (субагента)
#[deprecated]
pub enum BankAgentReward {}
#[allow(deprecated)]
impl FieldInternal for BankAgentReward {
    const PADDING: Padding = Padding::None { length: Some(8) };
    const TAG: u16 = 1010;
    type Type = u64;
}
#[allow(deprecated)]
impl MultiField for BankAgentReward {}

/// (1011) Размер вознаграждения платежного агента (субагента)
#[deprecated]
pub enum PaymentAgentReward {}
#[allow(deprecated)]
impl FieldInternal for PaymentAgentReward {
    const PADDING: Padding = Padding::None { length: Some(8) };
    const TAG: u16 = 1011;
    type Type = u64;
}
#[allow(deprecated)]
impl MultiField for PaymentAgentReward {}

/// (1012) Дата, время
///
/// Дата и время формирования ФД
pub enum DateTime {}
impl FieldInternal for DateTime {
    const PADDING: Padding = Padding::Right {
        length: 4,
        padding: b'\0',
    };
    const TAG: u16 = 1012;
    const JSON_NAME: Option<JsonName<'static>> = json_name("dateTime");
    type Type = chrono::NaiveDateTime;
}

/// (1013) Заводской номер ККТ
///
/// Заводской номер ККТ
pub enum KktSerial {}
impl FieldInternal for KktSerial {
    const PADDING: Padding = Padding::None { length: Some(20) };
    const TAG: u16 = 1013;
    const JSON_NAME: Option<JsonName<'static>> = json_name("kktNumber");
    type Type = String;
}

/// (1014) Значение типа строка
#[deprecated]
pub enum StringValue {}
#[allow(deprecated)]
impl FieldInternal for StringValue {
    const PADDING: Padding = Padding::None { length: Some(64) };
    const TAG: u16 = 1014;
    type Type = String;
}

/// (1015) Значение типа целое
#[deprecated]
pub enum IntegerValue {}
#[allow(deprecated)]
impl FieldInternal for IntegerValue {
    const PADDING: Padding = Padding::Right {
        length: 4,
        padding: b'\0',
    };
    const TAG: u16 = 1015;
    type Type = u32;
}

/// (1016) ИНН оператора перевода
///
/// Идентификационный номер налогоплательщика оператора по переводу денежных средств
pub enum TransferOperatorInn {}
impl FieldInternal for TransferOperatorInn {
    const PADDING: Padding = Padding::Right {
        length: 12,
        padding: b' ',
    };
    const TAG: u16 = 1016;
    const JSON_NAME: Option<JsonName<'static>> = json_name("transferOperatorInn");
    type Type = String;
}
impl MultiField for TransferOperatorInn {}

/// (1017) ИНН ОФД
///
/// Идентификационный номер налогоплательщика оператора фискальных данных
pub enum OfdInn {}
impl FieldInternal for OfdInn {
    const PADDING: Padding = Padding::Right {
        length: 12,
        padding: b' ',
    };
    const TAG: u16 = 1017;
    const JSON_NAME: Option<JsonName<'static>> = json_name("ofdInn");
    type Type = String;
}

/// (1018) ИНН пользователя
///
/// Идентификационный номер налогоплательщика пользователя
pub enum UserInn {}
impl FieldInternal for UserInn {
    const PADDING: Padding = Padding::Right {
        length: 12,
        padding: b' ',
    };
    const TAG: u16 = 1018;
    const JSON_NAME: Option<JsonName<'static>> = json_name("userInn");
    type Type = String;
}

/// (1019) Информационное сообщение
#[deprecated]
pub enum InfoMessage {}
#[allow(deprecated)]
impl FieldInternal for InfoMessage {
    const PADDING: Padding = Padding::None { length: Some(64) };
    const TAG: u16 = 1019;
    type Type = String;
}

/// (1020) Сумма расчета, указанного в чеке (БСО)
///
/// Сумма расчета с учетом скидок, наценок и НДС, указанная в кассовом чеке (БСО), или сумма коррекции, указанная в кассовом чеке коррекции (БСО коррекции)
pub enum TotalSum {}
impl FieldInternal for TotalSum {
    const PADDING: Padding = Padding::None { length: Some(6) };
    const TAG: u16 = 1020;
    const JSON_NAME: Option<JsonName<'static>> = json_name("totalSum");
    type Type = u64;
}

/// (1021) Кассир
///
/// Для кассового чека (БСО), кассового чека коррекции (БСО коррекции) должность и фамилия лица, осуществившего расчет с покупателем (клиентом), оформившего кассовый чек (БСО), кассовый чек коррекции (БСО коррекции) и выдавшего (передавшего) его покупателю (клиенту); для иных фискальных документов - должность и фамилия лица, уполномоченного пользователем на формирование иного фискального документа
pub enum Operator {}
impl FieldInternal for Operator {
    const PADDING: Padding = Padding::None { length: Some(64) };
    const TAG: u16 = 1021;
    const JSON_NAME: Option<JsonName<'static>> = json_name("operator");
    type Type = String;
}

/// (1022) Код ответа ОФД
///
/// Код информационного сообщения оператора фискальных данных
pub enum OfdResponseCode {}
impl FieldInternal for OfdResponseCode {
    const PADDING: Padding = Padding::Right {
        length: 1,
        padding: b'\0',
    };
    const TAG: u16 = 1022;
    type Type = enums::OfdResponse;
}

/// (1023) Количество предмета расчета
///
/// Количество товара, работ, услуг, платежей, выплат, иных предметов расчета
pub enum ItemQuantity {}
impl FieldInternal for ItemQuantity {
    const PADDING: Padding = Padding::None { length: Some(8) };
    const TAG: u16 = 1023;
    const JSON_NAME: Option<JsonName<'static>> = json_name("quantity");
    type Type = VarFloat;
}

/// (1024) Наименование банковского агента
#[deprecated]
pub enum BankAgentName {}
#[allow(deprecated)]
impl FieldInternal for BankAgentName {
    const PADDING: Padding = Padding::None { length: Some(64) };
    const TAG: u16 = 1024;
    type Type = String;
}

/// (1025) Наименование банковского субагента
#[deprecated]
pub enum BankSubagentName {}
#[allow(deprecated)]
impl FieldInternal for BankSubagentName {
    const PADDING: Padding = Padding::None { length: Some(64) };
    const TAG: u16 = 1025;
    type Type = String;
}

/// (1026) Наименование оператора перевода
///
/// Наименование оператора по переводу денежных средств
pub enum TransferOperatorName {}
impl FieldInternal for TransferOperatorName {
    const PADDING: Padding = Padding::None { length: Some(64) };
    const TAG: u16 = 1026;
    const JSON_NAME: Option<JsonName<'static>> = json_name("transferOperatorName");
    type Type = String;
}
impl MultiField for TransferOperatorName {}

/// (1027) Наименование платежного агента
#[deprecated]
pub enum PaymentAgentName {}
#[allow(deprecated)]
impl FieldInternal for PaymentAgentName {
    const PADDING: Padding = Padding::None { length: Some(64) };
    const TAG: u16 = 1027;
    type Type = String;
}

/// (1028) Наименование платежного субагента
#[deprecated]
pub enum PaymentSubagentName {}
#[allow(deprecated)]
impl FieldInternal for PaymentSubagentName {
    const PADDING: Padding = Padding::None { length: Some(64) };
    const TAG: u16 = 1028;
    type Type = String;
}

/// (1029) Наименование реквизита
#[deprecated]
pub enum PropertyName {}
#[allow(deprecated)]
impl FieldInternal for PropertyName {
    const PADDING: Padding = Padding::None { length: Some(64) };
    const TAG: u16 = 1029;
    type Type = String;
}

/// (1030) Наименование предмета расчета
///
/// Наименование товара, работы, услуги, платежа, выплаты, иного предмета расчета
pub enum ItemName {}
impl FieldInternal for ItemName {
    const PADDING: Padding = Padding::None { length: Some(128) };
    const TAG: u16 = 1030;
    const JSON_NAME: Option<JsonName<'static>> = json_name("name");
    type Type = String;
}

/// (1031) Сумма по чеку (БСО) наличными
///
/// Сумма расчета, указанная в кассовом чеке (БСО), или сумма корректировки расчета, указанная в кассовом чеке коррекции (БСО коррекции), подлежащая уплате наличными денежными средствами
pub enum TotalCashSum {}
impl FieldInternal for TotalCashSum {
    const PADDING: Padding = Padding::None { length: Some(6) };
    const TAG: u16 = 1031;
    const JSON_NAME: Option<JsonName<'static>> = json_name("cashTotalSum");
    type Type = u64;
}

/// (1032) Налог
#[deprecated]
pub enum Tax {}
#[allow(deprecated)]
impl FieldInternal for Tax {
    const PADDING: Padding = Padding::None { length: Some(33) };
    const TAG: u16 = 1032;
    type Type = Object;
}

/// (1033) Налоги
#[deprecated]
pub enum Taxes {}
#[allow(deprecated)]
impl FieldInternal for Taxes {
    const PADDING: Padding = Padding::None { length: Some(33) };
    const TAG: u16 = 1033;
    type Type = Object;
}

/// (1034) Наценка (ставка)
#[deprecated]
pub enum MarkupRate {}
#[allow(deprecated)]
impl FieldInternal for MarkupRate {
    const PADDING: Padding = Padding::None { length: Some(8) };
    const TAG: u16 = 1034;
    type Type = VarFloat;
}

/// (1035) Наценка (сумма)
#[deprecated]
pub enum MarkupSum {}
#[allow(deprecated)]
impl FieldInternal for MarkupSum {
    const PADDING: Padding = Padding::None { length: Some(8) };
    const TAG: u16 = 1035;
    type Type = u64;
}

/// (1036) Номер автомата
///
/// Заводской номер автоматического устройства для расчетов
pub enum MachineNumber {}
impl FieldInternal for MachineNumber {
    const PADDING: Padding = Padding::None { length: Some(20) };
    const TAG: u16 = 1036;
    const JSON_NAME: Option<JsonName<'static>> = json_name("machineNumber");
    type Type = String;
}

/// (1037) Регистрационный номер ККТ
///
/// Регистрационный номер контрольно-кассовой техники
pub enum KktRegNum {}
impl FieldInternal for KktRegNum {
    const PADDING: Padding = Padding::Right {
        length: 20,
        padding: b' ',
    };
    const TAG: u16 = 1037;
    const JSON_NAME: Option<JsonName<'static>> = json_name("kktRegId");
    type Type = String;
}

/// (1038) Номер смены
///
/// Порядковый номер смены с момента формирования отчета о регистрации ККТ или отчета об изменении параметров регистрации ККТ в связи с заменой фискального накопителя
pub enum ShiftNum {}
impl FieldInternal for ShiftNum {
    const PADDING: Padding = Padding::Right {
        length: 4,
        padding: b'\0',
    };
    const TAG: u16 = 1038;
    const JSON_NAME: Option<JsonName<'static>> = json_name("shiftNumber");
    type Type = u32;
}

/// (1039) Зарезервирован
#[deprecated]
pub enum Reserved {}
#[allow(deprecated)]
impl FieldInternal for Reserved {
    const PADDING: Padding = Padding::None { length: Some(12) };
    const TAG: u16 = 1039;
    type Type = String;
}

/// (1040) Номер ФД
///
/// Порядковый номер ФД с момента формирования отчета о регистрации ККТ или отчета об изменении параметров регистрации ККТ в связи с заменой фискального накопителя
pub enum DocNum {}
impl FieldInternal for DocNum {
    const PADDING: Padding = Padding::Right {
        length: 4,
        padding: b'\0',
    };
    const TAG: u16 = 1040;
    const JSON_NAME: Option<JsonName<'static>> = json_name("fiscalDocumentNumber");
    type Type = u32;
}

/// (1041) Номер ФН
///
/// Заводской номер фискального накопителя
pub enum DriveNum {}
impl FieldInternal for DriveNum {
    const PADDING: Padding = Padding::Fixed { length: 16 };
    const TAG: u16 = 1041;
    const JSON_NAME: Option<JsonName<'static>> = json_name("fiscalDriveNumber");
    type Type = String;
}

/// (1042) Номер чека за смену
///
/// Порядковый номер кассового чека, БСО, кассового чека коррекции и БСО коррекции за смену
pub enum ReceiptNum {}
impl FieldInternal for ReceiptNum {
    const PADDING: Padding = Padding::Right {
        length: 4,
        padding: b'\0',
    };
    const TAG: u16 = 1042;
    const JSON_NAME: Option<JsonName<'static>> = json_name("requestNumber");
    type Type = u32;
}

/// (1043) Стоимость предмета расчета с учетом скидок и наценок
///
/// Стоимость товара, работы, услуги, платежа, выплаты, иного предмета расчета с учетом скидок и наценок
pub enum ItemTotalPrice {}
impl FieldInternal for ItemTotalPrice {
    const PADDING: Padding = Padding::None { length: Some(6) };
    const TAG: u16 = 1043;
    const JSON_NAME: Option<JsonName<'static>> = json_name("sum");
    type Type = u64;
}

/// (1044) Операция банковского платежного агента
///
/// Наименование операции банковского платежного агента, банковского платежного субагента
pub enum PaymentAgentOperation {}
impl FieldInternal for PaymentAgentOperation {
    const PADDING: Padding = Padding::None { length: Some(24) };
    const TAG: u16 = 1044;
    const JSON_NAME: Option<JsonName<'static>> = json_name("paymentAgentOperation");
    type Type = String;
}
impl MultiField for PaymentAgentOperation {}

/// (1045) Операция банковского платежного субагента
///
/// Наименование операции банковского платежного агента, банковского платежного субагента
#[deprecated]
pub enum PaymentSubagentOperation {}
#[allow(deprecated)]
impl FieldInternal for PaymentSubagentOperation {
    const PADDING: Padding = Padding::None { length: Some(24) };
    const TAG: u16 = 1045;
    type Type = String;
}
#[allow(deprecated)]
impl MultiField for PaymentSubagentOperation {}

/// (1046) Наименование ОФД
///
/// Наименование оператора фискальных данных
pub enum OfdName {}
impl FieldInternal for OfdName {
    const PADDING: Padding = Padding::None { length: Some(256) };
    const TAG: u16 = 1046;
    const JSON_NAME: Option<JsonName<'static>> = json_name("ofdName");
    type Type = String;
}

/// (1047) Параметр настройки (содержит теги 1029/1014/1015)
#[deprecated]
pub enum ConfigParameter {}
#[allow(deprecated)]
impl FieldInternal for ConfigParameter {
    const PADDING: Padding = Padding::None { length: Some(144) };
    const TAG: u16 = 1047;
    type Type = Object;
}

/// (1048) Наименование пользователя
///
/// Наименование организации-пользователя или фамилия, имя, отчество (при наличии) индивидуального предпринимателя - пользователя
pub enum User {}
impl FieldInternal for User {
    const PADDING: Padding = Padding::None { length: Some(256) };
    const TAG: u16 = 1048;
    const JSON_NAME: Option<JsonName<'static>> = json_name("user");
    type Type = String;
}

/// (1049) Почтовый индекс
#[deprecated]
pub enum ZipCode {}
#[allow(deprecated)]
impl FieldInternal for ZipCode {
    const PADDING: Padding = Padding::None { length: Some(6) };
    const TAG: u16 = 1049;
    type Type = String;
}

/// (1050) Признак исчерпания ресурса ФН
///
/// Признак того, что до истечения срока действия ключей фискального признака в фискальном накопителе осталось менее 30 дней
pub enum DriveResourceExhaustionFlag {}
impl FieldInternal for DriveResourceExhaustionFlag {
    const PADDING: Padding = Padding::Right {
        length: 1,
        padding: b'\0',
    };
    const TAG: u16 = 1050;
    const JSON_NAME: Option<JsonName<'static>> = json_name("fiscalDriveExhaustionSign");
    type Type = bool;
}

/// (1051) Признак необходимости срочной замены ФН
///
/// Признак того, что до истечения срока действия ключей фискального признака в фискальном накопителе осталось менее 3 дней
pub enum DriveReplacementRequiredFlag {}
impl FieldInternal for DriveReplacementRequiredFlag {
    const PADDING: Padding = Padding::Right {
        length: 1,
        padding: b'\0',
    };
    const TAG: u16 = 1051;
    const JSON_NAME: Option<JsonName<'static>> = json_name("fiscalDriveReplaceRequiredSign");
    type Type = bool;
}

/// (1052) Признак заполнения памяти ФН
///
/// Признак того, что память фискального накопителя заполнена более чем на 99%
pub enum DriveMemoryFullFlag {}
impl FieldInternal for DriveMemoryFullFlag {
    const PADDING: Padding = Padding::Right {
        length: 1,
        padding: b'\0',
    };
    const TAG: u16 = 1052;
    const JSON_NAME: Option<JsonName<'static>> = json_name("fiscalDriveMemoryExceededSign");
    type Type = bool;
}

/// (1053) Признак превышения времени ожидания ответа ОФД
///
/// Признак того, что подтверждение оператора для переданного фискального документа отсутствует более двух дней
pub enum OfdResponseTimeoutFlag {}
impl FieldInternal for OfdResponseTimeoutFlag {
    const PADDING: Padding = Padding::Right {
        length: 1,
        padding: b'\0',
    };
    const TAG: u16 = 1053;
    const JSON_NAME: Option<JsonName<'static>> = json_name("ofdResponseTimeoutSign");
    type Type = bool;
}

/// (1054) Признак расчета
///
/// Признак расчета (получение средств от покупателя (клиента) «приход», возврат покупателю (клиенту) средств, полученных от него, «возврат прихода», выдача средств покупателю (клиенту) «расход», получение средств от покупателя (клиента), выданных ему, «возврат расхода»)
pub enum PaymentType {}
impl FieldInternal for PaymentType {
    const PADDING: Padding = Padding::Right {
        length: 1,
        padding: b'\0',
    };
    const TAG: u16 = 1054;
    const JSON_NAME: Option<JsonName<'static>> = json_name("operationType");
    type Type = enums::PaymentType;
}

/// (1055) Применяемая система налогообложения
///
/// Система налогообложения, применяемая пользователем при расчете с покупателем (клиентом)
pub enum TaxType {}
impl FieldInternal for TaxType {
    const PADDING: Padding = Padding::Right {
        length: 1,
        padding: b'0',
    };
    const TAG: u16 = 1055;
    const JSON_NAME: Option<JsonName<'static>> = json_name("appliedTaxationType");
    type Type = enums::TaxationTypes;
}

/// (1056) Признак шифрования
///
/// Признак передачи фискальных документов оператору фискальных данных в зашифрованном виде
pub enum EncryptionFlag {}
impl FieldInternal for EncryptionFlag {
    const PADDING: Padding = Padding::Right {
        length: 1,
        padding: b'\0',
    };
    const TAG: u16 = 1056;
    const JSON_NAME: Option<JsonName<'static>> = json_name("encryptionSign");
    type Type = bool;
}

/// (1057) Признак агента
///
/// Признак проведения расчетов (возможности проведения расчетов) пользователем, являющимся агентом, указанным в таблице7
pub enum PaymentAgentTypes {}
impl FieldInternal for PaymentAgentTypes {
    const PADDING: Padding = Padding::Right {
        length: 1,
        padding: b'\0',
    };
    const TAG: u16 = 1057;
    type Type = enums::AgentTypes;
}

/// (1058) Признак банковского агента
#[deprecated]
pub enum BankAgentTypes {}
#[allow(deprecated)]
impl FieldInternal for BankAgentTypes {
    const PADDING: Padding = Padding::Right {
        length: 1,
        padding: b'\0',
    };
    const TAG: u16 = 1058;
    type Type = u8;
}

/// (1059) Предмет расчета
///
/// Наименование (описание) товара, работы, услуги, платежа, выплаты, иного предмета расчета
pub enum ReceiptItem {}
impl FieldInternal for ReceiptItem {
    const PADDING: Padding = Padding::None { length: Some(1024) };
    const TAG: u16 = 1059;
    const JSON_NAME: Option<JsonName<'static>> = json_name("items");
    type Type = Object;
}
impl MultiField for ReceiptItem {}

/// (1060) Адрес сайта ФНС
///
/// Адрес сайта федерального органа исполнительной власти (далее – уполномоченный орган), уполномоченного по контролю и надзору за применением ККТ в информационно-телекоммуникационной сети «Интернет» (далее – сеть «Интернет»)
pub enum FnsUrl {}
impl FieldInternal for FnsUrl {
    const PADDING: Padding = Padding::None { length: Some(256) };
    const TAG: u16 = 1060;
    const JSON_NAME: Option<JsonName<'static>> = json_name("fnsUrl");
    type Type = String;
}

/// (1061) Адрес сайта ОФД
pub enum OfdUrl {}
impl FieldInternal for OfdUrl {
    const PADDING: Padding = Padding::None { length: Some(64) };
    const TAG: u16 = 1061;
    type Type = String;
}

/// (1062) Системы налогообложения
///
/// Системы налогообложения, которые пользователь может применять при осуществлении расчетов
pub enum TaxationTypes {}
impl FieldInternal for TaxationTypes {
    const PADDING: Padding = Padding::Right {
        length: 1,
        padding: b'\0',
    };
    const TAG: u16 = 1062;
    const JSON_NAME: Option<JsonName<'static>> = json_name("taxationType");
    type Type = enums::TaxationTypes;
}

/// (1063) Скидка (ставка)
#[deprecated]
pub enum DiscountRate {}
#[allow(deprecated)]
impl FieldInternal for DiscountRate {
    const PADDING: Padding = Padding::None { length: Some(8) };
    const TAG: u16 = 1063;
    type Type = VarFloat;
}

/// (1064) Скидка (сумма)
#[deprecated]
pub enum DiscountSum {}
#[allow(deprecated)]
impl FieldInternal for DiscountSum {
    const PADDING: Padding = Padding::None { length: Some(8) };
    const TAG: u16 = 1064;
    type Type = u64;
}

/// (1065) Сокращенное наименование налога
#[deprecated]
pub enum TaxName {}
#[allow(deprecated)]
impl FieldInternal for TaxName {
    const PADDING: Padding = Padding::None { length: Some(10) };
    const TAG: u16 = 1065;
    type Type = String;
}

/// (1066) Сообщение
#[deprecated]
pub enum Message {}
#[allow(deprecated)]
impl FieldInternal for Message {
    const PADDING: Padding = Padding::None { length: Some(256) };
    const TAG: u16 = 1066;
    type Type = String;
}

/// (1067) Сообщение оператора для ККТ (содержит теги 1019/1047)
#[deprecated]
pub enum OperatorMessageToKkt {}
#[allow(deprecated)]
impl FieldInternal for OperatorMessageToKkt {
    const PADDING: Padding = Padding::None { length: Some(328) };
    const TAG: u16 = 1067;
    type Type = Object;
}

/// (1068) Сообщение оператора для ФН (содержит теги 1022/1047)
///
/// Код информационного сообщения оператора фискальных данных
pub enum OperatorMessageToFn {}
impl FieldInternal for OperatorMessageToFn {
    const PADDING: Padding = Padding::None { length: Some(169) };
    const TAG: u16 = 1068;
    type Type = Object;
}

/// (1069) Сообщение оператору
#[deprecated]
pub enum MessageForOperator {}
#[allow(deprecated)]
impl FieldInternal for MessageForOperator {
    const PADDING: Padding = Padding::None { length: Some(225) };
    const TAG: u16 = 1069;
    type Type = Object;
}

/// (1070) Ставка налога
#[deprecated]
pub enum TaxRate {}
#[allow(deprecated)]
impl FieldInternal for TaxRate {
    const PADDING: Padding = Padding::None { length: Some(5) };
    const TAG: u16 = 1070;
    type Type = VarFloat;
}

/// (1071) Сторно товара (реквизиты) (содержит реквизиты товара в обычном формате)
#[deprecated]
pub enum StornoItems {}
#[allow(deprecated)]
impl FieldInternal for StornoItems {
    const PADDING: Padding = Padding::None { length: Some(132) };
    const TAG: u16 = 1071;
    type Type = Object;
}

/// (1072) Сумма налога
#[deprecated]
pub enum TaxSum {}
#[allow(deprecated)]
impl FieldInternal for TaxSum {
    const PADDING: Padding = Padding::None { length: Some(8) };
    const TAG: u16 = 1072;
    type Type = u64;
}

/// (1073) Телефон платежного агента
///
/// Номера телефонов платежного агента, платежного субагента, банковского платежного агента, банковского платежного субагента
pub enum PaymentAgentPhone {}
impl FieldInternal for PaymentAgentPhone {
    const PADDING: Padding = Padding::None { length: Some(19) };
    const TAG: u16 = 1073;
    const JSON_NAME: Option<JsonName<'static>> = json_name("paymentAgentPhone");
    type Type = String;
}
impl MultiField for PaymentAgentPhone {}

/// (1074) Телефон оператора по приему платежей
///
/// Номера контактных телефонов оператора по приему платежей
pub enum PaymentOperatorPhone {}
impl FieldInternal for PaymentOperatorPhone {
    const PADDING: Padding = Padding::None { length: Some(19) };
    const TAG: u16 = 1074;
    const JSON_NAME: Option<JsonName<'static>> = json_name("paymentOperatorPhone");
    type Type = String;
}
impl MultiField for PaymentOperatorPhone {}

/// (1075) Телефон оператора перевода
///
/// Номера телефонов оператора по переводу денежных средств
pub enum TransferOperatorPhone {}
impl FieldInternal for TransferOperatorPhone {
    const PADDING: Padding = Padding::None { length: Some(19) };
    const TAG: u16 = 1075;
    const JSON_NAME: Option<JsonName<'static>> = json_name("transferOperatorPhone");
    type Type = String;
}
impl MultiField for TransferOperatorPhone {}

/// (1076) Тип сообщения
#[deprecated]
pub enum MessageType {}
#[allow(deprecated)]
impl FieldInternal for MessageType {
    const PADDING: Padding = Padding::None { length: Some(64) };
    const TAG: u16 = 1076;
    type Type = String;
}

/// (1077) ФПД
///
/// Фискальный признак документа
pub enum DocFiscalSign {}
impl FieldInternal for DocFiscalSign {
    const PADDING: Padding = Padding::Fixed { length: 6 };
    const TAG: u16 = 1077;
    const JSON_NAME: Option<JsonName<'static>> = json_name("fiscalSign");
    type Type = [u8; 6];
}

/// (1078) ФПО
///
/// Фискальный признак оператора
pub enum OperatorFp {}
impl FieldInternal for OperatorFp {
    const PADDING: Padding = Padding::None { length: Some(16) };
    const TAG: u16 = 1078;
    type Type = Vec<u8>;
}

/// (1079) Цена за единицу предмета расчета с учетом скидок и наценок
///
/// Цена за единицу товара, работы, услуги, платежа, выплаты, иного предмета расчета с учетом скидок и наценок
pub enum ItemUnitPrice {}
impl FieldInternal for ItemUnitPrice {
    const PADDING: Padding = Padding::None { length: Some(6) };
    const TAG: u16 = 1079;
    const JSON_NAME: Option<JsonName<'static>> = json_name("price");
    type Type = u64;
}

/// (1080) Штриховой код EAN13
#[deprecated]
pub enum Ean13 {}
#[allow(deprecated)]
impl FieldInternal for Ean13 {
    const PADDING: Padding = Padding::None { length: Some(16) };
    const TAG: u16 = 1080;
    type Type = String;
}

/// (1081) Сумма по чеку (БСО) безналичными
///
/// Сумма расчета, указанная в кассовом чеке (БСО), или сумма корректировки расчета, указанная в кассовом чеке коррекции (БСО коррекции), подлежащая уплате в безналичном порядке
pub enum TotalEcashSum {}
impl FieldInternal for TotalEcashSum {
    const PADDING: Padding = Padding::None { length: Some(6) };
    const TAG: u16 = 1081;
    const JSON_NAME: Option<JsonName<'static>> = json_name("ecashTotalSum");
    type Type = u64;
}

/// (1082) Телефон банковского субагента
#[deprecated]
pub enum BankSubagentPhone {}
#[allow(deprecated)]
impl FieldInternal for BankSubagentPhone {
    const PADDING: Padding = Padding::None { length: Some(19) };
    const TAG: u16 = 1082;
    type Type = String;
}
#[allow(deprecated)]
impl MultiField for BankSubagentPhone {}

/// (1083) Телефон платежного субагента
#[deprecated]
pub enum PaymentSubagentPhone {}
#[allow(deprecated)]
impl FieldInternal for PaymentSubagentPhone {
    const PADDING: Padding = Padding::None { length: Some(19) };
    const TAG: u16 = 1082;
    type Type = String;
}
#[allow(deprecated)]
impl MultiField for PaymentSubagentPhone {}

/// (1084) Дополнительный реквизит пользователя
///
/// Дополнительный реквизит пользователя с учетом особенностей сферы деятельности, в которой осуществляются расчеты
pub enum AdditionalUserProp {}
impl FieldInternal for AdditionalUserProp {
    const PADDING: Padding = Padding::None { length: Some(320) };
    const TAG: u16 = 1084;
    const JSON_NAME: Option<JsonName<'static>> = json_name("properties");
    type Type = Object;
}

/// (1085) Наименование дополнительного реквизита пользователя
///
/// Наименование дополнительного реквизита пользователя с учетом особенностей сферы деятельности, в которой осуществляются расчеты
pub enum AdditionalUserPropName {}
impl FieldInternal for AdditionalUserPropName {
    const PADDING: Padding = Padding::None { length: Some(64) };
    const TAG: u16 = 1085;
    const JSON_NAME: Option<JsonName<'static>> = json_name("propertyName");
    type Type = String;
}

/// (1086) Значение дополнительного реквизита пользователя
///
/// Значение дополнительного реквизита пользователя с учетом особенностей сферы деятельности, в которой осуществляются расчеты
pub enum AdditionalUserPropValue {}
impl FieldInternal for AdditionalUserPropValue {
    const PADDING: Padding = Padding::None { length: Some(256) };
    const TAG: u16 = 1086;
    const JSON_NAME: Option<JsonName<'static>> = json_name("propertyValue");
    type Type = String;
}

/// (1087) Итог смены
#[deprecated]
pub enum ShiftTotal {}
#[allow(deprecated)]
impl FieldInternal for ShiftTotal {
    const PADDING: Padding = Padding::None { length: Some(8) };
    const TAG: u16 = 1087;
    type Type = u64;
}

/// (1088) Приход наличными
#[deprecated]
pub enum CashSale {}
#[allow(deprecated)]
impl FieldInternal for CashSale {
    const PADDING: Padding = Padding::None { length: Some(8) };
    const TAG: u16 = 1088;
    type Type = u64;
}

/// (1089) Приход электронными
#[deprecated]
pub enum EcashSale {}
#[allow(deprecated)]
impl FieldInternal for EcashSale {
    const PADDING: Padding = Padding::None { length: Some(8) };
    const TAG: u16 = 1089;
    type Type = u64;
}

/// (1090) Возврат прихода наличными
#[deprecated]
pub enum CashSaleReturn {}
#[allow(deprecated)]
impl FieldInternal for CashSaleReturn {
    const PADDING: Padding = Padding::None { length: Some(8) };
    const TAG: u16 = 1090;
    type Type = u64;
}

/// (1091) Возврат прихода электронными
#[deprecated]
pub enum EcashSaleReturn {}
#[allow(deprecated)]
impl FieldInternal for EcashSaleReturn {
    const PADDING: Padding = Padding::None { length: Some(8) };
    const TAG: u16 = 1091;
    type Type = u64;
}

/// (1092) Расход наличными
#[deprecated]
pub enum CashPurchase {}
#[allow(deprecated)]
impl FieldInternal for CashPurchase {
    const PADDING: Padding = Padding::None { length: Some(8) };
    const TAG: u16 = 1092;
    type Type = u64;
}

/// (1093) Расход электронными
#[deprecated]
pub enum EcashPurchase {}
#[allow(deprecated)]
impl FieldInternal for EcashPurchase {
    const PADDING: Padding = Padding::None { length: Some(8) };
    const TAG: u16 = 1093;
    type Type = u64;
}

/// (1094) Возврат расхода наличными
#[deprecated]
pub enum CashPurchaseReturn {}
#[allow(deprecated)]
impl FieldInternal for CashPurchaseReturn {
    const PADDING: Padding = Padding::None { length: Some(8) };
    const TAG: u16 = 1094;
    type Type = u64;
}

/// (1095) Возврат расхода электронными
#[deprecated]
pub enum EcashPurchaseReturn {}
#[allow(deprecated)]
impl FieldInternal for EcashPurchaseReturn {
    const PADDING: Padding = Padding::None { length: Some(8) };
    const TAG: u16 = 1095;
    type Type = u64;
}

/// (1096) Номер корректируемого фискального документа
#[deprecated]
pub enum CorrectedDocNum {}
#[allow(deprecated)]
impl FieldInternal for CorrectedDocNum {
    const PADDING: Padding = Padding::None { length: Some(8) };
    const TAG: u16 = 1096;
    type Type = u64;
}

/// (1097) Количество непереданных ФД
///
/// Количество ФД, по которым не были получены подтверждения оператора
pub enum UntransmittedDocCount {}
impl FieldInternal for UntransmittedDocCount {
    const PADDING: Padding = Padding::Right {
        length: 4,
        padding: b'\0',
    };
    const TAG: u16 = 1097;
    const JSON_NAME: Option<JsonName<'static>> = json_name("notTransmittedDocumentsQuantity");
    type Type = u32;
}

/// (1098) Дата первого из непереданных ФД
///
/// Дата первого ФД, для которого не было получено подтверждение оператора
pub enum UntransmittedDocDateTime {}
impl FieldInternal for UntransmittedDocDateTime {
    const PADDING: Padding = Padding::Right {
        length: 4,
        padding: b'\0',
    };
    const TAG: u16 = 1098;
    const JSON_NAME: Option<JsonName<'static>> = json_name("notTransmittedDocumentsDateTime");
    type Type = chrono::NaiveDate;
}

/// (1099) Сводный итог
#[deprecated]
pub enum SumTotal {}
#[allow(deprecated)]
impl FieldInternal for SumTotal {
    const PADDING: Padding = Padding::None { length: Some(8) };
    const TAG: u16 = 1099;
    type Type = u64;
}

/// (1101) Код причины перерегистрации
///
/// Причина изменения сведений о ККТ
pub enum ReregistrationReason {}
impl FieldInternal for ReregistrationReason {
    const PADDING: Padding = Padding::Right {
        length: 1,
        padding: b'\0',
    };
    const TAG: u16 = 1101;
    type Type = enums::ReregistrationReason;
}
impl MultiField for ReregistrationReason {}

/// (1102) Сумма НДС чека по ставке 20%
///
/// Сумма налога на добавленную стоимость, входящая в итоговую стоимость предмета расчета, по ставке налога на добавленную стоимость 20%
pub enum TotalVat20Sum {}
impl FieldInternal for TotalVat20Sum {
    const PADDING: Padding = Padding::None { length: Some(6) };
    const TAG: u16 = 1102;
    const JSON_NAME: Option<JsonName<'static>> = json_name("nds18");
    type Type = u64;
}

/// (1103) Сумма НДС чека по ставке 10%
///
/// Сумма налога на добавленную стоимость, входящая в итоговую стоимость предмета расчета, по ставке налога на добавленную стоимость 10%
pub enum TotalVat10Sum {}
impl FieldInternal for TotalVat10Sum {
    const PADDING: Padding = Padding::None { length: Some(6) };
    const TAG: u16 = 1103;
    const JSON_NAME: Option<JsonName<'static>> = json_name("nds10");
    type Type = u64;
}

/// (1104) Сумма расчета по чеку с НДС по ставке 0%
///
/// Сумма расчетов за предметы расчета, указанные в кассовом чеке (БСО), кассовом чеке коррекции (БСО коррекции), со ставкой налога на добавленную стоимость 0%
pub enum TotalSumWithVat0 {}
impl FieldInternal for TotalSumWithVat0 {
    const PADDING: Padding = Padding::None { length: Some(6) };
    const TAG: u16 = 1104;
    const JSON_NAME: Option<JsonName<'static>> = json_name("nds0");
    type Type = u64;
}

/// (1105) Сумма расчета по чеку без НДС
///
/// Сумма расчетов за предметы расчета, указанные в кассовом чеке (БСО), кассовом чеке коррекции (БСО коррекции), осуществленных пользователем, не являющимся налогоплательщиком налога на добавленную стоимость или освобожденным от исполнения обязанностей налогоплательщика налога на добавленную стоимость, а также сумма расчетов за предметы расчета, не подлежащие налогообложению (освобождаемые от налогообложения) налогом на добавленную стоимость
pub enum TotalSumWithNoVat {}
impl FieldInternal for TotalSumWithNoVat {
    const PADDING: Padding = Padding::None { length: Some(6) };
    const TAG: u16 = 1105;
    const JSON_NAME: Option<JsonName<'static>> = json_name("ndsNo");
    type Type = u64;
}

/// (1106) Сумма НДС чека по расч. ставке 20/120
///
/// Сумма налога на добавленную стоимость, входящая в итоговую стоимость предметов расчета, указанных в кассовом чеке (БСО), кассовом чеке коррекции (БСО коррекции), по расчетной ставке налога на добавленную стоимость 20/120
pub enum TotalVat20_120Sum {}
impl FieldInternal for TotalVat20_120Sum {
    const PADDING: Padding = Padding::None { length: Some(6) };
    const TAG: u16 = 1106;
    const JSON_NAME: Option<JsonName<'static>> = json_name("nds18118");
    type Type = u64;
}

/// (1107) Сумма НДС чека по расч. ставке 10/110
///
/// Сумма налога на добавленную стоимость, входящая в итоговую стоимость предметов расчета, указанных в кассовом чеке (БСО), кассовом чеке коррекции (БСО коррекции), по расчетной ставке налога на добавленную стоимость 10/110
pub enum TotalVat10_110Sum {}
impl FieldInternal for TotalVat10_110Sum {
    const PADDING: Padding = Padding::None { length: Some(6) };
    const TAG: u16 = 1107;
    const JSON_NAME: Option<JsonName<'static>> = json_name("nds10110");
    type Type = u64;
}

/// (1108) Признак ККТ для расчетов только в Интернет
///
/// Признак ККТ, предназначенной для осуществления расчетов только в сети «Интернет», в которой отсутствует устройство для печати фискальных документов в составе ККТ
pub enum OnlineKktFlag {}
impl FieldInternal for OnlineKktFlag {
    const PADDING: Padding = Padding::Right {
        length: 1,
        padding: b'\0',
    };
    const TAG: u16 = 1108;
    const JSON_NAME: Option<JsonName<'static>> = json_name("internetSign");
    type Type = bool;
}

/// (1109) Признак расчетов за услуги
///
/// Признак применения ККТ только при оказании услуг
pub enum ServiceFlag {}
impl FieldInternal for ServiceFlag {
    const PADDING: Padding = Padding::Right {
        length: 1,
        padding: b'\0',
    };
    const TAG: u16 = 1109;
    type Type = bool;
}

/// (1110) Признак АС БСО
///
/// Признак ККТ, являющейся автоматизированной системой для БСО (может формировать только БСО и применяться для осуществления расчетов только при оказании услуг)
pub enum BsoFlag {}
impl FieldInternal for BsoFlag {
    const PADDING: Padding = Padding::Right {
        length: 1,
        padding: b'\0',
    };
    const TAG: u16 = 1110;
    type Type = bool;
}

/// (1111) Общее количество ФД за смену
///
/// Общее количество ФД, сформированных ККТ за смену
pub enum DocCountPerShift {}
impl FieldInternal for DocCountPerShift {
    const PADDING: Padding = Padding::Right {
        length: 4,
        padding: b'\0',
    };
    const TAG: u16 = 1111;
    type Type = u32;
}

/// (1112) Скидка/наценка (содержит объекты с тегами 1113/1114/1063/1034/1064/1035)
#[deprecated]
pub enum Modifiers {}
#[allow(deprecated)]
impl FieldInternal for Modifiers {
    const PADDING: Padding = Padding::None { length: Some(160) };
    const TAG: u16 = 1112;
    type Type = Object;
}

/// (1113) Наименование скидки
#[deprecated]
pub enum DiscountName {}
#[allow(deprecated)]
impl FieldInternal for DiscountName {
    const PADDING: Padding = Padding::None { length: Some(64) };
    const TAG: u16 = 1113;
    type Type = String;
}

/// (1114) Наименование наценки
#[deprecated]
pub enum MarkupName {}
#[allow(deprecated)]
impl FieldInternal for MarkupName {
    const PADDING: Padding = Padding::None { length: Some(64) };
    const TAG: u16 = 1114;
    type Type = String;
}

/// (1115) Адрес сайта для проверки ФП
#[deprecated]
pub enum FiscalSignCheckUrl {}
#[allow(deprecated)]
impl FieldInternal for FiscalSignCheckUrl {
    const TAG: u16 = 1115;
    type Type = String;
}

/// (1116) Номер первого непереданного документа
///
/// Номер первого ФД из числа не переданных ОФД
pub enum UntransmittedDocNum {}
impl FieldInternal for UntransmittedDocNum {
    const PADDING: Padding = Padding::Right {
        length: 4,
        padding: b'\0',
    };
    const TAG: u16 = 1116;
    const JSON_NAME: Option<JsonName<'static>> = json_name("notTransmittedDocumentNumber");
    type Type = u32;
}

/// (1117) Адрес электронной почты отправителя чека
///
/// Адрес электронной почты отправителя кассового чека (БСО), кассового чека коррекции (БСО коррекции) в электронной форме, в том числе пользователя или ОФД, если отправителем является пользователь или ОФД, соответственно, в случае передачи покупателю (клиенту) кассового чека или бланка строгой отчетности в электронной форме
pub enum ReceiptSenderEmail {}
impl FieldInternal for ReceiptSenderEmail {
    const PADDING: Padding = Padding::None { length: Some(64) };
    const TAG: u16 = 1117;
    const JSON_NAME: Option<JsonName<'static>> = json_name("sellerAddress");
    type Type = String;
}

/// (1118) Количество кассовых чеков (БСО) за смену
///
/// Количество кассовых чеков (БСО) со всеми признаками расчетов и кассовых чеков коррекции (БСО коррекции) со всеми признаками расчетов, сформированных ККТ за текущую смену
pub enum ReceiptCountPerShift {}
impl FieldInternal for ReceiptCountPerShift {
    const PADDING: Padding = Padding::Right {
        length: 4,
        padding: b'\0',
    };
    const TAG: u16 = 1118;
    type Type = u32;
}

/// (1119) Телефон платежного субагента
#[deprecated]
pub enum OldPaymentOperatorPhone {}
#[allow(deprecated)]
impl FieldInternal for OldPaymentOperatorPhone {
    const PADDING: Padding = Padding::None { length: Some(19) };
    const TAG: u16 = 1119;
    type Type = String;
}
#[allow(deprecated)]
impl MultiField for OldPaymentOperatorPhone {}

/// (1120) Код справочника
#[deprecated]
pub enum ReferenceCode {}
#[allow(deprecated)]
impl FieldInternal for ReferenceCode {
    const PADDING: Padding = Padding::None { length: Some(16) };
    const TAG: u16 = 1120;
    type Type = u64;
}

/// (1121) Код классификации товара
#[deprecated]
pub enum ProductClassificationCode {}
#[allow(deprecated)]
impl FieldInternal for ProductClassificationCode {
    const PADDING: Padding = Padding::None { length: Some(16) };
    const TAG: u16 = 1121;
    type Type = u64;
}

/// (1122) Сведения о классификации товара
#[deprecated]
pub enum ProductClassificationInfo {}
#[allow(deprecated)]
impl FieldInternal for ProductClassificationInfo {
    const PADDING: Padding = Padding::None { length: Some(16) };
    const TAG: u16 = 1122;
    type Type = String;
}

/// (1123) Код классификации товара
#[deprecated]
pub enum ProductIdentificationCode {}
#[allow(deprecated)]
impl FieldInternal for ProductIdentificationCode {
    const PADDING: Padding = Padding::None { length: Some(24) };
    const TAG: u16 = 1123;
    type Type = u64;
}

/// (1124) Сведения о классификации товара
#[deprecated]
pub enum ProductIdentificationInfo {}
#[allow(deprecated)]
impl FieldInternal for ProductIdentificationInfo {
    const PADDING: Padding = Padding::None { length: Some(16) };
    const TAG: u16 = 1124;
    type Type = String;
}

/// (1125) Наименование ОФД
#[deprecated]
pub enum OldOfdName {}
#[allow(deprecated)]
impl FieldInternal for OldOfdName {
    const PADDING: Padding = Padding::None { length: Some(256) };
    const TAG: u16 = 1125;
    type Type = String;
}

/// (1126) Кол-во непереданных документов по наличным расчетам
#[deprecated]
pub enum UntransmittedCashReceiptCount {}
#[allow(deprecated)]
impl FieldInternal for UntransmittedCashReceiptCount {
    const PADDING: Padding = Padding::Right {
        length: 4,
        padding: b'\0',
    };
    const TAG: u16 = 1126;
    type Type = u32;
}

/// (1127) Кол-во непереданных документов по наличным расчетам
#[deprecated]
pub enum UntransmittedEcashReceiptCount {}
#[allow(deprecated)]
impl FieldInternal for UntransmittedEcashReceiptCount {
    const PADDING: Padding = Padding::Right {
        length: 4,
        padding: b'\0',
    };
    const TAG: u16 = 1127;
    type Type = u32;
}

/// (1126) Признак проведения лотереи
///
/// Признак применения ККТ при проведении расчетов при реализации лотерейных билетов, электронных лотерейных билетов, приеме лотерейных ставок и выплате денежных средств в виде выигрыша при осуществлении деятельности по проведению лотерей
pub enum LotteryFlag {}
impl FieldInternal for LotteryFlag {
    const PADDING: Padding = Padding::Right {
        length: 1,
        padding: b'\0',
    };
    const TAG: u16 = 1126;
    type Type = bool;
}

/// (1129) Счетчики операций «приход»
///
/// Итоговые количества и итоговые суммы расчетов кассовых чеков (БСО) и кассовых чеков коррекции (БСО коррекции) с признаком расчета «приход»
pub enum SaleStats {}
impl FieldInternal for SaleStats {
    const PADDING: Padding = Padding::None { length: Some(116) };
    const TAG: u16 = 1129;
    const JSON_NAME: Option<JsonName<'static>> = json_name("sellOper");
    type Type = Object;
}

/// (1130) Счетчики операций «возврат прихода»
///
/// Итоговые количества и итоговые суммы расчетов кассовых чеков (БСО) и кассовых чеков коррекции (БСО коррекции) с признаком расчета «возврат прихода»
pub enum SaleReturnStats {}
impl FieldInternal for SaleReturnStats {
    const PADDING: Padding = Padding::None { length: Some(116) };
    const TAG: u16 = 1130;
    const JSON_NAME: Option<JsonName<'static>> = json_name("sellReturnOper");
    type Type = Object;
}

/// (1131) Счетчики операций «расход»
///
/// Итоговые количества и итоговые суммы расчетов кассовых чеков (БСО) и кассовых чеков коррекции (БСО коррекции) с признаком расчета «расход»
pub enum PurchaseStats {}
impl FieldInternal for PurchaseStats {
    const PADDING: Padding = Padding::None { length: Some(116) };
    const TAG: u16 = 1131;
    const JSON_NAME: Option<JsonName<'static>> = json_name("buyOper");
    type Type = Object;
}

/// (1132) Счетчики операций «возврат расхода»
///
/// Итоговые количества и итоговые суммы расчетов кассовых чеков (БСО) и кассовых чеков коррекции (БСО коррекции) с признаком расчета «возврат расхода»
pub enum PurchaseReturnStats {}
impl FieldInternal for PurchaseReturnStats {
    const PADDING: Padding = Padding::None { length: Some(116) };
    const TAG: u16 = 1132;
    const JSON_NAME: Option<JsonName<'static>> = json_name("buyReturnOper");
    type Type = Object;
}

/// (1133) Счетчики операций по чекам коррекции (БСО коррекции)
///
/// Итоговые количества и итоговые суммы расчетов кассовых чеков коррекции (БСО коррекции)
pub enum CorrectionStats {}
impl FieldInternal for CorrectionStats {
    const PADDING: Padding = Padding::None { length: Some(216) };
    const TAG: u16 = 1133;
    const JSON_NAME: Option<JsonName<'static>> = json_name("receiptCorrection");
    type Type = Object;
}

/// (1134) Количество чеков (БСО) и чеков коррекции (БСО коррекции) со всеми признаками расчетов
///
/// Количество кассовых чеков (БСО) и кассовых чеков коррекции (БСО коррекции) со всеми признаками расчетов («приход», «расход», «возврат прихода», «возврат расхода»)
pub enum TotalReceiptAndCorrectionCount {}
impl FieldInternal for TotalReceiptAndCorrectionCount {
    const PADDING: Padding = Padding::Right {
        length: 4,
        padding: b'\0',
    };
    const TAG: u16 = 1134;
    const JSON_NAME: Option<JsonName<'static>> = json_name("totalReceiptBsoCount");
    type Type = u32;
}

/// (1135) Количество чеков (БСО) по признаку расчетов
///
/// Количество кассовых чеков (БСО) и (или) кассовых чеков коррекции (БСО коррекции) или непереданных кассовых чеков (БСО) и кассовых чеков коррекции (БСО коррекции) по одному из признаков расчетов («приход», «расход», «возврат прихода», «возврат расхода»)
pub enum AggregatedReceiptCount {}
impl FieldInternal for AggregatedReceiptCount {
    const PADDING: Padding = Padding::Right {
        length: 4,
        padding: b'\0',
    };
    const TAG: u16 = 1135;
    const JSON_NAME: Option<JsonName<'static>> = json_name("receiptBsoCount");
    type Type = u32;
}

/// (1136) Итоговая сумма в чеках (БСО) наличными денежными средствами
///
/// Итоговая сумма расчетов, указанных в кассовых чеках (БСО) и кассовых чеках коррекции (БСО коррекции), совершенных с использованием наличных денежных средств
pub enum AggregatedCashSum {}
impl FieldInternal for AggregatedCashSum {
    const PADDING: Padding = Padding::None { length: Some(6) };
    const TAG: u16 = 1136;
    const JSON_NAME: Option<JsonName<'static>> = json_name("cashSum");
    type Type = u64;
}

/// (1138) Итоговая сумма в чеках (БСО) безналичными
///
/// Итоговая сумма расчетов, указанных в кассовых чеках (БСО) и кассовых чеках коррекции (БСО коррекции), совершенных в безналичном порядке
pub enum AggregatedEcashSum {}
impl FieldInternal for AggregatedEcashSum {
    const PADDING: Padding = Padding::None { length: Some(6) };
    const TAG: u16 = 1138;
    const JSON_NAME: Option<JsonName<'static>> = json_name("ecashSum");
    type Type = u64;
}

/// (1139) Сумма НДС по ставке 20%
///
/// Итоговая сумма налога на добавленную стоимость по ставке 20%, указанная в кассовых чеках (БСО) и кассовых чеках коррекции (БСО коррекции) с одним из признаков расчета: «приход», «расход», «возврат прихода», «возврат расхода»
pub enum AggregatedVat20Sum {}
impl FieldInternal for AggregatedVat20Sum {
    const PADDING: Padding = Padding::None { length: Some(6) };
    const TAG: u16 = 1139;
    const JSON_NAME: Option<JsonName<'static>> = json_name("tax18Sum");
    type Type = u64;
}

/// (1140) Сумма НДС по ставке 10%
///
/// Итоговая сумма налога на добавленную стоимость по ставке 10%, указанная в кассовых чеках (БСО) и кассовых чеках коррекции (БСО коррекции) с одним из признаков расчета: «приход», «расход», «возврат прихода», «возврат расхода»
pub enum AggregatedVat10Sum {}
impl FieldInternal for AggregatedVat10Sum {
    const PADDING: Padding = Padding::None { length: Some(6) };
    const TAG: u16 = 1140;
    const JSON_NAME: Option<JsonName<'static>> = json_name("tax10Sum");
    type Type = u64;
}

/// (1141) Сумма НДС по расч. ставке 20/120
///
/// Итоговая сумма налога на добавленную стоимость по расчетной ставке 20/120, указанная в кассовых чеках (БСО) и кассовых чеках коррекции (БСО коррекции) с одним из признаков расчета: «приход», «расход», «возврат прихода», «возврат расхода»
pub enum AggregatedVat20_120Sum {}
impl FieldInternal for AggregatedVat20_120Sum {
    const PADDING: Padding = Padding::None { length: Some(6) };
    const TAG: u16 = 1141;
    const JSON_NAME: Option<JsonName<'static>> = json_name("tax18118Sum");
    type Type = u64;
}

/// (1142) Сумма НДС по расч. ставке 10/110
///
/// Итоговая сумма налога на добавленную стоимость по расчетной ставке 10/110, указанная в кассовых чеках (БСО) и кассовых чеках коррекции (БСО коррекции) с одним из признаков расчета: «приход», «расход», «возврат прихода», «возврат расхода»
pub enum AggregatedVat10_110Sum {}
impl FieldInternal for AggregatedVat10_110Sum {
    const PADDING: Padding = Padding::None { length: Some(6) };
    const TAG: u16 = 1142;
    const JSON_NAME: Option<JsonName<'static>> = json_name("tax10110Sum");
    type Type = u64;
}

/// (1143) Сумма расчетов с НДС по ставке 0%
///
/// Итоговая сумма расчетов, указанных в кассовых чеках (БСО) и кассовых чеках коррекции (БСО коррекции) со ставкой налога на добавленную стоимость 0%
pub enum AggregatedSumWithVat0 {}
impl FieldInternal for AggregatedSumWithVat0 {
    const PADDING: Padding = Padding::None { length: Some(6) };
    const TAG: u16 = 1143;
    const JSON_NAME: Option<JsonName<'static>> = json_name("tax0Sum");
    type Type = u64;
}

/// (1144) Количество чеков коррекции (БСО коррекции) или непереданных чеков (БСО) и чеков коррекции (БСО коррекции)
///
/// Количество кассовых чеков коррекции (БСО коррекции), сформированных ККТ, либо количество непереданных кассовых чеков (БСО) и кассовых чеков коррекции (БСО коррекции) ККТ со всеми признаками расчетов
pub enum CorrectionAndUntransmittedCount {}
impl FieldInternal for CorrectionAndUntransmittedCount {
    const PADDING: Padding = Padding::Right {
        length: 4,
        padding: b'\0',
    };
    const TAG: u16 = 1144;
    const JSON_NAME: Option<JsonName<'static>> = json_name("receiptCorrectionCount");
    type Type = u32;
}

/// (1145) Счетчики по признаку «приход»
///
/// Итоговые количества и итоговые суммы кассовых чеков коррекции (БСО коррекции), а также итоговые количества и итоговые суммы непереданных кассовых чеков (БСО) и кассовых чеков коррекции (БСО коррекции) с признаком расчета «приход»
pub enum UntransmittedSaleStats {}
impl FieldInternal for UntransmittedSaleStats {
    const PADDING: Padding = Padding::None { length: Some(32) };
    const TAG: u16 = 1145;
    const JSON_NAME: Option<JsonName<'static>> = json_name("sellCorrection");
    type Type = Object;
}

/// (1146) Счетчики по признаку «расход»
///
/// Итоговые количества и итоговые суммы кассовых чеков коррекции (БСО коррекции), а также итоговые количества и итоговые суммы непереданных кассовых чеков (БСО) и кассовых чеков коррекции (БСО коррекции) с признаком расчета «расход»
pub enum UntransmittedPurchaseStats {}
impl FieldInternal for UntransmittedPurchaseStats {
    const PADDING: Padding = Padding::None { length: Some(32) };
    const TAG: u16 = 1146;
    const JSON_NAME: Option<JsonName<'static>> = json_name("buyCorrection");
    type Type = Object;
}

/// (1157) Счетчики итогов ФН
///
/// Итоговые суммы расчетов, указанных в кассовых чеках (БСО) или в кассовых чеках коррекции (БСО коррекции), зафиксированные в счетчиках итогов ФН
pub enum DriveStats {}
impl FieldInternal for DriveStats {
    const PADDING: Padding = Padding::None { length: Some(708) };
    const TAG: u16 = 1157;
    const JSON_NAME: Option<JsonName<'static>> = json_name("fiscalDriveSumReports");
    type Type = Object;
}

/// (1158) Счетчики итогов непереданных ФД
///
/// Итоговые количества и итоговые суммы расчетов непереданных кассовых чеков (БСО) и кассовых чеков коррекции (БСО коррекции)
pub enum DriveUntransmittedStats {}
impl FieldInternal for DriveUntransmittedStats {
    const PADDING: Padding = Padding::None { length: Some(708) };
    const TAG: u16 = 1158;
    const JSON_NAME: Option<JsonName<'static>> = json_name("notTransmittedDocumentsSumReports");
    type Type = Object;
}

/// (1162) Код товара
///
/// Код товара, описание указано в таблице 26
pub enum ProductCode {}
impl FieldInternal for ProductCode {
    const PADDING: Padding = Padding::None { length: Some(32) };
    const TAG: u16 = 1162;
    const JSON_NAME: Option<JsonName<'static>> = json_name("productCode");
    type Type = Vec<u8>;
}

/// (1163) Код товара
///
/// Код товара, описание указано в таблице 118
pub enum ProductCodeNew {}
impl FieldInternal for ProductCodeNew {
    const PADDING: Padding = Padding::None { length: Some(256) };
    const TAG: u16 = 1163;
    const JSON_NAME: Option<JsonName<'static>> = json_name("productCodeNew");
    type Type = Object;
}

/// (1171) Телефон поставщика
///
/// Номера контактных телефонов поставщика
pub enum SupplierPhone {}
impl FieldInternal for SupplierPhone {
    const PADDING: Padding = Padding::None { length: Some(19) };
    const TAG: u16 = 1171;
    const JSON_NAME: Option<JsonName<'static>> = json_name("providerPhone");
    type Type = String;
}
impl MultiField for SupplierPhone {}

/// (1173) Тип коррекции
///
/// Тип коррекции
pub enum CorrectionType {}
impl FieldInternal for CorrectionType {
    const PADDING: Padding = Padding::Right {
        length: 1,
        padding: b'\0',
    };
    const TAG: u16 = 1173;
    const JSON_NAME: Option<JsonName<'static>> = json_name("correctionType");
    type Type = enums::CorrectionType;
}

/// (1174) Основание для коррекции
///
/// Основание для коррекции
pub enum CorrectionBasis {}
impl FieldInternal for CorrectionBasis {
    const PADDING: Padding = Padding::None { length: Some(292) };
    const TAG: u16 = 1174;
    const JSON_NAME: Option<JsonName<'static>> = json_name("сorrectionBase");
    type Type = Object;
}

/// (1178) Дата совершения корректируемого расчета
///
/// Дата совершения расчета, в отношении к которому формируется кассовый чек коррекции (БСО коррекции)
pub enum CorrectedPaymentDate {}
impl FieldInternal for CorrectedPaymentDate {
    const PADDING: Padding = Padding::Right {
        length: 4,
        padding: b'\0',
    };
    const TAG: u16 = 1178;
    const JSON_NAME: Option<JsonName<'static>> = json_name("correctionDocumentDate");
    type Type = chrono::NaiveDate;
}

/// (1179) Номер предписания налогового органа
///
/// Номер предписания налогового органа об устранении выявленного нарушения законодательства Российской Федерации о применении ККТ
pub enum FnsActNumber {}
impl FieldInternal for FnsActNumber {
    const PADDING: Padding = Padding::None { length: Some(32) };
    const TAG: u16 = 1179;
    const JSON_NAME: Option<JsonName<'static>> = json_name("correctionDocumentNumber");
    type Type = String;
}

/// (1183) Сумма расчетов без НДС
///
/// Итоговая сумма расчетов, указанных в кассовых чеках (БСО) и кассовых чеках коррекции (БСО коррекции) с одним из признаков расчета: «приход», «возврат прихода», «расход», «возврат расхода»
pub enum AggregatedSumWithNoVat {}
impl FieldInternal for AggregatedSumWithNoVat {
    const PADDING: Padding = Padding::None { length: Some(6) };
    const TAG: u16 = 1183;
    const JSON_NAME: Option<JsonName<'static>> = json_name("taxFreeSum");
    type Type = u64;
}

/// (1187) Место расчетов
///
/// Место осуществления расчетов между пользователем и покупателем (клиентом), позволяющее покупателю (клиенту) идентифицировать место расчета. В случае применения ККТ с автоматическим устройством для расчетов место нахождения этого автоматического устройства для расчетов
pub enum RetailPlace {}
impl FieldInternal for RetailPlace {
    const PADDING: Padding = Padding::None { length: Some(256) };
    const TAG: u16 = 1187;
    const JSON_NAME: Option<JsonName<'static>> = json_name("retailPlace");
    type Type = String;
}

/// (1188) Версия ККТ
///
/// Версия модели контрольно-кассовой техники
pub enum KktVer {}
impl FieldInternal for KktVer {
    const PADDING: Padding = Padding::None { length: Some(8) };
    const TAG: u16 = 1188;
    const JSON_NAME: Option<JsonName<'static>> = json_name("kktVersion");
    type Type = String;
}

/// (1189) Версия ФФД ККТ
///
/// Версия форматов фискальных документов с максимальным номером, реализованная в ККТ, в соответствии с реестром ККТ
pub enum KktFfdVer {}
impl FieldInternal for KktFfdVer {
    const PADDING: Padding = Padding::Right {
        length: 1,
        padding: b'\0',
    };
    const TAG: u16 = 1189;
    const JSON_NAME: Option<JsonName<'static>> = json_name("documentKktVersion");
    type Type = enums::FfdVersion;
}

/// (1190) Версия ФФД ФН
///
/// Версия форматов фискальных документов с максимальным номером, реализованная в ФН, в соответствии с реестром ФН
pub enum DriveFfdVer {}
impl FieldInternal for DriveFfdVer {
    const PADDING: Padding = Padding::Right {
        length: 1,
        padding: b'\0',
    };
    const TAG: u16 = 1190;
    const JSON_NAME: Option<JsonName<'static>> = json_name("documentFdVersion");
    type Type = enums::FfdVersion;
}

/// (1191) Дополнительный реквизит предмета расчета
///
/// Наименование дополнительного реквизита с учетом особенностей сферы деятельности, в которой осуществляются расчеты
pub enum AdditionalItemProp {}
impl FieldInternal for AdditionalItemProp {
    const PADDING: Padding = Padding::None { length: Some(64) };
    const TAG: u16 = 1191;
    const JSON_NAME: Option<JsonName<'static>> = json_name("propertiesItem");
    type Type = String;
}

/// (1192) Дополнительный реквизит чека (БСО)
///
/// Значение дополнительного реквизита с учетом особенностей сферы деятельности, в которой осуществляются расчеты
pub enum AdditionalReceiptProp {}
impl FieldInternal for AdditionalReceiptProp {
    const PADDING: Padding = Padding::None { length: Some(16) };
    const TAG: u16 = 1192;
    const JSON_NAME: Option<JsonName<'static>> = json_name("propertiesData");
    type Type = String;
}
impl MultiField for AdditionalReceiptProp {}

/// (1193) Признак проведения азартных игр
///
/// Признак применения ККТ при проведении расчетов при приеме ставок и выплате денежных средств в виде выигрыша при осуществлении деятельности по проведению азартных игр
pub enum GamblingFlag {}
impl FieldInternal for GamblingFlag {
    const PADDING: Padding = Padding::Right {
        length: 1,
        padding: b'\0',
    };
    const TAG: u16 = 1193;
    type Type = bool;
}

/// (1194) Счетчики итогов смены
///
/// Итоговые суммы расчетов, указанных в кассовых чеках (БСО) и кассовых чеках коррекции (БСО коррекции)
pub enum ShiftStats {}
impl FieldInternal for ShiftStats {
    const PADDING: Padding = Padding::None { length: Some(708) };
    const TAG: u16 = 1194;
    type Type = Object;
}

/// (1196) QR-код
///
/// Двумерный штриховой код, размером не менее 20 x 20 мм
pub enum QrCode {}
impl FieldInternal for QrCode {
    const TAG: u16 = 1196;
    type Type = String;
}

/// (1197) Единица измерения предмета расчета
///
/// Единица измерения товара, работы, услуги, платежа, выплаты, иного предмета расчета
pub enum Unit {}
impl FieldInternal for Unit {
    const PADDING: Padding = Padding::None { length: Some(16) };
    const TAG: u16 = 1197;
    type Type = String;
}

/// (1198) Размер НДС за единицу предмета расчета
///
/// Размер налога на добавленную стоимость для единицы товара, работы, услуги, платежа, выплаты, иного предмета расчета
pub enum ItemUnitVat {}
impl FieldInternal for ItemUnitVat {
    const PADDING: Padding = Padding::None { length: Some(6) };
    const TAG: u16 = 1198;
    const JSON_NAME: Option<JsonName<'static>> = json_name("unitNds");
    type Type = u64;
}

/// (1199) Ставка НДС
///
/// Ставка налога на добавленную стоимость товара, работы, услуги, платежа, выплаты, иного предмета расчета
pub enum VatRate {}
impl FieldInternal for VatRate {
    const PADDING: Padding = Padding::Right {
        length: 1,
        padding: b'\0',
    };
    const TAG: u16 = 1199;
    const JSON_NAME: Option<JsonName<'static>> = json_name("nds");
    type Type = enums::VatType;
}

/// (1200) Сумма НДС за предмет расчета
///
/// Сумма налога на добавленную стоимость за товар, работу, услугу, платеж, выплату, иной предмет расчета
pub enum ItemTotalVat {}
impl FieldInternal for ItemTotalVat {
    const PADDING: Padding = Padding::None { length: Some(6) };
    const TAG: u16 = 1200;
    const JSON_NAME: Option<JsonName<'static>> = json_name("ndsSum");
    type Type = u64;
}

/// (1201) Общая итоговая сумма в чеках (БСО)
///
/// Общие итоговые суммы расчетов, указанных в кассовых чеках (БСО) и (или) кассовых чеках коррекции (БСО коррекции), а также в непереданных кассовых чеках (БСО) и кассовых чеках коррекции (БСО коррекции), совершенных, в том числе в виде ранее внесенных оплат (зачетов авансов), последующих оплат (кредитов) и т.д.
pub enum AggregatedSum {}
impl FieldInternal for AggregatedSum {
    const PADDING: Padding = Padding::None { length: Some(6) };
    const TAG: u16 = 1201;
    const JSON_NAME: Option<JsonName<'static>> = json_name("totalSum");
    type Type = u64;
}

/// (1203) ИНН кассира
///
/// Для кассового чека (БСО), кассового чека коррекции (БСО коррекции) ИНН лица, осуществившего расчет с покупателем (клиентом), оформившего кассовый чек (БСО), кассовый чек коррекции (БСО коррекции) и выдавшего (передавшего) его покупателю (клиенту), для иных фискальных документов ИНН лица, уполномоченного пользователем на формирование фискального документа
pub enum OperatorInn {}
impl FieldInternal for OperatorInn {
    const PADDING: Padding = Padding::Right {
        length: 12,
        padding: b' ',
    };
    const TAG: u16 = 1203;
    const JSON_NAME: Option<JsonName<'static>> = json_name("operatorInn");
    type Type = String;
}

/// (1205) Коды причин изменения сведений о ККТ
///
/// Коды причин изменения сведений о ККТ
pub enum KktInfoUpdateReason {}
impl FieldInternal for KktInfoUpdateReason {
    const PADDING: Padding = Padding::Right {
        length: 4,
        padding: b'\0',
    };
    const TAG: u16 = 1205;
    const JSON_NAME: Option<JsonName<'static>> = json_name("correctionKktReasonCode");
    type Type = enums::KktInfoUpdateReasons;
}

/// (1206) Сообщение оператора
///
/// Код информационного сообщения оператора фискальных данных
pub enum OperatorMessage {}
impl FieldInternal for OperatorMessage {
    const PADDING: Padding = Padding::Right {
        length: 1,
        padding: b'\0',
    };
    const TAG: u16 = 1206;
    const JSON_NAME: Option<JsonName<'static>> = json_name("operatorMessage");
    type Type = enums::OperatorMessages;
}

/// (1207) Признак торговли подакцизными товарами
///
/// Признак применения ККТ при осуществлении торговли подакцизными товарами
pub enum ExciseFlag {}
impl FieldInternal for ExciseFlag {
    const PADDING: Padding = Padding::Right {
        length: 1,
        padding: b'\0',
    };
    const TAG: u16 = 1207;
    type Type = bool;
}

/// (1208) Сайт для получения чека
///
/// Адрес информационного ресурса, который размещен в сети «Интернет» и по которому кассовый чек (БСО), кассовый чек коррекции (БСО коррекции) может быть бесплатно получен покупателем (клиентом)
pub enum ReceiptRetrievalWebsite {}
impl FieldInternal for ReceiptRetrievalWebsite {
    const PADDING: Padding = Padding::None { length: Some(256) };
    const TAG: u16 = 1208;
    type Type = String;
}

/// (1209) Номер версии ФФД
///
/// Номер версии ФФД
pub enum FfdVer {}
impl FieldInternal for FfdVer {
    const PADDING: Padding = Padding::Right {
        length: 1,
        padding: b'\0',
    };
    const TAG: u16 = 1209;
    const JSON_NAME: Option<JsonName<'static>> = json_name("fiscalDocumentFormatVer");
    type Type = enums::FfdVersion;
}

/// (1212) Признак предмета расчета
///
/// Признак предмета товара, работы, услуги, платежа, выплаты, иного предмета расчета
pub enum ItemType {}
impl FieldInternal for ItemType {
    const PADDING: Padding = Padding::Right {
        length: 1,
        padding: b'\0',
    };
    const TAG: u16 = 1212;
    const JSON_NAME: Option<JsonName<'static>> = json_name("productType");
    type Type = enums::ItemType;
}

/// (1213) Ресурс ключей ФП
///
/// Срок действия ключей фискального признака. Значение реквизита определяется как период времени в днях до даты истечения срока действия ключей
pub enum FiscalSignValidityPeriod {}
impl FieldInternal for FiscalSignValidityPeriod {
    const PADDING: Padding = Padding::Right {
        length: 2,
        padding: b'\0',
    };
    const TAG: u16 = 1213;
    const JSON_NAME: Option<JsonName<'static>> = Some(JsonName {
        name: "fdKeyResource",
        enclosure_tag_overrides: &[(21, "keyResource")],
    });
    type Type = u16;
}

/// (1214) Признак способа расчета
///
/// Признак способа расчета
pub enum PaymentMethod {}
impl FieldInternal for PaymentMethod {
    const PADDING: Padding = Padding::Right {
        length: 1,
        padding: b'\0',
    };
    const TAG: u16 = 1214;
    const JSON_NAME: Option<JsonName<'static>> = json_name("paymentType");
    type Type = enums::PaymentMethod;
}

/// (1215) Сумма по чеку (БСО) предоплатой (зачетом аванса и (или) предыдущих платежей)
///
/// Сумма расчета, указанная в кассовом чеке (БСО), или сумма корректировки расчета, указанная в кассовом чеке коррекции (БСО коррекции), подлежащая уплате ранее внесенной предоплатой (зачетом аванса)
pub enum TotalPrepaidSum {}
impl FieldInternal for TotalPrepaidSum {
    const PADDING: Padding = Padding::None { length: Some(6) };
    const TAG: u16 = 1215;
    const JSON_NAME: Option<JsonName<'static>> = json_name("prepaidSum");
    type Type = u64;
}

/// (1216) Сумма по чеку (БСО) постоплатой (в кредит)
///
/// Сумма расчета, указанная в кассовом чеке (БСО), или сумма корректировки расчета, указанная в кассовом чеке коррекции (БСО коррекции), подлежащая последующей уплате (в кредит)
pub enum TotalCreditSum {}
impl FieldInternal for TotalCreditSum {
    const PADDING: Padding = Padding::None { length: Some(6) };
    const TAG: u16 = 1216;
    const JSON_NAME: Option<JsonName<'static>> = json_name("creditSum");
    type Type = u64;
}

/// (1217) Сумма по чеку (БСО) встречным предоставлением
///
/// Сумма расчета, указанная в кассовом чеке (БСО), или сумма корректировки расчета, указанная в кассовом чеке коррекции (БСО коррекции), подлежащая уплате встречным предоставлением покупателем (клиентом) пользователю предмета расчета, меной и иным аналогичным способом
pub enum TotalProvisionSum {}
impl FieldInternal for TotalProvisionSum {
    const PADDING: Padding = Padding::None { length: Some(6) };
    const TAG: u16 = 1217;
    const JSON_NAME: Option<JsonName<'static>> = json_name("provisionSum");
    type Type = u64;
}

/// (1218) Итоговая сумма в чеках (БСО) предоплатами (авансами)
///
/// Итоговая сумма расчетов, указанных в кассовых чеках (БСО) и кассовых чеках коррекции (БСО коррекции), совершенных с использованием ранее внесенных оплат (зачетов авансов)
pub enum AggregatedPrepaidSum {}
impl FieldInternal for AggregatedPrepaidSum {
    const PADDING: Padding = Padding::None { length: Some(6) };
    const TAG: u16 = 1218;
    const JSON_NAME: Option<JsonName<'static>> = json_name("prepaidSum");
    type Type = u64;
}

/// (1219) Итоговая сумма в чеках (БСО) постоплатами (кредитами)
///
/// Итоговая сумма расчетов, указанных в кассовых чеках (БСО) и кассовых чеках коррекции (БСО коррекции), с последующей уплатой (о суммах кредитов)
pub enum AggregatedCreditSum {}
impl FieldInternal for AggregatedCreditSum {
    const PADDING: Padding = Padding::None { length: Some(6) };
    const TAG: u16 = 1219;
    const JSON_NAME: Option<JsonName<'static>> = json_name("creditSum");
    type Type = u64;
}

/// (1220) Итоговая сумма в чеках (БСО) встречными предоставлениями
///
/// Итоговая сумма расчетов, указанных в кассовых чеках (БСО) и кассовых чеках коррекции (БСО коррекции), с уплатой встречными предоставлениями
pub enum AggregatedProvisionSum {}
impl FieldInternal for AggregatedProvisionSum {
    const PADDING: Padding = Padding::None { length: Some(6) };
    const TAG: u16 = 1220;
    const JSON_NAME: Option<JsonName<'static>> = json_name("provisionSum");
    type Type = u64;
}

/// (1221) Признак установки принтера в автомате
///
/// Признак установки устройства для печати фискальных документов в корпусе автоматического устройства для расчетов
pub enum PrinterFlag {}
impl FieldInternal for PrinterFlag {
    const PADDING: Padding = Padding::Right {
        length: 1,
        padding: b'\0',
    };
    const TAG: u16 = 1221;
    type Type = bool;
}

/// (1222) Признак агента по предмету расчета
///
/// Признак агента по предмету расчета
pub enum ItemAgentTypes {}
impl FieldInternal for ItemAgentTypes {
    const PADDING: Padding = Padding::Right {
        length: 1,
        padding: b'\0',
    };
    const TAG: u16 = 1222;
    const JSON_NAME: Option<JsonName<'static>> = json_name("paymentAgentByProductType");
    type Type = enums::AgentTypes;
}

/// (1223) Данные агента
///
/// Дополнительные сведения о пользователе, являющемся агентом, и о его контрагентах
pub enum PaymentAgentData {}
impl FieldInternal for PaymentAgentData {
    const PADDING: Padding = Padding::None { length: Some(512) };
    const TAG: u16 = 1223;
    const JSON_NAME: Option<JsonName<'static>> = json_name("paymentAgentData");
    type Type = Object;
}

/// (1224) Данные поставщика
///
/// Данные поставщика
pub enum SupplierData {}
impl FieldInternal for SupplierData {
    const PADDING: Padding = Padding::None { length: Some(512) };
    const TAG: u16 = 1224;
    const JSON_NAME: Option<JsonName<'static>> = json_name("providerData");
    type Type = Object;
}

/// (1225) Наименование поставщика
///
/// Наименование поставщика
pub enum SupplierName {}
impl FieldInternal for SupplierName {
    const PADDING: Padding = Padding::None { length: Some(256) };
    const TAG: u16 = 1225;
    const JSON_NAME: Option<JsonName<'static>> = json_name("providerName");
    type Type = String;
}

/// (1226) ИНН поставщика
///
/// ИНН поставщика
///
/// Данный реквизит принимает значение «000000000000» в случае если поставщику не присвоен ИНН на территории Российской Федерации.
pub enum SupplierInn {}
impl FieldInternal for SupplierInn {
    const PADDING: Padding = Padding::Right {
        length: 12,
        padding: b' ',
    };
    const TAG: u16 = 1226;
    const JSON_NAME: Option<JsonName<'static>> = json_name("providerInn");
    type Type = String;
}

/// (1227) Покупатель (клиент)
///
/// Наименование организации или фамилия, имя, отчество (при наличии), серия (при наличии) и номер документа удостоверяющего личность покупателя (клиента)
pub enum Client {}
impl FieldInternal for Client {
    const PADDING: Padding = Padding::None { length: Some(256) };
    const TAG: u16 = 1227;
    const JSON_NAME: Option<JsonName<'static>> = json_name("buyer");
    type Type = String;
}

/// (1228) ИНН покупателя (клиента)
///
/// ИНН организации или покупателя (клиента)
///
/// Данный реквизит принимает значение «000000000000» в случае если покупателю (клиенту) не присвоен ИНН на территории Российской Федерации.
pub enum BuyerInn {}
impl FieldInternal for BuyerInn {
    const PADDING: Padding = Padding::Right {
        length: 12,
        padding: b' ',
    };
    const TAG: u16 = 1228;
    const JSON_NAME: Option<JsonName<'static>> = json_name("buyerInn");
    type Type = String;
}

/// (1229) Акциз
///
/// Сумма акциза с учетом копеек, включенная в стоимость предмета расчета
pub enum ExciseDuty {}
impl FieldInternal for ExciseDuty {
    const PADDING: Padding = Padding::None { length: Some(6) };
    const TAG: u16 = 1229;
    const JSON_NAME: Option<JsonName<'static>> = json_name("exciseDuty");
    type Type = u64;
}

/// (1230) Код страны происхождения товара
///
/// Цифровой код страны происхождения товара в соответствии с Общероссийским классификатором стран мира
pub enum OriginCountry {}
impl FieldInternal for OriginCountry {
    const PADDING: Padding = Padding::Right {
        length: 3,
        padding: b' ',
    };
    const TAG: u16 = 1230;
    const JSON_NAME: Option<JsonName<'static>> = json_name("originCountryCode");
    type Type = String;
}

/// (1231) Номер декларации на товар
///
/// Номер таможенной декларации (декларации на товар) в соответствии с форматом, установленным решением Комиссии Таможенного союза от 20.05.2010 № 257 (в ред. 17.12.2019 № 223) «О форме декларации на товары и порядке ее заполнения»
pub enum CustomsDeclarationNum {}
impl FieldInternal for CustomsDeclarationNum {
    const PADDING: Padding = Padding::None { length: Some(32) };
    const TAG: u16 = 1231;
    const JSON_NAME: Option<JsonName<'static>> = json_name("customEntryNum");
    type Type = String;
}

/// (1232) Счетчики по признаку «возврат прихода»
///
/// Итоговые количества и итоговые суммы кассовых чеков коррекции (БСО коррекции), а также итоговые количества и итоговые суммы непереданных кассовых чеков (БСО) и кассовых чеков коррекции (БСО коррекции) с признаком расчета «возврат прихода»
pub enum UntransmittedSaleReturnStats {}
impl FieldInternal for UntransmittedSaleReturnStats {
    const PADDING: Padding = Padding::None { length: Some(32) };
    const TAG: u16 = 1232;
    const JSON_NAME: Option<JsonName<'static>> = json_name("sellReturnCorrection");
    type Type = Object;
}

/// (1233) Счетчики по признаку «возврат расхода»
///
/// Итоговые количества и итоговые суммы кассовых чеков коррекции (БСО коррекции), а также итоговые количества и итоговые суммы непереданных кассовых чеков (БСО) и кассовых чеков коррекции (БСО коррекции) с признаком расчета «возврат расхода»
pub enum UntransmittedPurchaseReturnStats {}
impl FieldInternal for UntransmittedPurchaseReturnStats {
    const PADDING: Padding = Padding::None { length: Some(32) };
    const TAG: u16 = 1233;
    const JSON_NAME: Option<JsonName<'static>> = json_name("buyReturnCorrection");
    type Type = Object;
}

/// (1243) Дата рождения покупателя (клиента)
///
/// Дата рождения покупателя (клиента)
pub enum BuyerBirthday {}
impl FieldInternal for BuyerBirthday {
    const PADDING: Padding = Padding::Fixed { length: 10 };
    const TAG: u16 = 1243;
    const JSON_NAME: Option<JsonName<'static>> = json_name("buyerBirthday");
    type Type = String;
}

/// (1244) Гражданство
///
/// Числовой код страны, гражданином которой является покупатель (клиент). Код страны указывается в соответствии с Общероссийским классификатором стран мира ОКСМ. При отсутствии у покупателя (клиента) гражданства указывается код страны, выдавшей документ, удостоверяющий его личность
pub enum Citizenship {}
impl FieldInternal for Citizenship {
    const PADDING: Padding = Padding::Right {
        length: 3,
        padding: b' ',
    };
    const TAG: u16 = 1244;
    const JSON_NAME: Option<JsonName<'static>> = json_name("buyerCitizenship");
    type Type = String;
}

/// (1245) Код вида документа, удостоверяющего личность
///
/// Числовой код вида документа, удостоверяющего личность; см. таблицу 116
pub enum BuyerIdType {}
impl FieldInternal for BuyerIdType {
    const PADDING: Padding = Padding::Right {
        length: 2,
        padding: b'\0',
    };
    const TAG: u16 = 1245;
    const JSON_NAME: Option<JsonName<'static>> = json_name("buyerDocumentCode");
    type Type = String;
}

/// (1246) Данные документа, удостоверяющего личность
///
/// Реквизиты документа, удостоверяющего личность
pub enum BuyerIdData {}
impl FieldInternal for BuyerIdData {
    const PADDING: Padding = Padding::None { length: Some(64) };
    const TAG: u16 = 1246;
    const JSON_NAME: Option<JsonName<'static>> = json_name("buyerDocumentData");
    type Type = String;
}

/// (1254) Адрес покупателя (клиента)
///
/// Адрес покупателя (клиента), грузополучателя
pub enum BuyerAddress {}
impl FieldInternal for BuyerAddress {
    const PADDING: Padding = Padding::None { length: Some(256) };
    const TAG: u16 = 1254;
    const JSON_NAME: Option<JsonName<'static>> = json_name("buyerAddress");
    type Type = String;
}

/// (1256) Сведения о покупателе (клиенте)
///
/// Сведения о покупателе (клиенте); см. таблицу 115
pub enum BuyerInfo {}
impl FieldInternal for BuyerInfo {
    const PADDING: Padding = Padding::None { length: Some(1024) };
    const TAG: u16 = 1256;
    const JSON_NAME: Option<JsonName<'static>> = json_name("buyerInformation");
    type Type = Object;
}

/// (1260) Отраслевой реквизит предмета расчета
///
/// Содержит сведения о нормативных актах, регламентирующих порядок заполнения реквизита «значение  отраслевого реквизита» (тег 1265)
pub enum IndustryItemProp {}
impl FieldInternal for IndustryItemProp {
    const PADDING: Padding = Padding::None { length: Some(317) };
    const TAG: u16 = 1260;
    const JSON_NAME: Option<JsonName<'static>> = json_name("itemsIndustryDetails");
    type Type = Object;
}
impl MultiField for IndustryItemProp {}

/// (1261) Отраслевой реквизит чека
///
/// Содержит сведения о нормативных актах, регламентирующих порядок заполнения реквизита «значение  отраслевого реквизита» (тег 1265)
pub enum IndustryReceiptProp {}
impl FieldInternal for IndustryReceiptProp {
    const PADDING: Padding = Padding::None { length: Some(317) };
    const TAG: u16 = 1261;
    const JSON_NAME: Option<JsonName<'static>> = json_name("industryReceiptDetails");
    type Type = Object;
}
impl MultiField for IndustryReceiptProp {}

/// (1262) Идентификатор ФОИВ
///
/// См. таблицу 149
pub enum FoivId {}
impl FieldInternal for FoivId {
    const PADDING: Padding = Padding::None { length: Some(3) };
    const TAG: u16 = 1262;
    const JSON_NAME: Option<JsonName<'static>> = json_name("idFoiv");
    type Type = String;
}

/// (1263) Дата документа основания
///
/// Дата нормативного акта федерального органа исполнительной власти, регламентирующего порядок заполнения реквизита «значение  отраслевого реквизита» (тег 1265)
pub enum FoundationDocDateTime {}
impl FieldInternal for FoundationDocDateTime {
    const PADDING: Padding = Padding::Fixed { length: 10 };
    const TAG: u16 = 1263;
    const JSON_NAME: Option<JsonName<'static>> = json_name("foundationDocDateTime");
    type Type = String;
}

/// (1264) Номер документа основания
///
/// Номер нормативного акта федерального органа исполнительной власти, регламентирующего порядок заполнения реквизита «значение  отраслевого реквизита» (тег 1265)
pub enum FoundationDocNum {}
impl FieldInternal for FoundationDocNum {
    const PADDING: Padding = Padding::None { length: Some(32) };
    const TAG: u16 = 1264;
    const JSON_NAME: Option<JsonName<'static>> = json_name("foundationDocNumber");
    type Type = String;
}

/// (1265) Значение отраслевого реквизита
///
/// Состав значений, определенных нормативного актом федерального органа исполнительной власти
pub enum IndustryPropValue {}
impl FieldInternal for IndustryPropValue {
    const PADDING: Padding = Padding::None { length: Some(256) };
    const TAG: u16 = 1265;
    const JSON_NAME: Option<JsonName<'static>> = json_name("industryPropValue");
    type Type = String;
}

/// (1270) Операционный реквизит чека
///
/// Дополнительный реквизит чека, условия применения и значение которого определяется ФНС России
pub enum OperationalProp {}
impl FieldInternal for OperationalProp {
    const PADDING: Padding = Padding::None { length: Some(144) };
    const TAG: u16 = 1270;
    const JSON_NAME: Option<JsonName<'static>> = json_name("operationalDetails");
    type Type = Object;
}

/// (1271) Идентификатор операции
///
/// Дополнительный реквизит чека, условия применения и значение которого определяется ФНС России
pub enum OperationId {}
impl FieldInternal for OperationId {
    const PADDING: Padding = Padding::Right {
        length: 1,
        padding: b'\0',
    };
    const TAG: u16 = 1271;
    const JSON_NAME: Option<JsonName<'static>> = json_name("operationId");
    type Type = u8;
}

/// (1272) Данные операции
///
/// Дополнительный реквизит чека, условия применения и значение которого определяется ФНС России
pub enum OperationData {}
impl FieldInternal for OperationData {
    const PADDING: Padding = Padding::None { length: Some(64) };
    const TAG: u16 = 1272;
    const JSON_NAME: Option<JsonName<'static>> = json_name("operationData");
    type Type = String;
}

/// (1273) Дата, время операции
///
/// Дополнительный реквизит чека, условия применения и значение которого определяется ФНС России
pub enum OperationDateTime {}
impl FieldInternal for OperationDateTime {
    const PADDING: Padding = Padding::Right {
        length: 4,
        padding: b'\0',
    };
    const TAG: u16 = 1273;
    const JSON_NAME: Option<JsonName<'static>> = json_name("dateTime");
    type Type = chrono::NaiveDateTime;
}

/// (1274) Дополнительный реквизит ОР
///
/// Дополнительный реквизит отчета о регистрации (отчета об изменении параметров регистрации)
pub enum FiscalReportAdditionalProp {}
impl FieldInternal for FiscalReportAdditionalProp {
    const PADDING: Padding = Padding::None { length: Some(32) };
    const TAG: u16 = 1274;
    const JSON_NAME: Option<JsonName<'static>> = json_name("additionalPropsFRC");
    type Type = String;
}

/// (1275) Дополнительные данные ОР
///
/// Дополнительные данные отчета о регистрации (отчета об изменении параметров регистрации)
pub enum FiscalReportAdditionalData {}
impl FieldInternal for FiscalReportAdditionalData {
    const PADDING: Padding = Padding::None { length: Some(32) };
    const TAG: u16 = 1275;
    const JSON_NAME: Option<JsonName<'static>> = json_name("additionalDataFRC");
    type Type = Vec<u8>;
}

/// (1276) Дополнительный реквизит ООС
///
/// Дополнительный реквизит отчета об открытии смены
pub enum OpenShiftAdditionalProp {}
impl FieldInternal for OpenShiftAdditionalProp {
    const PADDING: Padding = Padding::None { length: Some(32) };
    const TAG: u16 = 1276;
    const JSON_NAME: Option<JsonName<'static>> = json_name("additionalPropsOS");
    type Type = String;
}

/// (1277) Дополнительные данные ООС
///
/// Дополнительные данные отчета об открытии смены
pub enum OpenShiftAdditionalData {}
impl FieldInternal for OpenShiftAdditionalData {
    const PADDING: Padding = Padding::None { length: Some(32) };
    const TAG: u16 = 1277;
    const JSON_NAME: Option<JsonName<'static>> = json_name("additionalDataOS");
    type Type = Vec<u8>;
}

/// (1278) Дополнительный реквизит ОЗС
///
/// Дополнительный реквизит отчета о закрытии смены
pub enum CloseShiftAdditionalProp {}
impl FieldInternal for CloseShiftAdditionalProp {
    const PADDING: Padding = Padding::None { length: Some(32) };
    const TAG: u16 = 1278;
    type Type = String;
}

/// (1279) Дополнительные данные ОЗС
///
/// Дополнительные данные отчета о закрытии смены
pub enum CloseShiftAdditionalData {}
impl FieldInternal for CloseShiftAdditionalData {
    const PADDING: Padding = Padding::None { length: Some(32) };
    const TAG: u16 = 1279;
    type Type = Vec<u8>;
}

/// (1280) Дополнительный реквизит ОТР
///
/// Дополнительный реквизит отчета о текущем состоянии расчетов
pub enum CurrentStateAdditionalAttribute {}
impl FieldInternal for CurrentStateAdditionalAttribute {
    const PADDING: Padding = Padding::None { length: Some(32) };
    const TAG: u16 = 1280;
    const JSON_NAME: Option<JsonName<'static>> = json_name("additionalPropsCSR");
    type Type = String;
}

/// (1281) Дополнительные данные ОТР
///
/// Дополнительные данные отчета о текущем состоянии расчетов
pub enum CurrentStateAdditionalData {}
impl FieldInternal for CurrentStateAdditionalData {
    const PADDING: Padding = Padding::None { length: Some(32) };
    const TAG: u16 = 1281;
    const JSON_NAME: Option<JsonName<'static>> = json_name("additionalDataCSR");
    type Type = Vec<u8>;
}

/// (1282) Дополнительный реквизит ОЗФН
///
/// Дополнительный реквизит отчета о закрытии фискального накопителя
pub enum CloseArchiveAdditionalAttribute {}
impl FieldInternal for CloseArchiveAdditionalAttribute {
    const PADDING: Padding = Padding::None { length: Some(32) };
    const TAG: u16 = 1282;
    type Type = String;
}

/// (1283) Дополнительные данные ОЗФН
///
/// Дополнительные данные отчета о закрытии фискального накопителя
pub enum CloseArchiveAdditionalData {}
impl FieldInternal for CloseArchiveAdditionalData {
    const PADDING: Padding = Padding::None { length: Some(32) };
    const TAG: u16 = 1283;
    type Type = Vec<u8>;
}

/// (1290) Признаки условий применения ККТ
///
/// См. таблицу 103
pub enum KktUsageFlags {}
impl FieldInternal for KktUsageFlags {
    const PADDING: Padding = Padding::Right {
        length: 4,
        padding: b'\0',
    };
    const TAG: u16 = 1290;
    const JSON_NAME: Option<JsonName<'static>> = json_name("usageConditionSigns");
    type Type = enums::KktUsage;
}

/// (1291) Дробное количество маркированного товара
///
/// Дробное количество маркированного товара
pub enum MarkedProductFractionalQuantity {}
impl FieldInternal for MarkedProductFractionalQuantity {
    const PADDING: Padding = Padding::None { length: Some(52) };
    const TAG: u16 = 1291;
    const JSON_NAME: Option<JsonName<'static>> = json_name("labeledProdFractionalQuantity");
    type Type = Object;
}

/// (1292) Дробная часть
///
/// Дробная часть предмета расчета
pub enum FractionalPart {}
impl FieldInternal for FractionalPart {
    const PADDING: Padding = Padding::None { length: Some(24) };
    const TAG: u16 = 1292;
    const JSON_NAME: Option<JsonName<'static>> = json_name("fractionalPart");
    type Type = String;
}

/// (1293) Числитель
///
/// Числитель дробной части предмета расчета
pub enum Numerator {}
impl FieldInternal for Numerator {
    const PADDING: Padding = Padding::None { length: Some(8) };
    const TAG: u16 = 1293;
    const JSON_NAME: Option<JsonName<'static>> = json_name("numerator");
    type Type = u64;
}

/// (1294) Знаменатель
///
/// Знаменатель дробной части предмета расчета
pub enum Denominator {}
impl FieldInternal for Denominator {
    const PADDING: Padding = Padding::None { length: Some(8) };
    const TAG: u16 = 1294;
    const JSON_NAME: Option<JsonName<'static>> = json_name("denominator");
    type Type = u64;
}

/// (1300) КТ Н
///
/// Код товара, формат которого не идентифицирован
pub enum KtN {}
impl FieldInternal for KtN {
    const PADDING: Padding = Padding::None { length: Some(32) };
    const TAG: u16 = 1300;
    const JSON_NAME: Option<JsonName<'static>> = json_name("undefined");
    type Type = String;
}

/// (1301) КТ EAN-8
///
/// Код товара в формате EAN-8
pub enum KtEan8 {}
impl FieldInternal for KtEan8 {
    const PADDING: Padding = Padding::Fixed { length: 8 };
    const TAG: u16 = 1301;
    const JSON_NAME: Option<JsonName<'static>> = json_name("ean8");
    type Type = String;
}

/// (1302) КТ EAN-13
///
/// Код товара в формате EAN-13
pub enum KtEan13 {}
impl FieldInternal for KtEan13 {
    const PADDING: Padding = Padding::Fixed { length: 13 };
    const TAG: u16 = 1302;
    const JSON_NAME: Option<JsonName<'static>> = json_name("ean13");
    type Type = String;
}

/// (1303) КТ ITF-14
///
/// Код товара в формате ITF-14
pub enum KtItf14 {}
impl FieldInternal for KtItf14 {
    const PADDING: Padding = Padding::Fixed { length: 14 };
    const TAG: u16 = 1303;
    const JSON_NAME: Option<JsonName<'static>> = json_name("itf14");
    type Type = String;
}

/// (1304) КТ GS1.0
///
/// Код товара в формате GS1, нанесенный на товар, не подлежащий маркировке средствами идентификации
pub enum KtGs1_0 {}
impl FieldInternal for KtGs1_0 {
    const PADDING: Padding = Padding::None { length: Some(38) };
    const TAG: u16 = 1304;
    const JSON_NAME: Option<JsonName<'static>> = json_name("gs1");
    type Type = String;
}

/// (1305) КТ GS1.М
///
/// Код товара в формате GS1, нанесенный на товар, подлежащий маркировке средствами идентификации
pub enum KtGs1M {}
impl FieldInternal for KtGs1M {
    const PADDING: Padding = Padding::None { length: Some(38) };
    const TAG: u16 = 1305;
    const JSON_NAME: Option<JsonName<'static>> = json_name("gs1m");
    type Type = String;
}

/// (1306) КТ КМК
///
/// Код товара в формате короткого кода маркировки, нанесенный на товар, подлежащий маркировке средствами идентификации
pub enum KtKmk {}
impl FieldInternal for KtKmk {
    const PADDING: Padding = Padding::None { length: Some(38) };
    const TAG: u16 = 1306;
    const JSON_NAME: Option<JsonName<'static>> = json_name("kmk");
    type Type = String;
}

/// (1307) КТ МИ
///
/// Контрольно-идентификационный знак мехового изделия
pub enum KtMi {}
impl FieldInternal for KtMi {
    const PADDING: Padding = Padding::Fixed { length: 20 };
    const TAG: u16 = 1307;
    const JSON_NAME: Option<JsonName<'static>> = json_name("mi");
    type Type = String;
}

/// (1308) КТ ЕГАИС-2.0
///
/// Код товара в формате ЕГАИС-2.0
pub enum KtEgais2_0 {}
impl FieldInternal for KtEgais2_0 {
    const PADDING: Padding = Padding::Fixed { length: 23 };
    const TAG: u16 = 1308;
    const JSON_NAME: Option<JsonName<'static>> = json_name("egais2");
    type Type = String;
}

/// (1309) КТ ЕГАИС-3.0
///
/// Код товара в формате ЕГАИС-3.0
pub enum KtEgais3_0 {}
impl FieldInternal for KtEgais3_0 {
    const PADDING: Padding = Padding::Fixed { length: 14 };
    const TAG: u16 = 1309;
    const JSON_NAME: Option<JsonName<'static>> = json_name("egais3");
    type Type = String;
}

/// (1320) КТ Ф.1
///
/// Код товара в формате Ф.1
pub enum KtF1 {}
impl FieldInternal for KtF1 {
    const PADDING: Padding = Padding::None { length: Some(32) };
    const TAG: u16 = 1320;
    const JSON_NAME: Option<JsonName<'static>> = json_name("f1");
    type Type = String;
}

/// (1321) КТ Ф.2
///
/// Код товара в формате Ф.2
pub enum KtF2 {}
impl FieldInternal for KtF2 {
    const PADDING: Padding = Padding::None { length: Some(32) };
    const TAG: u16 = 1321;
    const JSON_NAME: Option<JsonName<'static>> = json_name("f2");
    type Type = String;
}

/// (1322) КТ Ф.3
///
/// Код товара в формате Ф.3
pub enum KtF3 {}
impl FieldInternal for KtF3 {
    const PADDING: Padding = Padding::None { length: Some(32) };
    const TAG: u16 = 1322;
    const JSON_NAME: Option<JsonName<'static>> = json_name("f3");
    type Type = String;
}

/// (1323) КТ Ф.4
///
/// Код товара в формате Ф.4
pub enum KtF4 {}
impl FieldInternal for KtF4 {
    const PADDING: Padding = Padding::None { length: Some(32) };
    const TAG: u16 = 1323;
    const JSON_NAME: Option<JsonName<'static>> = json_name("f4");
    type Type = String;
}

/// (1324) КТ Ф.5
///
/// Код товара в формате Ф.5
pub enum KtF5 {}
impl FieldInternal for KtF5 {
    const PADDING: Padding = Padding::None { length: Some(32) };
    const TAG: u16 = 1324;
    const JSON_NAME: Option<JsonName<'static>> = json_name("f5");
    type Type = String;
}

/// (1325) КТ Ф.6
///
/// Код товара в формате Ф.6
pub enum KtF6 {}
impl FieldInternal for KtF6 {
    const PADDING: Padding = Padding::None { length: Some(32) };
    const TAG: u16 = 1325;
    const JSON_NAME: Option<JsonName<'static>> = json_name("f6");
    type Type = String;
}

/// (2000) Код маркировки
///
/// Код маркировки товара, подлежащего обязательной маркировке средством идентификации
pub enum MarkingCode {}
impl FieldInternal for MarkingCode {
    const PADDING: Padding = Padding::None { length: Some(256) };
    const TAG: u16 = 2000;
    type Type = String;
}

/// (2001) Номер запроса
///
/// Порядковый номер запроса о коде маркировки
pub enum RequestNumber {}
impl FieldInternal for RequestNumber {
    const PADDING: Padding = Padding::Right {
        length: 4,
        padding: b'\0',
    };
    const TAG: u16 = 2001;
    type Type = u32;
}

/// (2002) Номер уведомления
///
/// Порядковый номер уведомления о реализации товара, подлежащего обязательной маркировке средством идентификации
pub enum NotificationNumber {}
impl FieldInternal for NotificationNumber {
    const PADDING: Padding = Padding::Right {
        length: 4,
        padding: b'\0',
    };
    const TAG: u16 = 2002;
    type Type = u32;
}

/// (2003) Планируемый статус товара
///
/// Планируемое изменение статуса товара, подлежащего обязательной маркировке средством идентификации (реализация, возврат)
pub enum PlannedProductStatus {}
impl FieldInternal for PlannedProductStatus {
    const PADDING: Padding = Padding::Right {
        length: 1,
        padding: b'\0',
    };
    const TAG: u16 = 2003;
    type Type = enums::ProductStatus;
}

/// (2004) Результат проверки КМ
///
/// Результат проверки КП КМ
pub enum KmCheckResult {}
impl FieldInternal for KmCheckResult {
    const PADDING: Padding = Padding::Right {
        length: 1,
        padding: b'\0',
    };
    const TAG: u16 = 2004;
    type Type = enums::MarkingCheckResult;
}

/// (2005) Результаты обработки запроса
///
/// Результаты обработки запроса о коде маркировки ОИСМ
pub enum RequestProcessingResults {}
impl FieldInternal for RequestProcessingResults {
    const PADDING: Padding = Padding::Right {
        length: 1,
        padding: b'\0',
    };
    const TAG: u16 = 2005;
    type Type = enums::MarkingCheckResult;
}

/// (2006) Результаты обработки уведомления
///
/// Признак наличия в уведомлении о реализации маркированных товаров КМ, проверка которых дала отрицательный результат
pub enum NotificationProcessingResults {}
impl FieldInternal for NotificationProcessingResults {
    const PADDING: Padding = Padding::Right {
        length: 1,
        padding: b'\0',
    };
    const TAG: u16 = 2006;
    type Type = enums::KmNotificationResult;
}

/// (2007) Данные о маркированном товаре
///
/// Данные о товаре, подлежащем обязательной маркировке средством идентификации
pub enum MarkedProductData {}
impl FieldInternal for MarkedProductData {
    const PADDING: Padding = Padding::None { length: Some(512) };
    const TAG: u16 = 2007;
    type Type = Object;
}
impl MultiField for MarkedProductData {}

/// (2100) Тип кода маркировки
///
/// Результат идентификации типа КМ
pub enum MarkingCodeType {}
impl FieldInternal for MarkingCodeType {
    const PADDING: Padding = Padding::Right {
        length: 1,
        padding: b'\0',
    };
    const TAG: u16 = 2100;
    type Type = enums::MarkingType;
}

/// (2101) Идентификатор товара
///
/// Идентификатор экземпляра товара, подлежащего обязательной маркировке средством идентификации
pub enum ProductId {}
impl FieldInternal for ProductId {
    const PADDING: Padding = Padding::None { length: Some(38) };
    const TAG: u16 = 2101;
    type Type = String;
}

/// (2102) Режим обработки кода маркировки
///
/// Режим обработки КМ при реализации товара подлежащего обязательной маркировке средством идентификации. Указанный реквизит должен принимать значение, равное «0»
pub enum MarkingCodeProcessingMode {}
impl FieldInternal for MarkingCodeProcessingMode {
    const PADDING: Padding = Padding::Right {
        length: 1,
        padding: b'\0',
    };
    const TAG: u16 = 2102;
    const JSON_NAME: Option<JsonName<'static>> = json_name("labelCodeProcesMode");
    type Type = u8;
}

/// (2104) Количество непереданных уведомлений
///
/// Количество уведомлений о реализации товаров, подлежащих обязательной маркировке средствами идентификации, для которых не была получена квитанция на уведомление или которые не были выгружены в отчет о реализации маркированного товара при работе ККТ в автономном режиме
pub enum UntransmittedNotificationCount {}
impl FieldInternal for UntransmittedNotificationCount {
    const PADDING: Padding = Padding::Right {
        length: 4,
        padding: b'\0',
    };
    const TAG: u16 = 2104;
    const JSON_NAME: Option<JsonName<'static>> = json_name("undeliveredNotificationsNumber");
    type Type = u32;
}

/// (2105) Коды обработки запроса
///
/// Коды результатов обработки запроса о коде маркировки ОИСМ
pub enum RequestProcessingCodes {}
impl FieldInternal for RequestProcessingCodes {
    const PADDING: Padding = Padding::Right {
        length: 1,
        padding: b'\0',
    };
    const TAG: u16 = 2105;
    type Type = enums::KmRequestStatus;
}

/// (2106) Результат проверки сведений о товаре
///
/// Результаты проверки кода проверки кода маркировки и проверки сведений о товаре, подлежащем обязательной маркировке средством идентификации, содержащихся у ОИСМ, выполненные для товара, подлежащего обязательной маркировке средством идентификации
pub enum ProductInfoCheckResult {}
impl FieldInternal for ProductInfoCheckResult {
    const PADDING: Padding = Padding::Right {
        length: 1,
        padding: b'\0',
    };
    const TAG: u16 = 2106;
    const JSON_NAME: Option<JsonName<'static>> = json_name("checkingProdInformationResult");
    type Type = enums::MarkingCheckResult;
}

/// (2107) Результаты проверки маркированных товаров
///
/// Признак наличия для товаров, подлежащих обязательной маркировке средствами идентификации, включенных в кассовый чек (БСО), кассовый чек коррекции (БСО коррекции) отрицательных результатов проверки КП КМ или проверки сведений о товаре, содержащихся у ОИСМ
pub enum MarkedProductCheckResults {}
impl FieldInternal for MarkedProductCheckResults {
    const PADDING: Padding = Padding::Right {
        length: 1,
        padding: b'\0',
    };
    const TAG: u16 = 2107;
    const JSON_NAME: Option<JsonName<'static>> = json_name("checkingLabeledProdResult");
    type Type = bool;
}
impl MultiField for MarkedProductCheckResults {}

/// (2108) Мера количества предмета расчета
///
/// Единицы измерения количества предмета расчета
pub enum ItemQuantityUnit {}
impl FieldInternal for ItemQuantityUnit {
    const PADDING: Padding = Padding::Right {
        length: 1,
        padding: b'\0',
    };
    const TAG: u16 = 2108;
    const JSON_NAME: Option<JsonName<'static>> = json_name("itemsQuantityMeasure");
    type Type = enums::Unit;
}

/// (2109) Ответ ОИСМ о статусе товара
///
/// Сведения о статусе товара, подлежащего обязательной маркировке средством идентификации, полученные от ОИСМ
pub enum OismProductStatusResponse {}
impl FieldInternal for OismProductStatusResponse {
    const PADDING: Padding = Padding::Right {
        length: 1,
        padding: b'\0',
    };
    const TAG: u16 = 2109;
    type Type = enums::OismProductStatus;
}

/// (2110) Присвоенный статус товара
///
/// Статус, присвоенный товару, подлежащему обязательной маркировке средством идентификации, в результате выполнения расчетов
pub enum AssignedProductStatus {}
impl FieldInternal for AssignedProductStatus {
    const PADDING: Padding = Padding::Right {
        length: 1,
        padding: b'\0',
    };
    const TAG: u16 = 2110;
    type Type = enums::ProductStatus;
}

/// (2111) Коды обработки уведомления
///
/// Коды результатов обработки уведомления
pub enum NotificationProcessingCodes {}
impl FieldInternal for NotificationProcessingCodes {
    const PADDING: Padding = Padding::Right {
        length: 1,
        padding: b'\0',
    };
    const TAG: u16 = 2111;
    type Type = enums::KmNotificationStatus;
}

/// (2112) Признак некорректных кодов маркировки
///
/// Признак некорректных кодов маркировки
pub enum IncorrectMarkingCodesFlags {}
impl FieldInternal for IncorrectMarkingCodesFlags {
    const PADDING: Padding = Padding::Right {
        length: 1,
        padding: b'\0',
    };
    const TAG: u16 = 2112;
    type Type = enums::IncorrectMarkingCodeFlags;
}

/// (2113) Признак некорректных запросов и уведомлений
///
/// Признак некорректных запросов и уведомлений
pub enum IncorrectRequestsAndNotificationsFlags {}
impl FieldInternal for IncorrectRequestsAndNotificationsFlags {
    const PADDING: Padding = Padding::Right {
        length: 1,
        padding: b'\0',
    };
    const TAG: u16 = 2113;
    type Type = enums::IncorrectDataFlags;
}

/// (2114) Дата и время запроса
///
/// Дата и время формирования запроса
pub enum RequestDateTime {}
impl FieldInternal for RequestDateTime {
    const PADDING: Padding = Padding::None { length: Some(4) };
    const TAG: u16 = 2114;
    type Type = chrono::NaiveDateTime;
}

/// (2115) Контрольный код КМ
///
/// Контрольное число кода маркировки
pub enum MarkingCodeControlCode {}
impl FieldInternal for MarkingCodeControlCode {
    const PADDING: Padding = Padding::Fixed { length: 4 };
    const TAG: u16 = 2115;
    const JSON_NAME: Option<JsonName<'static>> = json_name("controlCode");
    type Type = String;
}

/// (2116) Вид операции
///
/// Вид операции, послуживший основанием для формирования ФД
pub enum OperationType {}
impl FieldInternal for OperationType {
    const PADDING: Padding = Padding::Right {
        length: 1,
        padding: b'\0',
    };
    const TAG: u16 = 2116;
    type Type = enums::OperationType;
}

const ALL_REPRS_DATA: &[(u16, Repr, Option<JsonName<'static>>)] = &[
    (
        <RegistrationReport as FieldInternal>::TAG,
        <<RegistrationReport as FieldInternal>::Type as TlvType>::REPR,
        <RegistrationReport as FieldInternal>::JSON_NAME,
    ),
    (
        <ShiftStartReport as FieldInternal>::TAG,
        <<ShiftStartReport as FieldInternal>::Type as TlvType>::REPR,
        <ShiftStartReport as FieldInternal>::JSON_NAME,
    ),
    (
        <Receipt as FieldInternal>::TAG,
        <<Receipt as FieldInternal>::Type as TlvType>::REPR,
        <Receipt as FieldInternal>::JSON_NAME,
    ),
    (
        <Bso as FieldInternal>::TAG,
        <<Bso as FieldInternal>::Type as TlvType>::REPR,
        <Bso as FieldInternal>::JSON_NAME,
    ),
    (
        <ShiftEndReport as FieldInternal>::TAG,
        <<ShiftEndReport as FieldInternal>::Type as TlvType>::REPR,
        <ShiftEndReport as FieldInternal>::JSON_NAME,
    ),
    (
        <FnCloseReport as FieldInternal>::TAG,
        <<FnCloseReport as FieldInternal>::Type as TlvType>::REPR,
        <FnCloseReport as FieldInternal>::JSON_NAME,
    ),
    (
        <OperatorConfirmation as FieldInternal>::TAG,
        <<OperatorConfirmation as FieldInternal>::Type as TlvType>::REPR,
        <OperatorConfirmation as FieldInternal>::JSON_NAME,
    ),
    (
        <RegistrationParamUpdateReport as FieldInternal>::TAG,
        <<RegistrationParamUpdateReport as FieldInternal>::Type as TlvType>::REPR,
        <RegistrationParamUpdateReport as FieldInternal>::JSON_NAME,
    ),
    (
        <PaymentStateReport as FieldInternal>::TAG,
        <<PaymentStateReport as FieldInternal>::Type as TlvType>::REPR,
        <PaymentStateReport as FieldInternal>::JSON_NAME,
    ),
    (
        <CorrectionReceipt as FieldInternal>::TAG,
        <<CorrectionReceipt as FieldInternal>::Type as TlvType>::REPR,
        <CorrectionReceipt as FieldInternal>::JSON_NAME,
    ),
    (
        <CorrectionBso as FieldInternal>::TAG,
        <<CorrectionBso as FieldInternal>::Type as TlvType>::REPR,
        <CorrectionBso as FieldInternal>::JSON_NAME,
    ),
    (
        <MarkingCodeRequest as FieldInternal>::TAG,
        <<MarkingCodeRequest as FieldInternal>::Type as TlvType>::REPR,
        <MarkingCodeRequest as FieldInternal>::JSON_NAME,
    ),
    (
        <MarkedProductSaleNotification as FieldInternal>::TAG,
        <<MarkedProductSaleNotification as FieldInternal>::Type as TlvType>::REPR,
        <MarkedProductSaleNotification as FieldInternal>::JSON_NAME,
    ),
    (
        <MarkingResponse as FieldInternal>::TAG,
        <<MarkingResponse as FieldInternal>::Type as TlvType>::REPR,
        <MarkingResponse as FieldInternal>::JSON_NAME,
    ),
    (
        <NotificationReceipt as FieldInternal>::TAG,
        <<NotificationReceipt as FieldInternal>::Type as TlvType>::REPR,
        <NotificationReceipt as FieldInternal>::JSON_NAME,
    ),
    (
        <DocName as FieldInternal>::TAG,
        <<DocName as FieldInternal>::Type as TlvType>::REPR,
        <DocName as FieldInternal>::JSON_NAME,
    ),
    (
        <AutoModeFlag as FieldInternal>::TAG,
        <<AutoModeFlag as FieldInternal>::Type as TlvType>::REPR,
        <AutoModeFlag as FieldInternal>::JSON_NAME,
    ),
    (
        <OfflineModeFlag as FieldInternal>::TAG,
        <<OfflineModeFlag as FieldInternal>::Type as TlvType>::REPR,
        <OfflineModeFlag as FieldInternal>::JSON_NAME,
    ),
    (
        <TransferOperatorAddress as FieldInternal>::TAG,
        <<TransferOperatorAddress as FieldInternal>::Type as TlvType>::REPR,
        <TransferOperatorAddress as FieldInternal>::JSON_NAME,
    ),
    (
        <BuyerPhoneOrEmail as FieldInternal>::TAG,
        <<BuyerPhoneOrEmail as FieldInternal>::Type as TlvType>::REPR,
        <BuyerPhoneOrEmail as FieldInternal>::JSON_NAME,
    ),
    (
        <RetailPlaceAddress as FieldInternal>::TAG,
        <<RetailPlaceAddress as FieldInternal>::Type as TlvType>::REPR,
        <RetailPlaceAddress as FieldInternal>::JSON_NAME,
    ),
    (
        <DateTime as FieldInternal>::TAG,
        <<DateTime as FieldInternal>::Type as TlvType>::REPR,
        <DateTime as FieldInternal>::JSON_NAME,
    ),
    (
        <KktSerial as FieldInternal>::TAG,
        <<KktSerial as FieldInternal>::Type as TlvType>::REPR,
        <KktSerial as FieldInternal>::JSON_NAME,
    ),
    (
        <TransferOperatorInn as FieldInternal>::TAG,
        <<TransferOperatorInn as FieldInternal>::Type as TlvType>::REPR,
        <TransferOperatorInn as FieldInternal>::JSON_NAME,
    ),
    (
        <OfdInn as FieldInternal>::TAG,
        <<OfdInn as FieldInternal>::Type as TlvType>::REPR,
        <OfdInn as FieldInternal>::JSON_NAME,
    ),
    (
        <UserInn as FieldInternal>::TAG,
        <<UserInn as FieldInternal>::Type as TlvType>::REPR,
        <UserInn as FieldInternal>::JSON_NAME,
    ),
    (
        <TotalSum as FieldInternal>::TAG,
        <<TotalSum as FieldInternal>::Type as TlvType>::REPR,
        <TotalSum as FieldInternal>::JSON_NAME,
    ),
    (
        <Operator as FieldInternal>::TAG,
        <<Operator as FieldInternal>::Type as TlvType>::REPR,
        <Operator as FieldInternal>::JSON_NAME,
    ),
    (
        <OfdResponseCode as FieldInternal>::TAG,
        <<OfdResponseCode as FieldInternal>::Type as TlvType>::REPR,
        <OfdResponseCode as FieldInternal>::JSON_NAME,
    ),
    (
        <ItemQuantity as FieldInternal>::TAG,
        <<ItemQuantity as FieldInternal>::Type as TlvType>::REPR,
        <ItemQuantity as FieldInternal>::JSON_NAME,
    ),
    (
        <TransferOperatorName as FieldInternal>::TAG,
        <<TransferOperatorName as FieldInternal>::Type as TlvType>::REPR,
        <TransferOperatorName as FieldInternal>::JSON_NAME,
    ),
    (
        <ItemName as FieldInternal>::TAG,
        <<ItemName as FieldInternal>::Type as TlvType>::REPR,
        <ItemName as FieldInternal>::JSON_NAME,
    ),
    (
        <TotalCashSum as FieldInternal>::TAG,
        <<TotalCashSum as FieldInternal>::Type as TlvType>::REPR,
        <TotalCashSum as FieldInternal>::JSON_NAME,
    ),
    (
        <MachineNumber as FieldInternal>::TAG,
        <<MachineNumber as FieldInternal>::Type as TlvType>::REPR,
        <MachineNumber as FieldInternal>::JSON_NAME,
    ),
    (
        <KktRegNum as FieldInternal>::TAG,
        <<KktRegNum as FieldInternal>::Type as TlvType>::REPR,
        <KktRegNum as FieldInternal>::JSON_NAME,
    ),
    (
        <ShiftNum as FieldInternal>::TAG,
        <<ShiftNum as FieldInternal>::Type as TlvType>::REPR,
        <ShiftNum as FieldInternal>::JSON_NAME,
    ),
    (
        <DocNum as FieldInternal>::TAG,
        <<DocNum as FieldInternal>::Type as TlvType>::REPR,
        <DocNum as FieldInternal>::JSON_NAME,
    ),
    (
        <DriveNum as FieldInternal>::TAG,
        <<DriveNum as FieldInternal>::Type as TlvType>::REPR,
        <DriveNum as FieldInternal>::JSON_NAME,
    ),
    (
        <ReceiptNum as FieldInternal>::TAG,
        <<ReceiptNum as FieldInternal>::Type as TlvType>::REPR,
        <ReceiptNum as FieldInternal>::JSON_NAME,
    ),
    (
        <ItemTotalPrice as FieldInternal>::TAG,
        <<ItemTotalPrice as FieldInternal>::Type as TlvType>::REPR,
        <ItemTotalPrice as FieldInternal>::JSON_NAME,
    ),
    (
        <PaymentAgentOperation as FieldInternal>::TAG,
        <<PaymentAgentOperation as FieldInternal>::Type as TlvType>::REPR,
        <PaymentAgentOperation as FieldInternal>::JSON_NAME,
    ),
    (
        <OfdName as FieldInternal>::TAG,
        <<OfdName as FieldInternal>::Type as TlvType>::REPR,
        <OfdName as FieldInternal>::JSON_NAME,
    ),
    (
        <User as FieldInternal>::TAG,
        <<User as FieldInternal>::Type as TlvType>::REPR,
        <User as FieldInternal>::JSON_NAME,
    ),
    (
        <DriveResourceExhaustionFlag as FieldInternal>::TAG,
        <<DriveResourceExhaustionFlag as FieldInternal>::Type as TlvType>::REPR,
        <DriveResourceExhaustionFlag as FieldInternal>::JSON_NAME,
    ),
    (
        <DriveReplacementRequiredFlag as FieldInternal>::TAG,
        <<DriveReplacementRequiredFlag as FieldInternal>::Type as TlvType>::REPR,
        <DriveReplacementRequiredFlag as FieldInternal>::JSON_NAME,
    ),
    (
        <DriveMemoryFullFlag as FieldInternal>::TAG,
        <<DriveMemoryFullFlag as FieldInternal>::Type as TlvType>::REPR,
        <DriveMemoryFullFlag as FieldInternal>::JSON_NAME,
    ),
    (
        <OfdResponseTimeoutFlag as FieldInternal>::TAG,
        <<OfdResponseTimeoutFlag as FieldInternal>::Type as TlvType>::REPR,
        <OfdResponseTimeoutFlag as FieldInternal>::JSON_NAME,
    ),
    (
        <PaymentType as FieldInternal>::TAG,
        <<PaymentType as FieldInternal>::Type as TlvType>::REPR,
        <PaymentType as FieldInternal>::JSON_NAME,
    ),
    (
        <TaxType as FieldInternal>::TAG,
        <<TaxType as FieldInternal>::Type as TlvType>::REPR,
        <TaxType as FieldInternal>::JSON_NAME,
    ),
    (
        <EncryptionFlag as FieldInternal>::TAG,
        <<EncryptionFlag as FieldInternal>::Type as TlvType>::REPR,
        <EncryptionFlag as FieldInternal>::JSON_NAME,
    ),
    (
        <PaymentAgentTypes as FieldInternal>::TAG,
        <<PaymentAgentTypes as FieldInternal>::Type as TlvType>::REPR,
        <PaymentAgentTypes as FieldInternal>::JSON_NAME,
    ),
    (
        <ReceiptItem as FieldInternal>::TAG,
        <<ReceiptItem as FieldInternal>::Type as TlvType>::REPR,
        <ReceiptItem as FieldInternal>::JSON_NAME,
    ),
    (
        <FnsUrl as FieldInternal>::TAG,
        <<FnsUrl as FieldInternal>::Type as TlvType>::REPR,
        <FnsUrl as FieldInternal>::JSON_NAME,
    ),
    (
        <TaxationTypes as FieldInternal>::TAG,
        <<TaxationTypes as FieldInternal>::Type as TlvType>::REPR,
        <TaxationTypes as FieldInternal>::JSON_NAME,
    ),
    (
        <OperatorMessageToFn as FieldInternal>::TAG,
        <<OperatorMessageToFn as FieldInternal>::Type as TlvType>::REPR,
        <OperatorMessageToFn as FieldInternal>::JSON_NAME,
    ),
    (
        <PaymentAgentPhone as FieldInternal>::TAG,
        <<PaymentAgentPhone as FieldInternal>::Type as TlvType>::REPR,
        <PaymentAgentPhone as FieldInternal>::JSON_NAME,
    ),
    (
        <PaymentOperatorPhone as FieldInternal>::TAG,
        <<PaymentOperatorPhone as FieldInternal>::Type as TlvType>::REPR,
        <PaymentOperatorPhone as FieldInternal>::JSON_NAME,
    ),
    (
        <TransferOperatorPhone as FieldInternal>::TAG,
        <<TransferOperatorPhone as FieldInternal>::Type as TlvType>::REPR,
        <TransferOperatorPhone as FieldInternal>::JSON_NAME,
    ),
    (
        <DocFiscalSign as FieldInternal>::TAG,
        <<DocFiscalSign as FieldInternal>::Type as TlvType>::REPR,
        <DocFiscalSign as FieldInternal>::JSON_NAME,
    ),
    (
        <OperatorFp as FieldInternal>::TAG,
        <<OperatorFp as FieldInternal>::Type as TlvType>::REPR,
        <OperatorFp as FieldInternal>::JSON_NAME,
    ),
    (
        <ItemUnitPrice as FieldInternal>::TAG,
        <<ItemUnitPrice as FieldInternal>::Type as TlvType>::REPR,
        <ItemUnitPrice as FieldInternal>::JSON_NAME,
    ),
    (
        <TotalEcashSum as FieldInternal>::TAG,
        <<TotalEcashSum as FieldInternal>::Type as TlvType>::REPR,
        <TotalEcashSum as FieldInternal>::JSON_NAME,
    ),
    (
        <AdditionalUserProp as FieldInternal>::TAG,
        <<AdditionalUserProp as FieldInternal>::Type as TlvType>::REPR,
        <AdditionalUserProp as FieldInternal>::JSON_NAME,
    ),
    (
        <AdditionalUserPropName as FieldInternal>::TAG,
        <<AdditionalUserPropName as FieldInternal>::Type as TlvType>::REPR,
        <AdditionalUserPropName as FieldInternal>::JSON_NAME,
    ),
    (
        <AdditionalUserPropValue as FieldInternal>::TAG,
        <<AdditionalUserPropValue as FieldInternal>::Type as TlvType>::REPR,
        <AdditionalUserPropValue as FieldInternal>::JSON_NAME,
    ),
    (
        <UntransmittedDocCount as FieldInternal>::TAG,
        <<UntransmittedDocCount as FieldInternal>::Type as TlvType>::REPR,
        <UntransmittedDocCount as FieldInternal>::JSON_NAME,
    ),
    (
        <UntransmittedDocDateTime as FieldInternal>::TAG,
        <<UntransmittedDocDateTime as FieldInternal>::Type as TlvType>::REPR,
        <UntransmittedDocDateTime as FieldInternal>::JSON_NAME,
    ),
    (
        <ReregistrationReason as FieldInternal>::TAG,
        <<ReregistrationReason as FieldInternal>::Type as TlvType>::REPR,
        <ReregistrationReason as FieldInternal>::JSON_NAME,
    ),
    (
        <TotalVat20Sum as FieldInternal>::TAG,
        <<TotalVat20Sum as FieldInternal>::Type as TlvType>::REPR,
        <TotalVat20Sum as FieldInternal>::JSON_NAME,
    ),
    (
        <TotalVat10Sum as FieldInternal>::TAG,
        <<TotalVat10Sum as FieldInternal>::Type as TlvType>::REPR,
        <TotalVat10Sum as FieldInternal>::JSON_NAME,
    ),
    (
        <TotalSumWithVat0 as FieldInternal>::TAG,
        <<TotalSumWithVat0 as FieldInternal>::Type as TlvType>::REPR,
        <TotalSumWithVat0 as FieldInternal>::JSON_NAME,
    ),
    (
        <TotalSumWithNoVat as FieldInternal>::TAG,
        <<TotalSumWithNoVat as FieldInternal>::Type as TlvType>::REPR,
        <TotalSumWithNoVat as FieldInternal>::JSON_NAME,
    ),
    (
        <TotalVat20_120Sum as FieldInternal>::TAG,
        <<TotalVat20_120Sum as FieldInternal>::Type as TlvType>::REPR,
        <TotalVat20_120Sum as FieldInternal>::JSON_NAME,
    ),
    (
        <TotalVat10_110Sum as FieldInternal>::TAG,
        <<TotalVat10_110Sum as FieldInternal>::Type as TlvType>::REPR,
        <TotalVat10_110Sum as FieldInternal>::JSON_NAME,
    ),
    (
        <OnlineKktFlag as FieldInternal>::TAG,
        <<OnlineKktFlag as FieldInternal>::Type as TlvType>::REPR,
        <OnlineKktFlag as FieldInternal>::JSON_NAME,
    ),
    (
        <ServiceFlag as FieldInternal>::TAG,
        <<ServiceFlag as FieldInternal>::Type as TlvType>::REPR,
        <ServiceFlag as FieldInternal>::JSON_NAME,
    ),
    (
        <BsoFlag as FieldInternal>::TAG,
        <<BsoFlag as FieldInternal>::Type as TlvType>::REPR,
        <BsoFlag as FieldInternal>::JSON_NAME,
    ),
    (
        <DocCountPerShift as FieldInternal>::TAG,
        <<DocCountPerShift as FieldInternal>::Type as TlvType>::REPR,
        <DocCountPerShift as FieldInternal>::JSON_NAME,
    ),
    (
        <UntransmittedDocNum as FieldInternal>::TAG,
        <<UntransmittedDocNum as FieldInternal>::Type as TlvType>::REPR,
        <UntransmittedDocNum as FieldInternal>::JSON_NAME,
    ),
    (
        <ReceiptSenderEmail as FieldInternal>::TAG,
        <<ReceiptSenderEmail as FieldInternal>::Type as TlvType>::REPR,
        <ReceiptSenderEmail as FieldInternal>::JSON_NAME,
    ),
    (
        <ReceiptCountPerShift as FieldInternal>::TAG,
        <<ReceiptCountPerShift as FieldInternal>::Type as TlvType>::REPR,
        <ReceiptCountPerShift as FieldInternal>::JSON_NAME,
    ),
    (
        <LotteryFlag as FieldInternal>::TAG,
        <<LotteryFlag as FieldInternal>::Type as TlvType>::REPR,
        <LotteryFlag as FieldInternal>::JSON_NAME,
    ),
    (
        <SaleStats as FieldInternal>::TAG,
        <<SaleStats as FieldInternal>::Type as TlvType>::REPR,
        <SaleStats as FieldInternal>::JSON_NAME,
    ),
    (
        <SaleReturnStats as FieldInternal>::TAG,
        <<SaleReturnStats as FieldInternal>::Type as TlvType>::REPR,
        <SaleReturnStats as FieldInternal>::JSON_NAME,
    ),
    (
        <PurchaseStats as FieldInternal>::TAG,
        <<PurchaseStats as FieldInternal>::Type as TlvType>::REPR,
        <PurchaseStats as FieldInternal>::JSON_NAME,
    ),
    (
        <PurchaseReturnStats as FieldInternal>::TAG,
        <<PurchaseReturnStats as FieldInternal>::Type as TlvType>::REPR,
        <PurchaseReturnStats as FieldInternal>::JSON_NAME,
    ),
    (
        <CorrectionStats as FieldInternal>::TAG,
        <<CorrectionStats as FieldInternal>::Type as TlvType>::REPR,
        <CorrectionStats as FieldInternal>::JSON_NAME,
    ),
    (
        <TotalReceiptAndCorrectionCount as FieldInternal>::TAG,
        <<TotalReceiptAndCorrectionCount as FieldInternal>::Type as TlvType>::REPR,
        <TotalReceiptAndCorrectionCount as FieldInternal>::JSON_NAME,
    ),
    (
        <AggregatedReceiptCount as FieldInternal>::TAG,
        <<AggregatedReceiptCount as FieldInternal>::Type as TlvType>::REPR,
        <AggregatedReceiptCount as FieldInternal>::JSON_NAME,
    ),
    (
        <TotalCashSum as FieldInternal>::TAG,
        <<TotalCashSum as FieldInternal>::Type as TlvType>::REPR,
        <TotalCashSum as FieldInternal>::JSON_NAME,
    ),
    (
        <AggregatedEcashSum as FieldInternal>::TAG,
        <<AggregatedEcashSum as FieldInternal>::Type as TlvType>::REPR,
        <AggregatedEcashSum as FieldInternal>::JSON_NAME,
    ),
    (
        <AggregatedVat20Sum as FieldInternal>::TAG,
        <<AggregatedVat20Sum as FieldInternal>::Type as TlvType>::REPR,
        <AggregatedVat20Sum as FieldInternal>::JSON_NAME,
    ),
    (
        <AggregatedVat10Sum as FieldInternal>::TAG,
        <<AggregatedVat10Sum as FieldInternal>::Type as TlvType>::REPR,
        <AggregatedVat10Sum as FieldInternal>::JSON_NAME,
    ),
    (
        <AggregatedVat20_120Sum as FieldInternal>::TAG,
        <<AggregatedVat20_120Sum as FieldInternal>::Type as TlvType>::REPR,
        <AggregatedVat20_120Sum as FieldInternal>::JSON_NAME,
    ),
    (
        <AggregatedVat10_110Sum as FieldInternal>::TAG,
        <<AggregatedVat10_110Sum as FieldInternal>::Type as TlvType>::REPR,
        <AggregatedVat10_110Sum as FieldInternal>::JSON_NAME,
    ),
    (
        <AggregatedSumWithVat0 as FieldInternal>::TAG,
        <<AggregatedSumWithVat0 as FieldInternal>::Type as TlvType>::REPR,
        <AggregatedSumWithVat0 as FieldInternal>::JSON_NAME,
    ),
    (
        <CorrectionAndUntransmittedCount as FieldInternal>::TAG,
        <<CorrectionAndUntransmittedCount as FieldInternal>::Type as TlvType>::REPR,
        <CorrectionAndUntransmittedCount as FieldInternal>::JSON_NAME,
    ),
    (
        <SaleStats as FieldInternal>::TAG,
        <<SaleStats as FieldInternal>::Type as TlvType>::REPR,
        <SaleStats as FieldInternal>::JSON_NAME,
    ),
    (
        <PurchaseStats as FieldInternal>::TAG,
        <<PurchaseStats as FieldInternal>::Type as TlvType>::REPR,
        <PurchaseStats as FieldInternal>::JSON_NAME,
    ),
    (
        <DriveStats as FieldInternal>::TAG,
        <<DriveStats as FieldInternal>::Type as TlvType>::REPR,
        <DriveStats as FieldInternal>::JSON_NAME,
    ),
    (
        <DriveUntransmittedStats as FieldInternal>::TAG,
        <<DriveUntransmittedStats as FieldInternal>::Type as TlvType>::REPR,
        <DriveUntransmittedStats as FieldInternal>::JSON_NAME,
    ),
    (
        <ProductCode as FieldInternal>::TAG,
        <<ProductCode as FieldInternal>::Type as TlvType>::REPR,
        <ProductCode as FieldInternal>::JSON_NAME,
    ),
    (
        <SupplierPhone as FieldInternal>::TAG,
        <<SupplierPhone as FieldInternal>::Type as TlvType>::REPR,
        <SupplierPhone as FieldInternal>::JSON_NAME,
    ),
    (
        <CorrectionType as FieldInternal>::TAG,
        <<CorrectionType as FieldInternal>::Type as TlvType>::REPR,
        <CorrectionType as FieldInternal>::JSON_NAME,
    ),
    (
        <CorrectionBasis as FieldInternal>::TAG,
        <<CorrectionBasis as FieldInternal>::Type as TlvType>::REPR,
        <CorrectionBasis as FieldInternal>::JSON_NAME,
    ),
    (
        <CorrectedPaymentDate as FieldInternal>::TAG,
        <<CorrectedPaymentDate as FieldInternal>::Type as TlvType>::REPR,
        <CorrectedPaymentDate as FieldInternal>::JSON_NAME,
    ),
    (
        <FnsActNumber as FieldInternal>::TAG,
        <<FnsActNumber as FieldInternal>::Type as TlvType>::REPR,
        <FnsActNumber as FieldInternal>::JSON_NAME,
    ),
    (
        <AggregatedSumWithNoVat as FieldInternal>::TAG,
        <<AggregatedSumWithNoVat as FieldInternal>::Type as TlvType>::REPR,
        <AggregatedSumWithNoVat as FieldInternal>::JSON_NAME,
    ),
    (
        <RetailPlace as FieldInternal>::TAG,
        <<RetailPlace as FieldInternal>::Type as TlvType>::REPR,
        <RetailPlace as FieldInternal>::JSON_NAME,
    ),
    (
        <KktVer as FieldInternal>::TAG,
        <<KktVer as FieldInternal>::Type as TlvType>::REPR,
        <KktVer as FieldInternal>::JSON_NAME,
    ),
    (
        <KktFfdVer as FieldInternal>::TAG,
        <<KktFfdVer as FieldInternal>::Type as TlvType>::REPR,
        <KktFfdVer as FieldInternal>::JSON_NAME,
    ),
    (
        <DriveFfdVer as FieldInternal>::TAG,
        <<DriveFfdVer as FieldInternal>::Type as TlvType>::REPR,
        <DriveFfdVer as FieldInternal>::JSON_NAME,
    ),
    (
        <AdditionalItemProp as FieldInternal>::TAG,
        <<AdditionalItemProp as FieldInternal>::Type as TlvType>::REPR,
        <AdditionalItemProp as FieldInternal>::JSON_NAME,
    ),
    (
        <AdditionalReceiptProp as FieldInternal>::TAG,
        <<AdditionalReceiptProp as FieldInternal>::Type as TlvType>::REPR,
        <AdditionalReceiptProp as FieldInternal>::JSON_NAME,
    ),
    (
        <GamblingFlag as FieldInternal>::TAG,
        <<GamblingFlag as FieldInternal>::Type as TlvType>::REPR,
        <GamblingFlag as FieldInternal>::JSON_NAME,
    ),
    (
        <ShiftStats as FieldInternal>::TAG,
        <<ShiftStats as FieldInternal>::Type as TlvType>::REPR,
        <ShiftStats as FieldInternal>::JSON_NAME,
    ),
    (
        <QrCode as FieldInternal>::TAG,
        <<QrCode as FieldInternal>::Type as TlvType>::REPR,
        <QrCode as FieldInternal>::JSON_NAME,
    ),
    (
        <Unit as FieldInternal>::TAG,
        <<Unit as FieldInternal>::Type as TlvType>::REPR,
        <Unit as FieldInternal>::JSON_NAME,
    ),
    (
        <ItemUnitVat as FieldInternal>::TAG,
        <<ItemUnitVat as FieldInternal>::Type as TlvType>::REPR,
        <ItemUnitVat as FieldInternal>::JSON_NAME,
    ),
    (
        <VatRate as FieldInternal>::TAG,
        <<VatRate as FieldInternal>::Type as TlvType>::REPR,
        <VatRate as FieldInternal>::JSON_NAME,
    ),
    (
        <ItemTotalVat as FieldInternal>::TAG,
        <<ItemTotalVat as FieldInternal>::Type as TlvType>::REPR,
        <ItemTotalVat as FieldInternal>::JSON_NAME,
    ),
    (
        <AggregatedSum as FieldInternal>::TAG,
        <<AggregatedSum as FieldInternal>::Type as TlvType>::REPR,
        <AggregatedSum as FieldInternal>::JSON_NAME,
    ),
    (
        <OperatorInn as FieldInternal>::TAG,
        <<OperatorInn as FieldInternal>::Type as TlvType>::REPR,
        <OperatorInn as FieldInternal>::JSON_NAME,
    ),
    (
        <KktInfoUpdateReason as FieldInternal>::TAG,
        <<KktInfoUpdateReason as FieldInternal>::Type as TlvType>::REPR,
        <KktInfoUpdateReason as FieldInternal>::JSON_NAME,
    ),
    (
        <OperatorMessage as FieldInternal>::TAG,
        <<OperatorMessage as FieldInternal>::Type as TlvType>::REPR,
        <OperatorMessage as FieldInternal>::JSON_NAME,
    ),
    (
        <ExciseFlag as FieldInternal>::TAG,
        <<ExciseFlag as FieldInternal>::Type as TlvType>::REPR,
        <ExciseFlag as FieldInternal>::JSON_NAME,
    ),
    (
        <ReceiptRetrievalWebsite as FieldInternal>::TAG,
        <<ReceiptRetrievalWebsite as FieldInternal>::Type as TlvType>::REPR,
        <ReceiptRetrievalWebsite as FieldInternal>::JSON_NAME,
    ),
    (
        <FfdVer as FieldInternal>::TAG,
        <<FfdVer as FieldInternal>::Type as TlvType>::REPR,
        <FfdVer as FieldInternal>::JSON_NAME,
    ),
    (
        <ItemType as FieldInternal>::TAG,
        <<ItemType as FieldInternal>::Type as TlvType>::REPR,
        <ItemType as FieldInternal>::JSON_NAME,
    ),
    (
        <FiscalSignValidityPeriod as FieldInternal>::TAG,
        <<FiscalSignValidityPeriod as FieldInternal>::Type as TlvType>::REPR,
        <FiscalSignValidityPeriod as FieldInternal>::JSON_NAME,
    ),
    (
        <PaymentMethod as FieldInternal>::TAG,
        <<PaymentMethod as FieldInternal>::Type as TlvType>::REPR,
        <PaymentMethod as FieldInternal>::JSON_NAME,
    ),
    (
        <TotalPrepaidSum as FieldInternal>::TAG,
        <<TotalPrepaidSum as FieldInternal>::Type as TlvType>::REPR,
        <TotalPrepaidSum as FieldInternal>::JSON_NAME,
    ),
    (
        <TotalCreditSum as FieldInternal>::TAG,
        <<TotalCreditSum as FieldInternal>::Type as TlvType>::REPR,
        <TotalCreditSum as FieldInternal>::JSON_NAME,
    ),
    (
        <TotalProvisionSum as FieldInternal>::TAG,
        <<TotalProvisionSum as FieldInternal>::Type as TlvType>::REPR,
        <TotalProvisionSum as FieldInternal>::JSON_NAME,
    ),
    (
        <AggregatedPrepaidSum as FieldInternal>::TAG,
        <<AggregatedPrepaidSum as FieldInternal>::Type as TlvType>::REPR,
        <AggregatedPrepaidSum as FieldInternal>::JSON_NAME,
    ),
    (
        <AggregatedCreditSum as FieldInternal>::TAG,
        <<AggregatedCreditSum as FieldInternal>::Type as TlvType>::REPR,
        <AggregatedCreditSum as FieldInternal>::JSON_NAME,
    ),
    (
        <AggregatedProvisionSum as FieldInternal>::TAG,
        <<AggregatedProvisionSum as FieldInternal>::Type as TlvType>::REPR,
        <AggregatedProvisionSum as FieldInternal>::JSON_NAME,
    ),
    (
        <PrinterFlag as FieldInternal>::TAG,
        <<PrinterFlag as FieldInternal>::Type as TlvType>::REPR,
        <PrinterFlag as FieldInternal>::JSON_NAME,
    ),
    (
        <ItemAgentTypes as FieldInternal>::TAG,
        <<ItemAgentTypes as FieldInternal>::Type as TlvType>::REPR,
        <ItemAgentTypes as FieldInternal>::JSON_NAME,
    ),
    (
        <PaymentAgentData as FieldInternal>::TAG,
        <<PaymentAgentData as FieldInternal>::Type as TlvType>::REPR,
        <PaymentAgentData as FieldInternal>::JSON_NAME,
    ),
    (
        <SupplierData as FieldInternal>::TAG,
        <<SupplierData as FieldInternal>::Type as TlvType>::REPR,
        <SupplierData as FieldInternal>::JSON_NAME,
    ),
    (
        <SupplierName as FieldInternal>::TAG,
        <<SupplierName as FieldInternal>::Type as TlvType>::REPR,
        <SupplierName as FieldInternal>::JSON_NAME,
    ),
    (
        <SupplierInn as FieldInternal>::TAG,
        <<SupplierInn as FieldInternal>::Type as TlvType>::REPR,
        <SupplierInn as FieldInternal>::JSON_NAME,
    ),
    (
        <Client as FieldInternal>::TAG,
        <<Client as FieldInternal>::Type as TlvType>::REPR,
        <Client as FieldInternal>::JSON_NAME,
    ),
    (
        <BuyerInn as FieldInternal>::TAG,
        <<BuyerInn as FieldInternal>::Type as TlvType>::REPR,
        <BuyerInn as FieldInternal>::JSON_NAME,
    ),
    (
        <ExciseDuty as FieldInternal>::TAG,
        <<ExciseDuty as FieldInternal>::Type as TlvType>::REPR,
        <ExciseDuty as FieldInternal>::JSON_NAME,
    ),
    (
        <OriginCountry as FieldInternal>::TAG,
        <<OriginCountry as FieldInternal>::Type as TlvType>::REPR,
        <OriginCountry as FieldInternal>::JSON_NAME,
    ),
    (
        <CustomsDeclarationNum as FieldInternal>::TAG,
        <<CustomsDeclarationNum as FieldInternal>::Type as TlvType>::REPR,
        <CustomsDeclarationNum as FieldInternal>::JSON_NAME,
    ),
    (
        <SaleReturnStats as FieldInternal>::TAG,
        <<SaleReturnStats as FieldInternal>::Type as TlvType>::REPR,
        <SaleReturnStats as FieldInternal>::JSON_NAME,
    ),
    (
        <PurchaseReturnStats as FieldInternal>::TAG,
        <<PurchaseReturnStats as FieldInternal>::Type as TlvType>::REPR,
        <PurchaseReturnStats as FieldInternal>::JSON_NAME,
    ),
    (
        <BuyerBirthday as FieldInternal>::TAG,
        <<BuyerBirthday as FieldInternal>::Type as TlvType>::REPR,
        <BuyerBirthday as FieldInternal>::JSON_NAME,
    ),
    (
        <Citizenship as FieldInternal>::TAG,
        <<Citizenship as FieldInternal>::Type as TlvType>::REPR,
        <Citizenship as FieldInternal>::JSON_NAME,
    ),
    (
        <BuyerIdType as FieldInternal>::TAG,
        <<BuyerIdType as FieldInternal>::Type as TlvType>::REPR,
        <BuyerIdType as FieldInternal>::JSON_NAME,
    ),
    (
        <BuyerIdData as FieldInternal>::TAG,
        <<BuyerIdData as FieldInternal>::Type as TlvType>::REPR,
        <BuyerIdData as FieldInternal>::JSON_NAME,
    ),
    (
        <BuyerAddress as FieldInternal>::TAG,
        <<BuyerAddress as FieldInternal>::Type as TlvType>::REPR,
        <BuyerAddress as FieldInternal>::JSON_NAME,
    ),
    (
        <BuyerInfo as FieldInternal>::TAG,
        <<BuyerInfo as FieldInternal>::Type as TlvType>::REPR,
        <BuyerInfo as FieldInternal>::JSON_NAME,
    ),
    (
        <IndustryItemProp as FieldInternal>::TAG,
        <<IndustryItemProp as FieldInternal>::Type as TlvType>::REPR,
        <IndustryItemProp as FieldInternal>::JSON_NAME,
    ),
    (
        <IndustryReceiptProp as FieldInternal>::TAG,
        <<IndustryReceiptProp as FieldInternal>::Type as TlvType>::REPR,
        <IndustryReceiptProp as FieldInternal>::JSON_NAME,
    ),
    (
        <FoivId as FieldInternal>::TAG,
        <<FoivId as FieldInternal>::Type as TlvType>::REPR,
        <FoivId as FieldInternal>::JSON_NAME,
    ),
    (
        <FoundationDocDateTime as FieldInternal>::TAG,
        <<FoundationDocDateTime as FieldInternal>::Type as TlvType>::REPR,
        <FoundationDocDateTime as FieldInternal>::JSON_NAME,
    ),
    (
        <FoundationDocNum as FieldInternal>::TAG,
        <<FoundationDocNum as FieldInternal>::Type as TlvType>::REPR,
        <FoundationDocNum as FieldInternal>::JSON_NAME,
    ),
    (
        <IndustryPropValue as FieldInternal>::TAG,
        <<IndustryPropValue as FieldInternal>::Type as TlvType>::REPR,
        <IndustryPropValue as FieldInternal>::JSON_NAME,
    ),
    (
        <OperationalProp as FieldInternal>::TAG,
        <<OperationalProp as FieldInternal>::Type as TlvType>::REPR,
        <OperationalProp as FieldInternal>::JSON_NAME,
    ),
    (
        <OperationId as FieldInternal>::TAG,
        <<OperationId as FieldInternal>::Type as TlvType>::REPR,
        <OperationId as FieldInternal>::JSON_NAME,
    ),
    (
        <OperationData as FieldInternal>::TAG,
        <<OperationData as FieldInternal>::Type as TlvType>::REPR,
        <OperationData as FieldInternal>::JSON_NAME,
    ),
    (
        <OperationDateTime as FieldInternal>::TAG,
        <<OperationDateTime as FieldInternal>::Type as TlvType>::REPR,
        <OperationDateTime as FieldInternal>::JSON_NAME,
    ),
    (
        <FiscalReportAdditionalProp as FieldInternal>::TAG,
        <<FiscalReportAdditionalProp as FieldInternal>::Type as TlvType>::REPR,
        <FiscalReportAdditionalProp as FieldInternal>::JSON_NAME,
    ),
    (
        <FiscalReportAdditionalData as FieldInternal>::TAG,
        <<FiscalReportAdditionalData as FieldInternal>::Type as TlvType>::REPR,
        <FiscalReportAdditionalData as FieldInternal>::JSON_NAME,
    ),
    (
        <OpenShiftAdditionalProp as FieldInternal>::TAG,
        <<OpenShiftAdditionalProp as FieldInternal>::Type as TlvType>::REPR,
        <OpenShiftAdditionalProp as FieldInternal>::JSON_NAME,
    ),
    (
        <OpenShiftAdditionalData as FieldInternal>::TAG,
        <<OpenShiftAdditionalData as FieldInternal>::Type as TlvType>::REPR,
        <OpenShiftAdditionalData as FieldInternal>::JSON_NAME,
    ),
    (
        <CloseShiftAdditionalProp as FieldInternal>::TAG,
        <<CloseShiftAdditionalProp as FieldInternal>::Type as TlvType>::REPR,
        <CloseShiftAdditionalProp as FieldInternal>::JSON_NAME,
    ),
    (
        <CloseShiftAdditionalData as FieldInternal>::TAG,
        <<CloseShiftAdditionalData as FieldInternal>::Type as TlvType>::REPR,
        <CloseShiftAdditionalData as FieldInternal>::JSON_NAME,
    ),
    (
        <CurrentStateAdditionalAttribute as FieldInternal>::TAG,
        <<CurrentStateAdditionalAttribute as FieldInternal>::Type as TlvType>::REPR,
        <CurrentStateAdditionalAttribute as FieldInternal>::JSON_NAME,
    ),
    (
        <CurrentStateAdditionalData as FieldInternal>::TAG,
        <<CurrentStateAdditionalData as FieldInternal>::Type as TlvType>::REPR,
        <CurrentStateAdditionalData as FieldInternal>::JSON_NAME,
    ),
    (
        <CloseArchiveAdditionalAttribute as FieldInternal>::TAG,
        <<CloseArchiveAdditionalAttribute as FieldInternal>::Type as TlvType>::REPR,
        <CloseArchiveAdditionalAttribute as FieldInternal>::JSON_NAME,
    ),
    (
        <CloseArchiveAdditionalData as FieldInternal>::TAG,
        <<CloseArchiveAdditionalData as FieldInternal>::Type as TlvType>::REPR,
        <CloseArchiveAdditionalData as FieldInternal>::JSON_NAME,
    ),
    (
        <KktUsageFlags as FieldInternal>::TAG,
        <<KktUsageFlags as FieldInternal>::Type as TlvType>::REPR,
        <KktUsageFlags as FieldInternal>::JSON_NAME,
    ),
    (
        <MarkedProductFractionalQuantity as FieldInternal>::TAG,
        <<MarkedProductFractionalQuantity as FieldInternal>::Type as TlvType>::REPR,
        <MarkedProductFractionalQuantity as FieldInternal>::JSON_NAME,
    ),
    (
        <FractionalPart as FieldInternal>::TAG,
        <<FractionalPart as FieldInternal>::Type as TlvType>::REPR,
        <FractionalPart as FieldInternal>::JSON_NAME,
    ),
    (
        <Numerator as FieldInternal>::TAG,
        <<Numerator as FieldInternal>::Type as TlvType>::REPR,
        <Numerator as FieldInternal>::JSON_NAME,
    ),
    (
        <Denominator as FieldInternal>::TAG,
        <<Denominator as FieldInternal>::Type as TlvType>::REPR,
        <Denominator as FieldInternal>::JSON_NAME,
    ),
    (
        <KtN as FieldInternal>::TAG,
        <<KtN as FieldInternal>::Type as TlvType>::REPR,
        <KtN as FieldInternal>::JSON_NAME,
    ),
    (
        <KtEan8 as FieldInternal>::TAG,
        <<KtEan8 as FieldInternal>::Type as TlvType>::REPR,
        <KtEan8 as FieldInternal>::JSON_NAME,
    ),
    (
        <KtEan13 as FieldInternal>::TAG,
        <<KtEan13 as FieldInternal>::Type as TlvType>::REPR,
        <KtEan13 as FieldInternal>::JSON_NAME,
    ),
    (
        <KtItf14 as FieldInternal>::TAG,
        <<KtItf14 as FieldInternal>::Type as TlvType>::REPR,
        <KtItf14 as FieldInternal>::JSON_NAME,
    ),
    (
        <KtGs1_0 as FieldInternal>::TAG,
        <<KtGs1_0 as FieldInternal>::Type as TlvType>::REPR,
        <KtGs1_0 as FieldInternal>::JSON_NAME,
    ),
    (
        <KtGs1M as FieldInternal>::TAG,
        <<KtGs1M as FieldInternal>::Type as TlvType>::REPR,
        <KtGs1M as FieldInternal>::JSON_NAME,
    ),
    (
        <KtKmk as FieldInternal>::TAG,
        <<KtKmk as FieldInternal>::Type as TlvType>::REPR,
        <KtKmk as FieldInternal>::JSON_NAME,
    ),
    (
        <KtMi as FieldInternal>::TAG,
        <<KtMi as FieldInternal>::Type as TlvType>::REPR,
        <KtMi as FieldInternal>::JSON_NAME,
    ),
    (
        <KtEgais2_0 as FieldInternal>::TAG,
        <<KtEgais2_0 as FieldInternal>::Type as TlvType>::REPR,
        <KtEgais2_0 as FieldInternal>::JSON_NAME,
    ),
    (
        <KtEgais3_0 as FieldInternal>::TAG,
        <<KtEgais3_0 as FieldInternal>::Type as TlvType>::REPR,
        <KtEgais3_0 as FieldInternal>::JSON_NAME,
    ),
    (
        <KtF1 as FieldInternal>::TAG,
        <<KtF1 as FieldInternal>::Type as TlvType>::REPR,
        <KtF1 as FieldInternal>::JSON_NAME,
    ),
    (
        <KtF2 as FieldInternal>::TAG,
        <<KtF2 as FieldInternal>::Type as TlvType>::REPR,
        <KtF2 as FieldInternal>::JSON_NAME,
    ),
    (
        <KtF3 as FieldInternal>::TAG,
        <<KtF3 as FieldInternal>::Type as TlvType>::REPR,
        <KtF3 as FieldInternal>::JSON_NAME,
    ),
    (
        <KtF4 as FieldInternal>::TAG,
        <<KtF4 as FieldInternal>::Type as TlvType>::REPR,
        <KtF4 as FieldInternal>::JSON_NAME,
    ),
    (
        <KtF5 as FieldInternal>::TAG,
        <<KtF5 as FieldInternal>::Type as TlvType>::REPR,
        <KtF5 as FieldInternal>::JSON_NAME,
    ),
    (
        <KtF6 as FieldInternal>::TAG,
        <<KtF6 as FieldInternal>::Type as TlvType>::REPR,
        <KtF6 as FieldInternal>::JSON_NAME,
    ),
    (
        <MarkingCode as FieldInternal>::TAG,
        <<MarkingCode as FieldInternal>::Type as TlvType>::REPR,
        <MarkingCode as FieldInternal>::JSON_NAME,
    ),
    (
        <RequestNumber as FieldInternal>::TAG,
        <<RequestNumber as FieldInternal>::Type as TlvType>::REPR,
        <RequestNumber as FieldInternal>::JSON_NAME,
    ),
    (
        <NotificationNumber as FieldInternal>::TAG,
        <<NotificationNumber as FieldInternal>::Type as TlvType>::REPR,
        <NotificationNumber as FieldInternal>::JSON_NAME,
    ),
    (
        <PlannedProductStatus as FieldInternal>::TAG,
        <<PlannedProductStatus as FieldInternal>::Type as TlvType>::REPR,
        <PlannedProductStatus as FieldInternal>::JSON_NAME,
    ),
    (
        <KmCheckResult as FieldInternal>::TAG,
        <<KmCheckResult as FieldInternal>::Type as TlvType>::REPR,
        <KmCheckResult as FieldInternal>::JSON_NAME,
    ),
    (
        <RequestProcessingResults as FieldInternal>::TAG,
        <<RequestProcessingResults as FieldInternal>::Type as TlvType>::REPR,
        <RequestProcessingResults as FieldInternal>::JSON_NAME,
    ),
    (
        <NotificationProcessingResults as FieldInternal>::TAG,
        <<NotificationProcessingResults as FieldInternal>::Type as TlvType>::REPR,
        <NotificationProcessingResults as FieldInternal>::JSON_NAME,
    ),
    (
        <MarkedProductData as FieldInternal>::TAG,
        <<MarkedProductData as FieldInternal>::Type as TlvType>::REPR,
        <MarkedProductData as FieldInternal>::JSON_NAME,
    ),
    (
        <MarkingCodeType as FieldInternal>::TAG,
        <<MarkingCodeType as FieldInternal>::Type as TlvType>::REPR,
        <MarkingCodeType as FieldInternal>::JSON_NAME,
    ),
    (
        <ProductId as FieldInternal>::TAG,
        <<ProductId as FieldInternal>::Type as TlvType>::REPR,
        <ProductId as FieldInternal>::JSON_NAME,
    ),
    (
        <MarkingCodeProcessingMode as FieldInternal>::TAG,
        <<MarkingCodeProcessingMode as FieldInternal>::Type as TlvType>::REPR,
        <MarkingCodeProcessingMode as FieldInternal>::JSON_NAME,
    ),
    (
        <UntransmittedNotificationCount as FieldInternal>::TAG,
        <<UntransmittedNotificationCount as FieldInternal>::Type as TlvType>::REPR,
        <UntransmittedNotificationCount as FieldInternal>::JSON_NAME,
    ),
    (
        <RequestProcessingCodes as FieldInternal>::TAG,
        <<RequestProcessingCodes as FieldInternal>::Type as TlvType>::REPR,
        <RequestProcessingCodes as FieldInternal>::JSON_NAME,
    ),
    (
        <ProductInfoCheckResult as FieldInternal>::TAG,
        <<ProductInfoCheckResult as FieldInternal>::Type as TlvType>::REPR,
        <ProductInfoCheckResult as FieldInternal>::JSON_NAME,
    ),
    (
        <MarkedProductCheckResults as FieldInternal>::TAG,
        <<MarkedProductCheckResults as FieldInternal>::Type as TlvType>::REPR,
        <MarkedProductCheckResults as FieldInternal>::JSON_NAME,
    ),
    (
        <ItemQuantityUnit as FieldInternal>::TAG,
        <<ItemQuantityUnit as FieldInternal>::Type as TlvType>::REPR,
        <ItemQuantityUnit as FieldInternal>::JSON_NAME,
    ),
    (
        <OismProductStatusResponse as FieldInternal>::TAG,
        <<OismProductStatusResponse as FieldInternal>::Type as TlvType>::REPR,
        <OismProductStatusResponse as FieldInternal>::JSON_NAME,
    ),
    (
        <AssignedProductStatus as FieldInternal>::TAG,
        <<AssignedProductStatus as FieldInternal>::Type as TlvType>::REPR,
        <AssignedProductStatus as FieldInternal>::JSON_NAME,
    ),
    (
        <NotificationProcessingCodes as FieldInternal>::TAG,
        <<NotificationProcessingCodes as FieldInternal>::Type as TlvType>::REPR,
        <NotificationProcessingCodes as FieldInternal>::JSON_NAME,
    ),
    (
        <IncorrectMarkingCodesFlags as FieldInternal>::TAG,
        <<IncorrectMarkingCodesFlags as FieldInternal>::Type as TlvType>::REPR,
        <IncorrectMarkingCodesFlags as FieldInternal>::JSON_NAME,
    ),
    (
        <IncorrectRequestsAndNotificationsFlags as FieldInternal>::TAG,
        <<IncorrectRequestsAndNotificationsFlags as FieldInternal>::Type as TlvType>::REPR,
        <IncorrectRequestsAndNotificationsFlags as FieldInternal>::JSON_NAME,
    ),
    (
        <RequestDateTime as FieldInternal>::TAG,
        <<RequestDateTime as FieldInternal>::Type as TlvType>::REPR,
        <RequestDateTime as FieldInternal>::JSON_NAME,
    ),
    (
        <MarkingCodeControlCode as FieldInternal>::TAG,
        <<MarkingCodeControlCode as FieldInternal>::Type as TlvType>::REPR,
        <MarkingCodeControlCode as FieldInternal>::JSON_NAME,
    ),
    (
        <OperationType as FieldInternal>::TAG,
        <<OperationType as FieldInternal>::Type as TlvType>::REPR,
        <OperationType as FieldInternal>::JSON_NAME,
    ),
];

pub(crate) fn all_reprs() -> &'static BTreeMap<u16, Repr> {
    static CACHE: OnceLock<BTreeMap<u16, Repr>> = OnceLock::new();
    CACHE.get_or_init(|| ALL_REPRS_DATA.iter().map(|&(a, b, _)| (a, b)).collect())
}

/*pub(crate) fn all_json_names() -> &'static BTreeMap<u16, JsonName<'static>> {
    static CACHE: OnceLock<BTreeMap<u16, JsonName<'static>>> = OnceLock::new();
    CACHE.get_or_init(|| {
        ALL_REPRS_DATA
            .iter()
            .filter_map(|&(a, _, b)| Some((a, b?)))
            .collect()
    })
}*/
