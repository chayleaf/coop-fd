use std::time::Duration;

use fiscal_data::{enums, fields, Object};
use serde::Deserialize;

use crate::{
    ofd::{self, registry},
    Config, ReceiptFormatVersion, QR_DATE_FORMAT,
};

#[derive(Copy, Clone, Debug, Default, Deserialize, Eq, PartialEq)]
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

#[allow(dead_code)]
#[derive(Clone, Debug, Default, Deserialize)]
struct Company {
    name: String,
    inn: String,
}

#[allow(dead_code)]
#[derive(Clone, Debug, Default, Deserialize)]
struct Item {
    name: String,
    #[serde(default)]
    id: String,
    count: f64,
    unit: String,
    per_item: u64,
    total: u64,
    tax: u64,
}

#[allow(dead_code)]
#[derive(Clone, Debug, Default, Deserialize)]
struct Receipt {
    #[serde(default)]
    ofd: Ofd,
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

#[allow(unused)]
pub async fn migrate_json_to_ffd(config: &Config) {
    let mut have_errors = false;
    if let Ok(mut dir) = tokio::fs::read_dir(config.data_path("parsed")).await {
        let provider = registry().by_id("").unwrap();
        while let Some(file) = dir.next_entry().await.expect("failed to read ofd cache") {
            let path = file.path();
            let Some(ext) = path
                .extension()
                .and_then(|x| x.to_str())
                .map(str::to_lowercase)
            else {
                continue;
            };
            let Some(name) = path.file_stem().and_then(|x| x.to_str()) else {
                continue;
            };
            if ext != "json" {
                continue;
            }
            let mut spl = name.split('_');
            let Some(r#fn) = spl.next() else { continue };
            let Some(i) = spl.next().and_then(|i| i.parse::<u32>().ok()) else {
                continue;
            };
            let Some(fp) = spl.next().and_then(|x| x.parse::<u64>().ok()) else {
                continue;
            };
            let mut ffd_path = config.data_path(format!("ffd/{fn}_{i:07}.tlv"));
            if ffd_path.exists() {
                continue;
            }
            let Ok(data) = tokio::fs::read(&path).await else {
                continue;
            };
            let Ok(old) = serde_json::from_slice::<Receipt>(&data) else {
                continue;
            };
            let mut rec = Object::new();
            rec.set::<fiscal_data::fields::DateTime>(
                chrono::NaiveDateTime::parse_from_str(&old.date, QR_DATE_FORMAT).unwrap(),
            )
            .unwrap();
            rec.set::<fields::TotalSum>(old.total).unwrap();
            rec.set::<fields::DriveNum>(r#fn.to_owned()).unwrap();
            rec.set::<fields::PaymentType>(enums::PaymentType::Sale)
                .unwrap();
            let [_, _, a, b, c, d, e, f] = fp.to_be_bytes();
            rec.set::<fiscal_data::fields::DocFiscalSign>([a, b, c, d, e, f])
                .unwrap();
            rec.set::<fields::DocNum>(i).unwrap();
            if !old.id.is_empty() {
                rec.set::<ofd::custom::Id>(old.id).unwrap();
            }
            if ofd::fetch(config, rec).await.is_ok() {
                log::info!("migrated {fn}_{i}_{fp}");
            } else {
                have_errors = true;
            }
            // don't ddos providers
            tokio::time::sleep(Duration::from_secs(30));
        }
    }
    if !have_errors {
        log::info!("migration done");
        super::mutate_state(|state| state.receipt_version = ReceiptFormatVersion::Fns);
    }
}
