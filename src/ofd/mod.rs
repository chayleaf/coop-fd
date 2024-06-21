use async_trait::async_trait;
use fiscal_data::{fields, Document, Object, TlvType};
use std::{collections::BTreeMap, io};
use thiserror::Error;
use tokio::sync::OnceCell;

use crate::Config;

pub mod custom {
    pub enum Id {}
    impl fiscal_data::internal::FieldInternal for Id {
        const TAG: u16 = 29000;
        type Type = String;
    }
    pub enum ProviderId {}
    impl fiscal_data::internal::FieldInternal for ProviderId {
        const TAG: u16 = 29001;
        type Type = String;
    }
    impl fiscal_data::MultiField for ProviderId {}
    pub enum SessionId {}
    impl fiscal_data::internal::FieldInternal for SessionId {
        const TAG: u16 = 29002;
        type Type = String;
    }
    pub enum FdId {}
    impl fiscal_data::internal::FieldInternal for FdId {
        const TAG: u16 = 29003;
        type Type = u64;
    }
    pub enum IcomCode {}
    impl fiscal_data::internal::FieldInternal for IcomCode {
        const TAG: u16 = 29004;
        type Type = [u8; 3];
    }
}

mod astral;
// json, theoretically can give tlv but in practice it doesn't give tlv to mere mortals
// mod beeline;
// json, close to fns (changed user -> client_name, ФПС is 0)
// mod eofd;
mod irkkt_mobile;
// not an OFD, just a provider that returns a single URL
mod icom24;
// html
// mod magnit;
// json, !has full fiscal sign in base64!
mod ofd_ru;
// html
// mod platforma_ofd;
// json
// mod proverkacheka;
// json
mod private1;
// html
mod taxcom;

#[derive(Debug, Error)]
pub enum Error {
    #[error("{0}")]
    Reqwest(
        #[source]
        #[from]
        reqwest::Error,
    ),
    #[error("{0}")]
    ReqwestHeaderValue(
        #[source]
        #[from]
        reqwest::header::InvalidHeaderValue,
    ),
    #[error("{0}")]
    Io(
        #[source]
        #[from]
        io::Error,
    ),
    #[error("{0}")]
    FiscalData(
        #[source]
        #[from]
        fiscal_data::Error,
    ),
    #[error("{0}")]
    #[allow(unused)]
    Custom(String),
    #[error("{0}")]
    Json(
        #[source]
        #[from]
        serde_json::Error,
    ),
    #[error("missing data: {0}")]
    MissingData(&'static str),
    #[allow(clippy::enum_variant_names)]
    #[error("parse error")]
    ParseError,
    #[allow(dead_code)]
    #[error("no response")]
    NoResponse,
    #[error("redirect to {0}")]
    Redirect(String),
}

#[async_trait]
pub(crate) trait Provider: Send + Sync {
    fn id(&self) -> &'static str;
    fn name(&self) -> &'static str;
    fn url(&self) -> &'static str;
    fn exts(&self) -> &'static [&'static str];
    fn inn(&self) -> &'static str;
    async fn fetch_raw_data(&self, config: &Config, rec: &mut Object) -> Result<Vec<u8>, Error>;
    async fn parse(&self, config: &Config, data: &[u8], rec: Object) -> Result<Document, Error>;
    fn cache_id(&self, rec: &Object) -> Result<String, Error> {
        let drive_num = rec
            .get::<fields::DriveNum>()?
            .ok_or(Error::MissingData("fn"))?;
        let doc_num = rec
            .get::<fields::DocNum>()?
            .ok_or(Error::MissingData("fd"))?;
        Ok(format!("{drive_num}_{doc_num:07}"))
    }
    async fn register(&self, router: axum::Router) -> axum::Router {
        router
    }
    fn condition(&self, _rec: &Object) -> bool {
        true
    }
}

pub struct OfdRegistry {
    by_id: BTreeMap<String, Vec<&'static dyn Provider>>,
    all: Vec<&'static dyn Provider>,
}

impl OfdRegistry {
    pub async fn new(c: &Config, router: &mut axum::Router) -> Self {
        let mut ret = Self {
            by_id: BTreeMap::new(),
            all: Vec::new(),
        };
        if let Some(endpoint) = &c.private1_endpoint {
            ret.add(private1::Private1::new(endpoint), router).await;
        }
        if let Some(((client_secret, device_id), api_base)) = c
            .irkkt_mobile_client_secret
            .as_ref()
            .zip(c.irkkt_mobile_device_id.as_ref())
            .zip(c.irkkt_mobile_api_base.as_ref())
        {
            ret.add(
                irkkt_mobile::IrkktMobile::new(c, client_secret, device_id, api_base),
                router,
            )
            .await;
        }
        ret.add(astral::Astral, router).await;
        ret.add(icom24::Icom24, router).await;
        ret.add(ofd_ru::OfdRu, router).await;
        ret.add(taxcom::Taxcom, router).await;
        ret
    }
    pub async fn add(&mut self, ofd: impl Provider + 'static, router: &mut axum::Router) {
        let mut tmp = axum::Router::new();
        std::mem::swap(&mut tmp, router);
        *router = ofd.register(tmp).await;
        let ofd = Box::leak(Box::new(ofd));
        self.by_id.entry(ofd.id().to_owned()).or_default().push(ofd);
        self.all.push(ofd);
    }
    pub fn default<'a>(
        &'a self,
        rec: &'a Object,
    ) -> impl 'a + Iterator<Item = &'static dyn Provider> {
        self.by_id
            .get("private1")
            .into_iter()
            .flatten()
            .copied()
            .chain(
                self.by_id
                    .get("irkkt-mobile")
                    .into_iter()
                    .flatten()
                    .copied(),
            )
            .filter(move |x| x.condition(rec))
    }
    pub fn all(&self) -> impl '_ + Iterator<Item = &'static dyn Provider> {
        self.all.iter().copied()
    }
    pub fn by_id<'a>(
        &'a self,
        id: &str,
        rec: &'a Object,
    ) -> impl 'a + Iterator<Item = &'static dyn Provider> {
        self.by_id
            .get(id)
            .into_iter()
            .flatten()
            .copied()
            .chain(self.default(rec))
            .chain(self.all.iter().copied())
            .filter(move |x| x.condition(rec))
    }
    pub fn fill(&self, id: &str, rec: &mut Object) -> fiscal_data::Result<&dyn Provider> {
        let ofd = self
            .by_id(id, rec)
            .next()
            .ok_or(fiscal_data::Error::InvalidFormat)?;
        rec.push::<custom::ProviderId>(ofd.id().to_owned())?;
        Ok(ofd)
    }
}

static REG: OnceCell<OfdRegistry> = OnceCell::const_new();
pub async fn init_registry(config: &'static Config, router: &mut axum::Router) {
    REG.set(OfdRegistry::new(config, router).await)
        .unwrap_or_else(|_| panic!());
}
pub async fn registry() -> &'static OfdRegistry {
    REG.get_or_init(|| async {
        OfdRegistry::new(&Config::default(), &mut axum::Router::new()).await
    })
    .await
}
pub fn fill_missing_fields(a: &mut Object, b: &Object) {
    for (k, v) in b.iter_raw() {
        if !a.contains_raw(k) {
            a.set_raw(k, v);
        }
    }
}

async fn fetch_raw<P: Provider + ?Sized>(
    config: &Config,
    provider: &P,
    rec: &mut Object,
    force: bool,
) -> Result<Vec<u8>, Error> {
    let cache_id = provider.cache_id(rec)?;
    let mut raw_path = config.data_path("raw");
    raw_path.push(provider.id());
    let _ = tokio::fs::create_dir_all(&raw_path).await;
    raw_path.push(format!("{cache_id}.{}", provider.exts().first().unwrap()));
    if force || !raw_path.is_file() {
        let data = provider.fetch_raw_data(config, rec).await?;
        log::info!("raw data: {data:?}");
        log::info!("writing {raw_path:?}");
        let _ = tokio::fs::write(&raw_path, &data).await;
        Ok(data)
    } else {
        Ok(tokio::fs::read(&raw_path).await?)
    }
}

async fn fetch2<P: Provider + ?Sized>(
    config: &Config,
    provider: &P,
    mut rec: Object,
) -> Result<Document, Error> {
    let cache_id = provider.cache_id(&rec)?;
    let path = config.data_path(format!("ffd/{cache_id}.tlv"));
    if let Ok(data) = tokio::fs::read(&path).await {
        if let Ok(doc) = Document::from_bytes(data) {
            return Ok(doc);
        }
    }
    let mut raw_path = config.data_path("raw");
    raw_path.push(provider.id());
    let _ = tokio::fs::create_dir_all(&raw_path).await;
    raw_path.push(format!("{cache_id}.{}", provider.exts().first().unwrap()));
    let parsed = if let Ok(x) = tokio::fs::read(&raw_path).await {
        provider.parse(config, &x, rec.clone()).await.ok()
    } else {
        None
    };
    let mut parsed = if let Some(parsed) = parsed {
        parsed
    } else {
        let data = fetch_raw(config, provider, &mut rec, true).await?;
        provider.parse(config, &data, rec.clone()).await?
    };
    fill_missing_fields(parsed.data_mut(), &rec);
    if !path.is_symlink() && !path.is_file() {
        let drive_num = parsed
            .data()
            .get::<fields::DriveNum>()?
            .ok_or(Error::MissingData("fn"))?;
        let doc_num = parsed
            .data()
            .get::<fields::DocNum>()?
            .ok_or(Error::MissingData("fd"))?;
        let final_cache_id = format!("{drive_num}_{doc_num:07}");
        if cache_id != final_cache_id {
            let _ = tokio::fs::symlink(format!("{final_cache_id}.tlv"), path).await;
        }
    }
    Ok(parsed)
}
pub(crate) async fn fetch(config: &Config, rec: Object) -> Result<Document, Error> {
    let provider = registry()
        .await
        .by_id(&rec.get::<custom::ProviderId>()?.unwrap_or_default(), &rec)
        .next()
        .ok_or(Error::MissingData("provider"))?;
    let ret = fetch2(config, provider, rec).await?;
    let drive_num = ret
        .data()
        .get::<fields::DriveNum>()?
        .ok_or(Error::MissingData("fn"))?;
    let doc_num = ret
        .data()
        .get::<fields::DocNum>()?
        .ok_or(Error::MissingData("fd"))?;
    let final_cache_id = format!("{drive_num}_{doc_num:07}");
    let final_path = config.data_path(format!("ffd/{final_cache_id}.tlv"));
    if !final_path.is_file() {
        let _ = tokio::fs::write(&final_path, &ret.clone().into_bytes()?).await;
    }
    Ok(ret)
}
