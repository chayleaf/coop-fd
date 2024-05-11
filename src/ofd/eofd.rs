// https://e-ofd.ru/check?fn=7280440700289250&fp=3027402097&fd=3364
// https://e-ofd.ru/api/ofdcheck/checkf?fn=7280440700289250&fp=3027402097&fd=3364&format=71&preventcacheparam=1714232008940

use super::Provider;

pub struct Eofd;
impl Provider for Eofd {
    fn id(&self) -> &'static str {
        "eofd"
    }
    fn name(&self) -> &'static str {
        "ООО \"ГРУППА ЭЛЕМЕНТ\""
    }
    fn url(&self) -> &'static str {
        "e-ofd.ru"
    }
    fn exts(&self) -> &'static [&'static str] {
        &["json"]
    }
    fn inn(&self) -> &'static str {
        "7729642175"
    }
}
