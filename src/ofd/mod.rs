use serde::{Deserialize, Serialize};
use std::{fmt::Display, str::FromStr};

use crate::{Config, Receipt};

mod beeline;
mod magnit;
mod platforma_ofd;
mod taxcom;

#[derive(Copy, Clone, Debug, Default, Deserialize, Serialize, Eq, PartialEq)]
pub enum Ofd {
    #[serde(rename = "beeline")]
    Beeline,
    #[serde(rename = "magnit")]
    Magnit,
    #[default]
    #[allow(clippy::enum_variant_names)]
    #[serde(rename = "platforma-ofd")]
    PlatformaOfd,
    #[serde(rename = "taxcom")]
    Taxcom,
}

impl Ofd {
    // PlatformaOfd must go first since it's the default in the web UI
    pub const ALL: &'static [Self] = &[
        Self::PlatformaOfd,
        Self::Magnit,
        Self::Beeline,
        Self::Taxcom,
    ];
    pub const fn is_platforma_ofd(&self) -> bool {
        matches!(self, Self::PlatformaOfd)
    }
    pub const fn name(&self) -> &'static str {
        match self {
            Self::Beeline => "ОФД Билайн",
            Self::Magnit => "Магнит (Тандер)",
            Self::PlatformaOfd => "Платформа ОФД",
            Self::Taxcom => "Такском",
        }
    }
}

impl Display for Ofd {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::Beeline => "beeline",
            Self::Magnit => "magnit",
            Self::PlatformaOfd => "platforma-ofd",
            Self::Taxcom => "taxcom",
        })
    }
}

// !!!!!!!!!!!!!! also add below

impl FromStr for Ofd {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "beeline" => Self::Beeline,
            "magnit" => Self::Magnit,
            "platforma-ofd" => Self::PlatformaOfd,
            "taxcom" => Self::Taxcom,
            _ => return Err(()),
        })
    }
}

pub(crate) async fn fetch(config: &'static Config, rec: Receipt) -> Option<Receipt> {
    if let Some(fnifp) = rec.fnifp() {
        let mut path = config.data_path("parsed");
        path.push(fnifp + ".json");
        if let Ok(data) = tokio::fs::read(path).await {
            if let Ok(parsed) = serde_json::from_slice::<Receipt>(&data) {
                return Some(parsed);
            }
        }
    }
    let rec = match rec.ofd {
        Ofd::Beeline => beeline::fetch(config, rec).await?,
        Ofd::Magnit => magnit::fetch(config, rec).await?,
        Ofd::PlatformaOfd => platforma_ofd::fetch(config, rec).await?,
        Ofd::Taxcom => taxcom::fetch(config, rec).await?,
    };

    if let Some(fnifp) = rec.fnifp() {
        let mut path = config.data_path("parsed");
        path.push(fnifp + ".json");
        match serde_json::to_vec(&rec) {
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

    Some(rec)
}
