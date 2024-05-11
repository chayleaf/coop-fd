//! Note: some OFDs send the bit number instead of the entire bit flags for enums where only one
//! item must be set
use std::fmt;

use bitflags::bitflags;
use num_enum::{FromPrimitive, IntoPrimitive};
use serde::{Deserialize, Serialize};

use crate::{Result, TlvType};

bitflags! {
    /// Теги 1055/1062
    #[repr(transparent)]
    #[derive(Copy, Clone, Debug, Default, Hash, Eq, PartialEq, Deserialize, Serialize)]
    #[serde(from = "u8", into = "u8")]
    pub struct TaxationTypes: u8 {
        /// Общая система налогообложения;
        const GENERAL = 1;
        /// Упрощенная система налогообложения (доход);
        const SIMPLIFIED_GROSS = 2;
        /// Упрощенная система налогообложения (доход минус расход);
        const SIMPLIFIED_NET = 4;
        /// Единый налог на вмененный доход;
        const ENVD = 8;
        /// Единый сельскохозяйственный налог;
        const AGRICULTURAL = 16;
        /// Патентная система налогообложения.
        const PATENT = 32;
    }
}
impl From<u8> for TaxationTypes {
    fn from(value: u8) -> Self {
        Self::from_bits_retain(value)
    }
}
impl From<TaxationTypes> for u8 {
    fn from(value: TaxationTypes) -> Self {
        value.bits()
    }
}

impl TlvType for TaxationTypes {
    fn from_bytes(bytes: Vec<u8>) -> Result<Self> {
        Ok(Self::from_bits_retain(u8::from_bytes(bytes)?))
    }
    fn into_bytes(self) -> Result<Vec<u8>> {
        self.bits().into_bytes()
    }
}

bitflags! {
    /// Теги 1057/1222
    #[repr(transparent)]
    #[derive(Copy, Clone, Debug, Default, Hash, Eq, PartialEq, Deserialize, Serialize)]
    #[serde(from = "u8", into = "u8")]
    pub struct AgentTypes: u8 {
        /// Оказание услуг покупателю (клиенту) пользователем, являющимся банковским платежным агентом
        const BANK_PAYMENT_AGENT = 1;
        /// Оказание услуг покупателю (клиенту) пользователем, являющимся банковским платежным субагентом
        const BANK_PAYMENT_SUBAGENT = 2;
        /// Оказание услуг покупателю (клиенту) пользователем, являющимся платежным агентом
        const PAYMENT_AGENT = 4;
        /// Оказание услуг покупателю (клиенту) пользователем, являющимся платежным субагентом
        const PAYMENT_SUBAGENT = 8;
        /// Осуществление расчета с покупателем (клиентом) пользователем, являющимся поверенным
        const ATTORNEY = 16;
        /// Осуществление расчета с покупателем (клиентом) пользователем, являющимся комиссионером
        const COMMISSIONER = 32;
        /// Осуществление расчета с покупателем (клиентом) пользователем, являющимся агентом и не являющимся банковским платежным агентом (субагентом), платежным агентом (субагентом), поверенным, комиссионером
        const AGENT = 64;
    }
}
// FIXME: this is technically allowed to be -1 in ffd 1.05 json (why??), but i won't ever see this in production so
// who cares
impl From<u8> for AgentTypes {
    fn from(value: u8) -> Self {
        Self::from_bits_retain(value)
    }
}
impl From<AgentTypes> for u8 {
    fn from(value: AgentTypes) -> Self {
        value.bits()
    }
}

impl TlvType for AgentTypes {
    fn from_bytes(bytes: Vec<u8>) -> Result<Self> {
        Ok(Self::from_bits_retain(u8::from_bytes(bytes)?))
    }
    fn into_bytes(self) -> Result<Vec<u8>> {
        self.bits().into_bytes()
    }
}

/// Тег 1199
#[repr(u8)]
#[derive(
    Copy,
    Clone,
    Debug,
    Default,
    Deserialize,
    Serialize,
    FromPrimitive,
    IntoPrimitive,
    Hash,
    Eq,
    PartialEq,
)]
#[serde(from = "u8", into = "u8")]
pub enum VatType {
    #[default]
    Unknown = 0,
    /// Ставка НДС 20%
    Vat20 = 1,
    /// Ставка НДС 10%
    Vat10 = 2,
    /// Ставка НДС расч. 20/120%
    Vat20120 = 3,
    /// Ставка НДС расч. 10/110%
    Vat10110 = 4,
    /// Ставка НДС 0%
    Vat0 = 5,
    /// НДС не облагается
    NoVat = 6,
}

impl TlvType for VatType {
    fn from_bytes(bytes: Vec<u8>) -> Result<Self> {
        Ok(Self::from(u8::from_bytes(bytes)?))
    }
    fn into_bytes(self) -> Result<Vec<u8>> {
        (self as u8).into_bytes()
    }
}

/// Тег 1054
#[repr(u8)]
#[derive(
    Copy,
    Clone,
    Debug,
    Default,
    Deserialize,
    Serialize,
    FromPrimitive,
    IntoPrimitive,
    Hash,
    Eq,
    PartialEq,
)]
#[serde(from = "u8", into = "u8")]
pub enum PaymentType {
    #[default]
    Unknown = 0,
    /// Приход
    Sale = 1,
    /// Возврат прихода
    SaleReturn = 2,
    /// Расход
    Purchase = 3,
    /// Возврат расхода
    PurchaseReturn = 4,
}

impl TlvType for PaymentType {
    fn from_bytes(bytes: Vec<u8>) -> Result<Self> {
        Ok(Self::from(u8::from_bytes(bytes)?))
    }
    fn into_bytes(self) -> Result<Vec<u8>> {
        (self as u8).into_bytes()
    }
}

/// Без тега
#[repr(u16)]
#[derive(
    Copy,
    Clone,
    Debug,
    Default,
    Deserialize,
    Serialize,
    FromPrimitive,
    IntoPrimitive,
    Hash,
    Eq,
    PartialEq,
)]
#[serde(from = "u16", into = "u16")]
pub enum FormCode {
    #[default]
    Unknown = 0,
    /// Отчет о регистрации
    RegistrationReport = 1,
    /// Отчет об изменении параметров регистрации
    RegistrationParameterUpdateReport = 11,
    /// Отчет об открытии смены
    ShiftStartReport = 2,
    /// Отчет о текущем состоянии расчетов
    PaymentStateReport = 21,
    /// Кассовый чек
    Receipt = 3,
    /// Кассовый чек коррекции
    CorrectionReceipt = 31,
    /// Бланк строгой отчетности
    Bso = 4,
    /// Бланк строгой отчетности коррекции
    CorrectionBso = 41,
    /// Отчет о закрытии смены
    ShiftEndReport = 5,
    /// Отчет о закрытии фискального накопителя
    FnCloseReport = 6,
    /// Подтверждение оператора
    OperatorConfirmation = 7,
    /// Запрос о коде маркировки
    MarkingCodeRequest = 81,
    /// Уведомление о реализации маркированного товара
    MarkedProductSaleNotification = 82,
    /// Ответ на запрос
    Response = 83,
    /// Квитанция на уведомление
    NotificationReceipt = 84,
}

impl TlvType for FormCode {
    fn from_bytes(bytes: Vec<u8>) -> Result<Self> {
        Ok(Self::from(u16::from_bytes(bytes)?))
    }
    fn into_bytes(self) -> Result<Vec<u8>> {
        (self as u16).into_bytes()
    }
}

/// Тег 1101
#[repr(u8)]
#[derive(
    Copy,
    Clone,
    Debug,
    Default,
    Deserialize,
    Serialize,
    FromPrimitive,
    IntoPrimitive,
    Hash,
    Eq,
    PartialEq,
)]
#[serde(from = "u8", into = "u8")]
pub enum ReregistrationReason {
    #[default]
    Unknown = 0,
    /// Замена ФН
    FnReplacement = 1,
    /// Замена ОФД
    OfdReplacement = 2,
    /// Изменение реквизитов
    RequisiteUpdate = 3,
    /// Изменение настроек ККТ
    KktSettingsUpdate = 4,
}

impl TlvType for ReregistrationReason {
    fn from_bytes(bytes: Vec<u8>) -> Result<Self> {
        Ok(Self::from(u8::from_bytes(bytes)?))
    }
    fn into_bytes(self) -> Result<Vec<u8>> {
        (self as u8).into_bytes()
    }
}

/// Тег 1214
#[repr(u8)]
#[derive(
    Copy,
    Clone,
    Debug,
    Default,
    Deserialize,
    Serialize,
    FromPrimitive,
    IntoPrimitive,
    Hash,
    Eq,
    PartialEq,
)]
#[serde(from = "u8", into = "u8")]
pub enum PaymentMethod {
    #[default]
    Unknown = 0,
    /// Полная предварительная оплата до момента передачи предмета расчета
    FullPrepaid = 1,
    /// Частичная предварительная оплата до момента передачи предмета расчета
    Prepaid = 2,
    /// Аванс
    Advance = 3,
    /// Полная оплата, в том числе с учетом аванса (предварительной оплаты) в момент передачи предмета расчета
    Full = 4,
    /// Частичная оплата предмета расчета в момент его передачи с последующей оплатой в кредит
    PartialAndCredit = 5,
    /// Передача предмета расчета без его оплаты в момент его передачи с последующей оплатой в кредит
    Credit = 6,
    /// Оплата предмета расчета после его передачи с оплатой в кредит (оплата кредита)
    PaymentOfCredit = 7,
}

impl TlvType for PaymentMethod {
    fn from_bytes(bytes: Vec<u8>) -> Result<Self> {
        Ok(Self::from(u8::from_bytes(bytes)?))
    }
    fn into_bytes(self) -> Result<Vec<u8>> {
        (self as u8).into_bytes()
    }
}

/// Тег 1212
#[repr(u8)]
#[derive(
    Copy,
    Clone,
    Debug,
    Default,
    Deserialize,
    Serialize,
    FromPrimitive,
    IntoPrimitive,
    Hash,
    Eq,
    PartialEq,
)]
#[serde(from = "u8", into = "u8")]
pub enum ItemType {
    #[default]
    Unknown = 0,
    /// О реализуемом товаре, за исключением подакцизного товара (наименование и иные сведения, описывающие товар)
    Product = 1,
    /// О реализуемом подакцизном товаре (наименование и иные сведения, описывающие товар)
    ExcisableProduct = 2,
    /// О выполняемой работе (наименование и иные сведения, описывающие работу)
    Labor = 3,
    /// Об оказываемой услуге (наименование и иные сведения, описывающие услугу)
    Service = 4,
    /// О приеме ставок при осуществлении деятельности по проведению азартных игр
    Bet = 5,
    /// О выплате денежных средств в виде выигрыша при осуществлении деятельности по проведению азартных игр
    BetWinnings = 6,
    /// О приеме денежных средств при реализации лотерейных билетов, электронных лотерейных билетов, приеме лотерейных ставок при осуществлении деятельности по проведению лотерей
    LotteryTicket = 7,
    /// О выплате денежных средств в виде выигрыша при осуществлении деятельности по проведению лотерей
    LotteryWinnings = 8,
    /// О предоставлении прав на использование результатов интеллектуальной деятельности или средств индивидуализации
    NonMaterialGood = 9,
    /// Об авансе, задатке, предоплате и кредите
    Payment = 10,
    /// О вознаграждении пользователя, являющегося платежным агентом (субагентом), банковским платежным агентом (субагентом), комиссионером, поверенным или иным агентом
    AgentReward = 11,
    /// О взносе в счет оплаты пени, штрафе, вознаграждении, бонусе и ином аналогичном предмете расчета
    CompositePaymentItem = 12,
    /// О предмете расчета, не относящемуся к предметам расчета, которым может быть присвоено значение от «0» до «12» и от «14» до «26»
    OtherPaymentItem = 13,
    /// О передаче имущественных прав
    PropertyRight = 14,
    /// О внереализационном доходе
    NonRealizationIncome = 15,
    /// О суммах расходов, уменьшающих сумму налога (авансовых платежей) в соответствии с пунктом 3.1 статьи 346.21 Налогового кодекса Российской Федерации
    InsurancePremium = 16,
    /// О суммах уплаченного торгового сбора
    SalesFee = 17,
    /// О курортном сборе
    HotelTax = 18,
    /// О залоге
    Pledge = 19,
    /// О суммах произведенных расходов в соответствии со статьей 346.16 Налогового кодекса Российской Федерации, уменьшающих доход
    Expense = 20,
    /// О страховых взносах на обязательное пенсионное страхование, уплачиваемых ИП, не производящими выплаты и иные вознаграждения физическим лицам
    OpsPremiumIp = 21,
    /// О страховых взносах на обязательное пенсионное страхование, уплачиваемых организациями и ИП, производящими выплаты и иные вознаграждения физическим лицам
    OpsPremium = 22,
    /// О страховых взносах на обязательное медицинское страхование, уплачиваемых ИП, не производящими выплаты и иные вознаграждения физическим лицам
    OmsPremiumIp = 23,
    /// О страховых взносах на обязательное медицинское страхование, уплачиваемые организациями и ИП, производящими выплаты и иные вознаграждения физическим лицам
    OmsPremium = 24,
    /// О страховых взносах на обязательное социальное страхование на случай временной нетрудоспособности и в связи с материнством, на обязательное социальное страхование от несчастных случаев на производстве и профессиональных заболеваний
    OssPremium = 25,
    /// О приеме и выплате денежных средств при осуществлении казино и залами игровых автоматов расчетов с использованием обменных знаков игорного заведения
    CasinoPayment = 26,
    /// О выдаче денежных средств банковским платежным агентом
    Money = 27,
    /// О реализуемом подакцизном товаре, подлежащем маркировке средством идентификации, не имеющем кода маркировки
    Atnm = 30,
    /// О реализуемом подакцизном товаре, подлежащем маркировке средством идентификации, имеющем код маркировки
    Atm = 31,
    /// О реализуемом товаре, подлежащем маркировке средством идентификации, не имеющем кода маркировки, за исключением подакцизного товара
    Tnm = 32,
    /// О реализуемом товаре, подлежащем маркировке средством идентификации, имеющем код маркировки, за исключением подакцизного товара
    Tm = 33,
}

impl TlvType for ItemType {
    fn from_bytes(bytes: Vec<u8>) -> Result<Self> {
        Ok(Self::from(u8::from_bytes(bytes)?))
    }
    fn into_bytes(self) -> Result<Vec<u8>> {
        (self as u8).into_bytes()
    }
}

/// Тег 1022
#[repr(u8)]
#[derive(
    Copy,
    Clone,
    Debug,
    Default,
    Deserialize,
    Serialize,
    FromPrimitive,
    IntoPrimitive,
    Hash,
    Eq,
    PartialEq,
)]
#[serde(from = "u8", into = "u8")]
pub enum OfdResponse {
    #[default]
    Unknown = 255,
    /// Подтверждение успешного приема документа оператором
    Success = 0,
    /// Некорректный документ. Содержание документа не распознано
    NotRecognized = 11,
    /// Документ не прошел форматно-логический контроль
    InvalidFormat = 14,
}

impl TlvType for OfdResponse {
    fn from_bytes(bytes: Vec<u8>) -> Result<Self> {
        Ok(Self::from(u8::from_bytes(bytes)?))
    }
    fn into_bytes(self) -> Result<Vec<u8>> {
        (self as u8).into_bytes()
    }
}

/// Тег 1173
#[repr(u8)]
#[derive(
    Copy,
    Clone,
    Debug,
    Default,
    Deserialize,
    Serialize,
    FromPrimitive,
    IntoPrimitive,
    Hash,
    Eq,
    PartialEq,
)]
#[serde(from = "u8", into = "u8")]
pub enum CorrectionType {
    /// Самостоятельно
    #[default]
    SelfCorrection = 0,
    /// По предписанию
    MandatedCorrection = 1,
}

impl TlvType for CorrectionType {
    fn from_bytes(bytes: Vec<u8>) -> Result<Self> {
        Ok(if bool::from_bytes(bytes)? {
            Self::MandatedCorrection
        } else {
            Self::SelfCorrection
        })
    }
    fn into_bytes(self) -> Result<Vec<u8>> {
        (self as u8).into_bytes()
    }
}

/// Тег 1209
#[repr(u8)]
#[derive(
    Copy,
    Clone,
    Debug,
    Default,
    Deserialize,
    Serialize,
    FromPrimitive,
    IntoPrimitive,
    Hash,
    Eq,
    PartialEq,
)]
#[serde(from = "u8", into = "u8")]
pub enum FfdVersion {
    #[default]
    Unknown = 255,
    /// v1 beta
    V1Beta = 0,
    /// v1
    V1 = 1,
    /// v1.05
    V1_05 = 2,
    /// v1.1
    V1_1 = 3,
    /// v1.2
    V1_2 = 4,
}

impl TlvType for FfdVersion {
    fn from_bytes(bytes: Vec<u8>) -> Result<Self> {
        Ok(Self::from(u8::from_bytes(bytes)?))
    }
    fn into_bytes(self) -> Result<Vec<u8>> {
        (self as u8).into_bytes()
    }
}

bitflags! {
    /// Тег 1205
    #[repr(transparent)]
    #[derive(Copy, Clone, Debug, Default, Hash, Eq, PartialEq, Deserialize, Serialize)]
    #[serde(from = "u32", into = "u32")]
    pub struct KktInfoUpdateReasons: u32 {
        /// Замена фискального накопителя
        const FN_REPLACEMENT = 1;
        /// Замена оператора фискальных данных
        const FD_OPERATOR_REPLACEMENT = 2;
        /// Изменение наименования пользователя контрольно-кассовой техники
        const USER_CHANGE = 4;
        /// Изменение адреса и (или) места установки (применения) контрольно-кассовой техники
        const LOCATION_CHANGE = 8;
        /// Перевод ККТ из автономного режима в режим передачи данных
        const AUTONOMOUS_TO_DATA_TRANSFER = 0x10;
        /// Перевод ККТ из режима передачи данных в автономный режим
        const DATA_TRANSFER_TO_AUTONOMOUS = 0x20;
        /// Изменение версии модели ККТ
        const MODEL_VERSION_CHANGE = 0x40;
        /// Изменение перечня систем налогообложения, применяемых при осуществлении расчетов
        const TAXATION_SYSTEM_CHANGE = 0x80;
        /// Изменение номера автоматического устройства для расчетов, в составе которого применяется ККТ
        const AUTOMATIC_DEVICE_NUMBER_CHANGE = 0x100;
        /// Перевод ККТ из автоматического режима в неавтоматический режим (осуществление расчетов кассиром)
        const AUTOMATIC_TO_NON_AUTOMATIC = 0x200;
        /// Перевод ККТ из неавтоматического режима (осуществление расчетов кассиром) в автоматический режим
        const NON_AUTOMATIC_TO_AUTOMATIC = 0x400;
        /// Перевод ККТ из режима, не позволяющего формировать БСО, в режим, позволяющий формировать БСО
        const NON_BSO_TO_BSO = 0x800;
        /// Перевод ККТ из режима, позволяющего формировать БСО, в режим, не позволяющий формировать БСО
        const BSO_TO_NON_BSO = 0x1000;
        /// Перевод ККТ из режима расчетов в сети Интернет (позволяющего не печатать кассовый чек и БСО) в режим, позволяющий печатать кассовый чек и БСО
        const INTERNET_TO_PRINT = 0x2000;
        /// Перевод ККТ из режима, позволяющего печатать кассовый чек и БСО, в режим расчетов в сети Интернет (позволяющего не печатать кассовый чек и БСО)
        const PRINT_TO_INTERNET = 0x4000;
        /// Перевод ККТ из режима, позволяющего оказывать услуги платежного агента (субагента) или банковского платежного агента, в режим, не позволяющий оказывать услуги платежного агента (субагента) или банковского платежного агента
        const PAYMENT_AGENT_TO_NON_PAYMENT_AGENT = 0x8000;
        /// Перевод ККТ из режима, не позволяющего оказывать услуги платежного агента (субагента) или банковского платежного агента в режим, позволяющий оказывать услуги платежного агента (субагента) или банковского платежного агента
        const NON_PAYMENT_AGENT_TO_PAYMENT_AGENT = 0x10000;
        /// Перевод ККТ из режима, позволяющего применять ККТ при приеме ставок и выплате денежных средств в виде выигрыша при осуществлении деятельности по проведению азартных игр, в режим, не позволяющий применять ККТ при приеме ставок и выплате денежных средств в виде выигрыша при осуществлении деятельности по проведению азартных игр
        const GAMBLING_TO_NON_GAMBLING = 0x20000;
        /// Перевод ККТ из режима, не позволяющего применять ККТ при приеме ставок и выплате денежных средств в виде выигрыша при осуществлении деятельности по проведению азартных игр, в режим, позволяющий применять ККТ при приеме ставок и выплате денежных средств в виде выигрыша при осуществлении деятельности по проведению азартных игр
        const NON_GAMBLING_TO_GAMBLING = 0x40000;
        /// Перевод ККТ из режима, позволяющего применять ККТ при приеме денежных средств при реализации лотерейных билетов, электронных лотерейных билетов, приеме лотерейных ставок и выплате денежных средств в виде выигрыша при осуществлении деятельности по проведению лотерей, в режим, не позволяющий применять ККТ при приеме денежных средств при реализации лотерейных билетов, электронных лотерейных билетов, приеме лотерейных ставок и выплате денежных средств в виде выигрыша при осуществлении деятельности по проведению лотерей
        const LOTTERY_TO_NON_LOTTERY = 0x80000;
        /// Перевод ККТ из режима, не позволяющего применять ККТ при приеме денежных средств при реализации лотерейных билетов, электронных лотерейных билетов, приеме лотерейных ставок и выплате денежных средств в виде выигрыша при осуществлении деятельности по проведению лотерей, в режим, позволяющий применять ККТ при приеме денежных средств при реализации лотерейных билетов, электронных лотерейных билетов, приеме лотерейных ставок и выплате денежных средств в виде выигрыша при осуществлении деятельности по проведению лотерей
        const NON_LOTTERY_TO_LOTTERY = 0x100000;
        /// Изменение версии ФФД
        const FFD_VERSION_CHANGE = 0x200000;
        /// Иные причины
        const OTHER = 0x80000000;
    }
}
impl From<u32> for KktInfoUpdateReasons {
    fn from(value: u32) -> Self {
        Self::from_bits_retain(value)
    }
}
impl From<KktInfoUpdateReasons> for u32 {
    fn from(value: KktInfoUpdateReasons) -> Self {
        value.bits()
    }
}

impl TlvType for KktInfoUpdateReasons {
    fn from_bytes(bytes: Vec<u8>) -> Result<Self> {
        Ok(Self::from_bits_retain(u32::from_bytes(bytes)?))
    }
    fn into_bytes(self) -> Result<Vec<u8>> {
        self.bits().into_bytes()
    }
}

bitflags! {
    /// Тег 1206
    #[repr(transparent)]
    #[derive(Copy, Clone, Debug, Default, Hash, Eq, PartialEq, Deserialize, Serialize)]
    #[serde(from = "u8", into = "u8")]
    pub struct OperatorMessages: u8 {
        /// Ошибка форматно-логического контроля документа
        const FLK_ERROR = 2;
        /// Необходимо проверить сообщения в кабинете ККТ
        const CHECK_KKT_CABINET = 4;
        /// Требуется обновить версию ФФД в ККТ
        const FFD_UPDATE_REQUIRED = 8;
        /// ККТ включена в план проверок налогового органа
        const KKT_TO_BE_CHECKED = 16;
        /// Требуется связаться с ОФД для изменения настройки параметров связи ККТ и ОФД
        const UPDATE_OFD_COMM_PARAMS = 32;
        /// Оператор уведомляет пользователя ККТ о прекращении деятельности
        const OPERATOR_ANNULED = 64;
    }
}
impl From<u8> for OperatorMessages {
    fn from(value: u8) -> Self {
        Self::from_bits_retain(value)
    }
}
impl From<OperatorMessages> for u8 {
    fn from(value: OperatorMessages) -> Self {
        value.bits()
    }
}

impl TlvType for OperatorMessages {
    fn from_bytes(bytes: Vec<u8>) -> Result<Self> {
        Ok(Self::from_bits_retain(u8::from_bytes(bytes)?))
    }
    fn into_bytes(self) -> Result<Vec<u8>> {
        self.bits().into_bytes()
    }
}

bitflags! {
    /// Тег 1290
    #[repr(transparent)]
    #[derive(Copy, Clone, Debug, Default, Hash, Eq, PartialEq, Deserialize, Serialize)]
    #[serde(from = "u32", into = "u32")]
    pub struct KktUsage: u32 {
        /// Признак установки устройства для печати фискальных документов в корпусе автоматического устройства для расчетов
        const HAS_PRINTER = 1 << 1;
        /// Признак ККТ, являющейся автоматизированной системой для БСО (может формировать только БСО и применяться для осуществления расчетов только при оказании услуг)
        const AS_BSO = 1 << 2;
        /// Признак ККТ, предназначенной для осуществления расчетов только в сети «Интернет», в которой отсутствует устройство для печати фискальных документов в составе ККТ
        const INTERNET_ONLY = 1 << 5;
        /// Признак применения ККТ при осуществлении торговли подакцизными товарами
        const EXCISABLE_PRODUCT = 1 << 6;
        /// Признак применения ККТ при осуществлении торговли товарами, подлежащими обязательной маркировке средствами идентификации
        const MARKED_PRODUCTS = 1 << 8;
        /// Признак применения ККТ только при оказании услуг
        const SERVICES_ONLY = 1 << 9;
        /// Признак применения ККТ при проведении расчетов при приеме ставок и выплате денежных средств в виде выигрыша при осуществлении деятельности по проведению азартных игр
        const GAMBLING = 1 << 10;
        /// Признак применения ККТ при проведении расчетов при реализации лотерейных билетов, электронных лотерейных билетов, приеме лотерейных ставок и выплате денежных средств в виде выигрыша при осуществлении деятельности по проведению лотерей
        const LOTTERY = 1 << 11;
        /// Признак применения ККТ при осуществлении ломбардами кредитования граждан под залог принадлежащих гражданам вещей и деятельности по хранению вещей
        const PAWNSHOP = 1 << 12;
        /// Признак применения ККТ при осуществлении деятельности по страхованию, осуществляемой в соответствии с Законом Российской Федерации от 27 ноября 1992 года № 4015-1 «Об организации страхового дела в Российской Федерации»
        const INSURANCE = 1 << 13;
        /// Признак применения ККТ с автоматическим устройством для расчетов, содержащим внутри своего корпуса оборудование для осуществления выдачи товара покупателю в момент расчета за такой товар, в случаях, предусмотренных подпунктом 1 пункта 3.2 статьи 4.3 Федерального закона от 22.05.2003 N 54-ФЗ "О применении контрольно-кассовой техники при осуществлении расчетов в Российской Федерации"
        const VENDING_MACHINE = 1 << 14;
        /// Признак применения ККТ при оказании покупателю (клиенту) услуг общественного питания в случаях, предусмотренных подпунктом 2 пункта 3.2 статьи 4.3 Федерального закона от 22.05.2003 N 54-ФЗ "О применении контрольно-кассовой техники при осуществлении расчетов в Российской Федерации"
        const CATERING = 1 << 15;
        /// Признак применения ККТ при передаче маркированных товаров, которые имеют один код товара, входящий в состав кода идентификации, в случаях, предусмотренных подпунктом 3 пункта 3.2 статьи 4.3 Федерального закона от 22.05.2003 N 54-ФЗ "О применении контрольно-кассовой техники при осуществлении расчетов в Российской Федерации"
        const WHOLESALE = 1 << 16;
    }
}
impl From<u32> for KktUsage {
    fn from(value: u32) -> Self {
        Self::from_bits_retain(value)
    }
}
impl From<KktUsage> for u32 {
    fn from(value: KktUsage) -> Self {
        value.bits()
    }
}

impl TlvType for KktUsage {
    fn from_bytes(bytes: Vec<u8>) -> Result<Self> {
        Ok(Self::from_bits_retain(u32::from_bytes(bytes)?))
    }
    fn into_bytes(self) -> Result<Vec<u8>> {
        self.bits().into_bytes()
    }
}

/// Тег 2100
#[repr(u8)]
#[derive(
    Copy,
    Clone,
    Debug,
    Default,
    Deserialize,
    Serialize,
    FromPrimitive,
    IntoPrimitive,
    Hash,
    Eq,
    PartialEq,
)]
#[serde(from = "u8", into = "u8")]
pub enum MarkingType {
    #[default]
    Unknown = 255,
    /// Тип кода маркировки не идентифицирован
    Unidentified = 0,
    /// Короткий код маркировки
    Short = 1,
    /// Код маркировки со значением кода проверки длиной 88 символов, подлежащим проверке в ФН
    Len88ToCheck = 2,
    /// Код маркировки со значением кода проверки длиной 44 символа, не подлежащим проверке в ФН
    Len44NoCheck = 3,
    /// Код маркировки со значением кода проверки длиной 44 символа, подлежащим проверке в ФН
    Len44Check = 4,
    /// Код маркировки со значением кода проверки длиной 4 символа, не подлежащим проверке в ФН
    Len4NoCheck = 5,
}

impl TlvType for MarkingType {
    fn from_bytes(bytes: Vec<u8>) -> Result<Self> {
        Ok(Self::from(u8::from_bytes(bytes)?))
    }
    fn into_bytes(self) -> Result<Vec<u8>> {
        (self as u8).into_bytes()
    }
}

/// Тег 2003, 2110
#[repr(u8)]
#[derive(
    Copy,
    Clone,
    Debug,
    Default,
    Deserialize,
    Serialize,
    FromPrimitive,
    IntoPrimitive,
    Hash,
    Eq,
    PartialEq,
)]
#[serde(from = "u8", into = "u8")]
pub enum ProductStatus {
    #[default]
    Unknown = 0,
    /// Штучный товар, подлежащий обязательной маркировке средством идентификации, реализован
    IndividualProductSold = 1,
    /// Мерный товар, подлежащий обязательной маркировке средством идентификации, в стадии реализации
    MeasurableProductSold = 2,
    /// Штучный товар, подлежащий обязательной маркировке средством идентификации, возвращен
    IndividualProductReturned = 3,
    /// Часть товара, подлежащего обязательной маркировке средством идентификации, возвращена
    PartOfProductReturned = 4,
    /// Статус товара, подлежащего обязательной маркировке средством идентификации, не изменился
    Unchanged = 255,
}

impl TlvType for ProductStatus {
    fn from_bytes(bytes: Vec<u8>) -> Result<Self> {
        Ok(Self::from(u8::from_bytes(bytes)?))
    }
    fn into_bytes(self) -> Result<Vec<u8>> {
        (self as u8).into_bytes()
    }
}

/// Тег 2109
#[repr(u8)]
#[derive(
    Copy,
    Clone,
    Debug,
    Default,
    Deserialize,
    Serialize,
    FromPrimitive,
    IntoPrimitive,
    Hash,
    Eq,
    PartialEq,
)]
#[serde(from = "u8", into = "u8")]
pub enum OismProductStatus {
    #[default]
    Unknown = 0,
    /// Планируемый статус товара корректен
    CorrectPlannedStatus = 1,
    /// Планируемый статус товара некорректен
    IncorrectPlannedStatus = 2,
    /// Оборот товара приостановлен
    ProductOutOfSale = 3,
}

impl TlvType for OismProductStatus {
    fn from_bytes(bytes: Vec<u8>) -> Result<Self> {
        Ok(Self::from(u8::from_bytes(bytes)?))
    }
    fn into_bytes(self) -> Result<Vec<u8>> {
        (self as u8).into_bytes()
    }
}

bitflags! {
    /// Тег 2004, 2005, 2106
    #[repr(transparent)]
    #[derive(Copy, Clone, Debug, Default, Hash, Eq, PartialEq, Deserialize, Serialize)]
    #[serde(from = "u8", into = "u8")]
    pub struct MarkingCheckResult: u8 {
        /// Код маркировки проверен фискальным накопителем с использованием ключа проверки КП
        const GOT_KM_RESULT = 1;
        /// Результат проверки КП КМ фискальным накопителем с использованием ключа проверки КП положительный
        const KM_RESULT_POSITIVE = 2;
        /// Проверка статуса ОИСМ выполнена
        const GOT_OISM_RESULT = 4;
        /// От ОИСМ получены сведения, что планируемый статус товара корректен
        const OISM_RESULT_POSITIVE = 8;
        /// Результат проверки КП КМ сформирован ККТ, работающей в автономном режиме
        const KM_RESULT_FROM_AUTONOMOUS_KKT = 16;
    }
}
impl From<u8> for MarkingCheckResult {
    fn from(value: u8) -> Self {
        Self::from_bits_retain(value)
    }
}
impl From<MarkingCheckResult> for u8 {
    fn from(value: MarkingCheckResult) -> Self {
        value.bits()
    }
}

impl TlvType for MarkingCheckResult {
    fn from_bytes(bytes: Vec<u8>) -> Result<Self> {
        Ok(Self::from_bits_retain(u8::from_bytes(bytes)?))
    }
    fn into_bytes(self) -> Result<Vec<u8>> {
        self.bits().into_bytes()
    }
}

bitflags! {
    /// Тег 2112
    #[repr(transparent)]
    #[derive(Copy, Clone, Debug, Default, Hash, Eq, PartialEq, Deserialize, Serialize)]
    #[serde(from = "u8", into = "u8")]
    pub struct IncorrectMarkingCodeFlags: u8 {
        /// В течение смены или между сменами поступил ответ на запрос, содержащий сведения о некорректном КМ
        const GOT_INCORRECT_KM_RESPONSE = 2;
        /// В течение смены или между сменами поступила квитанция на уведомление, содержащая сведения о некорректном КМ
        const GOT_INCORRECT_KM_NOTIFICATION_RECEIPT = 4;
    }
}
impl From<u8> for IncorrectMarkingCodeFlags {
    fn from(value: u8) -> Self {
        Self::from_bits_retain(value)
    }
}
impl From<IncorrectMarkingCodeFlags> for u8 {
    fn from(value: IncorrectMarkingCodeFlags) -> Self {
        value.bits()
    }
}

impl TlvType for IncorrectMarkingCodeFlags {
    fn from_bytes(bytes: Vec<u8>) -> Result<Self> {
        Ok(Self::from_bits_retain(u8::from_bytes(bytes)?))
    }
    fn into_bytes(self) -> Result<Vec<u8>> {
        self.bits().into_bytes()
    }
}

bitflags! {
    /// Тег 2113
    #[repr(transparent)]
    #[derive(Copy, Clone, Debug, Default, Hash, Eq, PartialEq, Deserialize, Serialize)]
    #[serde(from = "u8", into = "u8")]
    pub struct IncorrectDataFlags: u8 {
        /// Отрицательный результат обработки запроса о коде маркировки
        const GOT_NEGATIVE_KM_RESPONSE = 1;
        /// Отрицательный результат обработки уведомления о реализации маркированного товара
        const GOT_NEGATIVE_KM_NOTIFICATION_RECEIPT = 2;
    }
}
impl From<u8> for IncorrectDataFlags {
    fn from(value: u8) -> Self {
        Self::from_bits_retain(value)
    }
}
impl From<IncorrectDataFlags> for u8 {
    fn from(value: IncorrectDataFlags) -> Self {
        value.bits()
    }
}

impl TlvType for IncorrectDataFlags {
    fn from_bytes(bytes: Vec<u8>) -> Result<Self> {
        Ok(Self::from_bits_retain(u8::from_bytes(bytes)?))
    }
    fn into_bytes(self) -> Result<Vec<u8>> {
        self.bits().into_bytes()
    }
}

/// Тег 2108
#[repr(u8)]
#[derive(
    Copy,
    Clone,
    Debug,
    Default,
    Deserialize,
    Serialize,
    FromPrimitive,
    IntoPrimitive,
    Hash,
    Eq,
    PartialEq,
)]
#[serde(from = "u8", into = "u8")]
pub enum Unit {
    #[default]
    Unknown = 0,
    /// Применяется для предметов расчета, которые могут быть реализованы поштучно или единицами
    Item = 1,
    /// Грамм
    Gram = 10,
    /// Килограмм
    Kilogram = 11,
    /// Тонна
    Ton = 12,
    /// Сантиметр
    Centimeter = 20,
    /// Дециметр
    Decimeter = 21,
    /// Метр
    Meter = 22,
    /// Квадратный сантиметр
    SquareCentimeter = 30,
    /// Квадратный дециметр
    SquareDecimeter = 31,
    /// Квадратный метр
    SquareMeter = 32,
    /// Миллилитр
    Milliliter = 40,
    /// Литр
    Liter = 41,
    /// Кубический метр
    CubeMeter = 42,
    /// Киловатт час
    KilowattHour = 50,
    /// Гигакалория
    Gigacalory = 51,
    /// Сутки (день)
    Day = 70,
    /// Час
    Hour = 71,
    /// Минута
    Minute = 72,
    /// Секунда
    Second = 73,
    /// Килобайт
    Kilobyte = 80,
    /// Мегабайт
    Megabyte = 81,
    /// Гигабайт
    Gigabyte = 82,
    /// Терабайт
    Terabyte = 83,
    /// Применяется при использовании иных единиц измерения, не поименованных в п.п. 1-23
    Other = 255,
}

impl fmt::Display for Unit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::Item => "шт.",
            Self::Gram => "г",
            Self::Kilogram => "кг",
            Self::Ton => "т",
            Self::Centimeter => "см",
            Self::Decimeter => "дм",
            Self::Meter => "м",
            Self::SquareCentimeter => "см²",
            Self::SquareDecimeter => "дм²",
            Self::SquareMeter => "м²",
            Self::Milliliter => "мл",
            Self::Liter => "л",
            Self::CubeMeter => "м²",
            Self::KilowattHour => "кВ/ч",
            Self::Gigacalory => "гкал",
            Self::Day => "сут.",
            Self::Hour => "час.",
            Self::Minute => "мин.",
            Self::Second => "сек.",
            Self::Kilobyte => "кБ",
            Self::Megabyte => "мБ",
            Self::Gigabyte => "гБ",
            Self::Terabyte => "тБ",
            Self::Other | Self::Unknown => "",
        })
    }
}

impl TlvType for Unit {
    fn from_bytes(bytes: Vec<u8>) -> Result<Self> {
        Ok(Self::from(u8::from_bytes(bytes)?))
    }
    fn into_bytes(self) -> Result<Vec<u8>> {
        (self as u8).into_bytes()
    }
}

/// Тег 1245
pub mod id {
    /// Паспорт гражданина Российской Федерации
    pub const PASSPORT: u8 = 21;
    /// Паспорт гражданина Российской Федерации, дипломатический паспорт, служебный паспорт, удостоверяющие личность гражданина Российской Федерации за пределами Российской Федерации;
    pub const FOREIGN_PASPORT: u8 = 22;
    /// Временное удостоверение личности гражданина Российской Федерации, выдаваемое на период оформления паспорта гражданина Российской Федерации
    pub const TEMP_ID: u8 = 26;
    /// Свидетельство о рождении гражданина Российской Федерации (для граждан Российской Федерации в возрасте до 14 лет)
    pub const BIRTH_CERTIFICATE: u8 = 27;
    /// Иные документы, признаваемые документами, удостоверяющими личность гражданина Российской Федерации в соответствии с законодательством Российской Федерации
    pub const OTHER_RUSSIAN_CITIZEN_ID: u8 = 28;
    /// Паспорт иностранного гражданина
    pub const FOREIGN_CITIZEN_PASSPORT: u8 = 31;
    /// Иные документы, признаваемые документами, удостоверяющими личность иностранного гражданина в соответствии с законодательством Российской Федерации и международным договором Российской Федерации
    pub const OTHER_FOREIGN_CITIZEN_ID: u8 = 32;
    /// Документ, выданный иностранным государством и признаваемый в соответствии с международным договором Российской Федерации в качестве документа, удостоверяющего личность лица без гражданства.
    pub const STATELESS_PERSON_ID: u8 = 33;
    /// Вид на жительство (для лиц без гражданства)
    pub const STATELESS_PERSON_RESIDENCE_PERMIT: u8 = 34;
    /// Разрешение на временное проживание (для лиц без гражданства)
    pub const STATELESS_PERSON_TEMP_RESIDENCE_PERMIT: u8 = 35;
    /// Свидетельство о рассмотрении ходатайства о признании лица без гражданства беженцем на территории Российской Федерации по существу
    pub const STATELESS_PERSON_REFUGEE_CONSIDERATION_CERTIFICATE: u8 = 36;
    /// Удостоверение беженца
    pub const REFUGEE_ID: u8 = 37;
    /// Иные документы, признаваемые документами, удостоверяющими личность лиц без гражданства в соответствии с законодательством Российской Федерации и международным договором Российской Федерации
    pub const OTHER_STATELESS_PERSON_ID: u8 = 38;
    /// Документ, удостоверяющий личность лица, не имеющего действительного документа, удостоверяющего личность, на период рассмотрения заявления о признании гражданином Российской Федерации или о приеме в гражданство Российской Федерации
    pub const TEMP_ID_DURING_CITIZENSHIP_APPLICATION: u8 = 40;
}

bitflags! {
    /// Тег 2116
    #[repr(transparent)]
    #[derive(Copy, Clone, Debug, Default, Hash, Eq, PartialEq, Deserialize, Serialize)]
    #[serde(from = "u8", into = "u8")]
    pub struct OperationType: u8 {
        /// Осуществление расчетов за маркированный товар при котором формируется кассовый чек
        const RECEIPT_WITH_MARKED_PRODUCT_PAYMENT = 1 << 0;
        /// Осуществление расчетов за маркированный товар при котором формируется кассовый чек коррекции
        const CORRECTION_RECEIPT_WITH_MARKED_PRODUCT_PAYMENT = 1 << 1;
        /// Осуществление расчетов за маркированный товар без формирования запроса о коде маркировки в случаях, предусмотренных подпунктом 1 пункта 3.2 статьи 4.3 Федерального закона от 22.05.2003 N 54-ФЗ "О применении контрольно-кассовой техники при осуществлении расчетов в Российской Федерации"
        const VENDING_MACHINE_WITHOUT_MC_REQUEST = 1 << 2;
        /// Осуществление расчетов за маркированный товар без формирования запроса о коде маркировки в случаях, предусмотренных подпунктом 2 пункта 3.2 статьи 4.3 Федерального закона от 22.05.2003 N 54-ФЗ "О применении контрольно-кассовой техники при осуществлении расчетов в Российской Федерации"
        const CATERING_WITHOUT_MC_REQUEST = 1 << 3;
        /// Осуществление расчетов за маркированный товар без формирования запроса о коде маркировки в случаях, предусмотренных подпунктом 3 пункта 3.2 статьи 4.3 Федерального закона от 22.05.2003 N 54-ФЗ "О применении контрольно-кассовой техники при осуществлении расчетов в Российской Федерации"
        const WHOLESALE_WITHOUT_MC_REQUEST = 1 << 4;

    }
}
impl From<u8> for OperationType {
    fn from(value: u8) -> Self {
        Self::from_bits_retain(value)
    }
}
impl From<OperationType> for u8 {
    fn from(value: OperationType) -> Self {
        value.bits()
    }
}

impl TlvType for OperationType {
    fn from_bytes(bytes: Vec<u8>) -> Result<Self> {
        Ok(Self::from_bits_retain(u8::from_bytes(bytes)?))
    }
    fn into_bytes(self) -> Result<Vec<u8>> {
        self.bits().into_bytes()
    }
}

/// Тег 2105
#[repr(u8)]
#[derive(
    Copy,
    Clone,
    Debug,
    Default,
    Deserialize,
    Serialize,
    FromPrimitive,
    IntoPrimitive,
    Hash,
    Eq,
    PartialEq,
)]
#[serde(from = "u8", into = "u8")]
pub enum KmRequestStatus {
    #[default]
    Unknown = 255,
    /// Запрос имеет корректный формат, в том числе корректный формат кода маркировки
    Correct = 0,
    /// Запрос имеет некорректный формат
    Incorrect = 1,
    /// Указанный в запросе код маркировки имеет некорректный формат (не распознан)
    IncorrectMarkingCode = 2,
}

impl TlvType for KmRequestStatus {
    fn from_bytes(bytes: Vec<u8>) -> Result<Self> {
        Ok(Self::from(u8::from_bytes(bytes)?))
    }
    fn into_bytes(self) -> Result<Vec<u8>> {
        (self as u8).into_bytes()
    }
}

/// Тег 2111
#[repr(u8)]
#[derive(
    Copy,
    Clone,
    Debug,
    Default,
    Deserialize,
    Serialize,
    FromPrimitive,
    IntoPrimitive,
    Hash,
    Eq,
    PartialEq,
)]
#[serde(from = "u8", into = "u8")]
pub enum KmNotificationStatus {
    #[default]
    Unknown = 255,
    /// Уведомление принято и обработано успешно
    Handled = 0,
    /// Некорректное уведомление
    Incorrect = 1,
}

impl TlvType for KmNotificationStatus {
    fn from_bytes(bytes: Vec<u8>) -> Result<Self> {
        Ok(Self::from(u8::from_bytes(bytes)?))
    }
    fn into_bytes(self) -> Result<Vec<u8>> {
        (self as u8).into_bytes()
    }
}

/// Тег 2006
#[repr(u8)]
#[derive(
    Copy,
    Clone,
    Debug,
    Default,
    Deserialize,
    Serialize,
    FromPrimitive,
    IntoPrimitive,
    Hash,
    Eq,
    PartialEq,
)]
#[serde(from = "u8", into = "u8")]
pub enum KmNotificationResult {
    #[default]
    Unknown = 255,
    /// Проверка всех кодов маркировки, включенных в уведомление, дала положительный результат
    Good = 0,
    /// При проверке хотя бы одного из кодов маркировки, включенных в уведомление, получен отрицательный результат
    Bad = 1,
}

impl TlvType for KmNotificationResult {
    fn from_bytes(bytes: Vec<u8>) -> Result<Self> {
        Ok(Self::from(u8::from_bytes(bytes)?))
    }
    fn into_bytes(self) -> Result<Vec<u8>> {
        (self as u8).into_bytes()
    }
}

/// Тег 1262
pub mod foiv {
    /// Министерство внутренних дел Российской Федерации
    pub const MVD: &str = "001";
    /// Министерство Российской Федерации по делам гражданской обороны, чрезвычайным ситуациям и ликвидации последствий стихийных бедствий
    pub const MCHS: &str = "002";
    /// Министерство иностранных дел Российской Федерации
    pub const MID: &str = "003";
    /// Федеральное агентство по делам Содружества Независимых Государств, соотечественников, проживающих за рубежом, и по международному гуманитарному сотрудничеству
    pub const RUSSOTRUDNICHESTVO: &str = "004";
    /// Министерство обороны Российской Федерации
    pub const MINOBORONY: &str = "005";
    /// Федеральная служба по военно-техническому сотрудничеству
    pub const FSVTS: &str = "006";
    /// Федеральная служба по техническому и экспортному контролю
    pub const FSTECH: &str = "007";
    /// Министерство юстиции Российской Федерации
    pub const MINJUST: &str = "008";
    /// Федеральная служба исполнения наказаний
    pub const FSIN: &str = "009";
    /// Федеральная служба судебных приставов
    pub const FSSP: &str = "010";
    /// Государственная фельдъегерская служба Российской Федерации (федеральная служба)
    pub const GFS: &str = "011";
    /// Служба внешней разведки Российской Федерации (федеральная служба)
    pub const SVR: &str = "012";
    /// Федеральная служба безопасности Российской Федерации (федеральная служба)
    pub const FSB: &str = "013";
    /// Федеральная служба войск национальной гвардии Российской Федерации (федеральная служба)
    pub const ROSGVARDIYA: &str = "014";
    /// Федеральная служба охраны Российской Федерации (федеральная служба)
    pub const FSO: &str = "015";
    /// Федеральная служба по финансовому мониторингу (федеральная служба)
    pub const ROSFINMONITORING: &str = "016";
    /// Федеральное архивное агентство (федеральное агентство)
    pub const ROSARKHIV: &str = "017";
    /// Главное управление специальных программ Президента Российской Федерации (федеральное агентство)
    pub const GUSPPRF: &str = "018";
    /// Управление делами Президента Российской Федерации (федеральное агентство)
    pub const UDPRF: &str = "019";
    /// Министерство здравоохранения Российской Федерации
    pub const MINZDRAV: &str = "020";
    /// Федеральная служба по надзору в сфере здравоохранения
    pub const ROSZDRAVNADZOR: &str = "021";
    /// Министерство культуры Российской Федерации
    pub const MINCULT: &str = "022";
    /// Министерство науки и высшего образования Российской Федерации
    pub const MINOBRNAUKI: &str = "023";
    /// Министерство природных ресурсов и экологии Российской Федерации
    pub const MINPRIRODY: &str = "024";
    /// Федеральная служба по гидрометеорологии и мониторингу окружающей среды
    pub const ROSHYDROMET: &str = "025";
    /// Федеральная служба по надзору в сфере природопользования
    pub const ROSPRIRODNADZOR: &str = "026";
    /// Федеральное агентство водных ресурсов
    pub const ROSVODRESURS: &str = "027";
    /// Федеральное агентство лесного хозяйства
    pub const ROSLESAGENTSVO: &str = "028";
    /// Федеральное агентство по недропользованию
    pub const ROSNEDRA: &str = "029";
    /// Министерство промышленности и торговли Российской Федерации
    pub const MINPROMTORG: &str = "030";
    /// Федеральное агентство по техническому регулированию и метрологии
    pub const ROSSTANDART: &str = "031";
    /// Министерство просвещения Российской Федерации
    pub const MINPROSVET: &str = "032";
    /// Министерство Российской Федерации по развитию Дальнего Востока и Арктики
    pub const MINVOSTOKRAZVITIYA: &str = "033";
    /// Министерство сельского хозяйства Российской Федерации
    pub const MINSELKHOZ: &str = "034";
    /// Федеральная служба по ветеринарному и фитосанитарному надзору
    pub const ROSSELKHOZNADZOR: &str = "035";
    /// Федеральное агентство по рыболовству
    pub const ROSRYBOLOVSTVO: &str = "036";
    /// Министерство спорта Российской Федерации
    pub const MINSPORT: &str = "037";
    /// Министерство строительства и жилищно-коммунального хозяйства Российской Федерации
    pub const MINGOSSTROY: &str = "038";
    /// Министерство транспорта Российской Федерации
    pub const MINTRANS: &str = "039";
    /// Федеральная служба по надзору в сфере транспорта
    pub const ROSTRANSNADZOR: &str = "040";
    /// Федеральное агентство воздушного транспорта
    pub const ROSAVIATION: &str = "041";
    /// Федеральное дорожное агентство
    pub const ROSAVTODOR: &str = "042";
    /// Федеральное агентство железнодорожного транспорта
    pub const ROSZHELDOR: &str = "043";
    /// Федеральное агентство морского и речного транспорта
    pub const ROSMORRECHFLOT: &str = "044";
    /// Министерство труда и социальной защиты Российской Федерации
    pub const MINTRUD: &str = "045";
    /// Федеральная служба по труду и занятости
    pub const ROSTRUD: &str = "046";
    /// Министерство финансов Российской Федерации
    pub const MINFIN: &str = "047";
    /// Федеральная налоговая служба
    pub const FNS: &str = "048";
    /// Федеральная пробирная палата (федеральная служба)
    pub const FPP: &str = "049";
    /// Федеральная служба по регулированию алкогольного рынка
    pub const ROSALCOHOLTABAKKONTROL: &str = "050";
    /// Федеральная таможенная служба
    pub const FTS: &str = "051";
    /// Федеральное казначейство (федеральная служба)
    pub const ROSKAZNA: &str = "052";
    /// Федеральное агентство по управлению государственным имуществом
    pub const ROSIMUSCHESTVO: &str = "053";
    /// Министерство цифрового развития, связи и массовых коммуникаций Российской Федерации
    pub const MINKOMSVYAZ: &str = "054";
    /// Федеральная служба по надзору в сфере связи, информационных технологий и массовых коммуникаций
    pub const ROSKOMNADZOR: &str = "055";
    /// Федеральное агентство по печати и массовым коммуникациям
    pub const ROSPECHAT: &str = "056";
    /// Федеральное агентство связи
    pub const ROSSVYAZ: &str = "057";
    /// Министерство экономического развития Российской Федерации
    pub const MINECON: &str = "058";
    /// Федеральная служба по аккредитации
    pub const ROSACCREDITATION: &str = "059";
    /// Федеральная служба государственной статистики
    pub const ROSSTAT: &str = "060";
    /// Федеральная служба по интеллектуальной собственности
    pub const ROSPATENT: &str = "061";
    /// Федеральное агентство по туризму
    pub const ROSTURIZM: &str = "062";
    /// Министерство энергетики Российской Федерации
    pub const MINENERGO: &str = "063";
    /// Федеральная антимонопольная служба
    pub const FAS: &str = "064";
    /// Федеральная служба государственной регистрации, кадастра и картографии
    pub const ROSREESTR: &str = "065";
    /// Федеральная служба по надзору в сфере защиты прав потребителей и благополучия человека
    pub const ROSPOTREBNADZOR: &str = "066";
    /// Федеральная служба по надзору в сфере образования и науки
    pub const ROSOBRNADZOR: &str = "067";
    /// Федеральная служба по экологическому, технологическому и атомному надзору
    pub const ROSTEKHNADZOR: &str = "068";
    /// Федеральное агентство по государственным резервам
    pub const ROSREZERV: &str = "069";
    /// Федеральное медико-биологическое агентство
    pub const FMBA: &str = "070";
    /// Федеральное агентство по делам молодежи
    pub const ROSMOLODEZH: &str = "071";
}
