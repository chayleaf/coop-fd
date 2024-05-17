use async_trait::async_trait;
use fiscal_data::{fields, Document, Object, TlvType};
use std::{collections::BTreeMap, io, sync::OnceLock};
use thiserror::Error;

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
}

pub struct OfdRegistry {
    pub(crate) by_id: BTreeMap<String, &'static dyn Provider>,
    all: Vec<&'static dyn Provider>,
}

impl OfdRegistry {
    pub fn new(c: &Config) -> Self {
        let mut ret = Self {
            by_id: BTreeMap::new(),
            all: Vec::new(),
        };
        if let Some(endpoint) = &c.private1_endpoint {
            ret.add(private1::Private1::new(endpoint));
        }
        ret.add(astral::Astral);
        ret.add(icom24::Icom24);
        ret.add(ofd_ru::OfdRu);
        ret.add(taxcom::Taxcom);
        ret
    }
    pub fn add(&mut self, ofd: impl Provider + 'static) {
        let ofd = Box::leak(Box::new(ofd));
        self.by_id.insert(ofd.id().to_owned(), ofd);
        self.all.push(ofd);
    }
    pub fn by_id(&self, id: &str) -> Option<&dyn Provider> {
        self.by_id
            .get(id)
            .or_else(|| self.by_id.get("private1"))
            .or_else(|| self.all.first())
            .copied()
    }
    pub fn fill(&self, id: &str, rec: &mut Object) -> fiscal_data::Result<&dyn Provider> {
        let ofd = self.by_id(id).ok_or(fiscal_data::Error::InvalidFormat)?;
        rec.push::<custom::ProviderId>(ofd.id().to_owned())?;
        Ok(ofd)
    }
}

static REG: OnceLock<OfdRegistry> = OnceLock::new();
pub fn init_registry(config: &'static Config) {
    REG.set(OfdRegistry::new(config))
        .unwrap_or_else(|_| panic!());
}
pub fn registry() -> &'static OfdRegistry {
    REG.get_or_init(|| OfdRegistry::new(&Config::default()))
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
        .by_id(&rec.get::<custom::ProviderId>()?.unwrap_or_default())
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
