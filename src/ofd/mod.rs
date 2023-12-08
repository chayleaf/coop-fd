use serde::{Deserialize, Serialize};
use std::str::FromStr;

use crate::{Config, Receipt};

mod magnit;
mod platforma_ofd;

#[derive(Copy, Clone, Debug, Default, Deserialize, Serialize, Eq, PartialEq)]
pub enum Ofd {
    #[serde(rename = "magnit")]
    Magnit,
    #[default]
    #[serde(rename = "platforma-ofd")]
    PlatformaOfd,
}

impl Ofd {
    // PlatformaOfd must go first since it's the default in the web UI
    pub const ALL: &[Self] = &[Self::PlatformaOfd, Self::Magnit];
    pub const fn is_platforma_ofd(&self) -> bool {
        matches!(self, Self::PlatformaOfd)
    }
    pub const fn name(&self) -> &'static str {
        match self {
            Self::Magnit => "Магнит (Тандер)",
            Self::PlatformaOfd => "Платформа ОФД",
        }
    }
    pub const fn id(&self) -> &'static str {
        match self {
            Self::Magnit => "magnit",
            Self::PlatformaOfd => "platforma-ofd",
        }
    }
}

impl FromStr for Ofd {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "magnit" => Self::Magnit,
            "platforma-ofd" => Self::PlatformaOfd,
            _ => return Err(()),
        })
    }
}

pub(crate) async fn fetch(config: &'static Config, rec: Receipt) -> Option<Receipt> {
    if let Some(fnifp) = rec.fnifp() {
        let mut path = config.data_path.clone();
        path.push("parsed");
        path.push(fnifp + ".json");
        if let Ok(data) = tokio::fs::read(path).await {
            if let Ok(parsed) = serde_json::from_slice::<Receipt>(&data) {
                return Some(parsed);
            }
        }
    }
    let rec = match rec.ofd {
        Ofd::Magnit => magnit::fetch(config, rec).await?,
        Ofd::PlatformaOfd => platforma_ofd::fetch(config, rec).await?,
    };

    if let Some(fnifp) = rec.fnifp() {
        let mut path = config.data_path.clone();
        path.push("parsed");
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
