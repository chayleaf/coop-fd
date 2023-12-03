use std::{
    collections::{BTreeSet, HashMap, HashSet},
    ffi::CStr,
    io,
    path::PathBuf,
};

use axum::response::IntoResponse;
use chrono::Utc;
use dashmap::DashMap;
use indexmap::IndexSet;
use liquid_core::{Display_filter, Filter, FilterReflection, ParseFilter, Runtime, ValueView};
use serde::{Deserialize, Serialize};
use tokio::sync::{mpsc, OnceCell, RwLock};

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
struct Company {
    name: String,
    inn: String,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
struct Receipt {
    company: Company,
    items: Vec<Item>,
    total: u64,
    total_cash: u64,
    total_card: u64,
    total_tax: u64,
    r#fn: String,
    fp: String,
    i: String,
    n: String,
    id: String,
    date: String,
}

impl Receipt {
    fn fnifp(&self) -> Option<String> {
        (!self.r#fn.is_empty() && !self.i.is_empty() && !self.fp.is_empty())
            .then(|| format!("{}_{}_{}", self.r#fn, self.i, self.fp))
    }
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
struct Item {
    name: String,
    count: f64,
    unit: String,
    per_item: u64,
    total: u64,
    tax: u64,
}

fn parse_qr(s: &str) -> Receipt {
    let mut ret = Receipt::default();
    for (k, v) in s.split('&').filter_map(|x| x.split_once('=')) {
        match k {
            "t" => ret.date = v.to_owned(),
            "s" => ret.total = v.replace('.', "").parse::<u64>().unwrap_or_default(),
            "fn" => ret.r#fn = v.to_owned(),
            "i" => ret.i = v.to_owned(),
            "fp" => ret.fp = v.to_owned(),
            "n" => ret.n = v.to_owned(),
            _ => {}
        }
    }
    ret
}

fn parse(data: &str, ret: &mut Receipt) -> Option<()> {
    // TODO: parse older formats? just for fun
    let data = data.split("fido_cheque_container\">").nth(1)?;
    let data = data.split('<').next()?;
    let data = data.trim();
    if let Some(data) = data
        .split("&lt;!-- Название --&gt;")
        .nth(1)
        .and_then(|x| x.split("&lt;!-- /Название --&gt;").next())
    {
        if let Some(x) = data
            .split("&lt;b&gt;")
            .nth(1)
            .and_then(|x| x.split("&lt;/b&gt;").next())
        {
            ret.company.name = html_escape::decode_html_entities(x).into_owned();
        }
        if let Some(x) = data
            .split("&gt;ИНН")
            .nth(1)
            .and_then(|x| x.split("&lt;").next())
        {
            ret.company.inn = x.trim().to_owned();
        }
    }
    if let Some(data) = data
        .split("&lt;!-- Предоплата --&gt;")
        .nth(1)
        .and_then(|x| x.split("&lt;!-- /Предоплата --&gt;").next())
    {
        // items
        for data in data.split("&lt;!-- Fragment - field --&gt;") {
            let mut item = Item::default();
            if let Some(x) = data
                .split("&lt;b&gt;")
                .nth(1)
                .and_then(|x| x.split("&lt;/b&gt;").next())
                .and_then(|x| x.split_once(' '))
                .map(|x| x.1)
            {
                item.name = html_escape::decode_html_entities(x).into_owned();
            }
            if let Some(data) = data
                .split("&lt;!-- Цена --&gt;")
                .nth(1)
                .and_then(|x| x.split("&lt;b&gt;").nth(1))
                .and_then(|x| x.split("&lt;/b&gt;").next())
            {
                if let Some(x) = data
                    .split("&lt;span&gt;")
                    .nth(1)
                    .and_then(|x| x.split_whitespace().next())
                    .and_then(|x| x.parse::<f64>().ok())
                {
                    item.count = x;
                }
                if let Some(x) = data
                    .split('x')
                    .next()
                    .and_then(|x| x.split("&lt;/span&gt;").nth(1))
                    .and_then(|x| x.split("&lt;span&gt;").last())
                {
                    if x != "&lt;!-- --&gt;" {
                        item.unit = html_escape::decode_html_entities(x).into_owned();
                    }
                }
                if let Some(x) = data
                    .split('x')
                    .nth(1)
                    .and_then(|x| x.split("&lt;span&gt;").nth(1))
                    .and_then(|x| x.split("&lt;/span&gt;").next())
                    .and_then(|x| x.replace('.', "").parse::<u64>().ok())
                {
                    item.per_item = x;
                }
            }
            if let Some(data) = data
                .split("&lt;!-- Общая стоимость позиции --&gt;")
                .nth(1)
                .and_then(|x| x.split("&lt;!-- /Общая стоимость позиции --&gt;").next())
            {
                if let Some(x) = data
                    .split("&lt;span")
                    .nth(2)
                    .and_then(|x| x.split("&quot;&gt;").nth(1))
                    .and_then(|x| x.split("&lt;/span&gt;").next())
                    .and_then(|x| x.replace('.', "").parse::<u64>().ok())
                {
                    item.total = x;
                }
            }
            if let Some(data) = data
                .split("&lt;!-- Сумма НДС за предмет расчета --&gt;")
                .nth(1)
                .and_then(|x| {
                    x.split("&lt;!-- /Сумма НДС за предмет расчета --&gt;")
                        .next()
                })
            {
                if let Some(x) = data
                    .split("&lt;span")
                    .nth(2)
                    .and_then(|x| x.split("&quot;&gt;").nth(1))
                    .and_then(|x| x.split("&lt;/span&gt;").next())
                    .and_then(|x| x.replace('.', "").parse::<u64>().ok())
                {
                    if item.total != x {
                        item.tax = x;
                    }
                }
            }
            ret.items.push(item);
        }
        ret.items.pop();
    }
    if let Some(data) = data
        .split("&lt;!-- ИТОГ --&gt;")
        .nth(1)
        .and_then(|x| x.split("&lt;!-- /ИТОГ --&gt;").next())
    {
        if let Some(x) = data
            .split("&lt;span&gt;")
            .nth(1)
            .and_then(|x| x.split("&lt;/span&gt;").next())
            .and_then(|x| x.replace('.', "").parse::<u64>().ok())
        {
            ret.total = x;
        }
    }
    if let Some(data) = data
        .split("&lt;!-- ИТОГ - Тело таблицы --&gt;")
        .nth(1)
        .and_then(|x| x.split("&lt;!-- /ИТОГ - Тело таблицы --&gt;").next())
    {
        let mut it = data
            .split("block&quot;&gt;")
            .skip(1)
            .map(|x| x.split("&lt;/span&gt;").next());
        while let Some(k) = it.next() {
            let v = it.next();
            let (Some(k), Some(Some(v))) = (k, v) else {
                continue;
            };
            let Ok(v) = v.replace('.', "").parse::<u64>() else {
                continue;
            };
            match k {
                "НАЛИЧНЫМИ" => ret.total_cash = v,
                "БЕЗНАЛИЧНЫМИ" => ret.total_card = v,
                x if x.starts_with("СУММА НДС ЧЕКА ПО СТАВКЕ") => {
                    ret.total_tax += v
                }
                _ => {}
            }
        }
    }
    Some(())
}

async fn fetch(config: &Config, rec: &mut Receipt) -> reqwest::Result<bool> {
    if let Some(fnifp) = rec.fnifp() {
        let mut path = config.data_path.clone();
        path.push("parsed");
        path.push(fnifp + ".json");
        if let Ok(data) = tokio::fs::read(path).await {
            if let Ok(parsed) = serde_json::from_slice::<Receipt>(&data) {
                *rec = parsed;
                return Ok(true);
            }
        }
    }
    let client = reqwest::Client::builder()
        .user_agent("Mozilla/5.0 (Windows NT 10.0; rv:109.0) Gecko/20100101 Firefox/118.0")
        .cookie_store(true)
        .build()?;
    let url = format!(
        "https://lk.platformaofd.ru/web/noauth/cheque/search?fn={}&fp={}&i={}",
        rec.r#fn, rec.fp, rec.i
    );
    let req = client.get(&url).build()?;
    let form_res = client.execute(req).await?;
    let form = form_res.text().await?;
    let mut succ = false;
    if let Some(captcha) = form
        .split("class=\"form-captcha-image\" src=\"")
        .nth(1)
        .and_then(|x| x.split('"').next())
    {
        let req = client
            .get(format!("https://lk.platformaofd.ru{captcha}"))
            .build()?;
        let captcha_res = client.execute(req).await?;
        let captcha_img = captcha_res.bytes().await?;
        let mut api = leptess::tesseract::TessApi::new(None, "fin").unwrap();
        api.raw
            .set_variable(
                leptess::Variable::TesseditCharWhitelist.as_cstr(),
                CStr::from_bytes_with_nul(b"0123456789\0").unwrap(),
            )
            .unwrap();
        if let Ok(pix) = leptess::leptonica::pix_read_mem(&captcha_img) {
            api.set_image(&pix);
        }
        let captcha = api
            .get_utf8_text()
            .unwrap_or_default()
            .as_str()
            .chars()
            .filter(|x| x.is_ascii_digit())
            .collect::<String>();
        println!("captcha: {captcha}");
        if let Some(csrf) = form
            .split("type=\"hidden\" name=\"_csrf\" value=\"")
            .nth(1)
            .and_then(|x| x.split('"').next())
        {
            let req = client
                .post("https://lk.platformaofd.ru/web/noauth/cheque/search")
                .form(&{
                    let mut form = HashMap::new();
                    form.insert("fn", rec.r#fn.clone());
                    form.insert("fp", rec.fp.clone());
                    form.insert("i", rec.i.clone());
                    form.insert("captcha", captcha);
                    form.insert("_csrf", csrf.to_owned());
                    form
                })
                .header("Referer", url)
                .header("Origin", "https://lk.platformaofd.ru")
                .build()?;
            let res = client.execute(req).await?;
            if res.url().path().ends_with("/id") {
                rec.id = res
                    .url()
                    .query_pairs()
                    .find_map(|(k, v)| (k == "id").then_some(v))
                    .unwrap_or_default()
                    .into_owned();
                let text = res.text().await?;
                let mut path = config.data_path.clone();
                path.push("raw");
                path.push(rec.id.clone() + ".html");
                if let Err(err) = tokio::fs::write(path, &text).await {
                    log::error!("failed to write raw receipt: {err:?}");
                }
                parse(&text, rec);
                if let Some(fnifp) = rec.fnifp() {
                    let mut path = config.data_path.clone();
                    path.push("parsed");
                    path.push(fnifp + ".json");
                    match serde_json::to_vec(rec) {
                        Ok(rec) => {
                            if let Err(err) = tokio::fs::write(path, &rec).await {
                                log::error!("failed to write receipt cache: {err:?}");
                            }
                        }
                        Err(err) => {
                            log::error!("failed to serialize receipt: {err:?}");
                        }
                    }
                }
                succ = true;
            }
        }
    }
    Ok(succ)
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

#[derive(Clone, Debug, Deserialize, Serialize)]
enum TransactionMeta {
    Receipt {
        r#fn: String,
        i: String,
        fp: String,
        paid: HashMap<String, BTreeSet<usize>>,
    },
    Comment(String),
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
    let mut path = config.data_path.clone();
    path.push("transactions");
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
    for (k, v) in tr.balance_changes.iter() {
        let x = lock.entry(k.to_owned()).or_default();
        *x = x.checked_add(*v).expect("balance overflowed");
    }
    lock.retain(|_, v| *v != 0);
    lock.clone()
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
struct Config {
    usernames: IndexSet<String>,
    listener: String,
    data_path: PathBuf,
    ignore_qr_condition: String,
}

fn copy_ref<T: ?Sized>(t: &T) -> &T {
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

#[derive(Clone, Debug, Deserialize, Serialize)]
struct ListItem {
    name: String,
    amount: f64,
}

async fn save_list(config: &Config, list: &[ListItem]) -> io::Result<()> {
    let mut path1 = config.data_path.clone();
    let mut path2 = config.data_path.clone();
    path1.push("list.json.tmp");
    path2.push("list.json");
    tokio::fs::write(
        &path1,
        serde_json::to_string(list).map_err(|err| io::Error::new(io::ErrorKind::Other, err))?,
    )
    .await?;
    tokio::fs::rename(path1, path2).await
}

#[tokio::main]
async fn main() {
    let config_path = std::env::vars_os()
        .find(|(k, _)| k == "CONFIG_FILE")
        .map(|(_, v)| v)
        .unwrap_or("config.json".into());
    let config = tokio::fs::read(config_path)
        .await
        .expect("failed to read config.json");
    let config = serde_json::from_slice::<Config>(&config).expect("invalid config.json");
    let config = Box::leak(Box::new(config));
    let data_path = |x| {
        let mut ret = config.data_path.clone();
        ret.push(x);
        ret
    };

    let (a, b, c) = tokio::join!(
        tokio::fs::create_dir_all(data_path("raw")),
        tokio::fs::create_dir_all(data_path("parsed")),
        tokio::fs::create_dir_all(data_path("transactions")),
    );
    a.expect("failed to create data/raw");
    b.expect("failed to create data/parsed");
    c.expect("failed to create data/transactions");

    // static
    let style = &*Box::leak(Box::new(
        tokio::fs::read_to_string("static/style.css")
            .await
            .unwrap_or_else(|_| include_str!("../static/style.css").to_owned()),
    ));
    let fzf = &*Box::leak(Box::new(
        tokio::fs::read_to_string("static/fzf.js")
            .await
            .unwrap_or_else(|_| include_str!("../static/fzf.js").to_owned()),
    ));
    let qr_scanner_worker = &*Box::leak(Box::new(
        tokio::fs::read_to_string("static/qr-scanner-worker.min.js")
            .await
            .unwrap_or_else(|_| include_str!("../static/qr-scanner-worker.min.js").to_owned()),
    ));
    let qr_scanner_worker_map = &*Box::leak(Box::new(
        tokio::fs::read_to_string("static/qr-scanner-worker.min.js.map")
            .await
            .unwrap_or_else(|_| include_str!("../static/qr-scanner-worker.min.js.map").to_owned()),
    ));
    let qr_scanner_umd = &*Box::leak(Box::new(
        tokio::fs::read_to_string("static/qr-scanner.umd.min.js")
            .await
            .unwrap_or_else(|_| include_str!("../static/qr-scanner.umd.min.js").to_owned()),
    ));
    let qr_scanner_umd_map = &*Box::leak(Box::new(
        tokio::fs::read_to_string("static/qr-scanner.umd.min.js.map")
            .await
            .unwrap_or_else(|_| include_str!("../static/qr-scanner.umd.min.js.map").to_owned()),
    ));

    // templates
    let parser = liquid::ParserBuilder::with_stdlib()
        .filter(CurrencyFilter)
        .build()
        .unwrap();
    let root_t = &*Box::leak(Box::new(
        parser
            .parse(
                &tokio::fs::read_to_string("templates/index.html")
                    .await
                    .unwrap_or_else(|_| include_str!("../templates/index.html").to_owned()),
            )
            .unwrap(),
    ));
    let submitted_t = &*Box::leak(Box::new(
        parser
            .parse(
                &tokio::fs::read_to_string("templates/submitted.html")
                    .await
                    .unwrap_or_else(|_| include_str!("../templates/submitted.html").to_owned()),
            )
            .unwrap(),
    ));
    let add_t = &*Box::leak(Box::new(
        parser
            .parse(
                &tokio::fs::read_to_string("templates/add.html")
                    .await
                    .unwrap_or_else(|_| include_str!("../templates/add.html").to_owned()),
            )
            .unwrap(),
    ));
    let list_t = &*Box::leak(Box::new(
        parser
            .parse(
                &tokio::fs::read_to_string("templates/list.html")
                    .await
                    .unwrap_or_else(|_| include_str!("../templates/list.html").to_owned()),
            )
            .unwrap(),
    ));

    let list: &RwLock<Vec<ListItem>> = &*Box::leak(Box::new(RwLock::new(
        serde_json::from_str(
            &tokio::fs::read_to_string(data_path("list.json"))
                .await
                .unwrap_or_else(|_| "[]".to_owned()),
        )
        .unwrap(),
    )));

    let mut balance = HashMap::<String, i64>::new();
    let paid_receipts = Box::leak(Box::new(dashmap::DashSet::<String>::new()));
    let path = data_path("transactions");
    let mut dir = tokio::fs::read_dir(path)
        .await
        .expect("failed to read transaction list");
    while let Some(file) = dir
        .next_entry()
        .await
        .expect("failed to read transaction list entry")
    {
        if !matches!(file.path().extension().and_then(|x| x.to_str()).map(|x| x.to_lowercase()), Some(x) if x.as_str() == "json")
        {
            continue;
        }
        let data = tokio::fs::read(file.path())
            .await
            .expect("failed to read transaction");
        let tr = serde_json::from_slice::<Transaction>(&data).unwrap_or_else(|_| {
            panic!(
                "failed to deserialize transaction {}",
                file.path().display()
            )
        });
        if let Some(TransactionMeta::Receipt {
            r#fn,
            i,
            fp,
            paid: _,
        }) = tr.meta
        {
            paid_receipts.insert(format!("{}_{}_{}", r#fn, i, fp));
        }
        for (k, v) in tr.balance_changes.iter() {
            let x = balance.entry(k.to_owned()).or_default();
            *x = x.checked_add(*v).expect("balance overflowed");
        }
    }
    balance.retain(|_, v| *v != 0);
    BALANCE.set(balance.into()).unwrap();

    let units = &*Box::leak(Box::new(DashMap::<String, String>::new()));
    let path = data_path("parsed");
    let mut dir = tokio::fs::read_dir(path)
        .await
        .expect("failed to read receipt list");
    while let Some(file) = dir
        .next_entry()
        .await
        .expect("failed to read receipt list entry")
    {
        if !matches!(file.path().extension().and_then(|x| x.to_str()).map(|x| x.to_lowercase()), Some(x) if x.as_str() == "json")
        {
            continue;
        }
        let data = tokio::fs::read(file.path())
            .await
            .expect("failed to read transaction");
        let rec = serde_json::from_slice::<Receipt>(&data)
            .unwrap_or_else(|err| panic!("failed to deserialize receipt {}: {err}", file.path().display()));
        for item in rec.items {
            units.insert(item.name, item.unit);
        }
    }

    let app = axum::Router::new()
        .route(
            "/",
            axum::routing::get(|| {
                let config = copy_ref(config);
                let root = copy_ref(root_t);
                async move {
                    axum::response::Html::from(root.render(&liquid::object!({
                        "extra_qr_processing": format!("if({})return;", config.ignore_qr_condition),
                        "usernames": &config.usernames,
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
                    async move {
                        (
                            [(
                                axum::http::header::CONTENT_TYPE,
                                axum::http::HeaderValue::from_static("application/json"),
                            )],
                            {
                                let meta = f
                                    .get("comment")
                                    .map(|x| TransactionMeta::Comment(x.to_owned()));
                                if let Some(to) = f.get("to") {
                                    if let Some(amt) = f.get("amount") {
                                        if let Some(amt) =
                                            amt.parse::<i64>().ok().filter(|x| *x != 0)
                                        {
                                            if let Some(from) = f.get("from") {
                                                let mut tr = Transaction::new(meta);
                                                tr.pay(from, to, amt);
                                                tr.finalize();
                                                serde_json::to_string(
                                                    &add_transaction(config, tr).await,
                                                )
                                                .expect("balance serialization failed")
                                            } else {
                                                let mut tr = Transaction::new(meta);
                                                let total = amt;
                                                for user in &config.usernames {
                                                    if user != to {
                                                        tr.pay(
                                                            user,
                                                            to,
                                                            total
                                                                / i64::try_from(
                                                                    config.usernames.len(),
                                                                )
                                                                .expect(
                                                                    "usize->i64 conversion failed",
                                                                ),
                                                        );
                                                    }
                                                }
                                                tr.finalize();
                                                serde_json::to_string(
                                                    &add_transaction(config, tr).await,
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
                let units = copy_ref(units);
                let list_t = copy_ref(list_t);
                let list = copy_ref(list);
                async move {
                    let items = units
                        .iter()
                        .map(|item| {
                            liquid::object!({
                                "name": item.key(),
                                "unit": item.value(),
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
                                        "unit": units.get(&x.name).map(|x| format!(" {}", x.value())).unwrap_or_default()
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
                            if let Some(amount) = f.get("amount").and_then(|x| x.parse::<f64>().ok()) {
                                let mut list = list.write().await;
                                let mut added = false;
                                for item in &mut *list {
                                    if &item.name == name {
                                        item.amount += amount;
                                        added = true;
                                    }
                                }
                                if !added {
                                    list.push(ListItem { name: name.to_owned(), amount });
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
                    let submitted = copy_ref(submitted_t);
                    let units = copy_ref(units);
                    let list = copy_ref(list);
                    async move {
                        let Some(r#fn) = f.get("fn") else {
                            return axum::response::Html::from("missing fn".to_owned());
                        };
                        let Some(i) = f.get("i") else {
                            return axum::response::Html::from("missing i".to_owned());
                        };
                        let Some(fp) = f.get("fp") else {
                            return axum::response::Html::from("missing fp".to_owned());
                        };
                        let Some(username) = f.get("username") else {
                            return axum::response::Html::from("missing username".to_owned());
                        };
                        let mut path = config.data_path.clone();
                        path.push("parsed");
                        path.push(format!("{}_{}_{}.json", r#fn, i, fp).replace(['/', '\\'], ""));
                        let Ok(data) = tokio::fs::read(path).await else {
                            return axum::response::Html::from("missing recept cache".to_owned());
                        };
                        let Ok(rec) = serde_json::from_slice::<Receipt>(&data) else {
                            return axum::response::Html::from("invalid recept cache".to_owned());
                        };
                        {
                            let mut list = list.write().await;
                            list.retain_mut(|list_item| {
                                for item in &rec.items {
                                    if item.name == list_item.name {
                                        list_item.amount -= item.count;
                                    }
                                }
                                list_item.amount > 0.0001
                            });
                            let _ = save_list(config, &list).await;
                        }
                        let mut paid = HashMap::<String, BTreeSet<usize>>::new();
                        let mut per_item = HashMap::<usize, HashSet<String>>::new();
                        for (k, v) in f.iter() {
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
                            v.sort();
                        }
                        let mut tr = Transaction::new(Some(TransactionMeta::Receipt {
                            r#fn: r#fn.to_owned(),
                            i: i.to_owned(),
                            fp: fp.to_owned(),
                            paid,
                        }));
                        for (k, v) in &groups {
                            let total: u128 = v
                                .iter()
                                .filter_map(|x| rec.items.get(*x))
                                .map(|x| x.total as u128)
                                .sum();
                            let total = u64::try_from(total).expect("receipt price too big");
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
                        tr.finalize();
                        let bal = add_transaction(config, tr).await;
                        for item in &rec.items {
                            units.insert(item.name.clone(), item.unit.clone());
                        }
                        if let Some(fnifp) = rec.fnifp() {
                            paid_receipts.insert(fnifp);
                        }
                        let mut bal = bal.into_iter().collect::<Vec<_>>();
                        bal.sort_by_key(|(k, _)| {
                            config
                                .usernames
                                .iter()
                                .enumerate()
                                .find(|(_, x)| &k == x)
                                .map(|x| x.0)
                        });
                        let bal = bal
                            .into_iter()
                            .map(|x| {
                                liquid::object!({
                                    "username": x.0,
                                    "balance": x.1,
                                })
                            })
                            .collect::<Vec<_>>();
                        axum::response::Html::from(
                            submitted
                                .render(&liquid::object!({
                                    "balance": bal,
                                    "username": username,
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
                    let add = copy_ref(add_t);
                    async move {
                        axum::response::Html::from(if let Some(q) = q {
                            if let Some(username) = cookies.get("username").map(|x| x.value()) {
                                let mut rec = parse_qr(&q);
                                if rec.r#fn.is_empty()
                                    || rec.fp.is_empty()
                                    || rec.i.is_empty() && q.starts_with("http")
                                {
                                    if let Ok(res) = reqwest::get(q).await {
                                        if let Ok(text) = res.text().await {
                                            if let Some(x) = text
                                                .split("<div>ФН №: <right>")
                                                .nth(1)
                                                .and_then(|x| x.split('<').next())
                                            {
                                                rec.r#fn = x.to_owned();
                                            }
                                            if let Some(x) = text
                                                .split("<div>ФП: <right>")
                                                .nth(1)
                                                .and_then(|x| x.split('<').next())
                                            {
                                                rec.fp = x.to_owned();
                                            }
                                            if let Some(x) = text
                                                .split("<div>ФД №: <right>")
                                                .nth(1)
                                                .and_then(|x| x.split('<').next())
                                            {
                                                rec.i = x.to_owned();
                                            }
                                            if rec.n.is_empty() {
                                                rec.n = "1".to_owned();
                                            }
                                            if let Some(x) = text
                                                .split("<div>Дата Время <right>")
                                                .nth(1)
                                                .and_then(|x| x.split('<').next())
                                            {
                                                // 09.11.2023 18:09 -> 20231109T1809
                                                let x = x.trim();
                                                rec.date.clear();
                                                if let Some((date, time)) = x.split_once(' ') {
                                                    for x in date.split('.').rev() {
                                                        rec.date += x;
                                                    }
                                                    rec.date += "T";
                                                    for x in time.split(':').take(2) {
                                                        rec.date += x;
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                                if let Some(fnifp) = rec.fnifp() {
                                    let (tx, mut rx) = mpsc::channel(1);
                                    for _ in 0..8 {
                                        let mut rec = rec.clone();
                                        let tx = tx.clone();
                                        let config = copy_ref(config);
                                        tokio::spawn(async move {
                                            for _ in 0..8 {
                                                if let Ok(true) = fetch(config, &mut rec).await {
                                                    let _ = tx.try_send(rec);
                                                    break;
                                                }
                                                if tx.is_closed() {
                                                    break;
                                                }
                                            }
                                        });
                                    }
                                    drop(tx);
                                    if let Some(rec) = rx.recv().await {
                                        drop(rx);
                                        add.render(&liquid::object!({
                                            "total": rec.total,
                                            "username": username,
                                            "already_paid": paid_receipts.contains(&fnifp),
                                            "fn": rec.r#fn,
                                            "i": rec.i,
                                            "fp": rec.fp,
                                            "items": rec.items.iter().enumerate().map(|(i, item)| {
                                                liquid::object!({
                                                    "num": i,
                                                    "name": item.name,
                                                    "count": item.count,
                                                    "unit": item.unit,
                                                    "per_item": item.per_item,
                                                    "total": item.total,
                                                })
                                            }).collect::<Vec<_>>(),
                                            "usernames": &config.usernames,
                                        }))
                                        .unwrap_or_else(|err| format!("Error: {err}"))
                                    } else {
                                        "error".to_owned()
                                    }
                                } else {
                                    "error".to_owned()
                                }
                            } else {
                                "missing username cookie".to_owned()
                            }
                        } else {
                            "missing qr info".to_owned()
                        })
                    }
                },
            ),
        );
    axum::Server::bind(&config.listener.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
