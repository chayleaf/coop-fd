use std::{
    collections::{BTreeSet, HashMap, HashSet},
    io,
    os::unix::fs::PermissionsExt,
    path::{Path, PathBuf},
};

use axum::response::IntoResponse;
use chrono::Utc;
use dashmap::DashMap;
use fiscal_data::{
    enums::{PaymentMethod, PaymentType},
    fields, Document, Object, TlvType,
};
use indexmap::IndexSet;
use liquid_core::{Display_filter, Filter, FilterReflection, ParseFilter, Runtime, ValueView};
use serde::{Deserialize, Serialize};
use tokio::sync::{mpsc, OnceCell, RwLock};

mod legacy;
mod ofd;

const QR_DATE_FORMAT1: &str = "%Y%m%dT%H%M";
const QR_DATE_FORMAT2: &str = "%Y%m%dT%H%M%S";

fn parse_qr_date(s: &str) -> Result<chrono::NaiveDateTime, chrono::ParseError> {
    chrono::NaiveDateTime::parse_from_str(s, QR_DATE_FORMAT1)
        .or_else(|_| chrono::NaiveDateTime::parse_from_str(s, QR_DATE_FORMAT2))
}

fn decode_hex_digit(c: u8) -> Option<u8> {
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

static BALANCE: OnceCell<RwLock<HashMap<String, i64>>> = OnceCell::const_new();

async fn add_transaction(config: &Config, mut tr: Transaction) -> HashMap<String, i64> {
    let mut lock = BALANCE.get().unwrap().write().await;
    if tr.balance_changes.is_empty() && tr.meta.is_none() {
        return lock.clone();
    }
    tr.date = chrono::Utc::now();
    tr.prev_state = Some(lock.clone());
    let mut path = config.data_path("transactions");
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
struct State {
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

const fn copy_ref<T: ?Sized>(t: &T) -> &T {
    t
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
    let config = Box::leak(Box::new(
        serde_json::from_slice::<Config>(
            &tokio::fs::read(config_path)
                .await
                .expect("failed to read config.json"),
        )
        .expect("invalid config.json"),
    ));

    let parser = liquid::ParserBuilder::with_stdlib()
        .filter(CurrencyFilter)
        .filter(CEscapeFilter)
        .build()
        .unwrap();

    let commodities = &*Box::leak(Box::new(DashMap::<String, Commodity>::new()));
    let comments = &*Box::leak(Box::new(DashMap::<String, Comment>::new()));
    let paid_receipts = Box::leak(Box::new(dashmap::DashSet::<String>::new()));

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

    let (
        style,
        fzf,
        qr_scanner_worker,
        qr_scanner_worker_map,
        qr_scanner_umd,
        qr_scanner_umd_map,
        root_t,
        submitted_t,
        add_t,
        list_t,
        list,
        ..,
    ) = tokio::join!(
        async {
            &*Box::leak(Box::new(
                tokio::fs::read_to_string("static/style.css")
                    .await
                    .unwrap_or_else(|_| include_str!("../static/style.css").to_owned()),
            ))
        },
        async {
            &*Box::leak(Box::new(
                tokio::fs::read_to_string("static/fzf.js")
                    .await
                    .unwrap_or_else(|_| include_str!("../static/fzf.js").to_owned()),
            ))
        },
        async {
            &*Box::leak(Box::new(
                tokio::fs::read_to_string("static/qr-scanner-worker.min.js")
                    .await
                    .unwrap_or_else(|_| {
                        include_str!("../static/qr-scanner-worker.min.js").to_owned()
                    }),
            ))
        },
        async {
            &*Box::leak(Box::new(
                tokio::fs::read_to_string("static/qr-scanner-worker.min.js.map")
                    .await
                    .unwrap_or_else(|_| {
                        include_str!("../static/qr-scanner-worker.min.js.map").to_owned()
                    }),
            ))
        },
        async {
            &*Box::leak(Box::new(
                tokio::fs::read_to_string("static/qr-scanner.umd.min.js")
                    .await
                    .unwrap_or_else(|_| include_str!("../static/qr-scanner.umd.min.js").to_owned()),
            ))
        },
        async {
            &*Box::leak(Box::new(
                tokio::fs::read_to_string("static/qr-scanner.umd.min.js.map")
                    .await
                    .unwrap_or_else(|_| {
                        include_str!("../static/qr-scanner.umd.min.js.map").to_owned()
                    }),
            ))
        },
        async {
            &*Box::leak(Box::new(
                parser
                    .parse(
                        &tokio::fs::read_to_string("templates/index.html")
                            .await
                            .unwrap_or_else(|_| include_str!("../templates/index.html").to_owned()),
                    )
                    .unwrap_or_else(|err| panic!("index:\n{err}")),
            ))
        },
        async {
            &*Box::leak(Box::new(
                parser
                    .parse(
                        &tokio::fs::read_to_string("templates/submitted.html")
                            .await
                            .unwrap_or_else(|_| {
                                include_str!("../templates/submitted.html").to_owned()
                            }),
                    )
                    .unwrap_or_else(|err| panic!("submitted:\n{err}")),
            ))
        },
        async {
            &*Box::leak(Box::new(
                parser
                    .parse(
                        &tokio::fs::read_to_string("templates/add.html")
                            .await
                            .unwrap_or_else(|_| include_str!("../templates/add.html").to_owned()),
                    )
                    .unwrap_or_else(|err| panic!("add:\n{err}")),
            ))
        },
        async {
            &*Box::leak(Box::new(
                parser
                    .parse(
                        &tokio::fs::read_to_string("templates/list.html")
                            .await
                            .unwrap_or_else(|_| include_str!("../templates/list.html").to_owned()),
                    )
                    .unwrap_or_else(|err| panic!("list:\n{err}")),
            ))
        },
        async {
            &*Box::leak(Box::new(RwLock::new(
                serde_json::from_str::<Vec<ListItem>>(
                    &tokio::fs::read_to_string(config.data_path("list.json"))
                        .await
                        .unwrap_or_else(|_| "[]".to_owned()),
                )
                .unwrap(),
            )))
        },
        async {
            // fill balance and paid receipt list (can be parallelized)
            let mut balance = HashMap::<String, i64>::new();
            let mut dir = tokio::fs::read_dir(config.data_path("transactions"))
                .await
                .expect("failed to read transaction list");
            let mut files = vec![];
            while let Some(file) = dir
                .next_entry()
                .await
                .expect("failed to read transaction list entry")
            {
                files.push(file.path());
            }
            files.sort_unstable();
            for file in files {
                if !matches!(file.extension().and_then(|x| x.to_str()).map(str::to_lowercase), Some(x) if x.as_str() == "json")
                {
                    continue;
                }
                let data = tokio::fs::read(&file)
                    .await
                    .expect("failed to read transaction");
                let tr = serde_json::from_slice::<Transaction>(&data).unwrap_or_else(|_| {
                    panic!("failed to deserialize transaction {}", file.display())
                });
                match tr.meta {
                    Some(TransactionMeta::Receipt { r#fn, i, paid: _ }) => {
                        paid_receipts.insert(format!("{fn}_{i:07}"));
                    }
                    Some(TransactionMeta::Comment(comment)) => {
                        let mut val = comments.entry(comment).or_default();
                        let val = val.value_mut();
                        val.last_time = tr.date;
                        val.count += 1;
                    }
                    Some(TransactionMeta::Comment2(comment, price)) => {
                        let mut val = comments.entry(comment).or_default();
                        let val = val.value_mut();
                        val.last_time = tr.date;
                        val.last_price = price;
                        val.count += 1;
                    }
                    None => {}
                }
                for (k, v) in &tr.balance_changes {
                    let x = balance.entry(k.clone()).or_default();
                    *x = x.checked_add(*v).expect("balance overflowed");
                }
            }
            balance.retain(|_, v| *v != 0);
            BALANCE.set(balance.into()).unwrap();
        },
        async {
            // fill commodities (can be parallelized)
            let mut dir = tokio::fs::read_dir(config.data_path("ffd"))
                .await
                .expect("failed to read receipt list");
            while let Some(file) = dir
                .next_entry()
                .await
                .expect("failed to read receipt list entry")
            {
                if !matches!(file.path().extension().and_then(|x| x.to_str()).map(str::to_lowercase), Some(x) if x.as_str() == "tlv")
                {
                    continue;
                }
                let data = tokio::fs::read(file.path())
                    .await
                    .expect("failed to read ffd");
                let doc = Document::from_bytes(data).unwrap_or_else(|err| {
                    panic!(
                        "failed to deserialize receipt {}: {err}",
                        file.path().display()
                    )
                });
                let rec = doc.data();
                let date = rec.get::<fields::DateTime>().ok().flatten();
                for item in rec.get_all::<fields::ReceiptItem>().unwrap_or_default() {
                    let name = item
                        .get::<fields::ItemName>()
                        .ok()
                        .flatten()
                        .expect("failed to read item name");
                    let mut val = commodities.entry(name).or_default();
                    let val = val.value_mut();
                    if let Some(unit) = item
                        .get::<fields::Unit>()
                        .ok()
                        .flatten()
                        .filter(|unit| !unit.is_empty())
                        .or_else(|| {
                            item.get::<fields::ItemQuantityUnit>()
                                .ok()
                                .flatten()
                                .map(|x| x.to_string())
                        })
                    {
                        val.unit = unit;
                    }
                    if let Some(date) = date {
                        val.last_time = val.last_time.max(date);
                    }
                    val.count += 1;
                }
            }
        },
        async {
            tokio::fs::create_dir_all(config.data_path("secret"))
                .await
                .unwrap();
            tokio::fs::set_permissions(
                config.data_path("secret"),
                std::fs::Permissions::from_mode(0o640),
            )
            .await
            .unwrap();
        },
    );

    let mut app = axum::Router::new();
    ofd::init_registry(config, &mut app).await;

    // state actor
    tokio::spawn(async {
        let path = config.data_path("state.json");
        let mut state = tokio::fs::read(&path)
            .await
            .ok()
            .and_then(|f| serde_json::from_slice::<State>(&f).ok())
            .unwrap_or_default();
        let (tx, mut rx) = mpsc::channel(32);
        if let ReceiptFormatVersion::Json = state.receipt_version {
            tokio::spawn(legacy::migrate_json_to_ffd(config, tx));
        }
        while let Some(func) = rx.recv().await {
            func(&mut state);
            tokio::fs::write(&path, serde_json::to_vec(&state).unwrap())
                .await
                .unwrap();
        }
    });

    let app = app
        .route(
            "/",
            axum::routing::get(|| {
                let config = copy_ref(config);
                let root_t = copy_ref(root_t);
                let comments = copy_ref(comments);
                async move {
                    let comments = comments
                        .iter()
                        .map(|item| {
                            let val = item.value();
                            liquid::object!({
                                "name": item.key(),
                                "last_price": val.last_price,
                                "count": val.count,
                                "last_timestamp": val.last_time.timestamp(),
                            })
                        })
                        .collect::<Vec<_>>();
                    axum::response::Html::from(root_t.render(&liquid::object!({
                        "comments": comments,
                        "extra_qr_processing": format!("if({})return;", config.ignore_qr_condition),
                        "usernames": &config.usernames,
                        "ofds": ofd::registry()
                            .await
                            .by_id
                            .iter()
                            .flat_map(|(k, v)| v.iter().map(|v| (k.clone(), v)))
                            .filter(|(_, v)| !v.name().is_empty())
                            .map(|(k, v)| liquid::object!({
                                "id": k,
                                "name": format!("{} ({})", v.name(), v.url()),
                            }))
                            .collect::<Vec<liquid::Object>>(),
                    })).unwrap_or_else(|err| format!("Error: {err}")))
                }
            }),
        )
        .route(
            "/fzf.js",
            axum::routing::get(|| async {
                (
                    [(
                        axum::http::header::CONTENT_TYPE,
                        axum::http::HeaderValue::from_static("text/javascript"),
                    )],
                    fzf.as_str(),
                )
                    .into_response()
            }),
        )
        .route(
            "/qr-scanner.umd.min.js",
            axum::routing::get(|| async {
                (
                    [(
                        axum::http::header::CONTENT_TYPE,
                        axum::http::HeaderValue::from_static("text/javascript"),
                    )],
                    qr_scanner_umd.as_str(),
                )
                    .into_response()
            }),
        )
        .route(
            "/qr-scanner.umd.min.js.map",
            axum::routing::get(|| async {
                (
                    [(
                        axum::http::header::CONTENT_TYPE,
                        axum::http::HeaderValue::from_static("application/json"),
                    )],
                    qr_scanner_umd_map.as_str(),
                )
                    .into_response()
            }),
        )
        .route(
            "/qr-scanner-worker.min.js",
            axum::routing::get(|| async {
                (
                    [(
                        axum::http::header::CONTENT_TYPE,
                        axum::http::HeaderValue::from_static("text/javascript"),
                    )],
                    qr_scanner_worker.as_str(),
                )
                    .into_response()
            }),
        )
        .route(
            "/qr-scanner-worker.min.js.map",
            axum::routing::get(|| async {
                (
                    [(
                        axum::http::header::CONTENT_TYPE,
                        axum::http::HeaderValue::from_static("application/json"),
                    )],
                    qr_scanner_worker_map.as_str(),
                )
                    .into_response()
            }),
        )
        .route(
            "/style.css",
            axum::routing::get(|| async {
                (
                    [(
                        axum::http::header::CONTENT_TYPE,
                        axum::http::HeaderValue::from_static("text/css"),
                    )],
                    style.as_str(),
                )
                    .into_response()
            }),
        )
        .route(
            "/api/balance",
            axum::routing::get(|| async {
                (
                    [(
                        axum::http::header::CONTENT_TYPE,
                        axum::http::HeaderValue::from_static("application/json"),
                    )],
                    serde_json::to_string(&*BALANCE.get().unwrap().read().await)
                        .expect("balance serialization failed"),
                )
                    .into_response()
            }),
        )
        .route(
            "/api/pay",
            axum::routing::post(
                |axum::extract::Form(f): axum::extract::Form<HashMap<String, String>>| {
                    let config = copy_ref(config);
                    let comments = copy_ref(comments);
                    let submitted_t = copy_ref(submitted_t);
                    async move {
                        (
                            [(
                                axum::http::header::CONTENT_TYPE,
                                axum::http::HeaderValue::from_static("application/json"),
                            )],
                            {
                                if let Some(to) = f.get("to") {
                                    if let Some(amt) = f.get("amount") {
                                        if let Some(amt) =
                                            amt.parse::<i64>().ok().filter(|x| *x != 0)
                                        {
                                            let mut tr = f
                                                .get("comment")
                                                .map_or_else(|| Transaction::new(None), |x| {
                                                    let mut val = comments.entry(x.clone()).or_default();
                                                    let val = val.value_mut();
                                                    val.last_price = amt;
                                                    val.count += 1;
                                                    let tr = Transaction::new(Some(TransactionMeta::Comment2(x.clone(), amt)));
                                                    val.last_time = tr.date;
                                                    tr
                                                });
                                            let is_html = matches!(f.get("response-format"), Some(x) if x == "html");
                                            if is_html {
                                                let mut all_payers = vec![];
                                                for (k, v) in &f {
                                                    let Some(username) = k.strip_prefix("from_") else {
                                                        continue;
                                                    };
                                                    if matches!(v.as_str(), "" | "off" | "0" | "false") {
                                                        continue;
                                                    }
                                                    all_payers.push(username);
                                                }
                                                let payers_len = i64::try_from(all_payers.len())
                                                    .expect("usize->i64 conversion failed");
                                                for user in all_payers {
                                                    if user != to {
                                                        tr.pay(
                                                            user,
                                                            to,
                                                            amt / payers_len,
                                                        );
                                                    }
                                                }
                                            } else if let Some(from) = f.get("from") {
                                                tr.pay(from, to, amt);
                                            } else {
                                                for user in &config.usernames {
                                                    if user != to {
                                                        tr.pay(
                                                            user,
                                                            to,
                                                            amt / i64::try_from(
                                                                config.usernames.len(),
                                                            )
                                                            .expect("usize->i64 conversion failed"),
                                                        );
                                                    }
                                                }
                                            }
                                            tr.finalize();
                                            let balance = add_transaction(config, tr).await;
                                            if is_html {
                                                let balance = balance
                                                    .into_iter()
                                                    .map(|(username, balance)| {
                                                        liquid::object!({
                                                            "username": username,
                                                            "balance": balance,
                                                        })
                                                    })
                                                    .collect::<Vec<_>>();
                                                return axum::response::Html::from(
                                                    submitted_t
                                                        .render(&liquid::object!({
                                                            "prefix": "..",
                                                            "balance": balance,
                                                            "username": to,
                                                            "removed": [],
                                                        }))
                                                        .unwrap_or_else(|err| format!("Error: {err}")),
                                                ).into_response();
                                            } else  {
                                                serde_json::to_string(
                                                    &balance,
                                                )
                                                .expect("balance serialization failed")
                                            }
                                        } else {
                                            "invalid amount".to_owned()
                                        }
                                    } else {
                                        "missing amount".to_owned()
                                    }
                                } else {
                                    "missing to".to_owned()
                                }
                            },
                        )
                            .into_response()
                    }
                },
            ),
        )
        .route(
            "/list",
            axum::routing::get(|| {
                let commodities = copy_ref(commodities);
                let list_t = copy_ref(list_t);
                let list = copy_ref(list);
                async move {
                    let items = commodities
                        .iter()
                        .map(|item| {
                            let val = item.value();
                            liquid::object!({
                                "name": item.key(),
                                "unit": val.unit,
                                "count": val.count,
                                "last_timestamp": val.last_time.and_utc().timestamp(),
                            })
                        })
                        .collect::<Vec<_>>();
                    axum::response::Html::from(
                        list_t
                            .render(&liquid::object!({
                                "items": items,
                                "list": list.read().await.iter().map(|x| {
                                    liquid::object!({
                                        "name": x.name,
                                        "amount": x.amount,
                                        "unit": commodities
                                            .get(&x.name)
                                            .filter(|x| !x.value().unit.is_empty())
                                            .map(|x| format!(" {}", x.value().unit))
                                            .unwrap_or_default()
                                    })
                                }).collect::<Vec<_>>(),
                            }))
                            .unwrap_or_else(|err| format!("Error: {err}")),
                    )
                }
            }),
        )
        .route(
            "/listremove",
            axum::routing::post(
                |axum::extract::Form(f): axum::extract::Form<HashMap<String, String>>| {
                    let config = copy_ref(config);
                    let list = copy_ref(list);
                    async move {
                        if let Some(name) = f.get("name") {
                            let mut list = list.write().await;
                            list.retain(|x| &x.name != name);
                            let _ = save_list(config, &list).await;
                        }
                        axum::response::Redirect::to("list")
                    }
                },
            ),
        )
        .route(
            "/listadd",
            axum::routing::post(
                |axum::extract::Form(f): axum::extract::Form<HashMap<String, String>>| {
                    let config = copy_ref(config);
                    let list = copy_ref(list);
                    async move {
                        if let Some(name) = f.get("name") {
                            if let Some(amount) =
                                f.get("amount").and_then(|x| x.parse::<f64>().ok())
                            {
                                let mut list = list.write().await;
                                let mut added = false;
                                for item in &mut *list {
                                    if &item.name == name {
                                        item.amount += amount;
                                        added = true;
                                    }
                                }
                                if !added {
                                    list.push(ListItem {
                                        name: name.clone(),
                                        amount,
                                    });
                                }
                                let _ = save_list(config, &list).await;
                            }
                        }
                        axum::response::Redirect::to("list")
                    }
                },
            ),
        )
        .route(
            "/submit",
            axum::routing::post(
                |axum::extract::Form(f): axum::extract::Form<HashMap<String, String>>| {
                    let config = copy_ref(config);
                    let paid_receipts = copy_ref(paid_receipts);
                    let submitted_t = copy_ref(submitted_t);
                    let commodities = copy_ref(commodities);
                    let list = copy_ref(list);
                    async move {
                        let Some(r#fn) = f.get("fn").filter(|x| x.bytes().all(|x| x.is_ascii_digit())) else {
                            return axum::response::Html::from("missing fn".to_owned());
                        };
                        let Some(i) = f.get("i").and_then(|x| x.parse::<u32>().ok()) else {
                            return axum::response::Html::from("missing i".to_owned());
                        };
                        let Some(username) = f.get("username") else {
                            return axum::response::Html::from("missing username".to_owned());
                        };
                        let path = config.data_path(format!("ffd/{fn}_{i:07}.tlv"));
                        let Ok(data) = tokio::fs::read(&path).await else {
                            log::error!("missing {path:?}");
                            return axum::response::Html::from("missing receipt cache 1".to_owned());
                        };
                        let Ok(doc) = Document::from_bytes(data) else {
                            return axum::response::Html::from("invalid receipt cache 2".to_owned());
                        };
                        let rec = doc.data();
                        let Ok(payment_type) = rec.get::<fields::PaymentType>() else {
                            return axum::response::Html::from("invalid payment type".to_owned());
                        };
                        let invert = match payment_type {
                            Some(PaymentType::Sale) => false,
                            Some(PaymentType::Purchase) => true,
                            Some(PaymentType::SaleReturn) => true,
                            Some(PaymentType::PurchaseReturn) => false,
                            _ => return axum::response::Html::from("invalid payment type".to_owned()),
                        };
                        let mut removed = Vec::<String>::new();
                        if !invert {
                            let mut list = list.write().await;
                            list.retain_mut(|list_item| {
                                let lower = list_item.name.to_lowercase();
                                let len = lower.chars().count();
                                for item in rec.get_all::<fields::ReceiptItem>().unwrap_or_default() {
                                    let Ok(Some(name)) = item.get::<fields::ItemName>() else {
                                        continue
                                    };
                                    if if len < 6 {
                                        name.to_lowercase().starts_with(&lower)
                                    } else {
                                        name.to_lowercase().contains(&lower)
                                    } {
                                        if let Ok(Some(count)) = item.get::<fields::ItemQuantity>() {
                                            list_item.amount -= count.f64_approximation();
                                        }
                                        break;
                                    }
                                }
                                let ret = list_item.amount > 0.01;
                                if !ret {
                                    removed.push(list_item.name.clone());
                                }
                                ret
                            });
                            let _ = save_list(config, &list).await;
                        }
                        let mut paid = HashMap::<String, BTreeSet<usize>>::new();
                        let mut per_item = HashMap::<usize, HashSet<String>>::new();
                        for (k, v) in &f {
                            let Some((username, idx)) = k.split_once('$') else {
                                continue;
                            };
                            if matches!(v.as_str(), "" | "off" | "0" | "false") {
                                continue;
                            }
                            let Ok(idx) = idx.parse::<usize>() else {
                                continue;
                            };
                            paid.entry(username.to_owned()).or_default().insert(idx);
                            per_item.entry(idx).or_default().insert(username.to_owned());
                        }
                        let mut groups = HashMap::<Vec<String>, Vec<usize>>::new();
                        for (k, v) in per_item {
                            let (mut k, v) = (v.into_iter().collect::<Vec<_>>(), k);
                            k.sort();
                            groups.entry(k).or_default().push(v);
                        }
                        for v in groups.values_mut() {
                            v.sort_unstable();
                        }
                        let mut tr = Transaction::new(Some(TransactionMeta::Receipt {
                            r#fn: r#fn.clone(),
                            i,
                            paid,
                        }));
                        let items = rec.get_all::<fields::ReceiptItem>().unwrap_or_default();
                        for (k, v) in &groups {
                            let total: u64 = v
                                .iter()
                                .filter_map(|x| items.get(*x))
                                .map(|item| {
                                    item.get::<fields::ItemTotalPrice>().ok().flatten().unwrap_or_default()
                                })
                                .sum();
                            for user in k {
                                if user != username {
                                    tr.pay(
                                        user,
                                        username,
                                        (total
                                            / u64::try_from(k.len())
                                                .expect("wtf, 128-bit cpus???"))
                                        .try_into()
                                        .expect("u64->i64 conversion failed"),
                                    );
                                }
                            }
                        }
                        if invert {
                            tr.invert();
                        }
                        tr.finalize();
                        let balance = add_transaction(config, tr).await;
                        let date = rec.get::<fields::DateTime>().ok().flatten();
                        for item in &items {
                            let name = item
                                .get::<fields::ItemName>()
                                .ok()
                                .flatten()
                                .expect("failed to read item name");
                            let mut val = commodities.entry(name).or_default();
                            let val = val.value_mut();
                            if let Some(unit) = item
                                .get::<fields::Unit>()
                                .ok()
                                .flatten()
                                .filter(|unit| !unit.is_empty())
                                .or_else(|| {
                                    item.get::<fields::ItemQuantityUnit>()
                                        .ok()
                                        .flatten()
                                        .map(|x| x.to_string())
                                })
                            {
                                val.unit = unit;
                            }
                            if let Some(date) = date {
                                val.last_time = date;
                            }
                            val.count += 1;
                        }
                        paid_receipts.insert(format!("{fn}_{i:07}"));
                        let mut balance = balance.into_iter().collect::<Vec<_>>();
                        balance.sort_by_key(|(k, _)| {
                            config
                                .usernames
                                .iter()
                                .enumerate()
                                .find(|(_, x)| &k == x)
                                .map(|x| x.0)
                        });
                        let balance = balance
                            .into_iter()
                            .map(|(username, balance)| {
                                liquid::object!({
                                    "username": username,
                                    "balance": balance,
                                })
                            })
                            .collect::<Vec<_>>();
                        axum::response::Html::from(
                            submitted_t
                                .render(&liquid::object!({
                                    "prefix": ".",
                                    "balance": balance,
                                    "username": username,
                                    "removed": removed,
                                }))
                                .unwrap_or_else(|err| format!("Error: {err}")),
                        )
                    }
                },
            ),
        )
        .route(
            "/add",
            axum::routing::get(
                |axum::extract::RawQuery(q): axum::extract::RawQuery,
                 cookies: axum_extra::extract::CookieJar| {
                    let config = copy_ref(config);
                    let paid_receipts = copy_ref(paid_receipts);
                    let add_t = copy_ref(add_t);
                    async move {
                        axum::response::Html::from(if let Some(q) = q {
                            if let Some(username) = cookies
                                .get("username")
                                .map(axum_extra::extract::cookie::Cookie::value)
                            {
                                let mut rec = parse_qr(&q).await;
                                if !rec.contains::<fields::DriveNum>()
                                    || !rec.contains::<fields::DocNum>()
                                    || !rec.contains::<fields::DocFiscalSign>() && q.starts_with("http")
                                {
                                    let q = if q.starts_with("ofd=") {
                                        q.split_once('&').map_or(q.as_str(), |x| x.1)
                                    } else {
                                        &q
                                    };
                                    if let Ok(res) = reqwest::get(q).await {
                                        if let Ok(text) = res.text().await {
                                            if let Some(x) = text
                                                .split("<div>ФН №: <right>")
                                                .nth(1)
                                                .and_then(|x| x.split('<').next())
                                            {
                                                if x.bytes().all(|x| x.is_ascii_digit()) {
                                                    let _ = rec.set::<fiscal_data::fields::DriveNum>(x.to_owned());
                                                }
                                            }
                                            if let Some(x) = text
                                                .split("<div>ФП: <right>")
                                                .nth(1)
                                                .and_then(|x| x.split('<').next())
                                            {
                                                if let Ok(x) = x.parse::<u64>() {
                                                    let [_, _, a, b, c, d, e, f] = x.to_be_bytes();
                                                    let _ = rec.set::<fiscal_data::fields::DocFiscalSign>([a, b, c, d, e, f]);
                                                }
                                            }
                                            if let Some(x) = text
                                                .split("<div>ФД №: <right>")
                                                .nth(1)
                                                .and_then(|x| x.split('<').next())
                                            {
                                                if let Ok(x) = x.parse::<u32>() {
                                                    let _ = rec.set::<fiscal_data::fields::DocNum>(x);
                                                }
                                            }
                                            if !rec.contains::<fiscal_data::fields::PaymentType>() {
                                                let _ = rec.set::<fiscal_data::fields::PaymentType>(PaymentType::Sale);
                                            }
                                            if let Some(x) = text
                                                .split("<div>Дата Время <right>")
                                                .nth(1)
                                                .and_then(|x| x.split('<').next())
                                                .and_then(|x| chrono::NaiveDateTime::parse_from_str(x, "%d.%m.%Y %H:%M").ok())
                                            {
                                                let _ = rec.set::<fields::DateTime>(x);
                                            }
                                            if let Some(x) = text
                                                .split("<big>ИТОГ <right>≡")
                                                .nth(1)
                                                .and_then(|x| x.split('<').next())
                                            {
                                                if let Some(sum) = parse_sum(x) {
                                                    let _ = rec.set::<fields::TotalSum>(sum);
                                                }
                                            }
                                        }
                                    }
                                }
                                match ofd::fetch(config, rec).await {
                                    Ok(doc) => {
                                        let rec = doc.data();
                                        let r#fn = rec.get::<fields::DriveNum>().ok().flatten().unwrap_or_default();
                                        let i = rec.get::<fields::DocNum>().ok().flatten().unwrap_or_default();
                                        let payment_type = rec.get::<fields::PaymentType>().ok().flatten().unwrap_or_default();
                                        let invert = match payment_type {
                                            PaymentType::Sale => false,
                                            PaymentType::Purchase => true,
                                            PaymentType::SaleReturn => true,
                                            PaymentType::PurchaseReturn => false,
                                            _ => return axum::response::Html::from("invalid payment type".to_owned()).into_response(),
                                        };
                                        let inv = |x: u64| if invert { -(x as i64) } else { x as i64 };
                                        let inv_f = |x: f64| if invert { -x } else { x };
                                        add_t.render(&liquid::object!({
                                            "total": rec.get::<fields::TotalSum>().ok().flatten().unwrap_or_default(),
                                            "username": username,
                                            "already_paid": paid_receipts.contains(&format!("{fn}_{i:07}")),
                                            "is_advance": is_advance(rec).unwrap_or_default(),
                                            "is_refund": invert,
                                            "fn": r#fn,
                                            "i": i,
                                            "items": rec.get_all::<fields::ReceiptItem>().unwrap_or_default().into_iter().enumerate().map(|(i, item)| {
                                                liquid::object!({
                                                    "is_advance": item_is_advance(&item).unwrap_or_default(),
                                                    "num": i,
                                                    "name": item
                                                        .get::<fields::ItemName>()
                                                        .ok()
                                                        .flatten()
                                                        .unwrap_or_default(),
                                                    "count": item
                                                        .get::<fields::ItemQuantity>()
                                                        .ok()
                                                        .flatten()
                                                        .map(|x| inv_f(x.f64_approximation()))
                                                        .unwrap_or_default(),
                                                    "unit": item
                                                        .get::<fields::Unit>()
                                                        .ok()
                                                        .flatten()
                                                        .filter(|unit| !unit.is_empty())
                                                        .or_else(|| {
                                                            item.get::<fields::ItemQuantityUnit>()
                                                                .ok()
                                                                .flatten()
                                                                .map(|x| x.to_string())
                                                        })
                                                        .unwrap_or_default(),
                                                    "per_item": item
                                                        .get::<fields::ItemUnitPrice>()
                                                        .ok()
                                                        .flatten()
                                                        .map(inv)
                                                        .unwrap_or_default(),
                                                    "total": item
                                                        .get::<fields::ItemTotalPrice>()
                                                        .ok()
                                                        .flatten()
                                                        .map(inv)
                                                        .unwrap_or_default(),
                                                })
                                            }).collect::<Vec<_>>(),
                                            "usernames": &config.usernames,
                                        }))
                                        .unwrap_or_else(|err| format!("Error: {err}"))
                                    }
                                    Err(ofd::Error::Redirect(url)) => {
                                        return axum::response::Redirect::temporary(&url).into_response();
                                    }
                                    Err(err) => {
                                        log::error!("ofd fetch failed: {err}");
                                        format!("error: {err}")
                                    }
                                }
                            } else {
                                "missing username cookie".to_owned()
                            }
                        } else {
                            "missing qr info".to_owned()
                        }).into_response()
                    }
                },
            ),
        );
    axum::Server::bind(&config.listener.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
