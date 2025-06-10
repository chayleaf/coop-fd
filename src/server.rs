use std::{
    collections::{BTreeSet, HashMap, HashSet},
    convert::Infallible,
    future::Future,
    sync::Arc,
};

use axum::{response::IntoResponse, routing::MethodRouter};
use dashmap::{DashMap, DashSet};
use fiscal_data::{enums::PaymentType, fields, Document, TlvType};
use liquid::Template;
use tokio::sync::RwLock;

use crate::{
    add_transaction, is_advance, item_is_advance, ofd, parse_qr, parse_sum, save_list,
    CEscapeFilter, Comment, Commodity, Config, CurrencyFilter, ListItem, Transaction,
    TransactionMeta,
};

type AxumState = axum::extract::State<State>;

pub struct FileRes<T> {
    #[cfg(not(debug_assertions))]
    value: Option<Arc<T>>,
    #[cfg(debug_assertions)]
    compute: Arc<
        dyn 'static
            + Send
            + Sync
            + Fn() -> std::pin::Pin<Box<dyn 'static + Future<Output = T> + Send + Sync>>,
    >,
}

impl<T> Clone for FileRes<T> {
    fn clone(&self) -> Self {
        Self {
            #[cfg(not(debug_assertions))]
            value: self.value.clone(),
            #[cfg(debug_assertions)]
            compute: self.compute.clone(),
        }
    }
}

impl<T: 'static> FileRes<T> {
    pub async fn new<F: 'static + Future<Output = T> + Send + Sync>(
        compute: impl 'static + Send + Sync + Fn() -> F,
    ) -> Self {
        Self {
            #[cfg(not(debug_assertions))]
            value: Some(Arc::new(compute().await)),
            #[cfg(debug_assertions)]
            compute: Arc::new(move || Box::pin(compute())),
        }
    }
    pub async fn get(&self) -> Arc<T> {
        #[cfg(not(debug_assertions))]
        {
            self.value.clone().unwrap()
        }
        #[cfg(debug_assertions)]
        {
            Arc::new((self.compute)().await)
        }
    }
}

macro_rules! file_res {
    ($s:literal) => {
        async {
            FileRes::new(|| async {
                tokio::fs::read_to_string($s)
                    .await
                    .unwrap_or_else(|_| include_str!(concat!("../", $s)).to_owned())
            })
            .await
        }
    };
    ($parser:expr; $s:literal) => {{
        let parser = $parser.clone();
        async move {
            let parser = parser.clone();
            FileRes::new(move || {
                let parser = parser.clone();
                async move {
                    parser
                        .parse(
                            &tokio::fs::read_to_string($s)
                                .await
                                .unwrap_or_else(|_| include_str!(concat!("../", $s)).to_owned()),
                        )
                        .unwrap_or_else(|err| panic!("{}: \n{err}", $s))
                }
            })
            .await
        }
    }};
}

impl<T> Default for FileRes<T> {
    fn default() -> Self {
        Self {
            #[cfg(not(debug_assertions))]
            value: None,
            #[cfg(debug_assertions)]
            compute: Arc::new(move || panic!("resource not properly initialized")),
        }
    }
}

#[derive(Clone, Default)]
pub struct State {
    pub config: Arc<Config>,
    pub style: FileRes<String>,
    pub fzf: FileRes<String>,
    pub qr_scanner_worker: FileRes<String>,
    pub qr_scanner_worker_map: FileRes<String>,
    pub qr_scanner_umd: FileRes<String>,
    pub qr_scanner_umd_map: FileRes<String>,
    pub root_t: FileRes<Template>,
    pub submitted_t: FileRes<Template>,
    pub add_t: FileRes<Template>,
    pub list_t: FileRes<Template>,
    pub balance: Arc<RwLock<HashMap<String, i64>>>,
    pub list: Arc<RwLock<Vec<ListItem>>>,
    pub commodities: Arc<DashMap<String, Commodity>>,
    pub comments: Arc<DashMap<String, Comment>>,
    pub paid_receipts: Arc<DashSet<String>>,
}

impl State {
    pub async fn new(config: Config) -> Self {
        let parser = Arc::new(
            liquid::ParserBuilder::with_stdlib()
                .filter(CurrencyFilter)
                .filter(CEscapeFilter)
                .build()
                .unwrap(),
        );
        let comments = DashMap::<String, Comment>::new();
        let paid_receipts = DashSet::<String>::new();
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
            balance,
            commodities,
        ) = tokio::join!(
            file_res!("static/style.css"),
            file_res!("static/fzf.js"),
            file_res!("static/qr-scanner-worker.min.js"),
            file_res!("static/qr-scanner-worker.min.js.map"),
            file_res!("static/qr-scanner.umd.min.js"),
            file_res!("static/qr-scanner.umd.min.js.map"),
            file_res!(parser; "templates/index.html"),
            file_res!(parser; "templates/submitted.html"),
            file_res!(parser; "templates/add.html"),
            file_res!(parser; "templates/list.html"),
            async {
                RwLock::new(
                    serde_json::from_str::<Vec<ListItem>>(
                        &tokio::fs::read_to_string(config.data_path("list.json"))
                            .await
                            .unwrap_or_else(|_| "[]".to_owned()),
                    )
                    .unwrap(),
                )
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
                balance
            },
            async {
                let commodities = DashMap::<String, Commodity>::new();

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
                commodities
            }
        );

        Self {
            config: config.into(),
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
            list: list.into(),
            balance: Arc::new(balance.into()),
            commodities: commodities.into(),
            comments: comments.into(),
            paid_receipts: paid_receipts.into(),
        }
    }
}

pub fn js(
    getter: fn(&State) -> &FileRes<String>,
) -> MethodRouter<State, axum::body::Body, Infallible> where {
    res_content_type(getter, "text/javascript")
}

pub fn json(
    getter: fn(&State) -> &FileRes<String>,
) -> MethodRouter<State, axum::body::Body, Infallible> where {
    res_content_type(getter, "application/json")
}

pub fn css(
    getter: fn(&State) -> &FileRes<String>,
) -> MethodRouter<State, axum::body::Body, Infallible> where {
    res_content_type(getter, "text/css")
}

pub fn res_content_type(
    getter: fn(&State) -> &FileRes<String>,
    content_type: &'static str,
) -> MethodRouter<State, axum::body::Body, Infallible> where {
    axum::routing::get(move |state: AxumState| async move {
        (
            [(
                axum::http::header::CONTENT_TYPE,
                axum::http::HeaderValue::from_static(content_type),
            )],
            (*getter(&state).get().await).clone(),
        )
            .into_response()
    })
}

pub async fn root(state: AxumState) -> axum::response::Html<String> {
    let comments = state
        .comments
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
    axum::response::Html::from(
        state
            .root_t
            .get()
            .await
            .render(&liquid::object!({
                "comments": comments,
                "extra_qr_processing": format!("if({})return;", state.config.ignore_qr_condition),
                "usernames": &state.config.usernames,
                "ofds": ofd::registry()
                    .await
                    .all()
                    .filter(|v| !v.name().is_empty())
                    .map(|v| liquid::object!({
                        "id": v.id(),
                        "name": format!("{} ({})", v.name(), v.url()),
                    }))
                    .collect::<Vec<liquid::Object>>(),
            }))
            .unwrap_or_else(|err| format!("Error: {err}")),
    )
}

pub fn json_endpoint_get<F: 'static + Send + Sync + Future<Output = String>>(
    func: fn(State) -> F,
) -> MethodRouter<State, axum::body::Body, Infallible> where {
    axum::routing::get(move |state: AxumState| async move {
        (
            [(
                axum::http::header::CONTENT_TYPE,
                axum::http::HeaderValue::from_static("application/json"),
            )],
            func(state.0).await,
        )
            .into_response()
    })
}

pub async fn api_pay(
    axum::extract::State(state): AxumState,
    axum::extract::Form(f): axum::extract::Form<HashMap<String, String>>,
) -> axum::http::Response<http_body::combinators::UnsyncBoxBody<axum::body::Bytes, axum::Error>> {
    (
        [(
            axum::http::header::CONTENT_TYPE,
            axum::http::HeaderValue::from_static("application/json"),
        )],
        {
            if let Some(to) = f.get("to") {
                if let Some(amt) = f.get("amount") {
                    if let Some(amt) = amt.parse::<i64>().ok().filter(|x| *x != 0) {
                        let mut tr = f.get("comment").map_or_else(
                            || Transaction::new(None),
                            |x| {
                                let mut val = state.comments.entry(x.clone()).or_default();
                                let val = val.value_mut();
                                val.last_price = amt;
                                val.count += 1;
                                let tr = Transaction::new(Some(TransactionMeta::Comment2(
                                    x.clone(),
                                    amt,
                                )));
                                val.last_time = tr.date;
                                tr
                            },
                        );
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
                                    tr.pay(user, to, amt / payers_len);
                                }
                            }
                        } else if let Some(from) = f.get("from") {
                            tr.pay(from, to, amt);
                        } else {
                            for user in &state.config.usernames {
                                if user != to {
                                    tr.pay(
                                        user,
                                        to,
                                        amt / i64::try_from(state.config.usernames.len())
                                            .expect("usize->i64 conversion failed"),
                                    );
                                }
                            }
                        }
                        tr.finalize();
                        let balance = add_transaction(&state, tr).await;
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
                                state
                                    .submitted_t
                                    .get()
                                    .await
                                    .render(&liquid::object!({
                                        "prefix": "..",
                                        "balance": balance,
                                        "username": to,
                                        "removed": [],
                                    }))
                                    .unwrap_or_else(|err| format!("Error: {err}")),
                            )
                            .into_response();
                        }
                        serde_json::to_string(&balance).expect("balance serialization failed")
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

pub async fn list(axum::extract::State(state): AxumState) -> axum::response::Html<String> {
    let items = state
        .commodities
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
        state
            .list_t
            .get()
            .await
            .render(&liquid::object!({
                "items": items,
                "list": state.list.read().await.iter().map(|x| {
                    liquid::object!({
                        "name": x.name,
                        "amount": x.amount,
                        "unit": state.commodities
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

pub async fn listremove(
    axum::extract::State(state): AxumState,
    axum::extract::Form(f): axum::extract::Form<HashMap<String, String>>,
) -> axum::response::Redirect {
    if let Some(name) = f.get("name") {
        let mut list = state.list.write().await;
        list.retain(|x| &x.name != name);
        let _ = save_list(&state.config, &list).await;
    }
    axum::response::Redirect::to("list")
}

pub async fn listadd(
    axum::extract::State(state): AxumState,
    axum::extract::Form(f): axum::extract::Form<HashMap<String, String>>,
) -> axum::response::Redirect {
    if let Some(name) = f.get("name") {
        if let Some(amount) = f.get("amount").and_then(|x| x.parse::<f64>().ok()) {
            let mut list = state.list.write().await;
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
            let _ = save_list(&state.config, &list).await;
        }
    }
    axum::response::Redirect::to("list")
}

pub async fn submit(
    axum::extract::State(state): AxumState,
    axum::extract::Form(f): axum::extract::Form<HashMap<String, String>>,
) -> axum::response::Html<String> {
    let Some(r#fn) = f
        .get("fn")
        .filter(|x| x.bytes().all(|x| x.is_ascii_digit()))
    else {
        return axum::response::Html::from("missing fn".to_owned());
    };
    let Some(i) = f.get("i").and_then(|x| x.parse::<u32>().ok()) else {
        return axum::response::Html::from("missing i".to_owned());
    };
    let Some(username) = f.get("username") else {
        return axum::response::Html::from("missing username".to_owned());
    };
    let path = state.config.data_path(format!("ffd/{fn}_{i:07}.tlv"));
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
        Some(PaymentType::Sale | PaymentType::PurchaseReturn) => false,
        Some(PaymentType::Purchase | PaymentType::SaleReturn) => true,
        _ => return axum::response::Html::from("invalid payment type".to_owned()),
    };
    let mut removed = Vec::<String>::new();
    if !invert {
        let mut list = state.list.write().await;
        list.retain_mut(|list_item| {
            let lower = list_item.name.to_lowercase();
            let len = lower.chars().count();
            for item in rec.get_all::<fields::ReceiptItem>().unwrap_or_default() {
                let Ok(Some(name)) = item.get::<fields::ItemName>() else {
                    continue;
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
        let _ = save_list(&state.config, &list).await;
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
                item.get::<fields::ItemTotalPrice>()
                    .ok()
                    .flatten()
                    .unwrap_or_default()
            })
            .sum();
        for user in k {
            if user != username {
                tr.pay(
                    user,
                    username,
                    (total / u64::try_from(k.len()).expect("wtf, 128-bit cpus???"))
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
    let balance = add_transaction(&state, tr).await;
    let date = rec.get::<fields::DateTime>().ok().flatten();
    for item in &items {
        let name = item
            .get::<fields::ItemName>()
            .ok()
            .flatten()
            .expect("failed to read item name");
        let mut val = state.commodities.entry(name).or_default();
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
    state.paid_receipts.insert(format!("{fn}_{i:07}"));
    let mut balance = balance.into_iter().collect::<Vec<_>>();
    balance.sort_by_key(|(k, _)| {
        state
            .config
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
        state
            .submitted_t
            .get()
            .await
            .render(&liquid::object!({
                "prefix": ".",
                "balance": balance,
                "username": username,
                "removed": removed,
            }))
            .unwrap_or_else(|err| format!("Error: {err}")),
    )
}

pub async fn add(
    axum::extract::RawQuery(q): axum::extract::RawQuery,
    cookies: axum_extra::extract::CookieJar,
    axum::extract::State(state): AxumState,
) -> axum::http::Response<http_body::combinators::UnsyncBoxBody<axum::body::Bytes, axum::Error>> {
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
                                let _ = rec
                                    .set::<fiscal_data::fields::DocFiscalSign>([a, b, c, d, e, f]);
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
                            .and_then(|x| {
                                chrono::NaiveDateTime::parse_from_str(x, "%d.%m.%Y %H:%M").ok()
                            })
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
            match ofd::fetch(&state, rec).await {
                Ok(doc) => {
                    let rec = doc.data();
                    let r#fn = rec
                        .get::<fields::DriveNum>()
                        .ok()
                        .flatten()
                        .unwrap_or_default();
                    let i = rec
                        .get::<fields::DocNum>()
                        .ok()
                        .flatten()
                        .unwrap_or_default();
                    let payment_type = rec
                        .get::<fields::PaymentType>()
                        .ok()
                        .flatten()
                        .unwrap_or_default();
                    let invert = match payment_type {
                        PaymentType::Sale | PaymentType::PurchaseReturn => false,
                        PaymentType::Purchase | PaymentType::SaleReturn => true,
                        PaymentType::Unknown => {
                            return axum::response::Html::from("invalid payment type".to_owned())
                                .into_response()
                        }
                    };
                    let inv = |x: u64| if invert { -(x as i64) } else { x as i64 };
                    let inv_f = |x: f64| if invert { -x } else { x };
                    state.add_t.get().await.render(&liquid::object!({
                        "total": rec.get::<fields::TotalSum>().ok().flatten().unwrap_or_default(),
                        "username": username,
                        "already_paid": state.paid_receipts.contains(&format!("{fn}_{i:07}")),
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
                        "usernames": &state.config.usernames,
                    }))
                    .unwrap_or_else(|err| format!("Error: {err}"))
                }
                Err(ofd::Error::Redirect(url)) => {
                    return axum::response::Redirect::to(&url).into_response();
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
    })
    .into_response()
}
