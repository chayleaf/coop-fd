use bitflags::bitflags;

pub struct FieldSet(pub &'static [FieldSpec]);

bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, Debug, Default, Hash, Eq, PartialEq)]
    pub struct Form: u8 {
        const NONE = 0;
        const ELECTRONIC = 1;
        const PRINTED = 2;
        const BOTH = 3;
    }
}

pub enum Req {
    Optional,
    SometimesRequired,
    Mandatory,
}

pub struct FieldSpec {
    pub tag: Option<u16>,
    pub forms: Form,
    pub req: Req,
    /// Currently always false
    pub multi: bool,
}

/// Таблица 10
///
/// Значения реквизита «дополнительный реквизит пользователя» (тег 1084) кассового чека (БСО, кассовый чек коррекции (БСО коррекции)), с учетом особенностей сферы деятельности, в которой осуществляются расчеты
pub const ADDITIONAL_USER_INFO: FieldSet = FieldSet(&[
    // Наименование дополнительного реквизита пользователя
    FieldSpec {
        tag: Some(1085),
        forms: Form::BOTH,
        req: Req::Optional,
        multi: false,
    },
    // Значение дополнительного реквизита пользователя
    FieldSpec {
        tag: Some(1086),
        forms: Form::BOTH,
        req: Req::Optional,
        multi: false,
    },
]);

/// Таблица 15
///
/// Реквизиты, содержащиеся в отчете о регистрации и отчете об изменении параметров регистрации
pub const REGISTRATION_REPORT_1_05: FieldSet = FieldSet(&[
    // Наименование документа
    FieldSpec {
        tag: Some(1000),
        forms: Form::PRINTED,
        req: Req::Mandatory,
        multi: false,
    },
    // Код формы ФД
    FieldSpec {
        tag: None,
        forms: Form::ELECTRONIC,
        req: Req::Mandatory,
        multi: false,
    },
    // Номер версии ФФД
    FieldSpec {
        tag: Some(1209),
        forms: Form::ELECTRONIC,
        req: Req::Mandatory,
        multi: false,
    },
    // Наименование пользователя
    FieldSpec {
        tag: Some(1048),
        forms: Form::BOTH,
        req: Req::Mandatory,
        multi: false,
    },
    // ИНН пользователя
    FieldSpec {
        tag: Some(1018),
        forms: Form::BOTH,
        req: Req::Mandatory,
        multi: false,
    },
    // Системы налогообложения
    FieldSpec {
        tag: Some(1062),
        forms: Form::BOTH,
        req: Req::Optional,
        multi: false,
    },
    // Дата, время
    FieldSpec {
        tag: Some(1012),
        forms: Form::BOTH,
        req: Req::Mandatory,
        multi: false,
    },
    // Регистрационный номер ККТ
    FieldSpec {
        tag: Some(1037),
        forms: Form::BOTH,
        req: Req::Mandatory,
        multi: false,
    },
    // Признак автономного режима
    FieldSpec {
        tag: Some(1002),
        forms: Form::BOTH,
        req: Req::Mandatory,
        multi: false,
    },
    // Признак установки принтера в автомате
    FieldSpec {
        tag: Some(1221),
        forms: Form::BOTH,
        req: Req::Mandatory,
        multi: false,
    },
    // Признак АС БСО
    FieldSpec {
        tag: Some(1110),
        forms: Form::BOTH,
        req: Req::Mandatory,
        multi: false,
    },
    // Признак шифрования
    FieldSpec {
        tag: Some(1056),
        forms: Form::BOTH,
        req: Req::Mandatory,
        multi: false,
    },
    // Признак автоматического режима
    FieldSpec {
        tag: Some(1001),
        forms: Form::BOTH,
        req: Req::Mandatory,
        multi: false,
    },
    // Признак ККТ для расчетов только в Интернет
    FieldSpec {
        tag: Some(1108),
        forms: Form::BOTH,
        req: Req::Mandatory,
        multi: false,
    },
    // Номер автомата
    FieldSpec {
        tag: Some(1036),
        forms: Form::BOTH,
        req: Req::SometimesRequired,
        multi: false,
    },
    // Признак торговли подакцизными товарами
    FieldSpec {
        tag: Some(1207),
        forms: Form::BOTH,
        req: Req::Optional,
        multi: false,
    },
    // Ресурс ключей ФП
    FieldSpec {
        tag: Some(1213),
        forms: Form::BOTH,
        req: Req::Optional,
        multi: false,
    },
    // Признак расчетов за услуги
    FieldSpec {
        tag: Some(1109),
        forms: Form::BOTH,
        req: Req::Optional,
        multi: false,
    },
    // Признак проведения азартных игр
    FieldSpec {
        tag: Some(1193),
        forms: Form::BOTH,
        req: Req::Optional,
        multi: false,
    },
    // Признак проведения лотереи
    FieldSpec {
        tag: Some(1126),
        forms: Form::BOTH,
        req: Req::Optional,
        multi: false,
    },
    // Признак агента
    FieldSpec {
        tag: Some(1057),
        forms: Form::BOTH,
        req: Req::Optional,
        multi: false,
    },
    // Заводской номер ККТ
    FieldSpec {
        tag: Some(1013),
        forms: Form::BOTH,
        req: Req::Mandatory,
        multi: false,
    },
    // Кассир
    FieldSpec {
        tag: Some(1021),
        forms: Form::BOTH,
        req: Req::Mandatory,
        multi: false,
    },
    // ИНН кассира
    FieldSpec {
        tag: Some(1203),
        forms: Form::ELECTRONIC,
        req: Req::Optional,
        multi: false,
    },
    // Адрес расчетов
    FieldSpec {
        tag: Some(1009),
        forms: Form::BOTH,
        req: Req::Mandatory,
        multi: false,
    },
    // Место расчетов
    FieldSpec {
        tag: Some(1187),
        forms: Form::BOTH,
        req: Req::Mandatory,
        multi: false,
    },
    // Адрес сайта ФНС
    FieldSpec {
        tag: Some(1060),
        forms: Form::BOTH,
        req: Req::SometimesRequired,
        multi: false,
    },
    // Адрес электронной почты отправителя чека
    FieldSpec {
        tag: Some(1117),
        forms: Form::BOTH,
        req: Req::SometimesRequired,
        multi: false,
    },
    // ИНН ОФД
    FieldSpec {
        tag: Some(1017),
        forms: Form::BOTH,
        req: Req::SometimesRequired,
        multi: false,
    },
    // Наименование ОФД
    FieldSpec {
        tag: Some(1046),
        forms: Form::BOTH,
        req: Req::SometimesRequired,
        multi: false,
    },
    // Версия ККТ
    FieldSpec {
        tag: Some(1188),
        forms: Form::BOTH,
        req: Req::Mandatory,
        multi: false,
    },
    // Версия ФФД ККТ
    FieldSpec {
        tag: Some(1189),
        forms: Form::BOTH,
        req: Req::Mandatory,
        multi: false,
    },
    // Версия ФФД ФН
    FieldSpec {
        tag: Some(1190),
        forms: Form::BOTH,
        req: Req::Optional,
        multi: false,
    },
    // Номер ФД
    FieldSpec {
        tag: Some(1040),
        forms: Form::BOTH,
        req: Req::Mandatory,
        multi: false,
    },
    // Номер ФН
    FieldSpec {
        tag: Some(1041),
        forms: Form::BOTH,
        req: Req::Mandatory,
        multi: false,
    },
    // ФПД (1)
    FieldSpec {
        tag: Some(1077),
        forms: Form::BOTH,
        req: Req::Mandatory,
        multi: false,
    },
    // ФПС (4)
    FieldSpec {
        tag: None,
        forms: Form::ELECTRONIC,
        req: Req::Mandatory,
        multi: false,
    },
]);

/// Таблица 16
///
/// Реквизиты, содержащиеся в отчете об изменении параметров регистрации
pub const REGISTRATION_PARAMETER_UPDATE_REPORT_1_05: FieldSet = FieldSet(&[
    // Код причины перерегистрации
    FieldSpec {
        tag: Some(1101),
        forms: Form::BOTH,
        req: Req::SometimesRequired,
        multi: true,
    },
]);

/// Таблица 18
///
/// Реквизиты, содержащиеся в отчете об открытии смены
pub const SHIFT_START_REPORT_1_05: FieldSet = FieldSet(&[
    // Наименование документа
    FieldSpec {
        tag: Some(1000),
        forms: Form::PRINTED,
        req: Req::Mandatory,
        multi: false,
    },
    // Код формы ФД
    FieldSpec {
        tag: None,
        forms: Form::ELECTRONIC,
        req: Req::Mandatory,
        multi: false,
    },
    // Номер версии ФФД
    FieldSpec {
        tag: Some(1209),
        forms: Form::ELECTRONIC,
        req: Req::Mandatory,
        multi: false,
    },
    // Наименование пользователя
    FieldSpec {
        tag: Some(1048),
        forms: Form::PRINTED,
        req: Req::Mandatory,
        multi: false,
    },
    // ИНН пользователя
    FieldSpec {
        tag: Some(1018),
        forms: Form::BOTH,
        req: Req::Mandatory,
        multi: false,
    },
    // Кассир
    FieldSpec {
        tag: Some(1021),
        forms: Form::BOTH,
        req: Req::SometimesRequired,
        multi: false,
    },
    // ИНН кассира
    FieldSpec {
        tag: Some(1203),
        forms: Form::ELECTRONIC,
        req: Req::Optional,
        multi: false,
    },
    // Адрес расчетов
    FieldSpec {
        tag: Some(1009),
        forms: Form::PRINTED,
        req: Req::Mandatory,
        multi: false,
    },
    // Место расчетов
    FieldSpec {
        tag: Some(1187),
        forms: Form::PRINTED,
        req: Req::Mandatory,
        multi: false,
    },
    // Дата, время
    FieldSpec {
        tag: Some(1012),
        forms: Form::BOTH,
        req: Req::Mandatory,
        multi: false,
    },
    // Номер смены
    FieldSpec {
        tag: Some(1038),
        forms: Form::BOTH,
        req: Req::Mandatory,
        multi: false,
    },
    // Регистрационный номер ККТ
    FieldSpec {
        tag: Some(1037),
        forms: Form::BOTH,
        req: Req::Mandatory,
        multi: false,
    },
    // Признак превышения времени ожидания ответа ОФД
    FieldSpec {
        tag: Some(1053),
        forms: Form::BOTH,
        req: Req::Optional,
        multi: false,
    },
    // Признак необходимости срочной замены ФН
    FieldSpec {
        tag: Some(1051),
        forms: Form::BOTH,
        req: Req::Optional,
        multi: false,
    },
    // Признак переполнения памяти ФН
    FieldSpec {
        tag: Some(1052),
        forms: Form::BOTH,
        req: Req::Optional,
        multi: false,
    },
    // Признак исчерпания ресурса ФН
    FieldSpec {
        tag: Some(1050),
        forms: Form::BOTH,
        req: Req::Optional,
        multi: false,
    },
    // Версия ККТ
    FieldSpec {
        tag: Some(1188),
        forms: Form::BOTH,
        req: Req::Mandatory,
        multi: false,
    },
    // Версия ФФД ККТ
    FieldSpec {
        tag: Some(1189),
        forms: Form::BOTH,
        req: Req::Mandatory,
        multi: false,
    },
    // Номер ФД
    FieldSpec {
        tag: Some(1040),
        forms: Form::BOTH,
        req: Req::Mandatory,
        multi: false,
    },
    // Номер ФН
    FieldSpec {
        tag: Some(1041),
        forms: Form::BOTH,
        req: Req::Mandatory,
        multi: false,
    },
    // ФПД (1)
    FieldSpec {
        tag: Some(1077),
        forms: Form::BOTH,
        req: Req::Mandatory,
        multi: false,
    },
    // ФПС (4)
    FieldSpec {
        tag: None,
        forms: Form::ELECTRONIC,
        req: Req::Mandatory,
        multi: false,
    },
]);

/// Таблица 19
///
/// Реквизиты, содержащиеся в отчете о текущем состоянии расчетов
pub const PAYMENT_STATE_REPORT_1_05: FieldSet = FieldSet(&[
    // Наименование документа
    FieldSpec {
        tag: Some(1000),
        forms: Form::PRINTED,
        req: Req::Mandatory,
        multi: false,
    },
    // Код формы ФД
    FieldSpec {
        tag: None,
        forms: Form::ELECTRONIC,
        req: Req::Mandatory,
        multi: false,
    },
    // Номер версии ФФД
    FieldSpec {
        tag: Some(1209),
        forms: Form::ELECTRONIC,
        req: Req::Mandatory,
        multi: false,
    },
    // Наименование пользователя
    FieldSpec {
        tag: Some(1048),
        forms: Form::PRINTED,
        req: Req::Mandatory,
        multi: false,
    },
    // ИНН пользователя
    FieldSpec {
        tag: Some(1018),
        forms: Form::BOTH,
        req: Req::Mandatory,
        multi: false,
    },
    // Адрес расчетов
    FieldSpec {
        tag: Some(1009),
        forms: Form::PRINTED,
        req: Req::Mandatory,
        multi: false,
    },
    // Место расчетов
    FieldSpec {
        tag: Some(1187),
        forms: Form::PRINTED,
        req: Req::Mandatory,
        multi: false,
    },
    // Дата, время
    FieldSpec {
        tag: Some(1012),
        forms: Form::BOTH,
        req: Req::Mandatory,
        multi: false,
    },
    // Номер смены
    FieldSpec {
        tag: Some(1038),
        forms: Form::BOTH,
        req: Req::SometimesRequired,
        multi: false,
    },
    // Признак автономного режима
    FieldSpec {
        tag: Some(1002),
        forms: Form::BOTH,
        req: Req::SometimesRequired,
        multi: false,
    },
    // Номер первого непереданного документа
    FieldSpec {
        tag: Some(1116),
        forms: Form::BOTH,
        req: Req::SometimesRequired,
        multi: false,
    },
    // Количество непереданных ФД
    FieldSpec {
        tag: Some(1097),
        forms: Form::BOTH,
        req: Req::SometimesRequired,
        multi: false,
    },
    // Дата первого из непереданных ФД
    FieldSpec {
        tag: Some(1098),
        forms: Form::BOTH,
        req: Req::SometimesRequired,
        multi: false,
    },
    // Регистрационный номер ККТ
    FieldSpec {
        tag: Some(1037),
        forms: Form::BOTH,
        req: Req::Mandatory,
        multi: false,
    },
    // Номер ФД
    FieldSpec {
        tag: Some(1040),
        forms: Form::BOTH,
        req: Req::Mandatory,
        multi: false,
    },
    // Номер ФН
    FieldSpec {
        tag: Some(1041),
        forms: Form::BOTH,
        req: Req::Mandatory,
        multi: false,
    },
    // ФПД (1)
    FieldSpec {
        tag: Some(1077),
        forms: Form::BOTH,
        req: Req::Mandatory,
        multi: false,
    },
    // ФПС (4)
    FieldSpec {
        tag: None,
        forms: Form::ELECTRONIC,
        req: Req::Mandatory,
        multi: false,
    },
]);

/// Таблица 20
///
/// Реквизиты, содержащиеся в кассовом чеке (БСО)
pub const RECEIPT_1_05: FieldSet = FieldSet(&[
    // Наименование документа
    FieldSpec {
        tag: Some(1000),
        forms: Form::PRINTED,
        req: Req::Mandatory,
        multi: false,
    },
    // Код формы ФД
    FieldSpec {
        tag: None,
        forms: Form::ELECTRONIC,
        req: Req::Mandatory,
        multi: false,
    },
    // Номер версии ФФД
    FieldSpec {
        tag: Some(1209),
        forms: Form::ELECTRONIC,
        req: Req::Mandatory,
        multi: false,
    },
    // Наименование пользователя
    FieldSpec {
        tag: Some(1048),
        forms: Form::BOTH,
        req: Req::Optional,
        multi: false,
    },
    // ИНН пользователя
    FieldSpec {
        tag: Some(1018),
        forms: Form::BOTH,
        req: Req::Optional,
        multi: false,
    },
    // Покупатель (клиент)
    FieldSpec {
        tag: Some(1227),
        forms: Form::BOTH,
        req: Req::Optional,
        multi: false,
    },
    // ИНН покупателя (клиента)
    FieldSpec {
        tag: Some(1228),
        forms: Form::BOTH,
        req: Req::Optional,
        multi: false,
    },
    // Номер чека за смену
    FieldSpec {
        tag: Some(1042),
        forms: Form::ELECTRONIC,
        req: Req::Mandatory,
        multi: false,
    },
    // Дата, время
    FieldSpec {
        tag: Some(1012),
        forms: Form::BOTH,
        req: Req::Mandatory,
        multi: false,
    },
    // Номер смены
    FieldSpec {
        tag: Some(1038),
        forms: Form::ELECTRONIC,
        req: Req::Mandatory,
        multi: false,
    },
    // Признак расчета
    FieldSpec {
        tag: Some(1054),
        forms: Form::BOTH,
        req: Req::Mandatory,
        multi: false,
    },
    // Применяемая система налогообложения
    FieldSpec {
        tag: Some(1055),
        forms: Form::BOTH,
        req: Req::Optional,
        multi: false,
    },
    // Кассир
    FieldSpec {
        tag: Some(1021),
        forms: Form::BOTH,
        req: Req::SometimesRequired,
        multi: false,
    },
    // ИНН кассира
    FieldSpec {
        tag: Some(1203),
        forms: Form::ELECTRONIC,
        req: Req::Optional,
        multi: false,
    },
    // Регистрационный номер ККТ
    FieldSpec {
        tag: Some(1037),
        forms: Form::BOTH,
        req: Req::Mandatory,
        multi: false,
    },
    // Номер автомата
    FieldSpec {
        tag: Some(1036),
        forms: Form::BOTH,
        req: Req::SometimesRequired,
        multi: false,
    },
    // Адрес расчетов
    FieldSpec {
        tag: Some(1009),
        forms: Form::BOTH,
        req: Req::Optional,
        multi: false,
    },
    // Место расчетов
    FieldSpec {
        tag: Some(1187),
        forms: Form::BOTH,
        req: Req::Optional,
        multi: false,
    },
    // Телефон или электронный адрес покупателя
    FieldSpec {
        tag: Some(1008),
        forms: Form::BOTH,
        req: Req::Optional,
        multi: false,
    },
    // Предмет расчета
    FieldSpec {
        tag: Some(1059),
        forms: Form::BOTH,
        req: Req::Mandatory,
        multi: true,
    },
    // Сумма расчета, указанного в чеке (БСО)
    FieldSpec {
        tag: Some(1020),
        forms: Form::BOTH,
        req: Req::Mandatory,
        multi: false,
    },
    // Сумма по чеку (БСО) наличными
    FieldSpec {
        tag: Some(1031),
        forms: Form::BOTH,
        req: Req::Optional,
        multi: false,
    },
    // Сумма по чеку (БСО) безналичными
    FieldSpec {
        tag: Some(1081),
        forms: Form::BOTH,
        req: Req::Optional,
        multi: false,
    },
    // Сумма по чеку (БСО) предоплатой (зачетом аванса и (или) предыдущих платежей)
    FieldSpec {
        tag: Some(1215),
        forms: Form::BOTH,
        req: Req::Optional,
        multi: false,
    },
    // Сумма по чеку (БСО) постоплатой (в кредит)
    FieldSpec {
        tag: Some(1216),
        forms: Form::BOTH,
        req: Req::Optional,
        multi: false,
    },
    // Сумма по чеку (БСО) встречным предоставлением
    FieldSpec {
        tag: Some(1217),
        forms: Form::BOTH,
        req: Req::Optional,
        multi: false,
    },
    // Сумма НДС чека по ставке 20%
    FieldSpec {
        tag: Some(1102),
        forms: Form::BOTH,
        req: Req::SometimesRequired,
        multi: false,
    },
    // Сумма НДС чека по ставке 10%
    FieldSpec {
        tag: Some(1103),
        forms: Form::BOTH,
        req: Req::SometimesRequired,
        multi: false,
    },
    // Сумма расчета по чеку с НДС по ставке 0%
    FieldSpec {
        tag: Some(1104),
        forms: Form::BOTH,
        req: Req::SometimesRequired,
        multi: false,
    },
    // Сумма расчета по чеку без НДС
    FieldSpec {
        tag: Some(1105),
        forms: Form::BOTH,
        req: Req::SometimesRequired,
        multi: false,
    },
    // Сумма НДС чека по расч. ставке 20/120
    FieldSpec {
        tag: Some(1106),
        forms: Form::BOTH,
        req: Req::SometimesRequired,
        multi: false,
    },
    // Сумма НДС чека по расч. ставке 10/110
    FieldSpec {
        tag: Some(1107),
        forms: Form::BOTH,
        req: Req::SometimesRequired,
        multi: false,
    },
    // Признак ККТ для расчетов только в Интернет
    FieldSpec {
        tag: Some(1108),
        forms: Form::ELECTRONIC,
        req: Req::Optional,
        multi: false,
    },
    // Адрес электронной почты отправителя чека
    FieldSpec {
        tag: Some(1117),
        forms: Form::ELECTRONIC,
        req: Req::SometimesRequired,
        multi: false,
    },
    // Признак агента
    FieldSpec {
        tag: Some(1057),
        forms: Form::BOTH,
        req: Req::SometimesRequired,
        multi: false,
    },
    // Телефон оператора перевода
    FieldSpec {
        tag: Some(1075),
        forms: Form::BOTH,
        req: Req::Optional,
        multi: true,
    },
    // Операция банковского платежного агента
    FieldSpec {
        tag: Some(1044),
        forms: Form::BOTH,
        req: Req::SometimesRequired,
        multi: false,
    },
    // Телефон платежного агента
    FieldSpec {
        tag: Some(1073),
        forms: Form::BOTH,
        req: Req::Optional,
        multi: true,
    },
    // Телефон оператора по приему платежей
    FieldSpec {
        tag: Some(1074),
        forms: Form::BOTH,
        req: Req::Optional,
        multi: true,
    },
    // Наименование оператора перевода
    FieldSpec {
        tag: Some(1026),
        forms: Form::BOTH,
        req: Req::SometimesRequired,
        multi: false,
    },
    // Адрес оператора перевода
    FieldSpec {
        tag: Some(1005),
        forms: Form::BOTH,
        req: Req::SometimesRequired,
        multi: false,
    },
    // ИНН оператора перевода
    FieldSpec {
        tag: Some(1016),
        forms: Form::BOTH,
        req: Req::SometimesRequired,
        multi: false,
    },
    // Телефон поставщика
    FieldSpec {
        tag: Some(1171),
        forms: Form::BOTH,
        req: Req::Optional,
        multi: true,
    },
    // Адрес сайта ФНС
    FieldSpec {
        tag: Some(1060),
        forms: Form::BOTH,
        req: Req::Optional,
        multi: false,
    },
    // Дополнительный реквизит чека (БСО)
    FieldSpec {
        tag: Some(1192),
        forms: Form::BOTH,
        req: Req::Optional,
        multi: false,
    },
    // Дополнительный реквизит пользователя
    FieldSpec {
        tag: Some(1084),
        forms: Form::BOTH,
        req: Req::Optional,
        multi: false,
    },
    // Номер ФД
    FieldSpec {
        tag: Some(1040),
        forms: Form::BOTH,
        req: Req::Mandatory,
        multi: false,
    },
    // Номер ФН
    FieldSpec {
        tag: Some(1041),
        forms: Form::BOTH,
        req: Req::Mandatory,
        multi: false,
    },
    // ФПД (1)
    FieldSpec {
        tag: Some(1077),
        forms: Form::BOTH,
        req: Req::Mandatory,
        multi: false,
    },
    // ФПС (4)
    FieldSpec {
        tag: None,
        forms: Form::ELECTRONIC,
        req: Req::Mandatory,
        multi: false,
    },
    // QR-код
    FieldSpec {
        tag: Some(1196),
        forms: Form::PRINTED,
        req: Req::Mandatory,
        multi: false,
    },
]);

/// Таблица 21
///
/// Структура данных реквизита «предмет расчета» (тег 1059)
pub const PAYMENT_ITEM_1_05: FieldSet = FieldSet(&[
    // Признак способа расчета
    FieldSpec {
        tag: Some(1214),
        forms: Form::BOTH,
        req: Req::SometimesRequired,
        multi: false,
    },
    // Признак предмета расчета
    FieldSpec {
        tag: Some(1212),
        forms: Form::BOTH,
        req: Req::Optional,
        multi: false,
    },
    // Признак агента по предмету расчета
    FieldSpec {
        tag: Some(1222),
        forms: Form::BOTH,
        req: Req::Optional,
        multi: false,
    },
    // Данные агента
    FieldSpec {
        tag: Some(1223),
        forms: Form::BOTH,
        req: Req::Optional,
        multi: false,
    },
    // Данные поставщика
    FieldSpec {
        tag: Some(1224),
        forms: Form::BOTH,
        req: Req::Optional,
        multi: false,
    },
    // ИНН поставщика
    FieldSpec {
        tag: Some(1226),
        forms: Form::BOTH,
        req: Req::Optional,
        multi: false,
    },
    // Наименование предмета расчета
    FieldSpec {
        tag: Some(1030),
        forms: Form::BOTH,
        req: Req::Mandatory,
        multi: false,
    },
    // Единица измерения предмета расчета
    FieldSpec {
        tag: Some(1197),
        forms: Form::BOTH,
        req: Req::Optional,
        multi: false,
    },
    // Код товара
    FieldSpec {
        tag: Some(1162),
        forms: Form::BOTH,
        req: Req::SometimesRequired,
        multi: false,
    },
    // Акциз
    FieldSpec {
        tag: Some(1229),
        forms: Form::BOTH,
        req: Req::Optional,
        multi: false,
    },
    // Код страны происхождения товара
    FieldSpec {
        tag: Some(1230),
        forms: Form::BOTH,
        req: Req::Optional,
        multi: false,
    },
    // Номер декларации на товар
    FieldSpec {
        tag: Some(1231),
        forms: Form::BOTH,
        req: Req::Optional,
        multi: false,
    },
    // Цена за единицу предмета расчета с учетом скидок и наценок
    FieldSpec {
        tag: Some(1079),
        forms: Form::BOTH,
        req: Req::Mandatory,
        multi: false,
    },
    // Размер НДС за единицу предмета расчета
    FieldSpec {
        tag: Some(1198),
        forms: Form::BOTH,
        req: Req::Optional,
        multi: false,
    },
    // Количество предмета расчета
    FieldSpec {
        tag: Some(1023),
        forms: Form::BOTH,
        req: Req::SometimesRequired,
        multi: false,
    },
    // Ставка НДС
    FieldSpec {
        tag: Some(1199),
        forms: Form::BOTH,
        req: Req::SometimesRequired,
        multi: false,
    },
    // Сумма НДС за предмет расчета
    FieldSpec {
        tag: Some(1200),
        forms: Form::BOTH,
        req: Req::Optional,
        multi: false,
    },
    // Стоимость предмета расчета с учетом скидок и наценок
    FieldSpec {
        tag: Some(1043),
        forms: Form::BOTH,
        req: Req::Mandatory,
        multi: false,
    },
    // Дополнительный реквизит предмета расчета
    FieldSpec {
        tag: Some(1191),
        forms: Form::BOTH,
        req: Req::Optional,
        multi: false,
    },
]);

/// Таблица 22
///
/// Значения реквизита «данные агента» (тег 1223)
pub const AGENT_INFO_1_05: FieldSet = FieldSet(&[
    // Телефон оператора перевода
    FieldSpec {
        tag: Some(1075),
        forms: Form::BOTH,
        req: Req::Optional,
        multi: true,
    },
    // Операция банковского платежного агента
    FieldSpec {
        tag: Some(1044),
        forms: Form::BOTH,
        req: Req::Optional,
        multi: false,
    },
    // Телефон платежного агента
    FieldSpec {
        tag: Some(1073),
        forms: Form::BOTH,
        req: Req::Optional,
        multi: true,
    },
    // Телефон оператора по приему платежей
    FieldSpec {
        tag: Some(1074),
        forms: Form::BOTH,
        req: Req::Optional,
        multi: true,
    },
    // Наименование оператора перевода
    FieldSpec {
        tag: Some(1026),
        forms: Form::BOTH,
        req: Req::Optional,
        multi: false,
    },
    // Адрес оператора перевода
    FieldSpec {
        tag: Some(1005),
        forms: Form::BOTH,
        req: Req::Optional,
        multi: false,
    },
    // ИНН оператора перевода
    FieldSpec {
        tag: Some(1016),
        forms: Form::BOTH,
        req: Req::Optional,
        multi: false,
    },
]);

/// Таблица 23
///
/// Значения реквизита «данные поставщика» (тег 1224)
pub const SUPPLIER_INFO_1_05: FieldSet = FieldSet(&[
    // Телефон поставщика
    FieldSpec {
        tag: Some(1171),
        forms: Form::BOTH,
        req: Req::Optional,
        multi: true,
    },
    // Наименование поставщика
    FieldSpec {
        tag: Some(1225),
        forms: Form::BOTH,
        req: Req::Optional,
        multi: false,
    },
]);

/// Таблица 27
///
/// Реквизиты, содержащиеся в кассовом чеке коррекции (БСО коррекции)
pub const CORRECTION_RECEIPT_1_05: FieldSet = FieldSet(&[
    // Наименование документа
    FieldSpec {
        tag: Some(1000),
        forms: Form::PRINTED,
        req: Req::Mandatory,
        multi: false,
    },
    // Код формы ФД
    FieldSpec {
        tag: None,
        forms: Form::ELECTRONIC,
        req: Req::Mandatory,
        multi: false,
    },
    // Номер версии ФФД
    FieldSpec {
        tag: Some(1209),
        forms: Form::ELECTRONIC,
        req: Req::Mandatory,
        multi: false,
    },
    // Наименование пользователя
    FieldSpec {
        tag: Some(1048),
        forms: Form::BOTH,
        req: Req::Optional,
        multi: false,
    },
    // ИНН пользователя
    FieldSpec {
        tag: Some(1018),
        forms: Form::BOTH,
        req: Req::Optional,
        multi: false,
    },
    // Покупатель (клиент)
    FieldSpec {
        tag: Some(1227),
        forms: Form::BOTH,
        req: Req::Optional,
        multi: false,
    },
    // ИНН покупателя (клиента)
    FieldSpec {
        tag: Some(1228),
        forms: Form::BOTH,
        req: Req::Optional,
        multi: false,
    },
    // Номер чека за смену
    FieldSpec {
        tag: Some(1042),
        forms: Form::ELECTRONIC,
        req: Req::Mandatory,
        multi: false,
    },
    // Дата, время
    FieldSpec {
        tag: Some(1012),
        forms: Form::BOTH,
        req: Req::Mandatory,
        multi: false,
    },
    // Номер смены
    FieldSpec {
        tag: Some(1038),
        forms: Form::ELECTRONIC,
        req: Req::Mandatory,
        multi: false,
    },
    // Признак расчета
    FieldSpec {
        tag: Some(1054),
        forms: Form::BOTH,
        req: Req::Mandatory,
        multi: false,
    },
    // Применяемая система налогообложения
    FieldSpec {
        tag: Some(1055),
        forms: Form::BOTH,
        req: Req::Optional,
        multi: false,
    },
    // Кассир
    FieldSpec {
        tag: Some(1021),
        forms: Form::BOTH,
        req: Req::SometimesRequired,
        multi: false,
    },
    // ИНН кассира
    FieldSpec {
        tag: Some(1203),
        forms: Form::ELECTRONIC,
        req: Req::Optional,
        multi: false,
    },
    // Регистрационный номер ККТ
    FieldSpec {
        tag: Some(1037),
        forms: Form::BOTH,
        req: Req::Mandatory,
        multi: false,
    },
    // Тип коррекции
    FieldSpec {
        tag: Some(1173),
        forms: Form::BOTH,
        req: Req::Optional,
        multi: false,
    },
    // Основание для коррекции
    FieldSpec {
        tag: Some(1174),
        forms: Form::BOTH,
        req: Req::Mandatory,
        multi: false,
    },
    // Номер автомата
    FieldSpec {
        tag: Some(1036),
        forms: Form::BOTH,
        req: Req::Optional,
        multi: false,
    },
    // Адрес расчетов
    FieldSpec {
        tag: Some(1009),
        forms: Form::BOTH,
        req: Req::Optional,
        multi: false,
    },
    // Место расчетов
    FieldSpec {
        tag: Some(1187),
        forms: Form::BOTH,
        req: Req::Optional,
        multi: false,
    },
    // Телефон или электронный адрес покупателя
    FieldSpec {
        tag: Some(1008),
        forms: Form::BOTH,
        req: Req::Optional,
        multi: false,
    },
    // Предмет расчета
    FieldSpec {
        tag: Some(1059),
        forms: Form::BOTH,
        req: Req::Optional,
        multi: true,
    },
    // Сумма расчета, указанного в чеке (БСО)
    FieldSpec {
        tag: Some(1020),
        forms: Form::BOTH,
        req: Req::Mandatory,
        multi: false,
    },
    // Сумма по чеку (БСО) наличными
    FieldSpec {
        tag: Some(1031),
        forms: Form::BOTH,
        req: Req::Optional,
        multi: false,
    },
    // Сумма по чеку (БСО) безналичными
    FieldSpec {
        tag: Some(1081),
        forms: Form::BOTH,
        req: Req::Optional,
        multi: false,
    },
    // Сумма по чеку (БСО) предоплатой (зачетом аванса и (или) предыдущих платежей)
    FieldSpec {
        tag: Some(1215),
        forms: Form::BOTH,
        req: Req::Optional,
        multi: false,
    },
    // Сумма по чеку (БСО) постоплатой (в кредит)
    FieldSpec {
        tag: Some(1216),
        forms: Form::BOTH,
        req: Req::Optional,
        multi: false,
    },
    // Сумма по чеку (БСО) встречным предоставлением
    FieldSpec {
        tag: Some(1217),
        forms: Form::BOTH,
        req: Req::Optional,
        multi: false,
    },
    // Сумма НДС чека по ставке 20%
    FieldSpec {
        tag: Some(1102),
        forms: Form::BOTH,
        req: Req::SometimesRequired,
        multi: false,
    },
    // Сумма НДС чека по ставке 10%
    FieldSpec {
        tag: Some(1103),
        forms: Form::BOTH,
        req: Req::SometimesRequired,
        multi: false,
    },
    // Сумма расчета по чеку с НДС по ставке 0%
    FieldSpec {
        tag: Some(1104),
        forms: Form::BOTH,
        req: Req::SometimesRequired,
        multi: false,
    },
    // Сумма расчета по чеку без НДС
    FieldSpec {
        tag: Some(1105),
        forms: Form::BOTH,
        req: Req::SometimesRequired,
        multi: false,
    },
    // Сумма НДС чека по расч. ставке 20/120
    FieldSpec {
        tag: Some(1106),
        forms: Form::BOTH,
        req: Req::SometimesRequired,
        multi: false,
    },
    // Сумма НДС чека по расч. ставке 10/110
    FieldSpec {
        tag: Some(1107),
        forms: Form::BOTH,
        req: Req::SometimesRequired,
        multi: false,
    },
    // Признак ККТ для расчетов только в Интернет
    FieldSpec {
        tag: Some(1108),
        forms: Form::ELECTRONIC,
        req: Req::Optional,
        multi: false,
    },
    // Адрес электронной почты отправителя чека
    FieldSpec {
        tag: Some(1117),
        forms: Form::ELECTRONIC,
        req: Req::Optional,
        multi: false,
    },
    // Признак агента
    FieldSpec {
        tag: Some(1057),
        forms: Form::BOTH,
        req: Req::Optional,
        multi: false,
    },
    // Телефон оператора перевода
    FieldSpec {
        tag: Some(1075),
        forms: Form::BOTH,
        req: Req::Optional,
        multi: true,
    },
    // Операция банковского платежного агента
    FieldSpec {
        tag: Some(1044),
        forms: Form::BOTH,
        req: Req::Optional,
        multi: false,
    },
    // Телефон платежного агента
    FieldSpec {
        tag: Some(1073),
        forms: Form::BOTH,
        req: Req::Optional,
        multi: true,
    },
    // Телефон оператора по приему платежей
    FieldSpec {
        tag: Some(1074),
        forms: Form::BOTH,
        req: Req::Optional,
        multi: true,
    },
    // Наименование оператора перевода
    FieldSpec {
        tag: Some(1026),
        forms: Form::BOTH,
        req: Req::Optional,
        multi: false,
    },
    // Адрес оператора перевода
    FieldSpec {
        tag: Some(1005),
        forms: Form::BOTH,
        req: Req::Optional,
        multi: false,
    },
    // ИНН оператора перевода
    FieldSpec {
        tag: Some(1016),
        forms: Form::BOTH,
        req: Req::Optional,
        multi: false,
    },
    // Телефон поставщика
    FieldSpec {
        tag: Some(1171),
        forms: Form::BOTH,
        req: Req::Optional,
        multi: true,
    },
    // Адрес сайта ФНС
    FieldSpec {
        tag: Some(1060),
        forms: Form::BOTH,
        req: Req::Optional,
        multi: false,
    },
    // Дополнительный реквизит чека (БСО)
    FieldSpec {
        tag: Some(1192),
        forms: Form::BOTH,
        req: Req::Optional,
        multi: false,
    },
    // Дополнительный реквизит пользователя
    FieldSpec {
        tag: Some(1084),
        forms: Form::BOTH,
        req: Req::Optional,
        multi: false,
    },
    // Номер ФД
    FieldSpec {
        tag: Some(1040),
        forms: Form::BOTH,
        req: Req::Mandatory,
        multi: false,
    },
    // Номер ФН
    FieldSpec {
        tag: Some(1041),
        forms: Form::BOTH,
        req: Req::Mandatory,
        multi: false,
    },
    // ФПД (1)
    FieldSpec {
        tag: Some(1077),
        forms: Form::BOTH,
        req: Req::Mandatory,
        multi: false,
    },
    // ФПС (4)
    FieldSpec {
        tag: None,
        forms: Form::ELECTRONIC,
        req: Req::Mandatory,
        multi: false,
    },
    // QR-код
    FieldSpec {
        tag: Some(1196),
        forms: Form::PRINTED,
        req: Req::Optional,
        multi: false,
    },
]);

/// Таблица 29
///
/// Структура данных для реквизита «основание для коррекции» (тег 1174)
pub const CORRECTION_BASIS_1_05: FieldSet = FieldSet(&[
    // Дата совершения корректируемого расчета
    FieldSpec {
        tag: Some(1178),
        forms: Form::BOTH,
        req: Req::SometimesRequired,
        multi: false,
    },
    // Номер предписания налогового органа
    FieldSpec {
        tag: Some(1179),
        forms: Form::ELECTRONIC,
        req: Req::Mandatory,
        multi: false,
    },
]);

/// Таблица 30
///
/// Реквизиты, содержащиеся в отчете о закрытии смены
pub const SHIFT_END_REPORT_1_05: FieldSet = FieldSet(&[
    // Наименование документа
    FieldSpec {
        tag: Some(1000),
        forms: Form::PRINTED,
        req: Req::Mandatory,
        multi: false,
    },
    // Код формы ФД
    FieldSpec {
        tag: None,
        forms: Form::ELECTRONIC,
        req: Req::Mandatory,
        multi: false,
    },
    // Номер версии ФФД
    FieldSpec {
        tag: Some(1209),
        forms: Form::ELECTRONIC,
        req: Req::Mandatory,
        multi: false,
    },
    // Наименование пользователя
    FieldSpec {
        tag: Some(1048),
        forms: Form::PRINTED,
        req: Req::Mandatory,
        multi: false,
    },
    // ИНН пользователя
    FieldSpec {
        tag: Some(1018),
        forms: Form::BOTH,
        req: Req::Mandatory,
        multi: false,
    },
    // Кассир
    FieldSpec {
        tag: Some(1021),
        forms: Form::BOTH,
        req: Req::SometimesRequired,
        multi: false,
    },
    // ИНН кассира
    FieldSpec {
        tag: Some(1203),
        forms: Form::ELECTRONIC,
        req: Req::Optional,
        multi: false,
    },
    // Адрес расчетов
    FieldSpec {
        tag: Some(1009),
        forms: Form::PRINTED,
        req: Req::SometimesRequired,
        multi: false,
    },
    // Место расчетов
    FieldSpec {
        tag: Some(1187),
        forms: Form::PRINTED,
        req: Req::SometimesRequired,
        multi: false,
    },
    // Дата, время
    FieldSpec {
        tag: Some(1012),
        forms: Form::BOTH,
        req: Req::Mandatory,
        multi: false,
    },
    // Номер смены
    FieldSpec {
        tag: Some(1038),
        forms: Form::BOTH,
        req: Req::Mandatory,
        multi: false,
    },
    // Регистрационный номер ККТ
    FieldSpec {
        tag: Some(1037),
        forms: Form::BOTH,
        req: Req::Mandatory,
        multi: false,
    },
    // Количество кассовых чеков (БСО) за смену
    FieldSpec {
        tag: Some(1118),
        forms: Form::BOTH,
        req: Req::Mandatory,
        multi: false,
    },
    // Общее количество ФД за смену
    FieldSpec {
        tag: Some(1111),
        forms: Form::BOTH,
        req: Req::Mandatory,
        multi: false,
    },
    // Количество непереданных ФД
    FieldSpec {
        tag: Some(1097),
        forms: Form::BOTH,
        req: Req::SometimesRequired,
        multi: false,
    },
    // Дата первого из непереданных ФД
    FieldSpec {
        tag: Some(1098),
        forms: Form::BOTH,
        req: Req::SometimesRequired,
        multi: false,
    },
    // Признак превышения времени ожидания ответа ОФД
    FieldSpec {
        tag: Some(1053),
        forms: Form::BOTH,
        req: Req::SometimesRequired,
        multi: false,
    },
    // Признак необходимости срочной замены ФН
    FieldSpec {
        tag: Some(1051),
        forms: Form::BOTH,
        req: Req::SometimesRequired,
        multi: false,
    },
    // Признак переполнения памяти ФН
    FieldSpec {
        tag: Some(1052),
        forms: Form::BOTH,
        req: Req::SometimesRequired,
        multi: false,
    },
    // Признак исчерпания ресурса ФН
    FieldSpec {
        tag: Some(1050),
        forms: Form::BOTH,
        req: Req::SometimesRequired,
        multi: false,
    },
    // Ресурс ключей ФП
    FieldSpec {
        tag: Some(1213),
        forms: Form::BOTH,
        req: Req::Optional,
        multi: false,
    },
    // Номер ФД
    FieldSpec {
        tag: Some(1040),
        forms: Form::BOTH,
        req: Req::Mandatory,
        multi: false,
    },
    // Номер ФН
    FieldSpec {
        tag: Some(1041),
        forms: Form::BOTH,
        req: Req::Mandatory,
        multi: false,
    },
    // ФПД (1)
    FieldSpec {
        tag: Some(1077),
        forms: Form::BOTH,
        req: Req::Mandatory,
        multi: false,
    },
    // ФПС (4)
    FieldSpec {
        tag: None,
        forms: Form::ELECTRONIC,
        req: Req::Mandatory,
        multi: false,
    },
]);

/// Таблица 31
///
/// Реквизиты, содержащиеся в отчете о закрытии фискального накопителя
pub const FN_CLOSE_REPORT_1_05: FieldSet = FieldSet(&[
    // Наименование документа
    FieldSpec {
        tag: Some(1000),
        forms: Form::PRINTED,
        req: Req::Mandatory,
        multi: false,
    },
    // Код формы ФД
    FieldSpec {
        tag: None,
        forms: Form::ELECTRONIC,
        req: Req::Mandatory,
        multi: false,
    },
    // Номер версии ФФД
    FieldSpec {
        tag: Some(1209),
        forms: Form::ELECTRONIC,
        req: Req::Mandatory,
        multi: false,
    },
    // Наименование пользователя
    FieldSpec {
        tag: Some(1048),
        forms: Form::BOTH,
        req: Req::Mandatory,
        multi: false,
    },
    // ИНН пользователя
    FieldSpec {
        tag: Some(1018),
        forms: Form::BOTH,
        req: Req::Mandatory,
        multi: false,
    },
    // Кассир
    FieldSpec {
        tag: Some(1021),
        forms: Form::BOTH,
        req: Req::SometimesRequired,
        multi: false,
    },
    // ИНН кассира
    FieldSpec {
        tag: Some(1203),
        forms: Form::ELECTRONIC,
        req: Req::Optional,
        multi: false,
    },
    // Адрес расчетов
    FieldSpec {
        tag: Some(1009),
        forms: Form::BOTH,
        req: Req::SometimesRequired,
        multi: false,
    },
    // Место расчетов
    FieldSpec {
        tag: Some(1187),
        forms: Form::BOTH,
        req: Req::SometimesRequired,
        multi: false,
    },
    // Дата, время
    FieldSpec {
        tag: Some(1012),
        forms: Form::BOTH,
        req: Req::Mandatory,
        multi: false,
    },
    // Номер смены
    FieldSpec {
        tag: Some(1038),
        forms: Form::BOTH,
        req: Req::Mandatory,
        multi: false,
    },
    // Регистрационный номер ККТ
    FieldSpec {
        tag: Some(1037),
        forms: Form::BOTH,
        req: Req::Mandatory,
        multi: false,
    },
    // Номер ФД
    FieldSpec {
        tag: Some(1040),
        forms: Form::BOTH,
        req: Req::Mandatory,
        multi: false,
    },
    // Номер ФН
    FieldSpec {
        tag: Some(1041),
        forms: Form::BOTH,
        req: Req::Mandatory,
        multi: false,
    },
    // ФПД (1)
    FieldSpec {
        tag: Some(1077),
        forms: Form::BOTH,
        req: Req::Mandatory,
        multi: false,
    },
    // ФПС (4)
    FieldSpec {
        tag: None,
        forms: Form::ELECTRONIC,
        req: Req::Mandatory,
        multi: false,
    },
]);

/// Таблица 32
///
/// Реквизиты, содержащиеся в подтверждении оператора
pub const OPERATOR_CONFIRMATION_1_05: FieldSet = FieldSet(&[
    // Наименование документа
    FieldSpec {
        tag: Some(1000),
        forms: Form::PRINTED,
        req: Req::Mandatory,
        multi: false,
    },
    // Код формы ФД
    FieldSpec {
        tag: None,
        forms: Form::ELECTRONIC,
        req: Req::Mandatory,
        multi: false,
    },
    // ИНН ОФД
    FieldSpec {
        tag: Some(1017),
        forms: Form::BOTH,
        req: Req::Mandatory,
        multi: false,
    },
    // Номер ФН
    FieldSpec {
        tag: Some(1041),
        forms: Form::BOTH,
        req: Req::Mandatory,
        multi: false,
    },
    // Номер ФД
    FieldSpec {
        tag: Some(1040),
        forms: Form::BOTH,
        req: Req::Mandatory,
        multi: false,
    },
    // Дата, время
    FieldSpec {
        tag: Some(1012),
        forms: Form::BOTH,
        req: Req::Mandatory,
        multi: false,
    },
    // Сообщение оператора для ФН
    FieldSpec {
        tag: Some(1068),
        forms: Form::ELECTRONIC,
        req: Req::SometimesRequired,
        multi: false,
    },
    // ФПО (2)
    FieldSpec {
        tag: Some(1078),
        forms: Form::BOTH,
        req: Req::Mandatory,
        multi: false,
    },
    // ФПП (3)
    FieldSpec {
        tag: None,
        forms: Form::ELECTRONIC,
        req: Req::Mandatory,
        multi: false,
    },
]);

/// Таблица 33
///
/// Структура реквизита «сообщение оператора для ФН» (тег 1068)
pub const OPERATOR_MESSAGE_TO_FN: FieldSet = FieldSet(&[
    // Код ответа ОФД
    FieldSpec {
        tag: Some(1022),
        forms: Form::ELECTRONIC,
        req: Req::Mandatory,
        multi: false,
    },
    // Реквизит типа «Строка»
    FieldSpec {
        tag: None,
        forms: Form::NONE,
        req: Req::Optional,
        multi: false,
    },
]);

/// Таблица 44
///
/// Формат ФД «Кассовый чек (БСО)», передаваемый покупателю (клиенту) в электронной форме
pub const FD_RECEIPT_1_05: FieldSet = FieldSet(&[
    // Наименование документа
    FieldSpec {
        tag: Some(1000),
        forms: Form::NONE,
        req: Req::Mandatory,
        multi: false,
    },
    // Наименование пользователя
    FieldSpec {
        tag: Some(1048),
        forms: Form::NONE,
        req: Req::Mandatory,
        multi: false,
    },
    // ИНН пользователя
    FieldSpec {
        tag: Some(1018),
        forms: Form::NONE,
        req: Req::Mandatory,
        multi: false,
    },
    // Номер чека за смену
    FieldSpec {
        tag: Some(1042),
        forms: Form::NONE,
        req: Req::Mandatory,
        multi: false,
    },
    // Покупатель (клиент)
    FieldSpec {
        tag: Some(1227),
        forms: Form::NONE,
        req: Req::Optional,
        multi: false,
    },
    // ИНН покупателя (клиента)
    FieldSpec {
        tag: Some(1228),
        forms: Form::NONE,
        req: Req::Optional,
        multi: false,
    },
    // Дата, время
    FieldSpec {
        tag: Some(1012),
        forms: Form::NONE,
        req: Req::Mandatory,
        multi: false,
    },
    // Номер смены
    FieldSpec {
        tag: Some(1038),
        forms: Form::NONE,
        req: Req::Mandatory,
        multi: false,
    },
    // Признак расчета
    FieldSpec {
        tag: Some(1054),
        forms: Form::NONE,
        req: Req::Mandatory,
        multi: false,
    },
    // Применяемая система налогообложения
    FieldSpec {
        tag: Some(1055),
        forms: Form::NONE,
        req: Req::Optional,
        multi: false,
    },
    // Кассир
    FieldSpec {
        tag: Some(1021),
        forms: Form::NONE,
        req: Req::SometimesRequired,
        multi: false,
    },
    // Регистрационный номер ККТ
    FieldSpec {
        tag: Some(1037),
        forms: Form::NONE,
        req: Req::Mandatory,
        multi: false,
    },
    // Номер автомата
    FieldSpec {
        tag: Some(1036),
        forms: Form::NONE,
        req: Req::SometimesRequired,
        multi: false,
    },
    // Адрес расчетов
    FieldSpec {
        tag: Some(1009),
        forms: Form::NONE,
        req: Req::SometimesRequired,
        multi: false,
    },
    // Место расчетов
    FieldSpec {
        tag: Some(1187),
        forms: Form::NONE,
        req: Req::SometimesRequired,
        multi: false,
    },
    // Телефон или электронный адрес покупателя
    FieldSpec {
        tag: Some(1008),
        forms: Form::NONE,
        req: Req::Optional,
        multi: false,
    },
    // Предмет расчета
    FieldSpec {
        tag: Some(1059),
        forms: Form::NONE,
        req: Req::Optional,
        multi: true,
    },
    // Телефон оператора перевода
    FieldSpec {
        tag: Some(1075),
        forms: Form::NONE,
        req: Req::SometimesRequired,
        multi: true,
    },
    // Операция банковского платежного агента
    FieldSpec {
        tag: Some(1044),
        forms: Form::NONE,
        req: Req::SometimesRequired,
        multi: true,
    },
    // Телефон платежного агента
    FieldSpec {
        tag: Some(1073),
        forms: Form::NONE,
        req: Req::SometimesRequired,
        multi: true,
    },
    // Телефон оператора по приему платежей
    FieldSpec {
        tag: Some(1074),
        forms: Form::NONE,
        req: Req::SometimesRequired,
        multi: true,
    },
    // Наименование оператора перевода
    FieldSpec {
        tag: Some(1026),
        forms: Form::NONE,
        req: Req::SometimesRequired,
        multi: true,
    },
    // Адрес оператора перевода
    FieldSpec {
        tag: Some(1005),
        forms: Form::NONE,
        req: Req::SometimesRequired,
        multi: true,
    },
    // ИНН оператора перевода
    FieldSpec {
        tag: Some(1016),
        forms: Form::NONE,
        req: Req::SometimesRequired,
        multi: true,
    },
    // Телефон поставщика
    FieldSpec {
        tag: Some(1171),
        forms: Form::NONE,
        req: Req::SometimesRequired,
        multi: true,
    },
    // Сумма расчета, указанного в чеке (БСО)
    FieldSpec {
        tag: Some(1020),
        forms: Form::NONE,
        req: Req::Mandatory,
        multi: false,
    },
    // Сумма по чеку (БСО) наличными
    FieldSpec {
        tag: Some(1031),
        forms: Form::NONE,
        req: Req::SometimesRequired,
        multi: false,
    },
    // Сумма по чеку (БСО) безналичными
    FieldSpec {
        tag: Some(1081),
        forms: Form::NONE,
        req: Req::SometimesRequired,
        multi: false,
    },
    // Сумма по чеку (БСО) предоплатой (зачетом аванса и (или) предыдущих платежей)
    FieldSpec {
        tag: Some(1215),
        forms: Form::NONE,
        req: Req::SometimesRequired,
        multi: false,
    },
    // Сумма по чеку (БСО) постоплатой (в кредит)
    FieldSpec {
        tag: Some(1216),
        forms: Form::NONE,
        req: Req::SometimesRequired,
        multi: false,
    },
    // Сумма по чеку (БСО) встречным предоставлением
    FieldSpec {
        tag: Some(1217),
        forms: Form::NONE,
        req: Req::SometimesRequired,
        multi: false,
    },
    // Сумма НДС чека по ставке 20%
    FieldSpec {
        tag: Some(1102),
        forms: Form::NONE,
        req: Req::SometimesRequired,
        multi: false,
    },
    // Сумма НДС чека по ставке 10%
    FieldSpec {
        tag: Some(1103),
        forms: Form::NONE,
        req: Req::SometimesRequired,
        multi: false,
    },
    // Сумма расчета по чеку с НДС по ставке 0%
    FieldSpec {
        tag: Some(1104),
        forms: Form::NONE,
        req: Req::SometimesRequired,
        multi: false,
    },
    // Сумма расчета по чеку без НДС
    FieldSpec {
        tag: Some(1105),
        forms: Form::NONE,
        req: Req::SometimesRequired,
        multi: false,
    },
    // Сумма НДС чека по расч. ставке 20/120
    FieldSpec {
        tag: Some(1106),
        forms: Form::NONE,
        req: Req::SometimesRequired,
        multi: false,
    },
    // Сумма НДС чека по расч. ставке 10/110
    FieldSpec {
        tag: Some(1107),
        forms: Form::NONE,
        req: Req::SometimesRequired,
        multi: false,
    },
    // Адрес электронной почты отправителя чека
    FieldSpec {
        tag: Some(1117),
        forms: Form::NONE,
        req: Req::SometimesRequired,
        multi: false,
    },
    // Признак агента
    FieldSpec {
        tag: Some(1057),
        forms: Form::NONE,
        req: Req::SometimesRequired,
        multi: false,
    },
    // Адрес сайта ФНС
    FieldSpec {
        tag: Some(1060),
        forms: Form::NONE,
        req: Req::SometimesRequired,
        multi: false,
    },
    // Дополнительный реквизит чека (БСО)
    FieldSpec {
        tag: Some(1192),
        forms: Form::NONE,
        req: Req::Optional,
        multi: true,
    },
    // Дополнительный реквизит пользователя
    FieldSpec {
        tag: Some(1084),
        forms: Form::NONE,
        req: Req::Optional,
        multi: false,
    },
    // Номер ФД
    FieldSpec {
        tag: Some(1040),
        forms: Form::NONE,
        req: Req::Mandatory,
        multi: false,
    },
    // Номер ФН
    FieldSpec {
        tag: Some(1041),
        forms: Form::NONE,
        req: Req::Mandatory,
        multi: false,
    },
    // ФПД
    FieldSpec {
        tag: Some(1077),
        forms: Form::NONE,
        req: Req::Mandatory,
        multi: false,
    },
    // QR-код
    FieldSpec {
        tag: Some(1196),
        forms: Form::NONE,
        req: Req::Optional,
        multi: false,
    },
]);

/// Таблицы 45, 83, 148
///
/// Формат сведений в электронной форме, идентифицирующих кассовый чек (БСО), передаваемый покупателю (клиенту) в электронной форме
pub const RECEIPT_ID: FieldSet = FieldSet(&[
    // Регистрационный номер ККТ
    FieldSpec {
        tag: Some(1037),
        forms: Form::NONE,
        req: Req::Mandatory,
        multi: false,
    },
    // Сумма расчета, указанного в чеке (БСО)
    FieldSpec {
        tag: Some(1020),
        forms: Form::NONE,
        req: Req::Mandatory,
        multi: false,
    },
    // Дата, время
    FieldSpec {
        tag: Some(1012),
        forms: Form::NONE,
        req: Req::Mandatory,
        multi: false,
    },
    // ФПД
    FieldSpec {
        tag: Some(1077),
        forms: Form::NONE,
        req: Req::Mandatory,
        multi: false,
    },
    // Сайт чеков
    FieldSpec {
        tag: Some(1208),
        forms: Form::NONE,
        req: Req::SometimesRequired,
        multi: false,
    },
]);

/// Таблица 49
///
/// Реквизиты, содержащиеся в отчете о регистрации и отчете об изменении
pub const REGISTRATION_REPORT_1_1: FieldSet = FieldSet(&[
    // Наименование документа
    FieldSpec {
        tag: Some(1000),
        forms: Form::PRINTED,
        req: Req::Mandatory,
        multi: false,
    },
    // Код формы ФД
    FieldSpec {
        tag: None,
        forms: Form::ELECTRONIC,
        req: Req::Mandatory,
        multi: false,
    },
    // Номер версии ФФД
    FieldSpec {
        tag: Some(1209),
        forms: Form::ELECTRONIC,
        req: Req::Mandatory,
        multi: false,
    },
    // Наименование пользователя
    FieldSpec {
        tag: Some(1048),
        forms: Form::BOTH,
        req: Req::Mandatory,
        multi: false,
    },
    // ИНН пользователя
    FieldSpec {
        tag: Some(1018),
        forms: Form::BOTH,
        req: Req::Mandatory,
        multi: false,
    },
    // Системы налогообложения
    FieldSpec {
        tag: Some(1062),
        forms: Form::BOTH,
        req: Req::Optional,
        multi: false,
    },
    // Дата, время
    FieldSpec {
        tag: Some(1012),
        forms: Form::BOTH,
        req: Req::Mandatory,
        multi: false,
    },
    // Регистрационный номер ККТ
    FieldSpec {
        tag: Some(1037),
        forms: Form::BOTH,
        req: Req::Mandatory,
        multi: false,
    },
    // Признак автономного режима
    FieldSpec {
        tag: Some(1002),
        forms: Form::BOTH,
        req: Req::Mandatory,
        multi: false,
    },
    // Признак установки принтера в автомате
    FieldSpec {
        tag: Some(1221),
        forms: Form::BOTH,
        req: Req::Mandatory,
        multi: false,
    },
    // Признак АС БСО
    FieldSpec {
        tag: Some(1110),
        forms: Form::BOTH,
        req: Req::Mandatory,
        multi: false,
    },
    // Признак шифрования
    FieldSpec {
        tag: Some(1056),
        forms: Form::BOTH,
        req: Req::Mandatory,
        multi: false,
    },
    // Признак автоматического режима
    FieldSpec {
        tag: Some(1001),
        forms: Form::BOTH,
        req: Req::Mandatory,
        multi: false,
    },
    // Признак ККТ для расчетов только в Интернет
    FieldSpec {
        tag: Some(1108),
        forms: Form::BOTH,
        req: Req::Mandatory,
        multi: false,
    },
    // Номер автомата
    FieldSpec {
        tag: Some(1036),
        forms: Form::BOTH,
        req: Req::SometimesRequired,
        multi: false,
    },
    // Признак торговли подакцизными товарами
    FieldSpec {
        tag: Some(1207),
        forms: Form::BOTH,
        req: Req::Mandatory,
        multi: false,
    },
    // Признак расчетов за услуги
    FieldSpec {
        tag: Some(1109),
        forms: Form::BOTH,
        req: Req::Optional,
        multi: false,
    },
    // Признак проведения азартных игр
    FieldSpec {
        tag: Some(1193),
        forms: Form::BOTH,
        req: Req::Mandatory,
        multi: false,
    },
    // Признак проведения лотереи
    FieldSpec {
        tag: Some(1126),
        forms: Form::BOTH,
        req: Req::Mandatory,
        multi: false,
    },
    // Признак агента
    FieldSpec {
        tag: Some(1057),
        forms: Form::BOTH,
        req: Req::Optional,
        multi: false,
    },
    // Заводской номер ККТ
    FieldSpec {
        tag: Some(1013),
        forms: Form::BOTH,
        req: Req::Mandatory,
        multi: false,
    },
    // Кассир
    FieldSpec {
        tag: Some(1021),
        forms: Form::BOTH,
        req: Req::Mandatory,
        multi: false,
    },
    // ИНН кассира
    FieldSpec {
        tag: Some(1203),
        forms: Form::ELECTRONIC,
        req: Req::Optional,
        multi: false,
    },
    // Адрес расчетов
    FieldSpec {
        tag: Some(1009),
        forms: Form::BOTH,
        req: Req::Mandatory,
        multi: false,
    },
    // Место расчетов
    FieldSpec {
        tag: Some(1187),
        forms: Form::BOTH,
        req: Req::Mandatory,
        multi: false,
    },
    // Адрес сайта ФНС
    FieldSpec {
        tag: Some(1060),
        forms: Form::BOTH,
        req: Req::SometimesRequired,
        multi: false,
    },
    // Адрес электронной почты отправителя чека
    FieldSpec {
        tag: Some(1117),
        forms: Form::BOTH,
        req: Req::SometimesRequired,
        multi: false,
    },
    // ИНН ОФД
    FieldSpec {
        tag: Some(1017),
        forms: Form::BOTH,
        req: Req::SometimesRequired,
        multi: false,
    },
    // Наименование ОФД
    FieldSpec {
        tag: Some(1046),
        forms: Form::BOTH,
        req: Req::SometimesRequired,
        multi: false,
    },
    // Версия ККТ
    FieldSpec {
        tag: Some(1188),
        forms: Form::BOTH,
        req: Req::Mandatory,
        multi: false,
    },
    // Версия ФФД ККТ
    FieldSpec {
        tag: Some(1189),
        forms: Form::BOTH,
        req: Req::Mandatory,
        multi: false,
    },
    // Версия ФФД ФН
    FieldSpec {
        tag: Some(1190),
        forms: Form::BOTH,
        req: Req::Mandatory,
        multi: false,
    },
    // Ресурс ключей ФП
    FieldSpec {
        tag: Some(1213),
        forms: Form::BOTH,
        req: Req::Mandatory,
        multi: false,
    },
    // Номер ФД
    FieldSpec {
        tag: Some(1040),
        forms: Form::BOTH,
        req: Req::Mandatory,
        multi: false,
    },
    // Номер ФН
    FieldSpec {
        tag: Some(1041),
        forms: Form::BOTH,
        req: Req::Mandatory,
        multi: false,
    },
    // ФПД (1)
    FieldSpec {
        tag: Some(1077),
        forms: Form::BOTH,
        req: Req::Mandatory,
        multi: false,
    },
    // ФПС (4)
    FieldSpec {
        tag: None,
        forms: Form::ELECTRONIC,
        req: Req::Mandatory,
        multi: false,
    },
]);

/// Таблица 50
///
/// Реквизиты, содержащиеся в отчете об изменении параметров регистрации
pub const REGISTRATION_PARAMETER_UPDATE_REPORT_1_1: FieldSet = FieldSet(&[
    // Коды причин изменения сведений о ККТ
    FieldSpec {
        tag: Some(1205),
        forms: Form::BOTH,
        req: Req::Optional,
        multi: false,
    },
    // Счетчики итогов ФН
    FieldSpec {
        tag: Some(1157),
        forms: Form::BOTH,
        req: Req::SometimesRequired,
        multi: false,
    },
]);

/// Таблицы 51, 89
///
/// Реквизиты, содержащиеся в структуре реквизитов «счетчики итогов ФН» (тег 1157), «счетчики итогов смены» (тег 1194)
pub const TOTAL_COUNTERS: FieldSet = FieldSet(&[
    // Количество чеков (БСО) и чеков коррекции (БСО) коррекции со всеми признаками расчетов
    FieldSpec {
        tag: Some(1134),
        forms: Form::NONE,
        req: Req::Optional,
        multi: false,
    },
    // Счетчики операций «приход»
    FieldSpec {
        tag: Some(1129),
        forms: Form::NONE,
        req: Req::Optional,
        multi: false,
    },
    // Счетчики операций «возврат прихода»
    FieldSpec {
        tag: Some(1130),
        forms: Form::NONE,
        req: Req::Optional,
        multi: false,
    },
    // Счетчики операций «расход»
    FieldSpec {
        tag: Some(1131),
        forms: Form::NONE,
        req: Req::Optional,
        multi: false,
    },
    // Счетчики операций «возврат расхода»
    FieldSpec {
        tag: Some(1132),
        forms: Form::NONE,
        req: Req::Optional,
        multi: false,
    },
    // Счетчики операций по чекам коррекции
    FieldSpec {
        tag: Some(1133),
        forms: Form::NONE,
        req: Req::Optional,
        multi: false,
    },
]);

/// Таблицы 52, 90
///
/// Реквизиты, содержащиеся в структуре реквизитов «счетчики операций «приход» (тег 1129), «счетчики операций «расход» (тег 1131), «счетчики операций «возврат прихода» (тег 1130), «счетчики операций «возврат расхода» (тег 1132)
pub const OPERATION_COUNTERS: FieldSet = FieldSet(&[
    // Количество чеков (БСО) по признаку расчетов
    FieldSpec {
        tag: Some(1135),
        forms: Form::NONE,
        req: Req::Optional,
        multi: false,
    },
    // Итоговая сумма в чеках (БСО) наличными денежными средствами
    FieldSpec {
        tag: Some(1136),
        forms: Form::NONE,
        req: Req::Optional,
        multi: false,
    },
    // Итоговая сумма в чеках (БСО) безналичными
    FieldSpec {
        tag: Some(1138),
        forms: Form::NONE,
        req: Req::Optional,
        multi: false,
    },
    // Итоговая сумма в чеках (БСО) предоплатами (авансами)
    FieldSpec {
        tag: Some(1218),
        forms: Form::NONE,
        req: Req::Optional,
        multi: false,
    },
    // Итоговая сумма в чеках (БСО) постоплатами (кредитами)
    FieldSpec {
        tag: Some(1219),
        forms: Form::NONE,
        req: Req::Optional,
        multi: false,
    },
    // Итоговая сумма в чеках (БСО) встречными предоставлениями
    FieldSpec {
        tag: Some(1220),
        forms: Form::NONE,
        req: Req::Optional,
        multi: false,
    },
    // Общая итоговая сумма в чеках (БСО)
    FieldSpec {
        tag: Some(1201),
        forms: Form::NONE,
        req: Req::Optional,
        multi: false,
    },
    // Сумма НДС по ставке 20%
    FieldSpec {
        tag: Some(1139),
        forms: Form::NONE,
        req: Req::Optional,
        multi: false,
    },
    // Сумма НДС по ставке 10%
    FieldSpec {
        tag: Some(1140),
        forms: Form::NONE,
        req: Req::Optional,
        multi: false,
    },
    // Сумма НДС по расч. ставке 20/120
    FieldSpec {
        tag: Some(1141),
        forms: Form::NONE,
        req: Req::Optional,
        multi: false,
    },
    // Сумма НДС по расч. ставке 10/110
    FieldSpec {
        tag: Some(1142),
        forms: Form::NONE,
        req: Req::Optional,
        multi: false,
    },
    // Сумма расчетов с НДС по ставке 0%
    FieldSpec {
        tag: Some(1143),
        forms: Form::NONE,
        req: Req::Optional,
        multi: false,
    },
    // Сумма расчетов без НДС
    FieldSpec {
        tag: Some(1183),
        forms: Form::NONE,
        req: Req::Optional,
        multi: false,
    },
]);

/// Таблицы 53, 91
///
/// Реквизиты, содержащиеся в структуре реквизитов «счетчики итогов непереданных ФД» (тег 1158) и «счетчики операций по чекам коррекции» (тег 1133)
pub const UNTRANSMITTED_OR_CORRECTION_COUNTERS: FieldSet = FieldSet(&[
    // Количество чеков коррекции (БСО коррекции) или непереданных чеков (БСО) и чеков коррекции (БСО коррекции)
    FieldSpec {
        tag: Some(1144),
        forms: Form::NONE,
        req: Req::Optional,
        multi: false,
    },
    // Счетчики по признаку «приход»
    FieldSpec {
        tag: Some(1145),
        forms: Form::NONE,
        req: Req::Optional,
        multi: false,
    },
    // Счетчики по признаку «расход»
    FieldSpec {
        tag: Some(1146),
        forms: Form::NONE,
        req: Req::Optional,
        multi: false,
    },
    // Счетчики по признаку «возврат прихода»
    FieldSpec {
        tag: Some(1232),
        forms: Form::NONE,
        req: Req::Optional,
        multi: false,
    },
    // Счетчики по признаку «возврат расхода»
    FieldSpec {
        tag: Some(1233),
        forms: Form::NONE,
        req: Req::Optional,
        multi: false,
    },
]);

/// Таблицы 54, 92
///
/// Реквизиты, содержащиеся в структуре реквизитов «счетчики по признаку «приход» (тег 1145), «счетчики по признаку «расход» (тег 1146), «счетчики по признаку «возврат прихода» (тег 1232) и «счетчики по признаку «возврат расхода» (тег 1233)
pub const PAYMENT_TYPE_SPECIFIC_COUNTERS: FieldSet = FieldSet(&[
    // Итоговая сумма в чеках (БСО) наличными денежными средствами
    FieldSpec {
        tag: Some(1136),
        forms: Form::NONE,
        req: Req::Optional,
        multi: false,
    },
    // Итоговая сумма в чеках (БСО) безналичными
    FieldSpec {
        tag: Some(1138),
        forms: Form::NONE,
        req: Req::Optional,
        multi: false,
    },
    // Итоговая сумма в чеках (БСО) предоплатами (авансами)
    FieldSpec {
        tag: Some(1218),
        forms: Form::NONE,
        req: Req::Optional,
        multi: false,
    },
    // Итоговая сумма в чеках (БСО) постоплатами (кредитами)
    FieldSpec {
        tag: Some(1219),
        forms: Form::NONE,
        req: Req::Optional,
        multi: false,
    },
    // Итоговая сумма в чеках (БСО) встречными предоставлениями
    FieldSpec {
        tag: Some(1220),
        forms: Form::NONE,
        req: Req::Optional,
        multi: false,
    },
    // Общая итоговая сумма в чеках (БСО)
    FieldSpec {
        tag: Some(1201),
        forms: Form::NONE,
        req: Req::Optional,
        multi: false,
    },
    // Количество чеков (БСО) по признаку расчетов
    FieldSpec {
        tag: Some(1135),
        forms: Form::NONE,
        req: Req::Optional,
        multi: false,
    },
]);

/// Таблица 56
///
/// Реквизиты, содержащиеся в отчете об открытии смены
pub const SHIFT_START_REPORT_1_1: FieldSet = FieldSet(&[
    // Наименование документа
    FieldSpec {
        tag: Some(1000),
        forms: Form::PRINTED,
        req: Req::Mandatory,
        multi: false,
    },
    // Код формы ФД
    FieldSpec {
        tag: None,
        forms: Form::ELECTRONIC,
        req: Req::Mandatory,
        multi: false,
    },
    // Номер версии ФФД
    FieldSpec {
        tag: Some(1209),
        forms: Form::ELECTRONIC,
        req: Req::Mandatory,
        multi: false,
    },
    // Наименование пользователя
    FieldSpec {
        tag: Some(1048),
        forms: Form::PRINTED,
        req: Req::Mandatory,
        multi: false,
    },
    // ИНН пользователя
    FieldSpec {
        tag: Some(1018),
        forms: Form::BOTH,
        req: Req::Mandatory,
        multi: false,
    },
    // Кассир
    FieldSpec {
        tag: Some(1021),
        forms: Form::BOTH,
        req: Req::SometimesRequired,
        multi: false,
    },
    // ИНН кассира
    FieldSpec {
        tag: Some(1203),
        forms: Form::ELECTRONIC,
        req: Req::Optional,
        multi: false,
    },
    // Адрес расчетов
    FieldSpec {
        tag: Some(1009),
        forms: Form::PRINTED,
        req: Req::Mandatory,
        multi: false,
    },
    // Место расчетов
    FieldSpec {
        tag: Some(1187),
        forms: Form::PRINTED,
        req: Req::Mandatory,
        multi: false,
    },
    // Дата, время
    FieldSpec {
        tag: Some(1012),
        forms: Form::BOTH,
        req: Req::Mandatory,
        multi: false,
    },
    // Номер смены
    FieldSpec {
        tag: Some(1038),
        forms: Form::BOTH,
        req: Req::Mandatory,
        multi: false,
    },
    // Регистрационный номер ККТ
    FieldSpec {
        tag: Some(1037),
        forms: Form::BOTH,
        req: Req::Mandatory,
        multi: false,
    },
    // Признак превышения времени ожидания ответа ОФД
    FieldSpec {
        tag: Some(1053),
        forms: Form::BOTH,
        req: Req::SometimesRequired,
        multi: false,
    },
    // Признак необходимости срочной замены ФН
    FieldSpec {
        tag: Some(1051),
        forms: Form::BOTH,
        req: Req::SometimesRequired,
        multi: false,
    },
    // Признак переполнения памяти ФН
    FieldSpec {
        tag: Some(1052),
        forms: Form::BOTH,
        req: Req::SometimesRequired,
        multi: false,
    },
    // Признак исчерпания ресурса ФН
    FieldSpec {
        tag: Some(1050),
        forms: Form::BOTH,
        req: Req::SometimesRequired,
        multi: false,
    },
    // Сообщение оператора
    FieldSpec {
        tag: Some(1206),
        forms: Form::BOTH,
        req: Req::SometimesRequired,
        multi: false,
    },
    // Версия ККТ
    FieldSpec {
        tag: Some(1188),
        forms: Form::BOTH,
        req: Req::Mandatory,
        multi: false,
    },
    // Версия ФФД ККТ
    FieldSpec {
        tag: Some(1189),
        forms: Form::BOTH,
        req: Req::Mandatory,
        multi: false,
    },
    // Номер ФД
    FieldSpec {
        tag: Some(1040),
        forms: Form::BOTH,
        req: Req::Mandatory,
        multi: false,
    },
    // Номер ФН
    FieldSpec {
        tag: Some(1041),
        forms: Form::BOTH,
        req: Req::Mandatory,
        multi: false,
    },
    // ФПД (1)
    FieldSpec {
        tag: Some(1077),
        forms: Form::BOTH,
        req: Req::Mandatory,
        multi: false,
    },
    // ФПС (4)
    FieldSpec {
        tag: None,
        forms: Form::ELECTRONIC,
        req: Req::Mandatory,
        multi: false,
    },
]);

/// Таблица 57
///
/// Реквизиты, содержащиеся в отчете о текущем состоянии расчетов
pub const PAYMENT_STATE_REPORT_1_1: FieldSet = FieldSet(&[
    // Наименование документа
    FieldSpec {
        tag: Some(1000),
        forms: Form::PRINTED,
        req: Req::Mandatory,
        multi: false,
    },
    // Код формы ФД
    FieldSpec {
        tag: None,
        forms: Form::ELECTRONIC,
        req: Req::Mandatory,
        multi: false,
    },
    // Номер версии ФФД
    FieldSpec {
        tag: Some(1209),
        forms: Form::ELECTRONIC,
        req: Req::Mandatory,
        multi: false,
    },
    // Наименование пользователя
    FieldSpec {
        tag: Some(1048),
        forms: Form::PRINTED,
        req: Req::Mandatory,
        multi: false,
    },
    // ИНН пользователя
    FieldSpec {
        tag: Some(1018),
        forms: Form::BOTH,
        req: Req::Mandatory,
        multi: false,
    },
    // Адрес расчетов
    FieldSpec {
        tag: Some(1009),
        forms: Form::PRINTED,
        req: Req::Mandatory,
        multi: false,
    },
    // Место расчетов
    FieldSpec {
        tag: Some(1187),
        forms: Form::PRINTED,
        req: Req::Mandatory,
        multi: false,
    },
    // Дата, время
    FieldSpec {
        tag: Some(1012),
        forms: Form::BOTH,
        req: Req::Mandatory,
        multi: false,
    },
    // Номер смены
    FieldSpec {
        tag: Some(1038),
        forms: Form::BOTH,
        req: Req::SometimesRequired,
        multi: false,
    },
    // Признак автономного режима
    FieldSpec {
        tag: Some(1002),
        forms: Form::BOTH,
        req: Req::SometimesRequired,
        multi: false,
    },
    // Номер первого непереданного документа
    FieldSpec {
        tag: Some(1116),
        forms: Form::BOTH,
        req: Req::SometimesRequired,
        multi: false,
    },
    // Количество непереданных ФД
    FieldSpec {
        tag: Some(1097),
        forms: Form::BOTH,
        req: Req::SometimesRequired,
        multi: false,
    },
    // Дата первого из непереданных ФД
    FieldSpec {
        tag: Some(1098),
        forms: Form::BOTH,
        req: Req::SometimesRequired,
        multi: false,
    },
    // Регистрационный номер ККТ
    FieldSpec {
        tag: Some(1037),
        forms: Form::BOTH,
        req: Req::Mandatory,
        multi: false,
    },
    // Счетчики итогов ФН
    FieldSpec {
        tag: Some(1157),
        forms: Form::BOTH,
        req: Req::Mandatory,
        multi: false,
    },
    // Счетчики итогов непереданных ФД
    FieldSpec {
        tag: Some(1158),
        forms: Form::BOTH,
        req: Req::SometimesRequired,
        multi: false,
    },
    // Номер ФД
    FieldSpec {
        tag: Some(1040),
        forms: Form::BOTH,
        req: Req::Mandatory,
        multi: false,
    },
    // Номер ФН
    FieldSpec {
        tag: Some(1041),
        forms: Form::BOTH,
        req: Req::Mandatory,
        multi: false,
    },
    // ФПД (1)
    FieldSpec {
        tag: Some(1077),
        forms: Form::BOTH,
        req: Req::Mandatory,
        multi: false,
    },
    // ФПС (4)
    FieldSpec {
        tag: None,
        forms: Form::ELECTRONIC,
        req: Req::Mandatory,
        multi: false,
    },
]);

/// Таблица 58
///
/// Реквизиты, содержащиеся в кассовом чеке (БСО)
pub const RECEIPT_1_1: FieldSet = FieldSet(&[
    // Наименование документа
    FieldSpec {
        tag: Some(1000),
        forms: Form::PRINTED,
        req: Req::Mandatory,
        multi: false,
    },
    // Код формы ФД
    FieldSpec {
        tag: None,
        forms: Form::ELECTRONIC,
        req: Req::Mandatory,
        multi: false,
    },
    // Номер версии ФФД
    FieldSpec {
        tag: Some(1209),
        forms: Form::ELECTRONIC,
        req: Req::Mandatory,
        multi: false,
    },
    // Наименование пользователя
    FieldSpec {
        tag: Some(1048),
        forms: Form::BOTH,
        req: Req::Optional,
        multi: false,
    },
    // ИНН пользователя
    FieldSpec {
        tag: Some(1018),
        forms: Form::BOTH,
        req: Req::Optional,
        multi: false,
    },
    // Покупатель (клиент)
    FieldSpec {
        tag: Some(1227),
        forms: Form::BOTH,
        req: Req::Optional,
        multi: false,
    },
    // ИНН покупателя (клиента)
    FieldSpec {
        tag: Some(1228),
        forms: Form::BOTH,
        req: Req::SometimesRequired,
        multi: false,
    },
    // Номер чека за смену
    FieldSpec {
        tag: Some(1042),
        forms: Form::ELECTRONIC,
        req: Req::Mandatory,
        multi: false,
    },
    // Дата, время
    FieldSpec {
        tag: Some(1012),
        forms: Form::BOTH,
        req: Req::Mandatory,
        multi: false,
    },
    // Номер смены
    FieldSpec {
        tag: Some(1038),
        forms: Form::ELECTRONIC,
        req: Req::Mandatory,
        multi: false,
    },
    // Признак расчета
    FieldSpec {
        tag: Some(1054),
        forms: Form::BOTH,
        req: Req::Mandatory,
        multi: false,
    },
    // Применяемая система налогообложения
    FieldSpec {
        tag: Some(1055),
        forms: Form::BOTH,
        req: Req::Optional,
        multi: false,
    },
    // Кассир
    FieldSpec {
        tag: Some(1021),
        forms: Form::BOTH,
        req: Req::SometimesRequired,
        multi: false,
    },
    // ИНН кассира
    FieldSpec {
        tag: Some(1203),
        forms: Form::ELECTRONIC,
        req: Req::Optional,
        multi: false,
    },
    // Регистрационный номер ККТ
    FieldSpec {
        tag: Some(1037),
        forms: Form::BOTH,
        req: Req::Mandatory,
        multi: false,
    },
    // Номер автомата
    FieldSpec {
        tag: Some(1036),
        forms: Form::BOTH,
        req: Req::SometimesRequired,
        multi: false,
    },
    // Адрес расчетов
    FieldSpec {
        tag: Some(1009),
        forms: Form::BOTH,
        req: Req::Optional,
        multi: false,
    },
    // Место расчетов
    FieldSpec {
        tag: Some(1187),
        forms: Form::BOTH,
        req: Req::Optional,
        multi: false,
    },
    // Телефон или электронный адрес покупателя
    FieldSpec {
        tag: Some(1008),
        forms: Form::BOTH,
        req: Req::Optional,
        multi: false,
    },
    // Предмет расчета
    FieldSpec {
        tag: Some(1059),
        forms: Form::BOTH,
        req: Req::Mandatory,
        multi: true,
    },
    // Сумма расчета, указанного в чеке (БСО)
    FieldSpec {
        tag: Some(1020),
        forms: Form::BOTH,
        req: Req::Mandatory,
        multi: false,
    },
    // Сумма по чеку (БСО) наличными
    FieldSpec {
        tag: Some(1031),
        forms: Form::BOTH,
        req: Req::Optional,
        multi: false,
    },
    // Сумма по чеку (БСО) безналичными
    FieldSpec {
        tag: Some(1081),
        forms: Form::BOTH,
        req: Req::Optional,
        multi: false,
    },
    // Сумма по чеку (БСО) предоплатой (зачетом аванса и (или) предыдущих платежей)
    FieldSpec {
        tag: Some(1215),
        forms: Form::BOTH,
        req: Req::Optional,
        multi: false,
    },
    // Сумма по чеку (БСО) постоплатой (в кредит)
    FieldSpec {
        tag: Some(1216),
        forms: Form::BOTH,
        req: Req::Optional,
        multi: false,
    },
    // Сумма по чеку (БСО) встречным предоставлением
    FieldSpec {
        tag: Some(1217),
        forms: Form::BOTH,
        req: Req::Optional,
        multi: false,
    },
    // Сумма НДС чека по ставке 20%
    FieldSpec {
        tag: Some(1102),
        forms: Form::BOTH,
        req: Req::SometimesRequired,
        multi: false,
    },
    // Сумма НДС чека по ставке 10%
    FieldSpec {
        tag: Some(1103),
        forms: Form::BOTH,
        req: Req::SometimesRequired,
        multi: false,
    },
    // Сумма расчета по чеку с НДС по ставке 0%
    FieldSpec {
        tag: Some(1104),
        forms: Form::BOTH,
        req: Req::SometimesRequired,
        multi: false,
    },
    // Сумма расчета по чеку без НДС
    FieldSpec {
        tag: Some(1105),
        forms: Form::BOTH,
        req: Req::SometimesRequired,
        multi: false,
    },
    // Сумма НДС чека по расч. ставке 20/120
    FieldSpec {
        tag: Some(1106),
        forms: Form::BOTH,
        req: Req::SometimesRequired,
        multi: false,
    },
    // Сумма НДС чека по расч. ставке 10/110
    FieldSpec {
        tag: Some(1107),
        forms: Form::BOTH,
        req: Req::SometimesRequired,
        multi: false,
    },
    // Признак ККТ для расчетов только в Интернет
    FieldSpec {
        tag: Some(1108),
        forms: Form::ELECTRONIC,
        req: Req::SometimesRequired,
        multi: false,
    },
    // Адрес электронной почты отправителя чека
    FieldSpec {
        tag: Some(1117),
        forms: Form::ELECTRONIC,
        req: Req::SometimesRequired,
        multi: false,
    },
    // Признак агента
    FieldSpec {
        tag: Some(1057),
        forms: Form::BOTH,
        req: Req::Optional,
        multi: false,
    },
    // Телефон оператора перевода
    FieldSpec {
        tag: Some(1075),
        forms: Form::BOTH,
        req: Req::Optional,
        multi: true,
    },
    // Операция банковского платежного агента
    FieldSpec {
        tag: Some(1044),
        forms: Form::BOTH,
        req: Req::SometimesRequired,
        multi: false,
    },
    // Телефон платежного агента
    FieldSpec {
        tag: Some(1073),
        forms: Form::BOTH,
        req: Req::Optional,
        multi: true,
    },
    // Телефон оператора по приему платежей
    FieldSpec {
        tag: Some(1074),
        forms: Form::BOTH,
        req: Req::Optional,
        multi: true,
    },
    // Наименование оператора перевода
    FieldSpec {
        tag: Some(1026),
        forms: Form::BOTH,
        req: Req::SometimesRequired,
        multi: false,
    },
    // Адрес оператора перевода
    FieldSpec {
        tag: Some(1005),
        forms: Form::BOTH,
        req: Req::SometimesRequired,
        multi: false,
    },
    // ИНН оператора перевода
    FieldSpec {
        tag: Some(1016),
        forms: Form::BOTH,
        req: Req::SometimesRequired,
        multi: false,
    },
    // Телефон поставщика
    FieldSpec {
        tag: Some(1171),
        forms: Form::BOTH,
        req: Req::Optional,
        multi: true,
    },
    // Адрес сайта ФНС
    FieldSpec {
        tag: Some(1060),
        forms: Form::BOTH,
        req: Req::Optional,
        multi: false,
    },
    // Дополнительный реквизит чека (БСО)
    FieldSpec {
        tag: Some(1192),
        forms: Form::BOTH,
        req: Req::Optional,
        multi: false,
    },
    // Дополнительный реквизит пользователя
    FieldSpec {
        tag: Some(1084),
        forms: Form::BOTH,
        req: Req::Optional,
        multi: false,
    },
    // Номер ФД
    FieldSpec {
        tag: Some(1040),
        forms: Form::BOTH,
        req: Req::Mandatory,
        multi: false,
    },
    // Номер ФН
    FieldSpec {
        tag: Some(1041),
        forms: Form::BOTH,
        req: Req::Mandatory,
        multi: false,
    },
    // ФПД (1)
    FieldSpec {
        tag: Some(1077),
        forms: Form::BOTH,
        req: Req::Mandatory,
        multi: false,
    },
    // ФПС (4)
    FieldSpec {
        tag: None,
        forms: Form::ELECTRONIC,
        req: Req::Mandatory,
        multi: false,
    },
    // ФПА (5)
    FieldSpec {
        tag: None,
        forms: Form::ELECTRONIC,
        req: Req::SometimesRequired,
        multi: false,
    },
    // QR-код
    FieldSpec {
        tag: Some(1196),
        forms: Form::PRINTED,
        req: Req::Mandatory,
        multi: false,
    },
]);

/// Таблица 59
///
/// Структура данных реквизита «предмет расчета» (тег 1059)
pub const PAYMENT_ITEM_1_1: FieldSet = FieldSet(&[
    // Признак способа расчета
    FieldSpec {
        tag: Some(1214),
        forms: Form::BOTH,
        req: Req::Optional,
        multi: false,
    },
    // Признак предмета расчета
    FieldSpec {
        tag: Some(1212),
        forms: Form::BOTH,
        req: Req::Optional,
        multi: false,
    },
    // Признак агента по предмету расчета
    FieldSpec {
        tag: Some(1222),
        forms: Form::BOTH,
        req: Req::Optional,
        multi: false,
    },
    // Данные агента
    FieldSpec {
        tag: Some(1223),
        forms: Form::BOTH,
        req: Req::Optional,
        multi: false,
    },
    // Данные поставщика
    FieldSpec {
        tag: Some(1224),
        forms: Form::BOTH,
        req: Req::Optional,
        multi: false,
    },
    // ИНН поставщика
    FieldSpec {
        tag: Some(1226),
        forms: Form::BOTH,
        req: Req::Optional,
        multi: false,
    },
    // Наименование предмета расчета
    FieldSpec {
        tag: Some(1030),
        forms: Form::BOTH,
        req: Req::Mandatory,
        multi: false,
    },
    // Единица измерения предмета расчета
    FieldSpec {
        tag: Some(1197),
        forms: Form::BOTH,
        req: Req::Optional,
        multi: false,
    },
    // Код товара
    FieldSpec {
        tag: Some(1162),
        forms: Form::BOTH,
        req: Req::SometimesRequired,
        multi: false,
    },
    // Код страны происхождения товара
    FieldSpec {
        tag: Some(1230),
        forms: Form::BOTH,
        req: Req::Optional,
        multi: false,
    },
    // Номер декларации на товар
    FieldSpec {
        tag: Some(1231),
        forms: Form::BOTH,
        req: Req::Optional,
        multi: false,
    },
    // Цена за единицу предмета расчета с учетом скидок и наценок
    FieldSpec {
        tag: Some(1079),
        forms: Form::BOTH,
        req: Req::Mandatory,
        multi: false,
    },
    // Размер НДС за единицу предмета расчета
    FieldSpec {
        tag: Some(1198),
        forms: Form::BOTH,
        req: Req::Optional,
        multi: false,
    },
    // Количество предмета расчета
    FieldSpec {
        tag: Some(1023),
        forms: Form::BOTH,
        req: Req::Mandatory,
        multi: false,
    },
    // Акциз
    FieldSpec {
        tag: Some(1229),
        forms: Form::BOTH,
        req: Req::Optional,
        multi: false,
    },
    // Ставка НДС
    FieldSpec {
        tag: Some(1199),
        forms: Form::BOTH,
        req: Req::SometimesRequired,
        multi: false,
    },
    // Сумма НДС за предмет расчета
    FieldSpec {
        tag: Some(1200),
        forms: Form::BOTH,
        req: Req::Optional,
        multi: false,
    },
    // Стоимость предмета расчета с учетом скидок и наценок
    FieldSpec {
        tag: Some(1043),
        forms: Form::BOTH,
        req: Req::Mandatory,
        multi: false,
    },
    // Дополнительный реквизит предмета расчета
    FieldSpec {
        tag: Some(1191),
        forms: Form::BOTH,
        req: Req::Optional,
        multi: false,
    },
]);

/// Таблица 60
///
/// Значения реквизита «данные агента» (тег 1223)
pub const AGENT_INFO_1_1: FieldSet = FieldSet(&[
    // Телефон оператора перевода
    FieldSpec {
        tag: Some(1075),
        forms: Form::BOTH,
        req: Req::SometimesRequired,
        multi: true,
    },
    // Операция банковского платежного агента
    FieldSpec {
        tag: Some(1044),
        forms: Form::BOTH,
        req: Req::SometimesRequired,
        multi: false,
    },
    // Телефон платежного агента
    FieldSpec {
        tag: Some(1073),
        forms: Form::BOTH,
        req: Req::SometimesRequired,
        multi: true,
    },
    // Телефон оператора по приему платежей
    FieldSpec {
        tag: Some(1074),
        forms: Form::BOTH,
        req: Req::SometimesRequired,
        multi: true,
    },
    // Наименование оператора перевода
    FieldSpec {
        tag: Some(1026),
        forms: Form::BOTH,
        req: Req::SometimesRequired,
        multi: false,
    },
    // Адрес оператора перевода
    FieldSpec {
        tag: Some(1005),
        forms: Form::BOTH,
        req: Req::SometimesRequired,
        multi: false,
    },
    // ИНН оператора перевода
    FieldSpec {
        tag: Some(1016),
        forms: Form::BOTH,
        req: Req::SometimesRequired,
        multi: false,
    },
]);

/// Таблица 61
///
/// Значения реквизита «данные поставщика» (тег 1224)
pub const SUPPLIER_INFO_1_1: FieldSet = FieldSet(&[
    // Телефон поставщика
    FieldSpec {
        tag: Some(1171),
        forms: Form::BOTH,
        req: Req::SometimesRequired,
        multi: true,
    },
    // Наименование поставщика
    FieldSpec {
        tag: Some(1225),
        forms: Form::BOTH,
        req: Req::SometimesRequired,
        multi: false,
    },
]);

/// Таблица 65
///
/// Реквизиты, содержащиеся в кассовом чеке коррекции (БСО коррекции)
pub const CORRECTION_RECEIPT_1_1: FieldSet = FieldSet(&[
    // Наименование документа
    FieldSpec {
        tag: Some(1000),
        forms: Form::PRINTED,
        req: Req::Mandatory,
        multi: false,
    },
    // Код формы ФД
    FieldSpec {
        tag: None,
        forms: Form::ELECTRONIC,
        req: Req::Mandatory,
        multi: false,
    },
    // Номер версии ФФД
    FieldSpec {
        tag: Some(1209),
        forms: Form::ELECTRONIC,
        req: Req::Mandatory,
        multi: false,
    },
    // Наименование пользователя
    FieldSpec {
        tag: Some(1048),
        forms: Form::BOTH,
        req: Req::Optional,
        multi: false,
    },
    // ИНН пользователя
    FieldSpec {
        tag: Some(1018),
        forms: Form::BOTH,
        req: Req::Optional,
        multi: false,
    },
    // Покупатель (клиент)
    FieldSpec {
        tag: Some(1227),
        forms: Form::BOTH,
        req: Req::Optional,
        multi: false,
    },
    // ИНН покупателя (клиента)
    FieldSpec {
        tag: Some(1228),
        forms: Form::BOTH,
        req: Req::SometimesRequired,
        multi: false,
    },
    // Номер чека за смену
    FieldSpec {
        tag: Some(1042),
        forms: Form::ELECTRONIC,
        req: Req::Mandatory,
        multi: false,
    },
    // Дата, время
    FieldSpec {
        tag: Some(1012),
        forms: Form::BOTH,
        req: Req::Mandatory,
        multi: false,
    },
    // Номер смены
    FieldSpec {
        tag: Some(1038),
        forms: Form::ELECTRONIC,
        req: Req::Mandatory,
        multi: false,
    },
    // Признак расчета
    FieldSpec {
        tag: Some(1054),
        forms: Form::BOTH,
        req: Req::Mandatory,
        multi: false,
    },
    // Применяемая система налогообложения
    FieldSpec {
        tag: Some(1055),
        forms: Form::BOTH,
        req: Req::Optional,
        multi: false,
    },
    // Кассир
    FieldSpec {
        tag: Some(1021),
        forms: Form::BOTH,
        req: Req::SometimesRequired,
        multi: false,
    },
    // ИНН кассира
    FieldSpec {
        tag: Some(1203),
        forms: Form::ELECTRONIC,
        req: Req::Optional,
        multi: false,
    },
    // Регистрационный номер ККТ
    FieldSpec {
        tag: Some(1037),
        forms: Form::BOTH,
        req: Req::Mandatory,
        multi: false,
    },
    // Тип коррекции
    FieldSpec {
        tag: Some(1173),
        forms: Form::BOTH,
        req: Req::Optional,
        multi: false,
    },
    // Основание для коррекции
    FieldSpec {
        tag: Some(1174),
        forms: Form::BOTH,
        req: Req::Mandatory,
        multi: false,
    },
    // Номер автомата
    FieldSpec {
        tag: Some(1036),
        forms: Form::BOTH,
        req: Req::SometimesRequired,
        multi: false,
    },
    // Адрес расчетов
    FieldSpec {
        tag: Some(1009),
        forms: Form::BOTH,
        req: Req::Optional,
        multi: false,
    },
    // Место расчетов
    FieldSpec {
        tag: Some(1187),
        forms: Form::BOTH,
        req: Req::Optional,
        multi: false,
    },
    // Телефон или электронный адрес покупателя
    FieldSpec {
        tag: Some(1008),
        forms: Form::BOTH,
        req: Req::Optional,
        multi: false,
    },
    // Предмет расчета
    FieldSpec {
        tag: Some(1059),
        forms: Form::BOTH,
        req: Req::Mandatory,
        multi: true,
    },
    // Сумма расчета, указанного в чеке (БСО)
    FieldSpec {
        tag: Some(1020),
        forms: Form::BOTH,
        req: Req::Mandatory,
        multi: false,
    },
    // Сумма по чеку (БСО) наличными
    FieldSpec {
        tag: Some(1031),
        forms: Form::BOTH,
        req: Req::Optional,
        multi: false,
    },
    // Сумма по чеку (БСО) безналичными
    FieldSpec {
        tag: Some(1081),
        forms: Form::BOTH,
        req: Req::Optional,
        multi: false,
    },
    // Сумма по чеку (БСО) предоплатой (зачетом аванса и (или) предыдущих платежей)
    FieldSpec {
        tag: Some(1215),
        forms: Form::BOTH,
        req: Req::Optional,
        multi: false,
    },
    // Сумма по чеку (БСО) постоплатой (в кредит)
    FieldSpec {
        tag: Some(1216),
        forms: Form::BOTH,
        req: Req::Optional,
        multi: false,
    },
    // Сумма по чеку (БСО) встречным предоставлением
    FieldSpec {
        tag: Some(1217),
        forms: Form::BOTH,
        req: Req::Optional,
        multi: false,
    },
    // Сумма НДС чека по ставке 20%
    FieldSpec {
        tag: Some(1102),
        forms: Form::BOTH,
        req: Req::SometimesRequired,
        multi: false,
    },
    // Сумма НДС чека по ставке 10%
    FieldSpec {
        tag: Some(1103),
        forms: Form::BOTH,
        req: Req::SometimesRequired,
        multi: false,
    },
    // Сумма расчета по чеку с НДС по ставке 0%
    FieldSpec {
        tag: Some(1104),
        forms: Form::BOTH,
        req: Req::SometimesRequired,
        multi: false,
    },
    // Сумма расчета по чеку без НДС
    FieldSpec {
        tag: Some(1105),
        forms: Form::BOTH,
        req: Req::SometimesRequired,
        multi: false,
    },
    // Сумма НДС чека по расч. ставке 20/120
    FieldSpec {
        tag: Some(1106),
        forms: Form::BOTH,
        req: Req::SometimesRequired,
        multi: false,
    },
    // Сумма НДС чека по расч. ставке 10/110
    FieldSpec {
        tag: Some(1107),
        forms: Form::BOTH,
        req: Req::SometimesRequired,
        multi: false,
    },
    // Признак ККТ для расчетов только в Интернет
    FieldSpec {
        tag: Some(1108),
        forms: Form::ELECTRONIC,
        req: Req::SometimesRequired,
        multi: false,
    },
    // Адрес электронной почты отправителя чека
    FieldSpec {
        tag: Some(1117),
        forms: Form::ELECTRONIC,
        req: Req::SometimesRequired,
        multi: false,
    },
    // Признак агента
    FieldSpec {
        tag: Some(1057),
        forms: Form::BOTH,
        req: Req::Optional,
        multi: false,
    },
    // Телефон оператора перевода
    FieldSpec {
        tag: Some(1075),
        forms: Form::BOTH,
        req: Req::Optional,
        multi: true,
    },
    // Операция банковского платежного агента
    FieldSpec {
        tag: Some(1044),
        forms: Form::BOTH,
        req: Req::SometimesRequired,
        multi: false,
    },
    // Телефон платежного агента
    FieldSpec {
        tag: Some(1073),
        forms: Form::BOTH,
        req: Req::Optional,
        multi: true,
    },
    // Телефон оператора по приему платежей
    FieldSpec {
        tag: Some(1074),
        forms: Form::BOTH,
        req: Req::Optional,
        multi: true,
    },
    // Наименование оператора перевода
    FieldSpec {
        tag: Some(1026),
        forms: Form::BOTH,
        req: Req::SometimesRequired,
        multi: false,
    },
    // Адрес оператора перевода
    FieldSpec {
        tag: Some(1005),
        forms: Form::BOTH,
        req: Req::SometimesRequired,
        multi: false,
    },
    // ИНН оператора перевода
    FieldSpec {
        tag: Some(1016),
        forms: Form::BOTH,
        req: Req::SometimesRequired,
        multi: false,
    },
    // Телефон поставщика
    FieldSpec {
        tag: Some(1171),
        forms: Form::BOTH,
        req: Req::Optional,
        multi: true,
    },
    // Адрес сайта ФНС
    FieldSpec {
        tag: Some(1060),
        forms: Form::ELECTRONIC,
        req: Req::SometimesRequired,
        multi: false,
    },
    // Дополнительный реквизит чека (БСО)
    FieldSpec {
        tag: Some(1192),
        forms: Form::BOTH,
        req: Req::Optional,
        multi: false,
    },
    // Дополнительный реквизит пользователя
    FieldSpec {
        tag: Some(1084),
        forms: Form::BOTH,
        req: Req::Optional,
        multi: false,
    },
    // Номер ФД
    FieldSpec {
        tag: Some(1040),
        forms: Form::BOTH,
        req: Req::Mandatory,
        multi: false,
    },
    // Номер ФН
    FieldSpec {
        tag: Some(1041),
        forms: Form::BOTH,
        req: Req::Mandatory,
        multi: false,
    },
    // ФПД (1)
    FieldSpec {
        tag: Some(1077),
        forms: Form::BOTH,
        req: Req::Mandatory,
        multi: false,
    },
    // ФПС (4)
    FieldSpec {
        tag: None,
        forms: Form::ELECTRONIC,
        req: Req::Mandatory,
        multi: false,
    },
    // ФПА (5)
    FieldSpec {
        tag: None,
        forms: Form::ELECTRONIC,
        req: Req::SometimesRequired,
        multi: false,
    },
    // QR-код
    FieldSpec {
        tag: Some(1196),
        forms: Form::PRINTED,
        req: Req::Optional,
        multi: false,
    },
]);

/// Таблицы 67, 122
///
/// Структура данных для реквизита «основание для коррекции» (тег 1174)
pub const CORRECTION_BASIS_1_1: FieldSet = FieldSet(&[
    // Дата совершения корректируемого расчета
    FieldSpec {
        tag: Some(1178),
        forms: Form::BOTH,
        req: Req::SometimesRequired,
        multi: false,
    },
    // Номер предписания налогового органа
    FieldSpec {
        tag: Some(1179),
        forms: Form::ELECTRONIC,
        req: Req::SometimesRequired,
        multi: false,
    },
]);

/// Таблица 68
///
/// Реквизиты, содержащиеся в отчете о закрытии смены
pub const SHIFT_END_REPORT_1_1: FieldSet = FieldSet(&[
    // Наименование документа
    FieldSpec {
        tag: Some(1000),
        forms: Form::PRINTED,
        req: Req::Mandatory,
        multi: false,
    },
    // Код формы ФД
    FieldSpec {
        tag: None,
        forms: Form::ELECTRONIC,
        req: Req::Mandatory,
        multi: false,
    },
    // Номер версии ФФД
    FieldSpec {
        tag: Some(1209),
        forms: Form::ELECTRONIC,
        req: Req::Mandatory,
        multi: false,
    },
    // Наименование пользователя
    FieldSpec {
        tag: Some(1048),
        forms: Form::PRINTED,
        req: Req::Mandatory,
        multi: false,
    },
    // ИНН пользователя
    FieldSpec {
        tag: Some(1018),
        forms: Form::BOTH,
        req: Req::Mandatory,
        multi: false,
    },
    // Кассир
    FieldSpec {
        tag: Some(1021),
        forms: Form::BOTH,
        req: Req::SometimesRequired,
        multi: false,
    },
    // ИНН кассира
    FieldSpec {
        tag: Some(1203),
        forms: Form::ELECTRONIC,
        req: Req::Optional,
        multi: false,
    },
    // Адрес расчетов
    FieldSpec {
        tag: Some(1009),
        forms: Form::PRINTED,
        req: Req::Mandatory,
        multi: false,
    },
    // Место расчетов
    FieldSpec {
        tag: Some(1187),
        forms: Form::PRINTED,
        req: Req::Mandatory,
        multi: false,
    },
    // Дата, время
    FieldSpec {
        tag: Some(1012),
        forms: Form::BOTH,
        req: Req::Mandatory,
        multi: false,
    },
    // Номер смены
    FieldSpec {
        tag: Some(1038),
        forms: Form::BOTH,
        req: Req::Mandatory,
        multi: false,
    },
    // Регистрационный номер ККТ
    FieldSpec {
        tag: Some(1037),
        forms: Form::BOTH,
        req: Req::Mandatory,
        multi: false,
    },
    // Количество кассовых чеков (БСО) за смену
    FieldSpec {
        tag: Some(1118),
        forms: Form::BOTH,
        req: Req::Mandatory,
        multi: false,
    },
    // Общее количество ФД за смену
    FieldSpec {
        tag: Some(1111),
        forms: Form::BOTH,
        req: Req::Mandatory,
        multi: false,
    },
    // Количество непереданных ФД
    FieldSpec {
        tag: Some(1097),
        forms: Form::BOTH,
        req: Req::SometimesRequired,
        multi: false,
    },
    // Дата первого из непереданных ФД
    FieldSpec {
        tag: Some(1098),
        forms: Form::BOTH,
        req: Req::SometimesRequired,
        multi: false,
    },
    // Признак превышения времени ожидания ответа ОФД
    FieldSpec {
        tag: Some(1053),
        forms: Form::BOTH,
        req: Req::SometimesRequired,
        multi: false,
    },
    // Признак необходимости срочной замены ФН
    FieldSpec {
        tag: Some(1051),
        forms: Form::BOTH,
        req: Req::SometimesRequired,
        multi: false,
    },
    // Признак переполнения памяти ФН
    FieldSpec {
        tag: Some(1052),
        forms: Form::BOTH,
        req: Req::SometimesRequired,
        multi: false,
    },
    // Признак исчерпания ресурса ФН
    FieldSpec {
        tag: Some(1050),
        forms: Form::BOTH,
        req: Req::SometimesRequired,
        multi: false,
    },
    // Сообщение оператора
    FieldSpec {
        tag: Some(1206),
        forms: Form::BOTH,
        req: Req::SometimesRequired,
        multi: false,
    },
    // Счетчики итогов смены
    FieldSpec {
        tag: Some(1194),
        forms: Form::BOTH,
        req: Req::Mandatory,
        multi: false,
    },
    // Счетчики итогов ФН
    FieldSpec {
        tag: Some(1157),
        forms: Form::BOTH,
        req: Req::Mandatory,
        multi: false,
    },
    // Ресурс ключей ФП
    FieldSpec {
        tag: Some(1213),
        forms: Form::BOTH,
        req: Req::Mandatory,
        multi: false,
    },
    // Номер ФД
    FieldSpec {
        tag: Some(1040),
        forms: Form::BOTH,
        req: Req::Mandatory,
        multi: false,
    },
    // Номер ФН
    FieldSpec {
        tag: Some(1041),
        forms: Form::BOTH,
        req: Req::Mandatory,
        multi: false,
    },
    // ФПД (1)
    FieldSpec {
        tag: Some(1077),
        forms: Form::BOTH,
        req: Req::Mandatory,
        multi: false,
    },
    // ФПС (4)
    FieldSpec {
        tag: None,
        forms: Form::ELECTRONIC,
        req: Req::Mandatory,
        multi: false,
    },
]);

/// Таблица 69
///
/// Реквизиты, содержащиеся в отчете о закрытии фискального накопителя
pub const FN_CLOSE_REPORT_1_1: FieldSet = FieldSet(&[
    // Наименование документа
    FieldSpec {
        tag: Some(1000),
        forms: Form::PRINTED,
        req: Req::Mandatory,
        multi: false,
    },
    // Код формы ФД
    FieldSpec {
        tag: None,
        forms: Form::ELECTRONIC,
        req: Req::Mandatory,
        multi: false,
    },
    // Номер версии ФФД
    FieldSpec {
        tag: Some(1209),
        forms: Form::ELECTRONIC,
        req: Req::Mandatory,
        multi: false,
    },
    // Наименование пользователя
    FieldSpec {
        tag: Some(1048),
        forms: Form::BOTH,
        req: Req::Mandatory,
        multi: false,
    },
    // ИНН пользователя
    FieldSpec {
        tag: Some(1018),
        forms: Form::BOTH,
        req: Req::Mandatory,
        multi: false,
    },
    // Кассир
    FieldSpec {
        tag: Some(1021),
        forms: Form::BOTH,
        req: Req::SometimesRequired,
        multi: false,
    },
    // ИНН кассира
    FieldSpec {
        tag: Some(1203),
        forms: Form::ELECTRONIC,
        req: Req::Optional,
        multi: false,
    },
    // Адрес расчетов
    FieldSpec {
        tag: Some(1009),
        forms: Form::BOTH,
        req: Req::SometimesRequired,
        multi: false,
    },
    // Место расчетов
    FieldSpec {
        tag: Some(1187),
        forms: Form::BOTH,
        req: Req::SometimesRequired,
        multi: false,
    },
    // Дата, время
    FieldSpec {
        tag: Some(1012),
        forms: Form::BOTH,
        req: Req::Mandatory,
        multi: false,
    },
    // Номер смены
    FieldSpec {
        tag: Some(1038),
        forms: Form::BOTH,
        req: Req::Mandatory,
        multi: false,
    },
    // Регистрационный номер ККТ
    FieldSpec {
        tag: Some(1037),
        forms: Form::BOTH,
        req: Req::Mandatory,
        multi: false,
    },
    // Счетчики итогов ФН
    FieldSpec {
        tag: Some(1157),
        forms: Form::BOTH,
        req: Req::Mandatory,
        multi: false,
    },
    // Номер ФД
    FieldSpec {
        tag: Some(1040),
        forms: Form::BOTH,
        req: Req::Mandatory,
        multi: false,
    },
    // Номер ФН
    FieldSpec {
        tag: Some(1041),
        forms: Form::BOTH,
        req: Req::Mandatory,
        multi: false,
    },
    // ФПД (1)
    FieldSpec {
        tag: Some(1077),
        forms: Form::BOTH,
        req: Req::Mandatory,
        multi: false,
    },
    // ФПС (4)
    FieldSpec {
        tag: None,
        forms: Form::ELECTRONIC,
        req: Req::Mandatory,
        multi: false,
    },
]);

/// Таблицы 70, 125
///
/// Реквизиты, содержащиеся в подтверждении оператора
pub const OPERATOR_CONFIRMATION_1_1: FieldSet = FieldSet(&[
    // Наименование документа
    FieldSpec {
        tag: Some(1000),
        forms: Form::PRINTED,
        req: Req::Optional,
        multi: false,
    },
    // Код формы ФД
    FieldSpec {
        tag: None,
        forms: Form::ELECTRONIC,
        req: Req::Mandatory,
        multi: false,
    },
    // ИНН ОФД
    FieldSpec {
        tag: Some(1017),
        forms: Form::BOTH,
        req: Req::Mandatory,
        multi: false,
    },
    // Номер ФН
    FieldSpec {
        tag: Some(1041),
        forms: Form::BOTH,
        req: Req::Mandatory,
        multi: false,
    },
    // Номер ФД
    FieldSpec {
        tag: Some(1040),
        forms: Form::BOTH,
        req: Req::Mandatory,
        multi: false,
    },
    // Дата, время
    FieldSpec {
        tag: Some(1012),
        forms: Form::BOTH,
        req: Req::Mandatory,
        multi: false,
    },
    // Сообщение оператора
    FieldSpec {
        tag: Some(1206),
        forms: Form::BOTH,
        req: Req::SometimesRequired,
        multi: false,
    },
    // ФПО (2)
    FieldSpec {
        tag: Some(1078),
        forms: Form::BOTH,
        req: Req::Mandatory,
        multi: false,
    },
    // ФПП (3)
    FieldSpec {
        tag: None,
        forms: Form::ELECTRONIC,
        req: Req::Mandatory,
        multi: false,
    },
]);

/// Таблица 81
///
/// Формат ФД «Кассовый чек (БСО)»,
pub const FD_RECEIPT_1_1: FieldSet = FieldSet(&[
    // Наименование документа
    FieldSpec {
        tag: Some(1000),
        forms: Form::NONE,
        req: Req::Mandatory,
        multi: false,
    },
    // Наименование пользователя
    FieldSpec {
        tag: Some(1048),
        forms: Form::NONE,
        req: Req::Mandatory,
        multi: false,
    },
    // ИНН пользователя
    FieldSpec {
        tag: Some(1018),
        forms: Form::NONE,
        req: Req::Mandatory,
        multi: false,
    },
    // Номер чека за смену
    FieldSpec {
        tag: Some(1042),
        forms: Form::NONE,
        req: Req::Mandatory,
        multi: false,
    },
    // Покупатель (клиент)
    FieldSpec {
        tag: Some(1227),
        forms: Form::NONE,
        req: Req::Optional,
        multi: false,
    },
    // ИНН покупателя (клиента)
    FieldSpec {
        tag: Some(1228),
        forms: Form::NONE,
        req: Req::SometimesRequired,
        multi: false,
    },
    // Дата, время
    FieldSpec {
        tag: Some(1012),
        forms: Form::NONE,
        req: Req::Mandatory,
        multi: false,
    },
    // Номер смены
    FieldSpec {
        tag: Some(1038),
        forms: Form::NONE,
        req: Req::Mandatory,
        multi: false,
    },
    // Признак расчета
    FieldSpec {
        tag: Some(1054),
        forms: Form::NONE,
        req: Req::Mandatory,
        multi: false,
    },
    // Применяемая система налогообложения
    FieldSpec {
        tag: Some(1055),
        forms: Form::NONE,
        req: Req::Optional,
        multi: false,
    },
    // Кассир
    FieldSpec {
        tag: Some(1021),
        forms: Form::NONE,
        req: Req::SometimesRequired,
        multi: false,
    },
    // Регистрационный номер ККТ
    FieldSpec {
        tag: Some(1037),
        forms: Form::NONE,
        req: Req::Mandatory,
        multi: false,
    },
    // Номер автомата
    FieldSpec {
        tag: Some(1036),
        forms: Form::NONE,
        req: Req::SometimesRequired,
        multi: false,
    },
    // Адрес расчетов
    FieldSpec {
        tag: Some(1009),
        forms: Form::NONE,
        req: Req::SometimesRequired,
        multi: false,
    },
    // Место расчетов
    FieldSpec {
        tag: Some(1187),
        forms: Form::NONE,
        req: Req::SometimesRequired,
        multi: false,
    },
    // Телефон или электронный адрес покупателя
    FieldSpec {
        tag: Some(1008),
        forms: Form::NONE,
        req: Req::Optional,
        multi: false,
    },
    // Предмет расчета
    FieldSpec {
        tag: Some(1059),
        forms: Form::NONE,
        req: Req::Optional,
        multi: true,
    },
    // Телефон оператора перевода
    FieldSpec {
        tag: Some(1075),
        forms: Form::NONE,
        req: Req::SometimesRequired,
        multi: true,
    },
    // Операция банковского платежного агента
    FieldSpec {
        tag: Some(1044),
        forms: Form::NONE,
        req: Req::SometimesRequired,
        multi: true,
    },
    // Телефон платежного агента
    FieldSpec {
        tag: Some(1073),
        forms: Form::NONE,
        req: Req::SometimesRequired,
        multi: true,
    },
    // Телефон оператора по приему платежей
    FieldSpec {
        tag: Some(1074),
        forms: Form::NONE,
        req: Req::SometimesRequired,
        multi: true,
    },
    // Наименование оператора перевода
    FieldSpec {
        tag: Some(1026),
        forms: Form::NONE,
        req: Req::SometimesRequired,
        multi: true,
    },
    // Адрес оператора перевода
    FieldSpec {
        tag: Some(1005),
        forms: Form::NONE,
        req: Req::SometimesRequired,
        multi: true,
    },
    // ИНН оператора перевода
    FieldSpec {
        tag: Some(1016),
        forms: Form::NONE,
        req: Req::SometimesRequired,
        multi: true,
    },
    // Телефон поставщика
    FieldSpec {
        tag: Some(1171),
        forms: Form::NONE,
        req: Req::SometimesRequired,
        multi: true,
    },
    // Сумма расчета, указанного в чеке (БСО)
    FieldSpec {
        tag: Some(1020),
        forms: Form::NONE,
        req: Req::Mandatory,
        multi: false,
    },
    // Сумма по чеку (БСО) наличными
    FieldSpec {
        tag: Some(1031),
        forms: Form::NONE,
        req: Req::SometimesRequired,
        multi: false,
    },
    // Сумма по чеку (БСО) безналичными
    FieldSpec {
        tag: Some(1081),
        forms: Form::NONE,
        req: Req::SometimesRequired,
        multi: false,
    },
    // Сумма по чеку (БСО) предоплатой (зачетом аванса и (или) предыдущих платежей)
    FieldSpec {
        tag: Some(1215),
        forms: Form::NONE,
        req: Req::SometimesRequired,
        multi: false,
    },
    // Сумма по чеку (БСО) постоплатой (в кредит)
    FieldSpec {
        tag: Some(1216),
        forms: Form::NONE,
        req: Req::SometimesRequired,
        multi: false,
    },
    // Сумма по чеку (БСО) встречным предоставлением
    FieldSpec {
        tag: Some(1217),
        forms: Form::NONE,
        req: Req::SometimesRequired,
        multi: false,
    },
    // Сумма НДС чека по ставке 20%
    FieldSpec {
        tag: Some(1102),
        forms: Form::NONE,
        req: Req::SometimesRequired,
        multi: false,
    },
    // Сумма НДС чека по ставке 10%
    FieldSpec {
        tag: Some(1103),
        forms: Form::NONE,
        req: Req::SometimesRequired,
        multi: false,
    },
    // Сумма расчета по чеку с НДС по ставке 0%
    FieldSpec {
        tag: Some(1104),
        forms: Form::NONE,
        req: Req::SometimesRequired,
        multi: false,
    },
    // Сумма расчета по чеку без НДС
    FieldSpec {
        tag: Some(1105),
        forms: Form::NONE,
        req: Req::SometimesRequired,
        multi: false,
    },
    // Сумма НДС чека по расч. ставке 20/120
    FieldSpec {
        tag: Some(1106),
        forms: Form::NONE,
        req: Req::SometimesRequired,
        multi: false,
    },
    // Сумма НДС чека по расч. ставке 10/110
    FieldSpec {
        tag: Some(1107),
        forms: Form::NONE,
        req: Req::SometimesRequired,
        multi: false,
    },
    // Адрес электронной почты отправителя чека
    FieldSpec {
        tag: Some(1117),
        forms: Form::NONE,
        req: Req::SometimesRequired,
        multi: false,
    },
    // Признак агента
    FieldSpec {
        tag: Some(1057),
        forms: Form::NONE,
        req: Req::Optional,
        multi: false,
    },
    // Адрес сайта ФНС
    FieldSpec {
        tag: Some(1060),
        forms: Form::NONE,
        req: Req::SometimesRequired,
        multi: false,
    },
    // Дополнительный реквизит чека (БСО)
    FieldSpec {
        tag: Some(1192),
        forms: Form::NONE,
        req: Req::Optional,
        multi: true,
    },
    // Дополнительный реквизит пользователя
    FieldSpec {
        tag: Some(1084),
        forms: Form::NONE,
        req: Req::Optional,
        multi: false,
    },
    // Номер ФД
    FieldSpec {
        tag: Some(1040),
        forms: Form::NONE,
        req: Req::Mandatory,
        multi: false,
    },
    // Номер ФН
    FieldSpec {
        tag: Some(1041),
        forms: Form::NONE,
        req: Req::Mandatory,
        multi: false,
    },
    // ФПД
    FieldSpec {
        tag: Some(1077),
        forms: Form::NONE,
        req: Req::Mandatory,
        multi: false,
    },
    // QR-код
    FieldSpec {
        tag: Some(1196),
        forms: Form::NONE,
        req: Req::SometimesRequired,
        multi: false,
    },
]);

/// Таблица 82
///
/// Формат ФД «Кассовый чек коррекции (БСО коррекции)»,
pub const FD_CORRECTION_RECEIPT_1_1: FieldSet = FieldSet(&[
    // Наименование документа
    FieldSpec {
        tag: Some(1000),
        forms: Form::NONE,
        req: Req::Mandatory,
        multi: false,
    },
    // Наименование пользователя
    FieldSpec {
        tag: Some(1048),
        forms: Form::NONE,
        req: Req::Mandatory,
        multi: false,
    },
    // ИНН пользователя
    FieldSpec {
        tag: Some(1018),
        forms: Form::NONE,
        req: Req::Mandatory,
        multi: false,
    },
    // Номер чека за смену
    FieldSpec {
        tag: Some(1042),
        forms: Form::NONE,
        req: Req::Mandatory,
        multi: false,
    },
    // Покупатель (клиент)
    FieldSpec {
        tag: Some(1227),
        forms: Form::NONE,
        req: Req::Optional,
        multi: false,
    },
    // ИНН покупателя (клиента)
    FieldSpec {
        tag: Some(1228),
        forms: Form::NONE,
        req: Req::SometimesRequired,
        multi: false,
    },
    // Дата, время
    FieldSpec {
        tag: Some(1012),
        forms: Form::NONE,
        req: Req::Mandatory,
        multi: false,
    },
    // Номер смены
    FieldSpec {
        tag: Some(1038),
        forms: Form::NONE,
        req: Req::Mandatory,
        multi: false,
    },
    // Признак расчета
    FieldSpec {
        tag: Some(1054),
        forms: Form::NONE,
        req: Req::Mandatory,
        multi: false,
    },
    // Применяемая система налогообложения
    FieldSpec {
        tag: Some(1055),
        forms: Form::NONE,
        req: Req::Optional,
        multi: false,
    },
    // Кассир
    FieldSpec {
        tag: Some(1021),
        forms: Form::NONE,
        req: Req::SometimesRequired,
        multi: false,
    },
    // Регистрационный номер ККТ
    FieldSpec {
        tag: Some(1037),
        forms: Form::NONE,
        req: Req::Mandatory,
        multi: false,
    },
    // Номер автомата
    FieldSpec {
        tag: Some(1036),
        forms: Form::NONE,
        req: Req::SometimesRequired,
        multi: false,
    },
    // Адрес расчетов
    FieldSpec {
        tag: Some(1009),
        forms: Form::NONE,
        req: Req::SometimesRequired,
        multi: false,
    },
    // Место расчетов
    FieldSpec {
        tag: Some(1187),
        forms: Form::NONE,
        req: Req::SometimesRequired,
        multi: false,
    },
    // Телефон или электронный адрес покупателя
    FieldSpec {
        tag: Some(1008),
        forms: Form::NONE,
        req: Req::Optional,
        multi: false,
    },
    // Предмет расчета
    FieldSpec {
        tag: Some(1059),
        forms: Form::NONE,
        req: Req::Optional,
        multi: true,
    },
    // Телефон оператора перевода
    FieldSpec {
        tag: Some(1075),
        forms: Form::NONE,
        req: Req::SometimesRequired,
        multi: true,
    },
    // Операция банковского платежного агента
    FieldSpec {
        tag: Some(1044),
        forms: Form::NONE,
        req: Req::SometimesRequired,
        multi: true,
    },
    // Телефон платежного агента
    FieldSpec {
        tag: Some(1073),
        forms: Form::NONE,
        req: Req::SometimesRequired,
        multi: true,
    },
    // Телефон оператора по приему платежей
    FieldSpec {
        tag: Some(1074),
        forms: Form::NONE,
        req: Req::SometimesRequired,
        multi: true,
    },
    // Наименование оператора перевода
    FieldSpec {
        tag: Some(1026),
        forms: Form::NONE,
        req: Req::SometimesRequired,
        multi: true,
    },
    // Адрес оператора перевода
    FieldSpec {
        tag: Some(1005),
        forms: Form::NONE,
        req: Req::SometimesRequired,
        multi: true,
    },
    // ИНН оператора перевода
    FieldSpec {
        tag: Some(1016),
        forms: Form::NONE,
        req: Req::SometimesRequired,
        multi: true,
    },
    // Телефон поставщика
    FieldSpec {
        tag: Some(1171),
        forms: Form::NONE,
        req: Req::SometimesRequired,
        multi: true,
    },
    // Сумма расчета, указанного в чеке (БСО)
    FieldSpec {
        tag: Some(1020),
        forms: Form::NONE,
        req: Req::Mandatory,
        multi: false,
    },
    // Сумма по чеку (БСО) наличными
    FieldSpec {
        tag: Some(1031),
        forms: Form::NONE,
        req: Req::SometimesRequired,
        multi: false,
    },
    // Сумма по чеку (БСО) безналичными
    FieldSpec {
        tag: Some(1081),
        forms: Form::NONE,
        req: Req::SometimesRequired,
        multi: false,
    },
    // Сумма по чеку (БСО) предоплатой (зачетом аванса и (или) предыдущих платежей)
    FieldSpec {
        tag: Some(1215),
        forms: Form::NONE,
        req: Req::SometimesRequired,
        multi: false,
    },
    // Сумма по чеку (БСО) постоплатой (в кредит)
    FieldSpec {
        tag: Some(1216),
        forms: Form::NONE,
        req: Req::SometimesRequired,
        multi: false,
    },
    // Сумма по чеку (БСО) встречным предоставлением
    FieldSpec {
        tag: Some(1217),
        forms: Form::NONE,
        req: Req::SometimesRequired,
        multi: false,
    },
    // Сумма НДС чека по ставке 20%
    FieldSpec {
        tag: Some(1102),
        forms: Form::NONE,
        req: Req::SometimesRequired,
        multi: false,
    },
    // Сумма НДС чека по ставке 10%
    FieldSpec {
        tag: Some(1103),
        forms: Form::NONE,
        req: Req::SometimesRequired,
        multi: false,
    },
    // Сумма расчета по чеку с НДС по ставке 0%
    FieldSpec {
        tag: Some(1104),
        forms: Form::NONE,
        req: Req::SometimesRequired,
        multi: false,
    },
    // Сумма расчета по чеку без НДС
    FieldSpec {
        tag: Some(1105),
        forms: Form::NONE,
        req: Req::SometimesRequired,
        multi: false,
    },
    // Сумма НДС чека по расч. ставке 20/120
    FieldSpec {
        tag: Some(1106),
        forms: Form::NONE,
        req: Req::SometimesRequired,
        multi: false,
    },
    // Сумма НДС чека по расч. ставке 10/110
    FieldSpec {
        tag: Some(1107),
        forms: Form::NONE,
        req: Req::SometimesRequired,
        multi: false,
    },
    // Адрес электронной почты отправителя чека
    FieldSpec {
        tag: Some(1117),
        forms: Form::NONE,
        req: Req::SometimesRequired,
        multi: false,
    },
    // Признак агента
    FieldSpec {
        tag: Some(1057),
        forms: Form::NONE,
        req: Req::Optional,
        multi: false,
    },
    // Адрес сайта ФНС
    FieldSpec {
        tag: Some(1060),
        forms: Form::NONE,
        req: Req::SometimesRequired,
        multi: false,
    },
    // Дополнительный реквизит чека (БСО)
    FieldSpec {
        tag: Some(1192),
        forms: Form::NONE,
        req: Req::Optional,
        multi: true,
    },
    // Дополнительный реквизит пользователя
    FieldSpec {
        tag: Some(1084),
        forms: Form::NONE,
        req: Req::Optional,
        multi: false,
    },
    // Номер ФД
    FieldSpec {
        tag: Some(1040),
        forms: Form::NONE,
        req: Req::Mandatory,
        multi: false,
    },
    // Номер ФН
    FieldSpec {
        tag: Some(1041),
        forms: Form::NONE,
        req: Req::Mandatory,
        multi: false,
    },
    // ФПД
    FieldSpec {
        tag: Some(1077),
        forms: Form::NONE,
        req: Req::Mandatory,
        multi: false,
    },
    // QR-код
    FieldSpec {
        tag: Some(1196),
        forms: Form::NONE,
        req: Req::Optional,
        multi: false,
    },
]);

/// Таблица 87
///
/// Реквизиты, содержащиеся в отчете о регистрации и отчете об изменении параметров регистрации
pub const REGISTRATION_REPORT_1_2: FieldSet = FieldSet(&[
    // Наименование документа
    FieldSpec {
        tag: Some(1000),
        forms: Form::PRINTED,
        req: Req::Mandatory,
        multi: false,
    },
    // Код формы ФД
    FieldSpec {
        tag: None,
        forms: Form::ELECTRONIC,
        req: Req::Mandatory,
        multi: false,
    },
    // Номер версии ФФД
    FieldSpec {
        tag: Some(1209),
        forms: Form::ELECTRONIC,
        req: Req::Mandatory,
        multi: false,
    },
    // Наименование пользователя
    FieldSpec {
        tag: Some(1048),
        forms: Form::BOTH,
        req: Req::Mandatory,
        multi: false,
    },
    // ИНН пользователя
    FieldSpec {
        tag: Some(1018),
        forms: Form::BOTH,
        req: Req::Mandatory,
        multi: false,
    },
    // Системы налогообложения
    FieldSpec {
        tag: Some(1062),
        forms: Form::BOTH,
        req: Req::Optional,
        multi: false,
    },
    // Дата, время
    FieldSpec {
        tag: Some(1012),
        forms: Form::BOTH,
        req: Req::Mandatory,
        multi: false,
    },
    // Регистрационный номер ККТ
    FieldSpec {
        tag: Some(1037),
        forms: Form::BOTH,
        req: Req::Mandatory,
        multi: false,
    },
    // Признак автономного режима
    FieldSpec {
        tag: Some(1002),
        forms: Form::BOTH,
        req: Req::Mandatory,
        multi: false,
    },
    // Признак шифрования
    FieldSpec {
        tag: Some(1056),
        forms: Form::BOTH,
        req: Req::Mandatory,
        multi: false,
    },
    // Признак автоматического режима
    FieldSpec {
        tag: Some(1001),
        forms: Form::BOTH,
        req: Req::Mandatory,
        multi: false,
    },
    // Номер автомата
    FieldSpec {
        tag: Some(1036),
        forms: Form::BOTH,
        req: Req::SometimesRequired,
        multi: false,
    },
    // Признаки условий применения ККТ
    FieldSpec {
        tag: Some(1290),
        forms: Form::BOTH,
        req: Req::Mandatory,
        multi: false,
    },
    // Заводской номер ККТ
    FieldSpec {
        tag: Some(1013),
        forms: Form::BOTH,
        req: Req::Mandatory,
        multi: false,
    },
    // Кассир
    FieldSpec {
        tag: Some(1021),
        forms: Form::BOTH,
        req: Req::Mandatory,
        multi: false,
    },
    // ИНН кассира
    FieldSpec {
        tag: Some(1203),
        forms: Form::ELECTRONIC,
        req: Req::Optional,
        multi: false,
    },
    // Адрес расчетов
    FieldSpec {
        tag: Some(1009),
        forms: Form::BOTH,
        req: Req::Mandatory,
        multi: false,
    },
    // Место расчетов
    FieldSpec {
        tag: Some(1187),
        forms: Form::BOTH,
        req: Req::Mandatory,
        multi: false,
    },
    // Адрес сайта ФНС
    FieldSpec {
        tag: Some(1060),
        forms: Form::BOTH,
        req: Req::SometimesRequired,
        multi: false,
    },
    // Адрес электронной почты отправителя чека
    FieldSpec {
        tag: Some(1117),
        forms: Form::BOTH,
        req: Req::SometimesRequired,
        multi: false,
    },
    // ИНН ОФД
    FieldSpec {
        tag: Some(1017),
        forms: Form::BOTH,
        req: Req::SometimesRequired,
        multi: false,
    },
    // Наименование ОФД
    FieldSpec {
        tag: Some(1046),
        forms: Form::BOTH,
        req: Req::SometimesRequired,
        multi: false,
    },
    // Версия ККТ
    FieldSpec {
        tag: Some(1188),
        forms: Form::BOTH,
        req: Req::Mandatory,
        multi: false,
    },
    // Версия ФФД ККТ
    FieldSpec {
        tag: Some(1189),
        forms: Form::BOTH,
        req: Req::Mandatory,
        multi: false,
    },
    // Версия ФФД ФН
    FieldSpec {
        tag: Some(1190),
        forms: Form::BOTH,
        req: Req::Mandatory,
        multi: false,
    },
    // Ресурс ключей ФП
    FieldSpec {
        tag: Some(1213),
        forms: Form::BOTH,
        req: Req::Mandatory,
        multi: false,
    },
    // Дополнительный реквизит ОР
    FieldSpec {
        tag: Some(1274),
        forms: Form::ELECTRONIC,
        req: Req::SometimesRequired,
        multi: false,
    },
    // Дополнительные данные ОР
    FieldSpec {
        tag: Some(1275),
        forms: Form::ELECTRONIC,
        req: Req::SometimesRequired,
        multi: false,
    },
    // Номер ФД
    FieldSpec {
        tag: Some(1040),
        forms: Form::BOTH,
        req: Req::Mandatory,
        multi: false,
    },
    // Номер ФН
    FieldSpec {
        tag: Some(1041),
        forms: Form::BOTH,
        req: Req::Mandatory,
        multi: false,
    },
    // ФПД (1)
    FieldSpec {
        tag: Some(1077),
        forms: Form::BOTH,
        req: Req::Mandatory,
        multi: false,
    },
    // ФПС (4)
    FieldSpec {
        tag: None,
        forms: Form::ELECTRONIC,
        req: Req::Mandatory,
        multi: false,
    },
]);

/// Таблица 88
///
/// Реквизиты, содержащиеся в отчете об изменении параметров регистрации
pub const REGISTRATION_PARAMETER_UPDATE_REPORT_1_2: FieldSet = FieldSet(&[
    // Коды причин изменения сведений о ККТ
    FieldSpec {
        tag: Some(1205),
        forms: Form::BOTH,
        req: Req::Mandatory,
        multi: false,
    },
    // Счетчики итогов ФН
    FieldSpec {
        tag: Some(1157),
        forms: Form::BOTH,
        req: Req::SometimesRequired,
        multi: false,
    },
]);

/// Таблица 94
///
/// Реквизиты, содержащиеся в отчете об открытии смены
pub const SHIFT_START_REPORT_1_2: FieldSet = FieldSet(&[
    // Наименование документа
    FieldSpec {
        tag: Some(1000),
        forms: Form::PRINTED,
        req: Req::Mandatory,
        multi: false,
    },
    // Код формы ФД
    FieldSpec {
        tag: None,
        forms: Form::ELECTRONIC,
        req: Req::Mandatory,
        multi: false,
    },
    // Номер версии ФФД
    FieldSpec {
        tag: Some(1209),
        forms: Form::ELECTRONIC,
        req: Req::Mandatory,
        multi: false,
    },
    // Наименование пользователя
    FieldSpec {
        tag: Some(1048),
        forms: Form::PRINTED,
        req: Req::Mandatory,
        multi: false,
    },
    // ИНН пользователя
    FieldSpec {
        tag: Some(1018),
        forms: Form::BOTH,
        req: Req::Mandatory,
        multi: false,
    },
    // Кассир
    FieldSpec {
        tag: Some(1021),
        forms: Form::BOTH,
        req: Req::SometimesRequired,
        multi: false,
    },
    // ИНН кассира
    FieldSpec {
        tag: Some(1203),
        forms: Form::ELECTRONIC,
        req: Req::Optional,
        multi: false,
    },
    // Адрес расчетов
    FieldSpec {
        tag: Some(1009),
        forms: Form::PRINTED,
        req: Req::Mandatory,
        multi: false,
    },
    // Место расчетов
    FieldSpec {
        tag: Some(1187),
        forms: Form::PRINTED,
        req: Req::Mandatory,
        multi: false,
    },
    // Дата, время
    FieldSpec {
        tag: Some(1012),
        forms: Form::BOTH,
        req: Req::Mandatory,
        multi: false,
    },
    // Номер смены
    FieldSpec {
        tag: Some(1038),
        forms: Form::BOTH,
        req: Req::Mandatory,
        multi: false,
    },
    // Регистрационный номер ККТ
    FieldSpec {
        tag: Some(1037),
        forms: Form::BOTH,
        req: Req::Mandatory,
        multi: false,
    },
    // Признак превышения времени ожидания ответа ОФД
    FieldSpec {
        tag: Some(1053),
        forms: Form::BOTH,
        req: Req::SometimesRequired,
        multi: false,
    },
    // Признак необходимости срочной замены ФН
    FieldSpec {
        tag: Some(1051),
        forms: Form::BOTH,
        req: Req::SometimesRequired,
        multi: false,
    },
    // Признак переполнения памяти ФН
    FieldSpec {
        tag: Some(1052),
        forms: Form::BOTH,
        req: Req::SometimesRequired,
        multi: false,
    },
    // Признак исчерпания ресурса ФН
    FieldSpec {
        tag: Some(1050),
        forms: Form::BOTH,
        req: Req::SometimesRequired,
        multi: false,
    },
    // Сообщение оператора
    FieldSpec {
        tag: Some(1206),
        forms: Form::BOTH,
        req: Req::SometimesRequired,
        multi: false,
    },
    // Версия ККТ
    FieldSpec {
        tag: Some(1188),
        forms: Form::BOTH,
        req: Req::Mandatory,
        multi: false,
    },
    // Версия ФФД ККТ
    FieldSpec {
        tag: Some(1189),
        forms: Form::BOTH,
        req: Req::Mandatory,
        multi: false,
    },
    // Дополнительный реквизит ООС
    FieldSpec {
        tag: Some(1276),
        forms: Form::ELECTRONIC,
        req: Req::SometimesRequired,
        multi: false,
    },
    // Дополнительные данные ООС
    FieldSpec {
        tag: Some(1277),
        forms: Form::ELECTRONIC,
        req: Req::SometimesRequired,
        multi: false,
    },
    // Номер ФД
    FieldSpec {
        tag: Some(1040),
        forms: Form::BOTH,
        req: Req::Mandatory,
        multi: false,
    },
    // Номер ФН
    FieldSpec {
        tag: Some(1041),
        forms: Form::BOTH,
        req: Req::Mandatory,
        multi: false,
    },
    // ФПД (1)
    FieldSpec {
        tag: Some(1077),
        forms: Form::BOTH,
        req: Req::Mandatory,
        multi: false,
    },
    // ФПС (4)
    FieldSpec {
        tag: None,
        forms: Form::ELECTRONIC,
        req: Req::Mandatory,
        multi: false,
    },
]);

/// Таблица 95
///
/// Реквизиты, содержащиеся в отчете о текущем состоянии расчетов
pub const PAYMENT_STATE_REPORT_1_2: FieldSet = FieldSet(&[
    // Наименование документа
    FieldSpec {
        tag: Some(1000),
        forms: Form::PRINTED,
        req: Req::Mandatory,
        multi: false,
    },
    // Код формы ФД
    FieldSpec {
        tag: None,
        forms: Form::ELECTRONIC,
        req: Req::Mandatory,
        multi: false,
    },
    // Номер версии ФФД
    FieldSpec {
        tag: Some(1209),
        forms: Form::ELECTRONIC,
        req: Req::Mandatory,
        multi: false,
    },
    // Наименование пользователя
    FieldSpec {
        tag: Some(1048),
        forms: Form::PRINTED,
        req: Req::Mandatory,
        multi: false,
    },
    // ИНН пользователя
    FieldSpec {
        tag: Some(1018),
        forms: Form::BOTH,
        req: Req::Mandatory,
        multi: false,
    },
    // Адрес расчетов
    FieldSpec {
        tag: Some(1009),
        forms: Form::PRINTED,
        req: Req::Mandatory,
        multi: false,
    },
    // Место расчетов
    FieldSpec {
        tag: Some(1187),
        forms: Form::PRINTED,
        req: Req::Mandatory,
        multi: false,
    },
    // Дата, время
    FieldSpec {
        tag: Some(1012),
        forms: Form::BOTH,
        req: Req::Mandatory,
        multi: false,
    },
    // Номер смены
    FieldSpec {
        tag: Some(1038),
        forms: Form::BOTH,
        req: Req::SometimesRequired,
        multi: false,
    },
    // Признак автономного режима
    FieldSpec {
        tag: Some(1002),
        forms: Form::BOTH,
        req: Req::SometimesRequired,
        multi: false,
    },
    // Номер первого непереданного документа
    FieldSpec {
        tag: Some(1116),
        forms: Form::BOTH,
        req: Req::SometimesRequired,
        multi: false,
    },
    // Количество непереданных ФД
    FieldSpec {
        tag: Some(1097),
        forms: Form::BOTH,
        req: Req::SometimesRequired,
        multi: false,
    },
    // Количество непереданных уведомлений
    FieldSpec {
        tag: Some(2104),
        forms: Form::BOTH,
        req: Req::SometimesRequired,
        multi: false,
    },
    // Дата первого из непереданных ФД
    FieldSpec {
        tag: Some(1098),
        forms: Form::BOTH,
        req: Req::SometimesRequired,
        multi: false,
    },
    // Ресурс ключей ФП
    FieldSpec {
        tag: Some(1213),
        forms: Form::BOTH,
        req: Req::Mandatory,
        multi: false,
    },
    // Регистрационный номер ККТ
    FieldSpec {
        tag: Some(1037),
        forms: Form::BOTH,
        req: Req::Mandatory,
        multi: false,
    },
    // Счетчики итогов ФН
    FieldSpec {
        tag: Some(1157),
        forms: Form::BOTH,
        req: Req::Mandatory,
        multi: false,
    },
    // Счетчики итогов непереданных ФД
    FieldSpec {
        tag: Some(1158),
        forms: Form::BOTH,
        req: Req::SometimesRequired,
        multi: false,
    },
    // Дополнительный реквизит ОТР
    FieldSpec {
        tag: Some(1280),
        forms: Form::ELECTRONIC,
        req: Req::SometimesRequired,
        multi: false,
    },
    // Дополнительные данные ОТР
    FieldSpec {
        tag: Some(1281),
        forms: Form::ELECTRONIC,
        req: Req::SometimesRequired,
        multi: false,
    },
    // Номер ФД
    FieldSpec {
        tag: Some(1040),
        forms: Form::BOTH,
        req: Req::Mandatory,
        multi: false,
    },
    // Номер ФН
    FieldSpec {
        tag: Some(1041),
        forms: Form::BOTH,
        req: Req::Mandatory,
        multi: false,
    },
    // ФПД (1)
    FieldSpec {
        tag: Some(1077),
        forms: Form::BOTH,
        req: Req::Mandatory,
        multi: false,
    },
    // ФПС (4)
    FieldSpec {
        tag: None,
        forms: Form::ELECTRONIC,
        req: Req::Mandatory,
        multi: false,
    },
]);

/// Таблица 96
///
/// Реквизиты, содержащиеся в кассовом чеке (БСО)
pub const RECEIPT_1_2: FieldSet = FieldSet(&[
    // Наименование документа
    FieldSpec {
        tag: Some(1000),
        forms: Form::PRINTED,
        req: Req::Mandatory,
        multi: false,
    },
    // Код формы ФД
    FieldSpec {
        tag: None,
        forms: Form::ELECTRONIC,
        req: Req::Mandatory,
        multi: false,
    },
    // Номер версии ФФД
    FieldSpec {
        tag: Some(1209),
        forms: Form::ELECTRONIC,
        req: Req::Mandatory,
        multi: false,
    },
    // Наименование пользователя
    FieldSpec {
        tag: Some(1048),
        forms: Form::BOTH,
        req: Req::Optional,
        multi: false,
    },
    // ИНН пользователя
    FieldSpec {
        tag: Some(1018),
        forms: Form::BOTH,
        req: Req::Optional,
        multi: false,
    },
    // Сведения о покупателе (клиенте)
    FieldSpec {
        tag: Some(1256),
        forms: Form::BOTH,
        req: Req::Optional,
        multi: false,
    },
    // Номер чека за смену
    FieldSpec {
        tag: Some(1042),
        forms: Form::ELECTRONIC,
        req: Req::Mandatory,
        multi: false,
    },
    // Дата, время
    FieldSpec {
        tag: Some(1012),
        forms: Form::BOTH,
        req: Req::Mandatory,
        multi: false,
    },
    // Номер смены
    FieldSpec {
        tag: Some(1038),
        forms: Form::ELECTRONIC,
        req: Req::Mandatory,
        multi: false,
    },
    // Признак расчета
    FieldSpec {
        tag: Some(1054),
        forms: Form::BOTH,
        req: Req::Mandatory,
        multi: false,
    },
    // Применяемая система налогообложения
    FieldSpec {
        tag: Some(1055),
        forms: Form::BOTH,
        req: Req::Optional,
        multi: false,
    },
    // Кассир
    FieldSpec {
        tag: Some(1021),
        forms: Form::BOTH,
        req: Req::SometimesRequired,
        multi: false,
    },
    // ИНН кассира
    FieldSpec {
        tag: Some(1203),
        forms: Form::ELECTRONIC,
        req: Req::Optional,
        multi: false,
    },
    // Регистрационный номер ККТ
    FieldSpec {
        tag: Some(1037),
        forms: Form::BOTH,
        req: Req::Mandatory,
        multi: false,
    },
    // Номер автомата
    FieldSpec {
        tag: Some(1036),
        forms: Form::BOTH,
        req: Req::SometimesRequired,
        multi: false,
    },
    // Адрес расчетов
    FieldSpec {
        tag: Some(1009),
        forms: Form::BOTH,
        req: Req::Mandatory,
        multi: false,
    },
    // Место расчетов
    FieldSpec {
        tag: Some(1187),
        forms: Form::BOTH,
        req: Req::Mandatory,
        multi: false,
    },
    // Телефон или электронный адрес покупателя
    FieldSpec {
        tag: Some(1008),
        forms: Form::BOTH,
        req: Req::Optional,
        multi: false,
    },
    // Предмет расчета
    FieldSpec {
        tag: Some(1059),
        forms: Form::BOTH,
        req: Req::Mandatory,
        multi: true,
    },
    // Сумма расчета, указанного в чеке (БСО)
    FieldSpec {
        tag: Some(1020),
        forms: Form::BOTH,
        req: Req::Mandatory,
        multi: false,
    },
    // Сумма по чеку (БСО) наличными
    FieldSpec {
        tag: Some(1031),
        forms: Form::BOTH,
        req: Req::Optional,
        multi: false,
    },
    // Сумма по чеку (БСО) безналичными
    FieldSpec {
        tag: Some(1081),
        forms: Form::BOTH,
        req: Req::Optional,
        multi: false,
    },
    // Сумма по чеку (БСО) предоплатой (зачетом аванса и (или) предыдущих платежей)
    FieldSpec {
        tag: Some(1215),
        forms: Form::BOTH,
        req: Req::Optional,
        multi: false,
    },
    // Сумма по чеку (БСО) постоплатой (в кредит)
    FieldSpec {
        tag: Some(1216),
        forms: Form::BOTH,
        req: Req::Optional,
        multi: false,
    },
    // Сумма по чеку (БСО) встречным предоставлением
    FieldSpec {
        tag: Some(1217),
        forms: Form::BOTH,
        req: Req::Optional,
        multi: false,
    },
    // Сумма НДС чека по ставке 20%
    FieldSpec {
        tag: Some(1102),
        forms: Form::BOTH,
        req: Req::SometimesRequired,
        multi: false,
    },
    // Сумма НДС чека по ставке 10%
    FieldSpec {
        tag: Some(1103),
        forms: Form::BOTH,
        req: Req::SometimesRequired,
        multi: false,
    },
    // Сумма расчета по чеку с НДС по ставке 0%
    FieldSpec {
        tag: Some(1104),
        forms: Form::BOTH,
        req: Req::SometimesRequired,
        multi: false,
    },
    // Сумма расчета по чеку без НДС
    FieldSpec {
        tag: Some(1105),
        forms: Form::BOTH,
        req: Req::SometimesRequired,
        multi: false,
    },
    // Сумма НДС чека по расч. ставке 20/120
    FieldSpec {
        tag: Some(1106),
        forms: Form::BOTH,
        req: Req::SometimesRequired,
        multi: false,
    },
    // Сумма НДС чека по расч. ставке 10/110
    FieldSpec {
        tag: Some(1107),
        forms: Form::BOTH,
        req: Req::SometimesRequired,
        multi: false,
    },
    // Признак ККТ для расчетов только в Интернет
    FieldSpec {
        tag: Some(1108),
        forms: Form::ELECTRONIC,
        req: Req::SometimesRequired,
        multi: false,
    },
    // Адрес электронной почты отправителя чека
    FieldSpec {
        tag: Some(1117),
        forms: Form::ELECTRONIC,
        req: Req::SometimesRequired,
        multi: false,
    },
    // Результаты проверки маркированных товаров
    FieldSpec {
        tag: Some(2107),
        forms: Form::BOTH,
        req: Req::SometimesRequired,
        multi: true,
    },
    // Адрес сайта ФНС
    FieldSpec {
        tag: Some(1060),
        forms: Form::BOTH,
        req: Req::Optional,
        multi: false,
    },
    // Операционный реквизит чека
    FieldSpec {
        tag: Some(1270),
        forms: Form::ELECTRONIC,
        req: Req::SometimesRequired,
        multi: false,
    },
    // Дополнительный реквизит чека (БСО)
    FieldSpec {
        tag: Some(1192),
        forms: Form::BOTH,
        req: Req::Optional,
        multi: false,
    },
    // Дополнительный реквизит пользователя
    FieldSpec {
        tag: Some(1084),
        forms: Form::BOTH,
        req: Req::Optional,
        multi: false,
    },
    // Отраслевой реквизит чека
    FieldSpec {
        tag: Some(1261),
        forms: Form::ELECTRONIC,
        req: Req::SometimesRequired,
        multi: true,
    },
    // Номер ФД
    FieldSpec {
        tag: Some(1040),
        forms: Form::BOTH,
        req: Req::Mandatory,
        multi: false,
    },
    // Номер ФН
    FieldSpec {
        tag: Some(1041),
        forms: Form::BOTH,
        req: Req::Mandatory,
        multi: false,
    },
    // ФПД (1)
    FieldSpec {
        tag: Some(1077),
        forms: Form::BOTH,
        req: Req::Mandatory,
        multi: false,
    },
    // ФПС (4)
    FieldSpec {
        tag: None,
        forms: Form::ELECTRONIC,
        req: Req::Mandatory,
        multi: false,
    },
    // ФПА (5)
    FieldSpec {
        tag: None,
        forms: Form::ELECTRONIC,
        req: Req::SometimesRequired,
        multi: false,
    },
    // QR-код
    FieldSpec {
        tag: Some(1196),
        forms: Form::PRINTED,
        req: Req::Mandatory,
        multi: false,
    },
]);

/// Таблица 97
///
/// Структура данных реквизита «предмет расчета» (тег 1059)
pub const PAYMENT_ITEM_1_2: FieldSet = FieldSet(&[
    // Признак способа расчета
    FieldSpec {
        tag: Some(1214),
        forms: Form::BOTH,
        req: Req::Optional,
        multi: false,
    },
    // Признак предмета расчета
    FieldSpec {
        tag: Some(1212),
        forms: Form::BOTH,
        req: Req::Optional,
        multi: false,
    },
    // Признак агента по предмету расчета
    FieldSpec {
        tag: Some(1222),
        forms: Form::BOTH,
        req: Req::Optional,
        multi: false,
    },
    // Данные агента
    FieldSpec {
        tag: Some(1223),
        forms: Form::BOTH,
        req: Req::Optional,
        multi: false,
    },
    // Данные поставщика
    FieldSpec {
        tag: Some(1224),
        forms: Form::BOTH,
        req: Req::Optional,
        multi: false,
    },
    // ИНН поставщика
    FieldSpec {
        tag: Some(1226),
        forms: Form::BOTH,
        req: Req::Optional,
        multi: false,
    },
    // Наименование предмета расчета
    FieldSpec {
        tag: Some(1030),
        forms: Form::BOTH,
        req: Req::SometimesRequired,
        multi: false,
    },
    // Мера количества предмета расчета
    FieldSpec {
        tag: Some(2108),
        forms: Form::BOTH,
        req: Req::SometimesRequired,
        multi: false,
    },
    // Код товара
    FieldSpec {
        tag: Some(1163),
        forms: Form::BOTH,
        req: Req::SometimesRequired,
        multi: false,
    },
    // Контрольный код КМ
    FieldSpec {
        tag: Some(2115),
        forms: Form::PRINTED,
        req: Req::SometimesRequired,
        multi: false,
    },
    // Режим обработки кода маркировки
    FieldSpec {
        tag: Some(2102),
        forms: Form::ELECTRONIC,
        req: Req::Optional,
        multi: false,
    },
    // Результат проверки сведений о товаре
    FieldSpec {
        tag: Some(2106),
        forms: Form::BOTH,
        req: Req::Optional,
        multi: false,
    },
    // Код страны происхождения товара
    FieldSpec {
        tag: Some(1230),
        forms: Form::BOTH,
        req: Req::Optional,
        multi: false,
    },
    // Номер декларации на товар
    FieldSpec {
        tag: Some(1231),
        forms: Form::BOTH,
        req: Req::Optional,
        multi: false,
    },
    // Цена за единицу предмета расчета с учетом скидок и наценок
    FieldSpec {
        tag: Some(1079),
        forms: Form::BOTH,
        req: Req::Mandatory,
        multi: false,
    },
    // Размер НДС за единицу предмета расчета
    FieldSpec {
        tag: Some(1198),
        forms: Form::BOTH,
        req: Req::Optional,
        multi: false,
    },
    // Количество предмета расчета
    FieldSpec {
        tag: Some(1023),
        forms: Form::BOTH,
        req: Req::Mandatory,
        multi: false,
    },
    // Дробное количество маркированного товара
    FieldSpec {
        tag: Some(1291),
        forms: Form::BOTH,
        req: Req::SometimesRequired,
        multi: false,
    },
    // Акциз
    FieldSpec {
        tag: Some(1229),
        forms: Form::BOTH,
        req: Req::Optional,
        multi: false,
    },
    // Ставка НДС
    FieldSpec {
        tag: Some(1199),
        forms: Form::BOTH,
        req: Req::SometimesRequired,
        multi: false,
    },
    // Сумма НДС за предмет расчета
    FieldSpec {
        tag: Some(1200),
        forms: Form::BOTH,
        req: Req::Optional,
        multi: false,
    },
    // Стоимость предмета расчета с учетом скидок и наценок
    FieldSpec {
        tag: Some(1043),
        forms: Form::BOTH,
        req: Req::Mandatory,
        multi: false,
    },
    // Отраслевой реквизит предмета расчета
    FieldSpec {
        tag: Some(1260),
        forms: Form::ELECTRONIC,
        req: Req::SometimesRequired,
        multi: true,
    },
    // Дополнительный реквизит предмета расчета
    FieldSpec {
        tag: Some(1191),
        forms: Form::BOTH,
        req: Req::Optional,
        multi: false,
    },
]);

/// Таблица 98
///
/// Значения реквизита «данные агента» (тег 1223)
pub const AGENT_INFO_1_2: FieldSet = FieldSet(&[
    // Телефон оператора перевода
    FieldSpec {
        tag: Some(1075),
        forms: Form::BOTH,
        req: Req::SometimesRequired,
        multi: true,
    },
    // Операция банковского платежного агента
    FieldSpec {
        tag: Some(1044),
        forms: Form::BOTH,
        req: Req::SometimesRequired,
        multi: false,
    },
    // Телефон платежного агента
    FieldSpec {
        tag: Some(1073),
        forms: Form::BOTH,
        req: Req::SometimesRequired,
        multi: true,
    },
    // Телефон оператора по приему платежей
    FieldSpec {
        tag: Some(1074),
        forms: Form::BOTH,
        req: Req::SometimesRequired,
        multi: true,
    },
    // Наименование оператора перевода
    FieldSpec {
        tag: Some(1026),
        forms: Form::BOTH,
        req: Req::SometimesRequired,
        multi: false,
    },
    // Адрес оператора перевода
    FieldSpec {
        tag: Some(1005),
        forms: Form::BOTH,
        req: Req::SometimesRequired,
        multi: false,
    },
    // ИНН оператора перевода
    FieldSpec {
        tag: Some(1016),
        forms: Form::BOTH,
        req: Req::SometimesRequired,
        multi: false,
    },
]);

/// Таблица 99
///
/// Значения реквизита «данные поставщика» (тег 1224)
pub const SUPPLIER_INFO_1_2: FieldSet = FieldSet(&[
    // Телефон поставщика
    FieldSpec {
        tag: Some(1171),
        forms: Form::BOTH,
        req: Req::SometimesRequired,
        multi: true,
    },
    // Наименование поставщика
    FieldSpec {
        tag: Some(1225),
        forms: Form::BOTH,
        req: Req::SometimesRequired,
        multi: false,
    },
]);

/// Таблица 102
///
/// Структура реквизитов «отраслевой реквизит предмета расчета» (тег 1260) и «отраслевой реквизит чека» (тег 1261)
pub const INDUSTRY_INFO: FieldSet = FieldSet(&[
    // Идентификатор ФОИВ
    FieldSpec {
        tag: Some(1262),
        forms: Form::ELECTRONIC,
        req: Req::Optional,
        multi: false,
    },
    // Дата документа основания
    FieldSpec {
        tag: Some(1263),
        forms: Form::ELECTRONIC,
        req: Req::Optional,
        multi: false,
    },
    // Номер документа основания
    FieldSpec {
        tag: Some(1264),
        forms: Form::ELECTRONIC,
        req: Req::Optional,
        multi: false,
    },
    // Значение отраслевого реквизита
    FieldSpec {
        tag: Some(1265),
        forms: Form::ELECTRONIC,
        req: Req::Optional,
        multi: false,
    },
]);

/// Таблица 115
///
/// Структура данных реквизита «сведения о покупателе (клиенте)» (тег 1256)
pub const CLIENT_INFO: FieldSet = FieldSet(&[
    // Покупатель (клиент)
    FieldSpec {
        tag: Some(1227),
        forms: Form::BOTH,
        req: Req::SometimesRequired,
        multi: false,
    },
    // ИНН покупателя (клиента)
    FieldSpec {
        tag: Some(1228),
        forms: Form::ELECTRONIC,
        req: Req::SometimesRequired,
        multi: false,
    },
    // Дата рождения покупателя (клиента)
    FieldSpec {
        tag: Some(1243),
        forms: Form::ELECTRONIC,
        req: Req::SometimesRequired,
        multi: false,
    },
    // Гражданство
    FieldSpec {
        tag: Some(1244),
        forms: Form::ELECTRONIC,
        req: Req::Optional,
        multi: false,
    },
    // Код вида документа, удостоверяющего личность
    FieldSpec {
        tag: Some(1245),
        forms: Form::ELECTRONIC,
        req: Req::SometimesRequired,
        multi: false,
    },
    // Данные документа, удостоверяющего личность
    FieldSpec {
        tag: Some(1246),
        forms: Form::ELECTRONIC,
        req: Req::SometimesRequired,
        multi: false,
    },
    // Адрес покупателя (клиента)
    FieldSpec {
        tag: Some(1254),
        forms: Form::BOTH,
        req: Req::Optional,
        multi: false,
    },
]);

/// Таблица 117
///
/// Структура данных реквизита «код товара» (тег 1163)
pub const PRODUCT_CODE: FieldSet = FieldSet(&[
    // Нераспознанный код товара
    FieldSpec {
        tag: Some(1300),
        forms: Form::ELECTRONIC,
        req: Req::SometimesRequired,
        multi: false,
    },
    // КТ EAN-8
    FieldSpec {
        tag: Some(1301),
        forms: Form::ELECTRONIC,
        req: Req::SometimesRequired,
        multi: false,
    },
    // КТ EAN-13
    FieldSpec {
        tag: Some(1302),
        forms: Form::ELECTRONIC,
        req: Req::SometimesRequired,
        multi: false,
    },
    // КТ ITF-14
    FieldSpec {
        tag: Some(1303),
        forms: Form::ELECTRONIC,
        req: Req::SometimesRequired,
        multi: false,
    },
    // КТ GS1.0
    FieldSpec {
        tag: Some(1304),
        forms: Form::ELECTRONIC,
        req: Req::SometimesRequired,
        multi: false,
    },
    // КТ GS1.М
    FieldSpec {
        tag: Some(1305),
        forms: Form::ELECTRONIC,
        req: Req::SometimesRequired,
        multi: false,
    },
    // КТ КМК
    FieldSpec {
        tag: Some(1306),
        forms: Form::ELECTRONIC,
        req: Req::SometimesRequired,
        multi: false,
    },
    // КТ МИ
    FieldSpec {
        tag: Some(1307),
        forms: Form::ELECTRONIC,
        req: Req::SometimesRequired,
        multi: false,
    },
    // КТ ЕГАИС-2.0
    FieldSpec {
        tag: Some(1308),
        forms: Form::ELECTRONIC,
        req: Req::SometimesRequired,
        multi: false,
    },
    // КТ ЕГАИС-3.0
    FieldSpec {
        tag: Some(1309),
        forms: Form::ELECTRONIC,
        req: Req::SometimesRequired,
        multi: false,
    },
    // КТ Ф.1
    FieldSpec {
        tag: Some(1320),
        forms: Form::ELECTRONIC,
        req: Req::SometimesRequired,
        multi: false,
    },
    // КТ Ф.2
    FieldSpec {
        tag: Some(1321),
        forms: Form::ELECTRONIC,
        req: Req::SometimesRequired,
        multi: false,
    },
    // КТ Ф.3
    FieldSpec {
        tag: Some(1322),
        forms: Form::BOTH,
        req: Req::SometimesRequired,
        multi: false,
    },
    // КТ Ф.4
    FieldSpec {
        tag: Some(1323),
        forms: Form::ELECTRONIC,
        req: Req::SometimesRequired,
        multi: false,
    },
    // КТ Ф.5
    FieldSpec {
        tag: Some(1324),
        forms: Form::ELECTRONIC,
        req: Req::SometimesRequired,
        multi: false,
    },
    // КТ Ф.6
    FieldSpec {
        tag: Some(1325),
        forms: Form::BOTH,
        req: Req::SometimesRequired,
        multi: false,
    },
]);

/// Таблица 119
///
/// Структура данных реквизита «операционный реквизит чека» (тег 1270)
pub const OPERATION_INFO: FieldSet = FieldSet(&[
    // Дата, время операции
    FieldSpec {
        tag: Some(1273),
        forms: Form::ELECTRONIC,
        req: Req::SometimesRequired,
        multi: false,
    },
    // Идентификатор операции
    FieldSpec {
        tag: Some(1271),
        forms: Form::ELECTRONIC,
        req: Req::SometimesRequired,
        multi: false,
    },
    // Данные операции
    FieldSpec {
        tag: Some(1272),
        forms: Form::ELECTRONIC,
        req: Req::SometimesRequired,
        multi: false,
    },
]);

/// Таблица 120
///
/// Реквизиты, содержащиеся в кассовом чеке коррекции (БСО коррекции)
pub const CORRECTION_RECEIPT_1_2: FieldSet = FieldSet(&[
    // Наименование документа
    FieldSpec {
        tag: Some(1000),
        forms: Form::PRINTED,
        req: Req::Mandatory,
        multi: false,
    },
    // Код формы ФД
    FieldSpec {
        tag: None,
        forms: Form::ELECTRONIC,
        req: Req::Mandatory,
        multi: false,
    },
    // Номер версии ФФД
    FieldSpec {
        tag: Some(1209),
        forms: Form::ELECTRONIC,
        req: Req::Mandatory,
        multi: false,
    },
    // Наименование пользователя
    FieldSpec {
        tag: Some(1048),
        forms: Form::BOTH,
        req: Req::Optional,
        multi: false,
    },
    // ИНН пользователя
    FieldSpec {
        tag: Some(1018),
        forms: Form::BOTH,
        req: Req::Optional,
        multi: false,
    },
    // Сведения о покупателе (клиенте)
    FieldSpec {
        tag: Some(1256),
        forms: Form::BOTH,
        req: Req::Optional,
        multi: false,
    },
    // Номер чека за смену
    FieldSpec {
        tag: Some(1042),
        forms: Form::ELECTRONIC,
        req: Req::Mandatory,
        multi: false,
    },
    // Дата, время
    FieldSpec {
        tag: Some(1012),
        forms: Form::BOTH,
        req: Req::Mandatory,
        multi: false,
    },
    // Номер смены
    FieldSpec {
        tag: Some(1038),
        forms: Form::ELECTRONIC,
        req: Req::Mandatory,
        multi: false,
    },
    // Признак расчета
    FieldSpec {
        tag: Some(1054),
        forms: Form::BOTH,
        req: Req::Mandatory,
        multi: false,
    },
    // Применяемая система налогообложения
    FieldSpec {
        tag: Some(1055),
        forms: Form::BOTH,
        req: Req::Optional,
        multi: false,
    },
    // Кассир
    FieldSpec {
        tag: Some(1021),
        forms: Form::BOTH,
        req: Req::SometimesRequired,
        multi: false,
    },
    // ИНН кассира
    FieldSpec {
        tag: Some(1203),
        forms: Form::ELECTRONIC,
        req: Req::Optional,
        multi: false,
    },
    // Регистрационный номер ККТ
    FieldSpec {
        tag: Some(1037),
        forms: Form::BOTH,
        req: Req::Mandatory,
        multi: false,
    },
    // Тип коррекции
    FieldSpec {
        tag: Some(1173),
        forms: Form::BOTH,
        req: Req::Optional,
        multi: false,
    },
    // Основание для коррекции
    FieldSpec {
        tag: Some(1174),
        forms: Form::BOTH,
        req: Req::Mandatory,
        multi: false,
    },
    // Номер автомата
    FieldSpec {
        tag: Some(1036),
        forms: Form::BOTH,
        req: Req::SometimesRequired,
        multi: false,
    },
    // Адрес расчетов
    FieldSpec {
        tag: Some(1009),
        forms: Form::BOTH,
        req: Req::Mandatory,
        multi: false,
    },
    // Место расчетов
    FieldSpec {
        tag: Some(1187),
        forms: Form::BOTH,
        req: Req::Mandatory,
        multi: false,
    },
    // Телефон или электронный адрес покупателя
    FieldSpec {
        tag: Some(1008),
        forms: Form::BOTH,
        req: Req::Optional,
        multi: false,
    },
    // Предмет расчета
    FieldSpec {
        tag: Some(1059),
        forms: Form::BOTH,
        req: Req::Mandatory,
        multi: true,
    },
    // Сумма расчета, указанного в чеке (БСО)
    FieldSpec {
        tag: Some(1020),
        forms: Form::BOTH,
        req: Req::Mandatory,
        multi: false,
    },
    // Сумма по чеку (БСО) наличными
    FieldSpec {
        tag: Some(1031),
        forms: Form::BOTH,
        req: Req::Optional,
        multi: false,
    },
    // Сумма по чеку (БСО) безналичными
    FieldSpec {
        tag: Some(1081),
        forms: Form::BOTH,
        req: Req::Optional,
        multi: false,
    },
    // Сумма по чеку (БСО) предоплатой (зачетом аванса и (или) предыдущих платежей)
    FieldSpec {
        tag: Some(1215),
        forms: Form::BOTH,
        req: Req::Optional,
        multi: false,
    },
    // Сумма по чеку (БСО) постоплатой (в кредит)
    FieldSpec {
        tag: Some(1216),
        forms: Form::BOTH,
        req: Req::Optional,
        multi: false,
    },
    // Сумма по чеку (БСО) встречным предоставлением
    FieldSpec {
        tag: Some(1217),
        forms: Form::BOTH,
        req: Req::Optional,
        multi: false,
    },
    // Сумма НДС чека по ставке 20%
    FieldSpec {
        tag: Some(1102),
        forms: Form::BOTH,
        req: Req::SometimesRequired,
        multi: false,
    },
    // Сумма НДС чека по ставке 10%
    FieldSpec {
        tag: Some(1103),
        forms: Form::BOTH,
        req: Req::SometimesRequired,
        multi: false,
    },
    // Сумма расчета по чеку с НДС по ставке 0%
    FieldSpec {
        tag: Some(1104),
        forms: Form::BOTH,
        req: Req::SometimesRequired,
        multi: false,
    },
    // Сумма расчета по чеку без НДС
    FieldSpec {
        tag: Some(1105),
        forms: Form::BOTH,
        req: Req::SometimesRequired,
        multi: false,
    },
    // Сумма НДС чека по расч. ставке 20/120
    FieldSpec {
        tag: Some(1106),
        forms: Form::BOTH,
        req: Req::SometimesRequired,
        multi: false,
    },
    // Сумма НДС чека по расч. ставке 10/110
    FieldSpec {
        tag: Some(1107),
        forms: Form::BOTH,
        req: Req::SometimesRequired,
        multi: false,
    },
    // Признак ККТ для расчетов только в Интернет
    FieldSpec {
        tag: Some(1108),
        forms: Form::ELECTRONIC,
        req: Req::SometimesRequired,
        multi: false,
    },
    // Адрес электронной почты отправителя чека
    FieldSpec {
        tag: Some(1117),
        forms: Form::ELECTRONIC,
        req: Req::SometimesRequired,
        multi: false,
    },
    // Результаты проверки маркированных товаров
    FieldSpec {
        tag: Some(2107),
        forms: Form::BOTH,
        req: Req::Optional,
        multi: true,
    },
    // Адрес сайта ФНС
    FieldSpec {
        tag: Some(1060),
        forms: Form::ELECTRONIC,
        req: Req::Optional,
        multi: false,
    },
    // Операционный реквизит чека
    FieldSpec {
        tag: Some(1270),
        forms: Form::ELECTRONIC,
        req: Req::SometimesRequired,
        multi: false,
    },
    // Дополнительный реквизит чека (БСО)
    FieldSpec {
        tag: Some(1192),
        forms: Form::BOTH,
        req: Req::Optional,
        multi: false,
    },
    // Дополнительный реквизит пользователя
    FieldSpec {
        tag: Some(1084),
        forms: Form::BOTH,
        req: Req::Optional,
        multi: false,
    },
    // Отраслевой реквизит чека
    FieldSpec {
        tag: Some(1261),
        forms: Form::ELECTRONIC,
        req: Req::SometimesRequired,
        multi: true,
    },
    // Номер ФД
    FieldSpec {
        tag: Some(1040),
        forms: Form::BOTH,
        req: Req::Mandatory,
        multi: false,
    },
    // Номер ФН
    FieldSpec {
        tag: Some(1041),
        forms: Form::BOTH,
        req: Req::Mandatory,
        multi: false,
    },
    // ФПД (1)
    FieldSpec {
        tag: Some(1077),
        forms: Form::BOTH,
        req: Req::Mandatory,
        multi: false,
    },
    // ФПС (4)
    FieldSpec {
        tag: None,
        forms: Form::ELECTRONIC,
        req: Req::Mandatory,
        multi: false,
    },
    // ФПА (5)
    FieldSpec {
        tag: None,
        forms: Form::ELECTRONIC,
        req: Req::SometimesRequired,
        multi: false,
    },
    // QR-код
    FieldSpec {
        tag: Some(1196),
        forms: Form::PRINTED,
        req: Req::Optional,
        multi: false,
    },
]);

/// Таблица 123
///
/// Реквизиты, содержащиеся в отчете о закрытии смены
pub const SHIFT_END_REPORT_1_2: FieldSet = FieldSet(&[
    // Наименование документа
    FieldSpec {
        tag: Some(1000),
        forms: Form::PRINTED,
        req: Req::Mandatory,
        multi: false,
    },
    // Код формы ФД
    FieldSpec {
        tag: None,
        forms: Form::ELECTRONIC,
        req: Req::Mandatory,
        multi: false,
    },
    // Номер версии ФФД
    FieldSpec {
        tag: Some(1209),
        forms: Form::ELECTRONIC,
        req: Req::Mandatory,
        multi: false,
    },
    // Наименование пользователя
    FieldSpec {
        tag: Some(1048),
        forms: Form::PRINTED,
        req: Req::Mandatory,
        multi: false,
    },
    // ИНН пользователя
    FieldSpec {
        tag: Some(1018),
        forms: Form::BOTH,
        req: Req::Mandatory,
        multi: false,
    },
    // Кассир
    FieldSpec {
        tag: Some(1021),
        forms: Form::BOTH,
        req: Req::SometimesRequired,
        multi: false,
    },
    // ИНН кассира
    FieldSpec {
        tag: Some(1203),
        forms: Form::ELECTRONIC,
        req: Req::Optional,
        multi: false,
    },
    // Адрес расчетов
    FieldSpec {
        tag: Some(1009),
        forms: Form::PRINTED,
        req: Req::Mandatory,
        multi: false,
    },
    // Место расчетов
    FieldSpec {
        tag: Some(1187),
        forms: Form::PRINTED,
        req: Req::Mandatory,
        multi: false,
    },
    // Дата, время
    FieldSpec {
        tag: Some(1012),
        forms: Form::BOTH,
        req: Req::Mandatory,
        multi: false,
    },
    // Номер смены
    FieldSpec {
        tag: Some(1038),
        forms: Form::BOTH,
        req: Req::Mandatory,
        multi: false,
    },
    // Регистрационный номер ККТ
    FieldSpec {
        tag: Some(1037),
        forms: Form::BOTH,
        req: Req::Mandatory,
        multi: false,
    },
    // Количество кассовых чеков (БСО) за смену
    FieldSpec {
        tag: Some(1118),
        forms: Form::BOTH,
        req: Req::Mandatory,
        multi: false,
    },
    // Общее количество ФД за смену
    FieldSpec {
        tag: Some(1111),
        forms: Form::BOTH,
        req: Req::Mandatory,
        multi: false,
    },
    // Количество непереданных ФД
    FieldSpec {
        tag: Some(1097),
        forms: Form::BOTH,
        req: Req::SometimesRequired,
        multi: false,
    },
    // Количество непереданных уведомлений
    FieldSpec {
        tag: Some(2104),
        forms: Form::BOTH,
        req: Req::SometimesRequired,
        multi: false,
    },
    // Дата первого из непереданных ФД
    FieldSpec {
        tag: Some(1098),
        forms: Form::BOTH,
        req: Req::SometimesRequired,
        multi: false,
    },
    // Признак превышения времени ожидания ответа ОФД
    FieldSpec {
        tag: Some(1053),
        forms: Form::BOTH,
        req: Req::SometimesRequired,
        multi: false,
    },
    // Признак необходимости срочной замены ФН
    FieldSpec {
        tag: Some(1051),
        forms: Form::BOTH,
        req: Req::SometimesRequired,
        multi: false,
    },
    // Признак переполнения памяти ФН
    FieldSpec {
        tag: Some(1052),
        forms: Form::BOTH,
        req: Req::SometimesRequired,
        multi: false,
    },
    // Признак исчерпания ресурса ФН
    FieldSpec {
        tag: Some(1050),
        forms: Form::BOTH,
        req: Req::SometimesRequired,
        multi: false,
    },
    // Сообщение оператора
    FieldSpec {
        tag: Some(1206),
        forms: Form::BOTH,
        req: Req::SometimesRequired,
        multi: false,
    },
    // Признак некорректных кодов маркировки
    FieldSpec {
        tag: Some(2112),
        forms: Form::BOTH,
        req: Req::SometimesRequired,
        multi: false,
    },
    // Признак некорректных запросов и уведомлений
    FieldSpec {
        tag: Some(2113),
        forms: Form::BOTH,
        req: Req::SometimesRequired,
        multi: false,
    },
    // Счетчики итогов смены
    FieldSpec {
        tag: Some(1194),
        forms: Form::BOTH,
        req: Req::Mandatory,
        multi: false,
    },
    // Счетчики итогов ФН
    FieldSpec {
        tag: Some(1157),
        forms: Form::BOTH,
        req: Req::Mandatory,
        multi: false,
    },
    // Ресурс ключей ФП
    FieldSpec {
        tag: Some(1213),
        forms: Form::BOTH,
        req: Req::Mandatory,
        multi: false,
    },
    // Дополнительный реквизит ОЗС
    FieldSpec {
        tag: Some(1278),
        forms: Form::ELECTRONIC,
        req: Req::SometimesRequired,
        multi: false,
    },
    // Дополнительные данные ОЗС
    FieldSpec {
        tag: Some(1279),
        forms: Form::ELECTRONIC,
        req: Req::SometimesRequired,
        multi: false,
    },
    // Номер ФД
    FieldSpec {
        tag: Some(1040),
        forms: Form::BOTH,
        req: Req::Mandatory,
        multi: false,
    },
    // Номер ФН
    FieldSpec {
        tag: Some(1041),
        forms: Form::BOTH,
        req: Req::Mandatory,
        multi: false,
    },
    // ФПД (1)
    FieldSpec {
        tag: Some(1077),
        forms: Form::BOTH,
        req: Req::Mandatory,
        multi: false,
    },
    // ФПС (4)
    FieldSpec {
        tag: None,
        forms: Form::ELECTRONIC,
        req: Req::Mandatory,
        multi: false,
    },
]);

/// Таблица 124
///
/// Реквизиты, содержащиеся в отчете о закрытии фискального накопителя
pub const FN_CLOSE_REPORT_1_2: FieldSet = FieldSet(&[
    // Наименование документа
    FieldSpec {
        tag: Some(1000),
        forms: Form::PRINTED,
        req: Req::Mandatory,
        multi: false,
    },
    // Код формы ФД
    FieldSpec {
        tag: None,
        forms: Form::ELECTRONIC,
        req: Req::Mandatory,
        multi: false,
    },
    // Номер версии ФФД
    FieldSpec {
        tag: Some(1209),
        forms: Form::ELECTRONIC,
        req: Req::Mandatory,
        multi: false,
    },
    // Наименование пользователя
    FieldSpec {
        tag: Some(1048),
        forms: Form::BOTH,
        req: Req::Mandatory,
        multi: false,
    },
    // ИНН пользователя
    FieldSpec {
        tag: Some(1018),
        forms: Form::BOTH,
        req: Req::Mandatory,
        multi: false,
    },
    // Кассир
    FieldSpec {
        tag: Some(1021),
        forms: Form::BOTH,
        req: Req::SometimesRequired,
        multi: false,
    },
    // ИНН кассира
    FieldSpec {
        tag: Some(1203),
        forms: Form::ELECTRONIC,
        req: Req::Optional,
        multi: false,
    },
    // Адрес расчетов
    FieldSpec {
        tag: Some(1009),
        forms: Form::BOTH,
        req: Req::Mandatory,
        multi: false,
    },
    // Место расчетов
    FieldSpec {
        tag: Some(1187),
        forms: Form::BOTH,
        req: Req::Mandatory,
        multi: false,
    },
    // Дата, время
    FieldSpec {
        tag: Some(1012),
        forms: Form::BOTH,
        req: Req::Mandatory,
        multi: false,
    },
    // Номер смены
    FieldSpec {
        tag: Some(1038),
        forms: Form::BOTH,
        req: Req::Mandatory,
        multi: false,
    },
    // Регистрационный номер ККТ
    FieldSpec {
        tag: Some(1037),
        forms: Form::BOTH,
        req: Req::Mandatory,
        multi: false,
    },
    // Счетчики итогов ФН
    FieldSpec {
        tag: Some(1157),
        forms: Form::BOTH,
        req: Req::Mandatory,
        multi: false,
    },
    // Дополнительный реквизит ОЗФН
    FieldSpec {
        tag: Some(1282),
        forms: Form::ELECTRONIC,
        req: Req::SometimesRequired,
        multi: false,
    },
    // Дополнительные данные ОЗФН
    FieldSpec {
        tag: Some(1283),
        forms: Form::ELECTRONIC,
        req: Req::SometimesRequired,
        multi: false,
    },
    // Номер ФД
    FieldSpec {
        tag: Some(1040),
        forms: Form::BOTH,
        req: Req::Mandatory,
        multi: false,
    },
    // Номер ФН
    FieldSpec {
        tag: Some(1041),
        forms: Form::BOTH,
        req: Req::Mandatory,
        multi: false,
    },
    // ФПД (1)
    FieldSpec {
        tag: Some(1077),
        forms: Form::BOTH,
        req: Req::Mandatory,
        multi: false,
    },
    // ФПС (4)
    FieldSpec {
        tag: None,
        forms: Form::ELECTRONIC,
        req: Req::Mandatory,
        multi: false,
    },
]);

/// Таблица 127
///
/// Реквизиты, содержащиеся в запросе о коде маркировки
pub const MARKING_CODE_REQUEST: FieldSet = FieldSet(&[
    // Код формы ФД
    FieldSpec {
        tag: None,
        forms: Form::ELECTRONIC,
        req: Req::Mandatory,
        multi: false,
    },
    // Номер запроса
    FieldSpec {
        tag: Some(2001),
        forms: Form::ELECTRONIC,
        req: Req::Mandatory,
        multi: false,
    },
    // ИНН пользователя
    FieldSpec {
        tag: Some(1018),
        forms: Form::ELECTRONIC,
        req: Req::Mandatory,
        multi: false,
    },
    // Дата и время запроса
    FieldSpec {
        tag: Some(2114),
        forms: Form::ELECTRONIC,
        req: Req::Mandatory,
        multi: false,
    },
    // Номер ФН
    FieldSpec {
        tag: Some(1041),
        forms: Form::ELECTRONIC,
        req: Req::Mandatory,
        multi: false,
    },
    // Результат проверки КМ
    FieldSpec {
        tag: Some(2004),
        forms: Form::ELECTRONIC,
        req: Req::Mandatory,
        multi: false,
    },
    // Планируемый статус товара
    FieldSpec {
        tag: Some(2003),
        forms: Form::ELECTRONIC,
        req: Req::Mandatory,
        multi: false,
    },
    // Количество предмета расчета
    FieldSpec {
        tag: Some(1023),
        forms: Form::ELECTRONIC,
        req: Req::SometimesRequired,
        multi: false,
    },
    // Мера количества предмета расчета
    FieldSpec {
        tag: Some(2108),
        forms: Form::ELECTRONIC,
        req: Req::SometimesRequired,
        multi: false,
    },
    // Дробное количество маркированного товара
    FieldSpec {
        tag: Some(1291),
        forms: Form::ELECTRONIC,
        req: Req::SometimesRequired,
        multi: false,
    },
    // Тип кода маркировки
    FieldSpec {
        tag: Some(2100),
        forms: Form::ELECTRONIC,
        req: Req::Mandatory,
        multi: false,
    },
    // Код маркировки
    FieldSpec {
        tag: Some(2000),
        forms: Form::ELECTRONIC,
        req: Req::Mandatory,
        multi: false,
    },
    // Режим обработки кода маркировки
    FieldSpec {
        tag: Some(2102),
        forms: Form::ELECTRONIC,
        req: Req::Mandatory,
        multi: false,
    },
    // ФПУ (6)
    FieldSpec {
        tag: None,
        forms: Form::ELECTRONIC,
        req: Req::Mandatory,
        multi: false,
    },
]);

/// Таблица 128
///
/// Реквизиты, содержащиеся в уведомлении о реализации маркированного товара
pub const MARKED_PRODUCT_SALE_NOTIFICATION: FieldSet = FieldSet(&[
    // Код формы ФД
    FieldSpec {
        tag: None,
        forms: Form::ELECTRONIC,
        req: Req::Mandatory,
        multi: false,
    },
    // Номер уведомления
    FieldSpec {
        tag: Some(2002),
        forms: Form::ELECTRONIC,
        req: Req::Mandatory,
        multi: false,
    },
    // Дата, время
    FieldSpec {
        tag: Some(1012),
        forms: Form::ELECTRONIC,
        req: Req::Mandatory,
        multi: false,
    },
    // Номер ФН
    FieldSpec {
        tag: Some(1041),
        forms: Form::ELECTRONIC,
        req: Req::Mandatory,
        multi: false,
    },
    // ИНН пользователя
    FieldSpec {
        tag: Some(1018),
        forms: Form::ELECTRONIC,
        req: Req::SometimesRequired,
        multi: false,
    },
    // ИНН покупателя (клиента)
    FieldSpec {
        tag: Some(1228),
        forms: Form::ELECTRONIC,
        req: Req::SometimesRequired,
        multi: false,
    },
    // Адрес расчетов
    FieldSpec {
        tag: Some(1009),
        forms: Form::ELECTRONIC,
        req: Req::SometimesRequired,
        multi: false,
    },
    // Применяемая система налогообложения
    FieldSpec {
        tag: Some(1055),
        forms: Form::ELECTRONIC,
        req: Req::Mandatory,
        multi: false,
    },
    // Признак расчета
    FieldSpec {
        tag: Some(1054),
        forms: Form::ELECTRONIC,
        req: Req::Mandatory,
        multi: false,
    },
    // Вид операции
    FieldSpec {
        tag: Some(2116),
        forms: Form::ELECTRONIC,
        req: Req::Mandatory,
        multi: false,
    },
    // Данные о маркированном товаре
    FieldSpec {
        tag: Some(2007),
        forms: Form::ELECTRONIC,
        req: Req::Mandatory,
        multi: true,
    },
    // Отраслевой реквизит чека
    FieldSpec {
        tag: Some(1261),
        forms: Form::ELECTRONIC,
        req: Req::SometimesRequired,
        multi: true,
    },
    // дополнительный реквизит пользователя
    FieldSpec {
        tag: Some(1084),
        forms: Form::ELECTRONIC,
        req: Req::Optional,
        multi: false,
    },
    // ФПУ (6)
    FieldSpec {
        tag: None,
        forms: Form::ELECTRONIC,
        req: Req::Mandatory,
        multi: false,
    },
]);

/// Таблица 130
///
/// Структура данных реквизита «данные о маркированном товаре» (тег 2007)
pub const MARKED_PRODUCT_INFO: FieldSet = FieldSet(&[
    // Код маркировки
    FieldSpec {
        tag: Some(2000),
        forms: Form::ELECTRONIC,
        req: Req::Mandatory,
        multi: false,
    },
    // Тип кода маркировки
    FieldSpec {
        tag: Some(2100),
        forms: Form::ELECTRONIC,
        req: Req::Mandatory,
        multi: false,
    },
    // Идентификатор товара
    FieldSpec {
        tag: Some(2101),
        forms: Form::ELECTRONIC,
        req: Req::Mandatory,
        multi: false,
    },
    // Контрольный код КМ
    FieldSpec {
        tag: Some(2115),
        forms: Form::ELECTRONIC,
        req: Req::Mandatory,
        multi: false,
    },
    // Присвоенный статус товара
    FieldSpec {
        tag: Some(2110),
        forms: Form::ELECTRONIC,
        req: Req::Mandatory,
        multi: false,
    },
    // Результат проверки сведений о товаре
    FieldSpec {
        tag: Some(2106),
        forms: Form::ELECTRONIC,
        req: Req::Mandatory,
        multi: false,
    },
    // Количество предмета расчета
    FieldSpec {
        tag: Some(1023),
        forms: Form::ELECTRONIC,
        req: Req::SometimesRequired,
        multi: false,
    },
    // Мера количества предмета расчета
    FieldSpec {
        tag: Some(2108),
        forms: Form::ELECTRONIC,
        req: Req::SometimesRequired,
        multi: false,
    },
    // Дробное количество маркированного товара
    FieldSpec {
        tag: Some(1291),
        forms: Form::BOTH,
        req: Req::SometimesRequired,
        multi: false,
    },
    // Режим обработки кода маркировки
    FieldSpec {
        tag: Some(2102),
        forms: Form::ELECTRONIC,
        req: Req::Mandatory,
        multi: false,
    },
    // Признак способа расчета
    FieldSpec {
        tag: Some(1214),
        forms: Form::ELECTRONIC,
        req: Req::Mandatory,
        multi: false,
    },
    // Наименование товара
    FieldSpec {
        tag: Some(1030),
        forms: Form::ELECTRONIC,
        req: Req::Mandatory,
        multi: false,
    },
    // ИНН поставщика
    FieldSpec {
        tag: Some(1226),
        forms: Form::ELECTRONIC,
        req: Req::SometimesRequired,
        multi: false,
    },
    // Признак агента по предмету расчета
    FieldSpec {
        tag: Some(1222),
        forms: Form::ELECTRONIC,
        req: Req::SometimesRequired,
        multi: false,
    },
    // Данные агента
    FieldSpec {
        tag: Some(1223),
        forms: Form::ELECTRONIC,
        req: Req::SometimesRequired,
        multi: false,
    },
    // Данные поставщика
    FieldSpec {
        tag: Some(1224),
        forms: Form::ELECTRONIC,
        req: Req::SometimesRequired,
        multi: false,
    },
    // Цена за единицу предмета расчета с учетом скидок и наценок
    FieldSpec {
        tag: Some(1079),
        forms: Form::ELECTRONIC,
        req: Req::Mandatory,
        multi: false,
    },
    // Стоимость предмета расчета с учетом скидок и наценок
    FieldSpec {
        tag: Some(1043),
        forms: Form::ELECTRONIC,
        req: Req::Mandatory,
        multi: false,
    },
    // Ставка НДС
    FieldSpec {
        tag: Some(1199),
        forms: Form::ELECTRONIC,
        req: Req::Mandatory,
        multi: false,
    },
    // Сумма НДС за предмет расчета
    FieldSpec {
        tag: Some(1200),
        forms: Form::ELECTRONIC,
        req: Req::Mandatory,
        multi: false,
    },
    // Отраслевой реквизит предмета расчета
    FieldSpec {
        tag: Some(1260),
        forms: Form::ELECTRONIC,
        req: Req::SometimesRequired,
        multi: true,
    },
]);

/// Таблица 131
///
/// Реквизиты, содержащиеся в ответе на запрос
pub const MARKING_RESPONSE: FieldSet = FieldSet(&[
    // Код формы ФД
    FieldSpec {
        tag: None,
        forms: Form::ELECTRONIC,
        req: Req::Mandatory,
        multi: false,
    },
    // Номер запроса
    FieldSpec {
        tag: Some(2001),
        forms: Form::ELECTRONIC,
        req: Req::Mandatory,
        multi: false,
    },
    // Дата и время запроса
    FieldSpec {
        tag: Some(2114),
        forms: Form::ELECTRONIC,
        req: Req::Mandatory,
        multi: false,
    },
    // Режим обработки кода маркировки
    FieldSpec {
        tag: Some(2102),
        forms: Form::ELECTRONIC,
        req: Req::SometimesRequired,
        multi: false,
    },
    // Тип кода маркировки
    FieldSpec {
        tag: Some(2100),
        forms: Form::ELECTRONIC,
        req: Req::SometimesRequired,
        multi: false,
    },
    // Ответ ОИСМ о статусе товара
    FieldSpec {
        tag: Some(2109),
        forms: Form::ELECTRONIC,
        req: Req::SometimesRequired,
        multi: false,
    },
    // Идентификатор товара
    FieldSpec {
        tag: Some(2101),
        forms: Form::ELECTRONIC,
        req: Req::SometimesRequired,
        multi: false,
    },
    // Коды обработки запроса
    FieldSpec {
        tag: Some(2105),
        forms: Form::ELECTRONIC,
        req: Req::Mandatory,
        multi: false,
    },
    // Результаты обработки запроса
    FieldSpec {
        tag: Some(2005),
        forms: Form::ELECTRONIC,
        req: Req::SometimesRequired,
        multi: false,
    },
    // ФПК (7)
    FieldSpec {
        tag: None,
        forms: Form::ELECTRONIC,
        req: Req::Mandatory,
        multi: false,
    },
]);

/// Таблица 133
///
/// Реквизиты, содержащиеся в квитанции на уведомление
pub const NOTIFICATION_RECEIPT: FieldSet = FieldSet(&[
    // Код формы ФД
    FieldSpec {
        tag: None,
        forms: Form::ELECTRONIC,
        req: Req::Mandatory,
        multi: false,
    },
    // Номер уведомления
    FieldSpec {
        tag: Some(2002),
        forms: Form::ELECTRONIC,
        req: Req::Mandatory,
        multi: false,
    },
    // Дата, время
    FieldSpec {
        tag: Some(1012),
        forms: Form::ELECTRONIC,
        req: Req::Mandatory,
        multi: false,
    },
    // Коды обработки уведомления
    FieldSpec {
        tag: Some(2111),
        forms: Form::ELECTRONIC,
        req: Req::Mandatory,
        multi: false,
    },
    // Результаты обработки уведомления
    FieldSpec {
        tag: Some(2006),
        forms: Form::ELECTRONIC,
        req: Req::SometimesRequired,
        multi: false,
    },
    // ФПК (7)
    FieldSpec {
        tag: None,
        forms: Form::ELECTRONIC,
        req: Req::Mandatory,
        multi: false,
    },
]);

/// Таблица 146
///
/// Формат ФД «Кассовый чек (БСО)», передаваемый покупателю (клиенту)
pub const FD_RECEIPT_1_2: FieldSet = FieldSet(&[
    // Наименование документа
    FieldSpec {
        tag: Some(1000),
        forms: Form::NONE,
        req: Req::Mandatory,
        multi: false,
    },
    // Наименование пользователя
    FieldSpec {
        tag: Some(1048),
        forms: Form::NONE,
        req: Req::Mandatory,
        multi: false,
    },
    // ИНН пользователя
    FieldSpec {
        tag: Some(1018),
        forms: Form::NONE,
        req: Req::Mandatory,
        multi: false,
    },
    // Номер чека за смену
    FieldSpec {
        tag: Some(1042),
        forms: Form::NONE,
        req: Req::Mandatory,
        multi: false,
    },
    // Сведения о покупателе (клиенте)
    FieldSpec {
        tag: Some(1256),
        forms: Form::NONE,
        req: Req::SometimesRequired,
        multi: false,
    },
    // Дата, время
    FieldSpec {
        tag: Some(1012),
        forms: Form::NONE,
        req: Req::Mandatory,
        multi: false,
    },
    // Номер смены
    FieldSpec {
        tag: Some(1038),
        forms: Form::NONE,
        req: Req::Mandatory,
        multi: false,
    },
    // Признак расчета
    FieldSpec {
        tag: Some(1054),
        forms: Form::NONE,
        req: Req::Mandatory,
        multi: false,
    },
    // Применяемая система налогообложения
    FieldSpec {
        tag: Some(1055),
        forms: Form::NONE,
        req: Req::Optional,
        multi: false,
    },
    // Кассир
    FieldSpec {
        tag: Some(1021),
        forms: Form::NONE,
        req: Req::SometimesRequired,
        multi: false,
    },
    // Регистрационный номер ККТ
    FieldSpec {
        tag: Some(1037),
        forms: Form::NONE,
        req: Req::Mandatory,
        multi: false,
    },
    // Номер автомата
    FieldSpec {
        tag: Some(1036),
        forms: Form::NONE,
        req: Req::SometimesRequired,
        multi: false,
    },
    // Адрес расчетов
    FieldSpec {
        tag: Some(1009),
        forms: Form::NONE,
        req: Req::Mandatory,
        multi: false,
    },
    // Место расчетов
    FieldSpec {
        tag: Some(1187),
        forms: Form::NONE,
        req: Req::Mandatory,
        multi: false,
    },
    // Телефон или электронный адрес покупателя
    FieldSpec {
        tag: Some(1008),
        forms: Form::NONE,
        req: Req::Optional,
        multi: false,
    },
    // Предмет расчета
    FieldSpec {
        tag: Some(1059),
        forms: Form::NONE,
        req: Req::Optional,
        multi: true,
    },
    // Сумма расчета, указанного в чеке (БСО)
    FieldSpec {
        tag: Some(1020),
        forms: Form::NONE,
        req: Req::Mandatory,
        multi: false,
    },
    // Сумма по чеку (БСО) наличными
    FieldSpec {
        tag: Some(1031),
        forms: Form::NONE,
        req: Req::SometimesRequired,
        multi: false,
    },
    // Сумма по чеку (БСО) безналичными
    FieldSpec {
        tag: Some(1081),
        forms: Form::NONE,
        req: Req::SometimesRequired,
        multi: false,
    },
    // Сумма по чеку (БСО) предоплатой (зачетом аванса и (или) предыдущих платежей)
    FieldSpec {
        tag: Some(1215),
        forms: Form::NONE,
        req: Req::SometimesRequired,
        multi: false,
    },
    // Сумма по чеку (БСО) постоплатой (в кредит)
    FieldSpec {
        tag: Some(1216),
        forms: Form::NONE,
        req: Req::SometimesRequired,
        multi: false,
    },
    // Сумма по чеку (БСО) встречным предоставлением
    FieldSpec {
        tag: Some(1217),
        forms: Form::NONE,
        req: Req::SometimesRequired,
        multi: false,
    },
    // Сумма НДС чека по ставке 20%
    FieldSpec {
        tag: Some(1102),
        forms: Form::NONE,
        req: Req::SometimesRequired,
        multi: false,
    },
    // Сумма НДС чека по ставке 10%
    FieldSpec {
        tag: Some(1103),
        forms: Form::NONE,
        req: Req::SometimesRequired,
        multi: false,
    },
    // Сумма расчета по чеку с НДС по ставке 0%
    FieldSpec {
        tag: Some(1104),
        forms: Form::NONE,
        req: Req::SometimesRequired,
        multi: false,
    },
    // Сумма расчета по чеку без НДС
    FieldSpec {
        tag: Some(1105),
        forms: Form::NONE,
        req: Req::SometimesRequired,
        multi: false,
    },
    // Сумма НДС чека по расч. ставке 20/120
    FieldSpec {
        tag: Some(1106),
        forms: Form::NONE,
        req: Req::SometimesRequired,
        multi: false,
    },
    // Сумма НДС чека по расч. ставке 10/110
    FieldSpec {
        tag: Some(1107),
        forms: Form::NONE,
        req: Req::SometimesRequired,
        multi: false,
    },
    // Адрес электронной почты отправителя чека
    FieldSpec {
        tag: Some(1117),
        forms: Form::NONE,
        req: Req::SometimesRequired,
        multi: false,
    },
    // Результаты проверки маркированных товаров
    FieldSpec {
        tag: Some(2107),
        forms: Form::NONE,
        req: Req::SometimesRequired,
        multi: false,
    },
    // Адрес сайта ФНС
    FieldSpec {
        tag: Some(1060),
        forms: Form::NONE,
        req: Req::SometimesRequired,
        multi: false,
    },
    // Отраслевой реквизит чека
    FieldSpec {
        tag: Some(1261),
        forms: Form::NONE,
        req: Req::SometimesRequired,
        multi: true,
    },
    // Дополнительный реквизит чека (БСО)
    FieldSpec {
        tag: Some(1192),
        forms: Form::NONE,
        req: Req::SometimesRequired,
        multi: true,
    },
    // Дополнительный реквизит пользователя
    FieldSpec {
        tag: Some(1084),
        forms: Form::NONE,
        req: Req::SometimesRequired,
        multi: false,
    },
    // Номер ФД
    FieldSpec {
        tag: Some(1040),
        forms: Form::NONE,
        req: Req::Mandatory,
        multi: false,
    },
    // Номер ФН
    FieldSpec {
        tag: Some(1041),
        forms: Form::NONE,
        req: Req::Mandatory,
        multi: false,
    },
    // ФПД
    FieldSpec {
        tag: Some(1077),
        forms: Form::NONE,
        req: Req::Mandatory,
        multi: false,
    },
    // QR-код
    FieldSpec {
        tag: Some(1196),
        forms: Form::NONE,
        req: Req::SometimesRequired,
        multi: false,
    },
]);

/// Таблица 147
///
/// Формат ФД «Кассовый чек коррекции (БСО коррекции)», передаваемый покупателю (клиенту) в электронной форме
pub const FD_CORRECTION_RECEIPT_1_2: FieldSet = FieldSet(&[
    // Наименование документа
    FieldSpec {
        tag: Some(1000),
        forms: Form::NONE,
        req: Req::Mandatory,
        multi: false,
    },
    // Наименование пользователя
    FieldSpec {
        tag: Some(1048),
        forms: Form::NONE,
        req: Req::Mandatory,
        multi: false,
    },
    // ИНН пользователя
    FieldSpec {
        tag: Some(1018),
        forms: Form::NONE,
        req: Req::Mandatory,
        multi: false,
    },
    // Номер чека за смену
    FieldSpec {
        tag: Some(1042),
        forms: Form::NONE,
        req: Req::Mandatory,
        multi: false,
    },
    // Сведения о покупателе (клиенте)
    FieldSpec {
        tag: Some(1256),
        forms: Form::NONE,
        req: Req::SometimesRequired,
        multi: false,
    },
    // Тип коррекции
    FieldSpec {
        tag: Some(1173),
        forms: Form::NONE,
        req: Req::Mandatory,
        multi: false,
    },
    // Основание для коррекции
    FieldSpec {
        tag: Some(1174),
        forms: Form::NONE,
        req: Req::Optional,
        multi: false,
    },
    // Дата, время
    FieldSpec {
        tag: Some(1012),
        forms: Form::NONE,
        req: Req::Mandatory,
        multi: false,
    },
    // Номер смены
    FieldSpec {
        tag: Some(1038),
        forms: Form::NONE,
        req: Req::Mandatory,
        multi: false,
    },
    // Признак расчета
    FieldSpec {
        tag: Some(1054),
        forms: Form::NONE,
        req: Req::Mandatory,
        multi: false,
    },
    // Применяемая система налогообложения
    FieldSpec {
        tag: Some(1055),
        forms: Form::NONE,
        req: Req::Optional,
        multi: false,
    },
    // Кассир
    FieldSpec {
        tag: Some(1021),
        forms: Form::NONE,
        req: Req::SometimesRequired,
        multi: false,
    },
    // Регистрационный номер ККТ
    FieldSpec {
        tag: Some(1037),
        forms: Form::NONE,
        req: Req::Mandatory,
        multi: false,
    },
    // Номер автомата
    FieldSpec {
        tag: Some(1036),
        forms: Form::NONE,
        req: Req::SometimesRequired,
        multi: false,
    },
    // Адрес расчетов
    FieldSpec {
        tag: Some(1009),
        forms: Form::NONE,
        req: Req::Mandatory,
        multi: false,
    },
    // Место расчетов
    FieldSpec {
        tag: Some(1187),
        forms: Form::NONE,
        req: Req::Mandatory,
        multi: false,
    },
    // Телефон или электронный адрес покупателя
    FieldSpec {
        tag: Some(1008),
        forms: Form::NONE,
        req: Req::Optional,
        multi: false,
    },
    // Предмет расчета
    FieldSpec {
        tag: Some(1059),
        forms: Form::NONE,
        req: Req::Mandatory,
        multi: true,
    },
    // Сумма расчета, указанного в чеке (БСО)
    FieldSpec {
        tag: Some(1020),
        forms: Form::NONE,
        req: Req::Mandatory,
        multi: false,
    },
    // Сумма по чеку (БСО) наличными
    FieldSpec {
        tag: Some(1031),
        forms: Form::NONE,
        req: Req::SometimesRequired,
        multi: false,
    },
    // Сумма по чеку (БСО) безналичными
    FieldSpec {
        tag: Some(1081),
        forms: Form::NONE,
        req: Req::SometimesRequired,
        multi: false,
    },
    // Сумма по чеку (БСО) предоплатой (зачетом аванса и (или) предыдущих платежей)
    FieldSpec {
        tag: Some(1215),
        forms: Form::NONE,
        req: Req::SometimesRequired,
        multi: false,
    },
    // Сумма по чеку (БСО) постоплатой (в кредит)
    FieldSpec {
        tag: Some(1216),
        forms: Form::NONE,
        req: Req::SometimesRequired,
        multi: false,
    },
    // Сумма по чеку (БСО) встречным предоставлением
    FieldSpec {
        tag: Some(1217),
        forms: Form::NONE,
        req: Req::SometimesRequired,
        multi: false,
    },
    // Сумма НДС чека по ставке 20%
    FieldSpec {
        tag: Some(1102),
        forms: Form::NONE,
        req: Req::SometimesRequired,
        multi: false,
    },
    // Сумма НДС чека по ставке 10%
    FieldSpec {
        tag: Some(1103),
        forms: Form::NONE,
        req: Req::SometimesRequired,
        multi: false,
    },
    // Сумма расчета по чеку с НДС по ставке 0%
    FieldSpec {
        tag: Some(1104),
        forms: Form::NONE,
        req: Req::SometimesRequired,
        multi: false,
    },
    // Сумма расчета по чеку без НДС
    FieldSpec {
        tag: Some(1105),
        forms: Form::NONE,
        req: Req::SometimesRequired,
        multi: false,
    },
    // Сумма НДС чека по расч. ставке 20/120
    FieldSpec {
        tag: Some(1106),
        forms: Form::NONE,
        req: Req::SometimesRequired,
        multi: false,
    },
    // Сумма НДС чека по расч. ставке 10/110
    FieldSpec {
        tag: Some(1107),
        forms: Form::NONE,
        req: Req::SometimesRequired,
        multi: false,
    },
    // Адрес электронной почты отправителя чека
    FieldSpec {
        tag: Some(1117),
        forms: Form::NONE,
        req: Req::SometimesRequired,
        multi: false,
    },
    // Результаты проверки маркированных товаров
    FieldSpec {
        tag: Some(2107),
        forms: Form::NONE,
        req: Req::SometimesRequired,
        multi: false,
    },
    // Адрес сайта ФНС
    FieldSpec {
        tag: Some(1060),
        forms: Form::NONE,
        req: Req::SometimesRequired,
        multi: false,
    },
    // Отраслевой реквизит чека
    FieldSpec {
        tag: Some(1261),
        forms: Form::NONE,
        req: Req::SometimesRequired,
        multi: true,
    },
    // Дополнительный реквизит чека (БСО)
    FieldSpec {
        tag: Some(1192),
        forms: Form::NONE,
        req: Req::Optional,
        multi: true,
    },
    // Дополнительный реквизит пользователя
    FieldSpec {
        tag: Some(1084),
        forms: Form::NONE,
        req: Req::Optional,
        multi: false,
    },
    // Номер ФД
    FieldSpec {
        tag: Some(1040),
        forms: Form::NONE,
        req: Req::Mandatory,
        multi: false,
    },
    // Номер ФН
    FieldSpec {
        tag: Some(1041),
        forms: Form::NONE,
        req: Req::Mandatory,
        multi: false,
    },
    // ФПД
    FieldSpec {
        tag: Some(1077),
        forms: Form::NONE,
        req: Req::Mandatory,
        multi: false,
    },
    // QR-код
    FieldSpec {
        tag: Some(1196),
        forms: Form::NONE,
        req: Req::Optional,
        multi: false,
    },
]);

/// Таблица 150
///
/// Значения реквизита «дробное количество маркированного товара» (тег 1291)
pub const MARKED_FRACTIONAL_QUANITITY: FieldSet = FieldSet(&[
    // Дробная часть
    FieldSpec {
        tag: Some(1292),
        forms: Form::PRINTED,
        req: Req::Mandatory,
        multi: false,
    },
    // Числитель
    FieldSpec {
        tag: Some(1293),
        forms: Form::ELECTRONIC,
        req: Req::Mandatory,
        multi: false,
    },
    // Знаменатель
    FieldSpec {
        tag: Some(1294),
        forms: Form::ELECTRONIC,
        req: Req::Mandatory,
        multi: false,
    },
]);
