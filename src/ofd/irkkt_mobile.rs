use std::{io, path::PathBuf, sync::Arc};

use crate::{
    server::{FileRes, State},
    Config,
};
use async_trait::async_trait;
use axum::response::IntoResponse;
use fiscal_data::fields;
use serde::{Deserialize, Serialize};
use tokio::sync::{OnceCell, RwLock};

use super::{Error, Provider};

#[derive(Clone, Default, Deserialize, Serialize)]
struct Auth {
    session_id: String,
    refresh_token: String,
}

struct IrkktMobileAuth(PathBuf, OnceCell<RwLock<Auth>>);

impl IrkktMobileAuth {
    async fn init(&self) -> &RwLock<Auth> {
        self.1
            .get_or_init(|| async {
                tokio::fs::create_dir_all(self.0.parent().unwrap())
                    .await
                    .unwrap();
                RwLock::new(
                    tokio::fs::read(&self.0)
                        .await
                        .ok()
                        .and_then(|data| serde_json::from_slice(&data).ok())
                        .unwrap_or_default(),
                )
            })
            .await
    }
    pub async fn get(&self) -> Auth {
        self.init().await.read().await.clone()
    }
    pub async fn set(&self, auth: Auth) -> io::Result<()> {
        let mut lock = self.init().await.write().await;
        tokio::fs::write(&self.0, serde_json::to_vec(&auth)?).await?;
        *lock = auth;
        Ok(())
    }
}

#[derive(Clone)]
pub struct IrkktMobile {
    client_secret: String,
    device_id: String,
    api_base: String,
    auth: Arc<IrkktMobileAuth>,
}

// #[derive(Deserialize)]
// struct EsiaAuthUrl {
//     url: String,
// }
//
// #[derive(Serialize)]
// struct EsiaAuthRequest {
//     client_secret: String,
//     authorization_code: String,
//     state: String,
// }

#[derive(Serialize)]
struct PhoneRequest {
    client_secret: String,
    phone: String,
}

#[derive(Serialize)]
struct PhoneAuthRequest {
    client_secret: String,
    phone: String,
    code: String,
}

#[derive(Deserialize)]
struct RefreshResponse {
    #[serde(rename = "sessionId")]
    session_id: String,
    refresh_token: String,
}

#[derive(Deserialize)]
struct AuthResponse {
    #[serde(rename = "sessionId")]
    session_id: String,
    refresh_token: String,
    #[serde(default)]
    #[allow(unused)]
    phone: String,
    #[serde(default)]
    #[allow(unused)]
    email: String,
    #[serde(default)]
    #[allow(unused)]
    name: String,
    #[serde(default)]
    #[allow(unused)]
    surname: String,
}

#[derive(Serialize)]
struct RefreshRequest {
    client_secret: String,
    refresh_token: String,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct TicketQuery {
    #[serde(with = "fiscal_data::json::as_localtime")]
    date: chrono::NaiveDateTime,
    operation_type: fiscal_data::enums::PaymentType,
    sum: u64,
    fs_id: String,
    document_id: u32,
    fiscal_sign: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct TicketRequest {
    fiscal_data: TicketQuery,
    send_to_email: bool,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct TicketResponse {
    id: String,
    // status: i32,
    // and a bunch of other fields
}

// #[derive(Deserialize)]
// struct RedirectQuery {
//     code: String,
//     state: String,
// }

impl IrkktMobile {
    pub fn new(cfg: &Config, client_secret: &str, device_id: &str, api_base: &str) -> Self {
        Self {
            client_secret: client_secret.to_owned(),
            device_id: device_id.to_owned(),
            api_base: api_base.to_owned(),
            auth: Arc::new(IrkktMobileAuth(
                cfg.data_path("secret/irkkt-mobile/auth.json"),
                OnceCell::new(),
            )),
        }
    }
    fn client(&self) -> Result<reqwest::Client, Error> {
        Ok(reqwest::ClientBuilder::new()
            .default_headers({
                let mut headers = reqwest::header::HeaderMap::new();

                headers.insert("User-Agent", "okhttp/4.2.2".try_into().unwrap());
                headers.insert("ClientVersion", "2.11.1".try_into().unwrap());
                headers.insert("Device-OS", "Android".try_into().unwrap());
                headers.insert("Device-Id", (&self.device_id).try_into().unwrap());
                headers.insert(
                    "Accept-Language",
                    "ru-RU;q=1, en-US;q=0.9".try_into().unwrap(),
                );
                headers
            })
            .build()?)
    }
    async fn try_fetch(&self, req: reqwest::Request) -> Result<reqwest::Response, Error> {
        let client = self.client()?;
        let mut req1 = Some(req);
        for _ in 0..2 {
            let auth = self.auth.get().await;
            if auth.session_id.is_empty() || auth.refresh_token.is_empty() {
                break;
            }
            let Some(mut req) = req1.take() else {
                break;
            };
            req1 = req.try_clone();
            req.headers_mut()
                .insert("sessionId", (&auth.session_id).try_into()?);
            let ret = client.execute(req).await?;
            match ret.status() {
                reqwest::StatusCode::UNAUTHORIZED => {}
                _ => return Ok(ret),
            }
            let RefreshResponse {
                session_id,
                refresh_token,
            } = client
                .execute(
                    client
                        .post(format!("{}/v2/mobile/users/refresh", self.api_base))
                        .header("sessionId", &auth.session_id)
                        .json(&RefreshRequest {
                            client_secret: self.client_secret.clone(),
                            refresh_token: auth.refresh_token.clone(),
                        })
                        .build()?,
                )
                .await?
                .json()
                .await?;
            self.auth
                .set(Auth {
                    session_id,
                    refresh_token,
                })
                .await?;
        }
        // let url = client
        //     .execute(
        //         client
        //             .get(&format!("{}/v2/mobile/users/esia/auth/url", self.api_base))
        //             .build()?,
        //     )
        //     .await?
        //     .json::<EsiaAuthUrl>()
        //     .await?
        //     .url;
        // let escape_c = |c: u8| -> String {
        //     match c {
        //         b'0'..=b'9' | b'a'..=b'z' | b'A'..=b'Z' => (c as char).into(),
        //         x => format!("%{x:02x}"),
        //     }
        // };
        // let escape = |s: &str| -> String { s.bytes().map(escape_c).collect::<Vec<_>>().join("") };
        // let url = url.replace(
        //     &format!(
        //         "&redirect_uri={}&",
        //         escape("https://irkkt-mobile.nalog.ru:8888")
        //     ),
        //     &format!(
        //         "&redirect_uri={}&",
        //         escape(
        //             &(cfg
        //                 .public_url
        //                 .strip_suffix('/')
        //                 .unwrap_or(&cfg.public_url)
        //                 .to_owned()
        //                 + "/ofd/irkkt-mobile/redirect")
        //         )
        //     ),
        // );
        // Err(Error::Redirect(url))
        Err(Error::Redirect("ofd/irkkt-mobile/auth".to_owned()))
    }
}

#[derive(Deserialize)]
struct FnsAuthSubmitRequest {
    phone: String,
    code: String,
}

#[derive(Deserialize)]
struct Ticket {
    document: fiscal_data::json::Document,
}

// #[derive(Deserialize)]
// struct Org {
//     name: String,
//     inn: String,
// }

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct FullTicketResponse {
    // status: i32,
    // status_real: i32,
    // id: String,
    // kind: String,
    // created_at: chrono::DateTime<chrono::Utc>,
    // status_description: serde_json::Value,
    // qr: String,
    // operation: { date, type, sum } - date is iso8601 without seconds and timezone, type/sum are ints
    // process: Vec<{ time: full iso8601 string, result: int }>,
    // query: TicketQuery,
    ticket: Ticket,
    // organization: Org,
    // seller: Org,
}

#[async_trait]
impl Provider for IrkktMobile {
    fn id(&self) -> &'static str {
        "irkkt-mobile"
    }
    fn url(&self) -> &'static str {
        ""
    }
    fn inn(&self) -> &'static str {
        ""
    }
    fn name(&self) -> &'static str {
        ""
    }
    fn exts(&self) -> &'static [&'static str] {
        &["json"]
    }
    async fn fetch_raw_data(
        &self,
        _state: &State,
        rec: &mut fiscal_data::Object,
    ) -> Result<Vec<u8>, Error> {
        let date = rec
            .get::<fields::DateTime>()?
            .ok_or(Error::MissingData("date"))?;
        let operation_type = rec
            .get::<fields::PaymentType>()?
            .ok_or(Error::MissingData("operationType"))?;
        let sum = rec
            .get::<fields::TotalSum>()?
            .ok_or(Error::MissingData("sum"))?;
        let fs_id = rec
            .get::<fields::DriveNum>()?
            .ok_or(Error::MissingData("fsId"))?;
        let document_id = rec
            .get::<fields::DocNum>()?
            .ok_or(Error::MissingData("documentId"))?;
        let [_, _, a, b, c, d] = rec
            .get::<fields::DocFiscalSign>()?
            .ok_or(Error::MissingData("fiscalSign"))?;
        let fiscal_sign = u32::from_be_bytes([a, b, c, d]).to_string();
        let client = self.client()?;
        let req = client
            .post(format!("{}/v2/ticket", self.api_base))
            .json(&TicketRequest {
                fiscal_data: TicketQuery {
                    date,
                    operation_type,
                    sum,
                    fs_id,
                    document_id,
                    fiscal_sign,
                },
                send_to_email: false,
            })
            .build()?;
        let res = self.try_fetch(req).await?.error_for_status()?;
        let id = res.json::<TicketResponse>().await?.id;
        let req = client
            .get(format!("{}/v2/tickets/{id}", self.api_base))
            .build()?;
        let res = self.try_fetch(req).await?.error_for_status()?;
        let ret = res.bytes().await?.to_vec();
        rec.set::<super::custom::Id>(id)?;
        Ok(ret)
    }
    async fn parse(
        &self,
        _state: &State,
        data: &[u8],
        _rec: fiscal_data::Object,
    ) -> Result<fiscal_data::Document, Error> {
        let data: FullTicketResponse = serde_json::from_slice(data)?;
        Ok(data.ticket.document.try_into()?)
    }
    async fn register(
        &self,
        router: axum::Router<crate::server::State>,
    ) -> axum::Router<crate::server::State> {
        let this = self.clone();
        let parser = Arc::new(
            liquid::ParserBuilder::with_stdlib()
                .filter(crate::CurrencyFilter)
                .filter(crate::CEscapeFilter)
                .build()
                .unwrap(),
        );
        let auth_t = FileRes::new(move || {
            let parser = parser.clone();
            async move {
                parser
                    .parse(
                        &tokio::fs::read_to_string("templates/irkkt-mobile/auth.html")
                            .await
                            .unwrap_or_else(|_| {
                                include_str!("../../templates/irkkt-mobile/auth.html").to_owned()
                            }),
                    )
                    .unwrap_or_else(|err| panic!("irkkt_auth:\n{err}"))
            }
        })
        .await;
        router
            .route(
                "/ofd/irkkt-mobile/auth",
                axum::routing::get(move || {
                    let auth_t = auth_t.clone();
                    async move {
                        axum::response::Html::from(
                            auth_t
                                .get()
                                .await
                                .render(&liquid::object!({}))
                                .unwrap_or_else(|err| format!("Error: {err}")),
                        )
                    }
                }),
            )
            .route(
                "/ofd/irkkt-mobile/auth/submit",
                axum::routing::post(
                    move |axum::extract::Form(f): axum::extract::Form<FnsAuthSubmitRequest>| {
                        let this = this.clone();
                        async move {
                            let is_auth = !f.code.is_empty();
                            let phone: String = f
                                .phone
                                .chars()
                                .filter(|x| matches!(x, '0'..='9' | '+'))
                                .collect();
                            let res: Result<(), Error> = async {
                                let client = this.client()?;
                                if f.code.is_empty() {
                                    let res = client
                                        .execute(
                                            client
                                                .post(format!(
                                                    "{}/v2/auth/phone/request",
                                                    this.api_base
                                                ))
                                                .json(&PhoneRequest {
                                                    client_secret: this.client_secret.clone(),
                                                    phone,
                                                })
                                                .build()?,
                                        )
                                        .await?;
                                    res.error_for_status()?;
                                } else {
                                    let res = client
                                        .execute(
                                            client
                                                .post(format!(
                                                    "{}/v2/auth/phone/verify",
                                                    this.api_base
                                                ))
                                                .json(&PhoneAuthRequest {
                                                    client_secret: this.client_secret.clone(),
                                                    phone,
                                                    code: f.code,
                                                })
                                                .build()?,
                                        )
                                        .await?
                                        .error_for_status()?;
                                    let AuthResponse {
                                        session_id,
                                        refresh_token,
                                        ..
                                    } = res.json::<AuthResponse>().await?;
                                    this.auth
                                        .set(Auth {
                                            session_id,
                                            refresh_token,
                                        })
                                        .await?;
                                }
                                Ok(())
                            }
                            .await;
                            match res {
                                Ok(()) => axum::response::Redirect::to(if is_auth {
                                    "../../.."
                                } else {
                                    "../auth"
                                })
                                .into_response(),
                                Err(err) => {
                                    log::error!("irkkt mobile phone error: {err}");
                                    axum::response::Html::from(format!("Error: {err}"))
                                        .into_response()
                                }
                            }
                        }
                    },
                ),
            )
        //     .route(
        //         "/ofd/irkkt-mobile/redirect",
        //         axum::routing::get(
        //             move |axum::extract::Query(x): axum::extract::Query<RedirectQuery>| {
        //                 let this = this.clone();
        //                 async move {
        //                     let res: Result<_, Error> = {
        //                         async {
        //                             let client = this.client()?;
        //                             Ok(client
        //                                 .execute(
        //                                     client
        //                                         .post(&format!(
        //                                             "{}/v2/mobile/users/esia/auth",
        //                                             this.api_base
        //                                         ))
        //                                         .json(&EsiaAuthRequest {
        //                                             client_secret: this.client_secret.clone(),
        //                                             authorization_code: x.code,
        //                                             state: x.state,
        //                                         })
        //                                         .build()?,
        //                                 )
        //                                 .await?
        //                                 .json::<AuthResponse>()
        //                                 .await?)
        //                         }
        //                     }
        //                     .await;
        //                     let AuthResponse {
        //                         session_id,
        //                         refresh_token,
        //                     } = match res {
        //                         Ok(res) => res,
        //                         Err(err) => {
        //                             log::error!("irkkt mobile redirect failed: {err}");
        //                             return axum::response::Html::from(format!("error: {err}"))
        //                                 .into_response();
        //                         }
        //                     };
        //                     this.auth
        //                         .set(Auth {
        //                             session_id,
        //                             refresh_token,
        //                         })
        //                         .await;
        //                     axum::response::Redirect::to("/").into_response()
        //                 }
        //             },
        //         ),
        //     )
    }
}

#[cfg(test)]
mod test {
    use fiscal_data::{fields, Object};

    use crate::{ofd::Provider, server::State};

    #[test]
    fn test() {
        tokio_test::block_on(async {
            let state = State::default();
            let doc = super::IrkktMobile::new(&state.config, "", "", "")
                .parse(
                    &state,
                    include_bytes!("../../test_data/irkkt-mobile1.json"),
                    Object::new(),
                )
                .as_mut()
                .await
                .unwrap();
            assert_eq!(
                doc.data().get::<fields::FnsUrl>().unwrap().unwrap(),
                "www.nalog.gov.ru"
            );
        });
    }
}
