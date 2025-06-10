use std::{
    collections::{BTreeSet, HashMap},
    io,
    os::unix::fs::PermissionsExt,
    path::{Path, PathBuf},
};

use chrono::Utc;
use fiscal_data::{enums::PaymentMethod, fields, Object};
use indexmap::IndexSet;
use liquid_core::{Display_filter, Filter, FilterReflection, ParseFilter, Runtime, ValueView};
use serde::{Deserialize, Serialize};

mod ofd;
mod server;

const QR_DATE_FORMAT1: &str = "%Y%m%dT%H%M";
const QR_DATE_FORMAT2: &str = "%Y%m%dT%H%M%S";

fn parse_qr_date(s: &str) -> Result<chrono::NaiveDateTime, chrono::ParseError> {
    chrono::NaiveDateTime::parse_from_str(s, QR_DATE_FORMAT1)
        .or_else(|_| chrono::NaiveDateTime::parse_from_str(s, QR_DATE_FORMAT2))
}

const fn decode_hex_digit(c: u8) -> Option<u8> {
    match c {
        b'0'..=b'9' => Some(c - b'0'),
        b'A'..=b'F' => Some(c - b'A' + 10),
        b'a'..=b'f' => Some(c - b'a' + 10),
        _ => None,
    }
}

fn decode_hex(s: &str) -> Option<Vec<u8>> {
    s.as_bytes()
        .chunks(2)
        .map(|x| {
            if let &[a, b] = x {
                Some(decode_hex_digit(a)? * 16 + decode_hex_digit(b)?)
            } else {
                None
            }
        })
        .collect()
}

fn parse_sum(s: &str) -> Option<u64> {
    if let Some((a, b)) = s.split_once('.') {
        if b.len() > 2 || b.is_empty() {
            return None;
        }
        Some(
            a.parse::<u64>().ok()? * 100
                + b.parse::<u64>().ok()? * if b.len() == 1 { 10 } else { 1 },
        )
    } else {
        s.parse().ok().map(|x: u64| x * 100)
    }
}

async fn parse_qr(s: &str) -> Object {
    let mut ret = Object::new();
    for (k, v) in s.split('&').filter_map(|x| x.split_once('=')) {
        match k {
            "ofd" => {
                let _ = ofd::registry().await.fill(v, &mut ret);
            }
            "date" => {
                if let Ok(x) = chrono::NaiveDate::parse_from_str(v, "%Y-%m-%d") {
                    let _ = ret.set::<fiscal_data::fields::DateTime>(x.into());
                }
            }
            "code" => {
                if let Some(x) = decode_hex(v).and_then(|x| x.try_into().ok()) {
                    let _ = ret.set::<ofd::custom::IcomCode>(x);
                }
            }
            "t" => {
                if let Ok(x) = parse_qr_date(v) {
                    let _ = ret.set::<fiscal_data::fields::DateTime>(x);
                }
            }
            "s" => {
                if let Some(x) = parse_sum(v) {
                    let _ = ret.set::<fiscal_data::fields::TotalSum>(x);
                }
            }
            "fn" => {
                if v.bytes().all(|x| x.is_ascii_digit()) {
                    let _ = ret.set::<fiscal_data::fields::DriveNum>(v.to_owned());
                }
            }
            "n" => {
                if let Ok(x) = v.parse::<u8>() {
                    let _ = ret.set::<fiscal_data::fields::PaymentType>(
                        fiscal_data::enums::PaymentType::from(x),
                    );
                }
            }
            "fp" => {
                if let Ok(x) = v.parse::<u64>() {
                    let [_, _, a, b, c, d, e, f] = x.to_be_bytes();
                    let _ = ret.set::<fiscal_data::fields::DocFiscalSign>([a, b, c, d, e, f]);
                }
            }
            "i" => {
                if let Ok(x) = v.parse::<u32>() {
                    let _ = ret.set::<fiscal_data::fields::DocNum>(x);
                }
            }
            _ => {}
        }
    }
    ret
}

mod iso8601 {
    use serde::{
        de::{self, Unexpected},
        Deserializer, Serializer,
    };

    #[derive(Debug)]
    pub struct Visitor;
    impl<'de> de::Visitor<'de> for Visitor {
        type Value = chrono::DateTime<chrono::Utc>;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("an iso8601 timestamp")
        }

        fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            chrono::DateTime::<chrono::FixedOffset>::parse_from_rfc3339(v)
                .map(|x| x.with_timezone(&chrono::Utc))
                .map_err(|_| E::invalid_value(Unexpected::Str(v), &"a valid iso8601 timestamp"))
        }

        fn visit_borrowed_str<E>(self, v: &'de str) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            chrono::DateTime::<chrono::FixedOffset>::parse_from_rfc3339(v)
                .map(|x| x.with_timezone(&chrono::Utc))
                .map_err(|_| E::invalid_value(Unexpected::Str(v), &"a valid iso8601 timestamp"))
        }
        fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            chrono::DateTime::<chrono::FixedOffset>::parse_from_rfc3339(&v)
                .map(|x| x.with_timezone(&chrono::Utc))
                .map_err(|_| E::invalid_value(Unexpected::Str(&v), &"a valid iso8601 timestamp"))
        }
    }

    pub fn serialize<S>(
        dt: &chrono::DateTime<chrono::Utc>,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&dt.to_rfc3339_opts(chrono::SecondsFormat::AutoSi, true))
    }
    pub fn deserialize<'de, D>(d: D) -> Result<chrono::DateTime<chrono::Utc>, D::Error>
    where
        D: Deserializer<'de>,
    {
        d.deserialize_any(Visitor)
    }
}

mod str_or_int {
    use serde::{
        de::{Error, Unexpected, Visitor},
        Deserializer,
    };

    struct Vis;
    impl<'de> Visitor<'de> for Vis {
        type Value = u32;
        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("an integer")
        }
        fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where
            E: Error,
        {
            v.parse()
                .map_err(|_| E::invalid_value(Unexpected::Str(v), &self))
        }
        fn visit_borrowed_str<E>(self, v: &'de str) -> Result<Self::Value, E>
        where
            E: Error,
        {
            self.visit_str(v)
        }
        fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
        where
            E: Error,
        {
            self.visit_str(&v)
        }
        fn visit_u8<E>(self, v: u8) -> Result<Self::Value, E>
        where
            E: Error,
        {
            Ok(v.into())
        }
        fn visit_u16<E>(self, v: u16) -> Result<Self::Value, E>
        where
            E: Error,
        {
            Ok(v.into())
        }
        fn visit_u32<E>(self, v: u32) -> Result<Self::Value, E>
        where
            E: Error,
        {
            Ok(v)
        }
        fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
        where
            E: Error,
        {
            v.try_into()
                .map_err(|_| E::invalid_value(Unexpected::Unsigned(v), &self))
        }
        fn visit_i8<E>(self, v: i8) -> Result<Self::Value, E>
        where
            E: Error,
        {
            v.try_into()
                .map_err(|_| E::invalid_value(Unexpected::Signed(v.into()), &self))
        }
        fn visit_i16<E>(self, v: i16) -> Result<Self::Value, E>
        where
            E: Error,
        {
            v.try_into()
                .map_err(|_| E::invalid_value(Unexpected::Signed(v.into()), &self))
        }
        fn visit_i32<E>(self, v: i32) -> Result<Self::Value, E>
        where
            E: Error,
        {
            v.try_into()
                .map_err(|_| E::invalid_value(Unexpected::Signed(v.into()), &self))
        }
        fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
        where
            E: Error,
        {
            v.try_into()
                .map_err(|_| E::invalid_value(Unexpected::Signed(v), &self))
        }
    }
    pub fn deserialize<'de, D>(d: D) -> Result<u32, D::Error>
    where
        D: Deserializer<'de>,
    {
        d.deserialize_any(Vis)
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
enum TransactionMeta {
    Receipt {
        r#fn: String,
        #[serde(deserialize_with = "str_or_int::deserialize")]
        i: u32,
        paid: HashMap<String, BTreeSet<usize>>,
    },
    Comment(String),
    Comment2(String, i64),
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
struct Transaction {
    balance_changes: HashMap<String, i64>,
    #[serde(with = "iso8601")]
    date: chrono::DateTime<Utc>,
    meta: Option<TransactionMeta>,
    /// This is redundant, and I store this just in case the FS breaks and I lose files or something
    #[serde(default)]
    prev_state: Option<HashMap<String, i64>>,
}

impl Transaction {
    pub fn new(meta: Option<TransactionMeta>) -> Self {
        Self {
            balance_changes: HashMap::new(),
            date: chrono::Utc::now(),
            prev_state: None,
            meta,
        }
    }
    pub fn pay(&mut self, src: &str, dst: &str, cnt: i64) {
        *self.balance_changes.entry(src.to_owned()).or_default() -= cnt;
        *self.balance_changes.entry(dst.to_owned()).or_default() += cnt;
    }
    pub fn invert(&mut self) {
        for val in self.balance_changes.values_mut() {
            *val = -*val;
        }
    }
    pub fn finalize(&mut self) {
        self.balance_changes.retain(|_, v| *v != 0);
    }
}

async fn add_transaction(state: &server::State, mut tr: Transaction) -> HashMap<String, i64> {
    let mut lock = state.balance.write().await;
    if tr.balance_changes.is_empty() && tr.meta.is_none() {
        return lock.clone();
    }
    tr.date = chrono::Utc::now();
    tr.prev_state = Some(lock.clone());
    let mut path = state.config.data_path("transactions");
    path.push(
        tr.date.to_rfc3339_opts(chrono::SecondsFormat::Secs, true)
            + "_"
            + &uuid::Uuid::new_v4().to_string()
            + ".json",
    );
    let b = serde_json::to_vec(&tr).expect("failed to serialize transaction");
    tokio::fs::write(path, b)
        .await
        .expect("failed to write transaction");
    for (k, v) in &tr.balance_changes {
        let x = lock.entry(k.clone()).or_default();
        *x = x.checked_add(*v).expect("balance overflowed");
    }
    lock.retain(|_, v| *v != 0);
    lock.clone()
}

#[repr(u8)]
#[derive(Copy, Clone, Debug, Default, Deserialize, Serialize)]
#[serde(from = "u8", into = "u8")]
enum ReceiptFormatVersion {
    #[default]
    Json = 0,
    Fns = 1,
}
impl From<u8> for ReceiptFormatVersion {
    fn from(value: u8) -> Self {
        match value {
            1 => Self::Fns,
            _ => Self::Json,
        }
    }
}
impl From<ReceiptFormatVersion> for u8 {
    fn from(value: ReceiptFormatVersion) -> Self {
        match value {
            ReceiptFormatVersion::Json => 0,
            ReceiptFormatVersion::Fns => 1,
        }
    }
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
struct DbState {
    receipt_version: ReceiptFormatVersion,
}

#[derive(Clone, Debug, Default, Deserialize)]
struct Config {
    usernames: IndexSet<String>,
    listener: String,
    data_path: PathBuf,
    ignore_qr_condition: String,
    // public_url: String,
    #[serde(default)]
    irkkt_mobile_client_secret: Option<String>,
    #[serde(default)]
    irkkt_mobile_device_id: Option<String>,
    #[serde(default)]
    irkkt_mobile_api_base: Option<String>,
    #[serde(default)]
    private1_endpoint: Option<String>,
}

impl Config {
    pub fn data_path(&self, path: impl AsRef<Path>) -> PathBuf {
        let mut ret = self.data_path.clone();
        ret.push(path.as_ref());
        ret
    }
}

#[derive(Clone, Debug, Default, Display_filter, ParseFilter, FilterReflection)]
#[name = "currency"]
#[filter(
    name = "currency",
    description = "Currency filter (12300 -> 123.00)",
    parsed(CurrencyFilter)
)]
pub struct CurrencyFilter;

impl Filter for CurrencyFilter {
    fn evaluate(
        &self,
        input: &dyn ValueView,
        _runtime: &dyn Runtime,
    ) -> liquid_core::Result<liquid_core::Value> {
        input
            .as_scalar()
            .and_then(|scal| {
                scal.to_integer()
                    .map(|x| x as f64)
                    .or_else(|| scal.to_float())
            })
            .map(|x| liquid_core::Value::Scalar(format!("{:.2}", x / 100.).into()))
            .ok_or_else(|| liquid_core::Error::with_msg("currency expects an integer or a float"))
    }
}

#[derive(Clone, Debug, Default, Display_filter, ParseFilter, FilterReflection)]
#[name = "cescape"]
#[filter(
    name = "cescape",
    description = "C-style string escape",
    parsed(CEscapeFilter)
)]
pub struct CEscapeFilter;

impl Filter for CEscapeFilter {
    fn evaluate(
        &self,
        input: &dyn ValueView,
        _runtime: &dyn Runtime,
    ) -> liquid_core::Result<liquid_core::Value> {
        input
            .as_scalar()
            .map(|scal| {
                liquid_core::Value::Scalar(
                    scal.to_kstr().escape_default().collect::<String>().into(),
                )
            })
            .ok_or_else(|| liquid_core::Error::with_msg("cescape expects a string"))
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
struct ListItem {
    name: String,
    amount: f64,
}

#[derive(Default)]
struct Commodity {
    unit: String,
    last_time: chrono::NaiveDateTime,
    count: usize,
}

#[derive(Default)]
struct Comment {
    last_price: i64,
    last_time: chrono::DateTime<Utc>,
    count: usize,
}

async fn save_list(config: &Config, list: &[ListItem]) -> io::Result<()> {
    let path1 = config.data_path("list.json.tmp");
    let path2 = config.data_path("list.json");
    tokio::fs::write(
        &path1,
        serde_json::to_string(list).map_err(|err| io::Error::new(io::ErrorKind::Other, err))?,
    )
    .await?;
    tokio::fs::rename(path1, path2).await
}

fn item_is_advance(item: &Object) -> fiscal_data::Result<bool> {
    Ok(matches!(
        item.get::<fields::PaymentMethod>()?,
        Some(
            PaymentMethod::Advance
                | PaymentMethod::Prepaid
                | PaymentMethod::FullPrepaid
                | PaymentMethod::PaymentOfCredit
        )
    ))
}

fn is_advance(rec: &Object) -> fiscal_data::Result<bool> {
    for item in rec.get_all::<fields::ReceiptItem>()? {
        if item_is_advance(&item)? {
            return Ok(true);
        }
    }
    Ok(false)
}

#[tokio::main]
async fn main() {
    env_logger::init();

    let config_path = std::env::vars_os()
        .find(|(k, _)| k == "CONFIG_FILE")
        .map_or_else(|| "config.json".into(), |(_, v)| v);
    let config = serde_json::from_slice::<Config>(
        &tokio::fs::read(config_path)
            .await
            .expect("failed to read config.json"),
    )
    .expect("invalid config.json");

    tokio::join!(
        async {
            tokio::fs::create_dir_all(config.data_path("ffd"))
                .await
                .unwrap();
        },
        async {
            tokio::fs::create_dir_all(config.data_path("transactions"))
                .await
                .unwrap();
        },
    );

    let (state, ..) = tokio::join!(server::State::new(config.clone()), async {
        tokio::fs::create_dir_all(config.data_path("secret"))
            .await
            .unwrap();
        tokio::fs::set_permissions(
            config.data_path("secret"),
            std::fs::Permissions::from_mode(0o750),
        )
        .await
        .unwrap();
    },);

    let mut app = axum::Router::new();
    ofd::init_registry(&state, &mut app).await;

    // state actor
    let state1 = state.clone();
    tokio::spawn(async move {
        let path = state1.config.data_path("state.json");
        let state = tokio::fs::read(&path)
            .await
            .ok()
            .and_then(|f| serde_json::from_slice::<DbState>(&f).ok())
            .unwrap_or_default();
        if matches!(state.receipt_version, ReceiptFormatVersion::Json) {
            panic!("please use an older version to migrate to the new format");
        }
    });

    let app = app
        .route("/", axum::routing::get(server::root))
        .route("/fzf.js", server::js(|state| &state.fzf))
        .route(
            "/qr-scanner.umd.min.js",
            server::js(|state| &state.qr_scanner_umd),
        )
        .route(
            "/qr-scanner.umd.min.js.map",
            server::json(|state| &state.qr_scanner_umd_map),
        )
        .route(
            "/qr-scanner-worker.min.js",
            server::js(|state| &state.qr_scanner_worker),
        )
        .route(
            "/qr-scanner-worker.min.js.map",
            server::json(|state| &state.qr_scanner_worker_map),
        )
        .route("/style.css", server::css(|state| &state.style))
        .route(
            "/api/balance",
            server::json_endpoint_get(|state| async move {
                serde_json::to_string(&*state.balance.read().await)
                    .expect("balance serialization failed")
            }),
        )
        .route("/api/pay", axum::routing::post(server::api_pay))
        .route("/list", axum::routing::get(server::list))
        .route("/listremove", axum::routing::post(server::listremove))
        .route("/listadd", axum::routing::post(server::listadd))
        .route("/submit", axum::routing::post(server::submit))
        .route("/add", axum::routing::get(server::add));
    let app = app.with_state(state);
    axum::Server::bind(&config.listener.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
