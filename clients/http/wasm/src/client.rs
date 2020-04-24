use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, Response};

use async_trait::async_trait;
use rustic_http_client::{HttpClient, RusticHttpClient};
use serde::de::DeserializeOwned;
use serde::Serialize;

#[derive(Debug, Clone)]
pub struct RusticWasmHttpClient;

impl RusticWasmHttpClient {
    pub const fn new() -> RusticHttpClient<RusticWasmHttpClient> {
        RusticHttpClient {
            client: RusticWasmHttpClient
        }
    }
}

#[async_trait(?Send)]
impl HttpClient for RusticWasmHttpClient {
    type Error = JsValue;

    async fn get<T>(&self, url: &str) -> Result<T, Self::Error>
        where T: DeserializeOwned {
        let mut opts = RequestInit::new();
        opts.method("GET");

        let request = Request::new_with_str_and_init(url, &opts)?;

        let window = web_sys::window().unwrap();
        let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;

        assert!(resp_value.is_instance_of::<Response>());
        let resp: Response = resp_value.dyn_into().unwrap();

        let json = JsFuture::from(resp.json()?).await?;

        let model: T = json.into_serde().unwrap();

        Ok(model)
    }

    async fn post<TReq, TRes>(&self, url: &str, body: TReq) -> Result<TRes, Self::Error>
        where TRes: DeserializeOwned,
        TReq: Serialize {
        let body = JsValue::from_serde(&body).unwrap();
        let mut opts = RequestInit::new();
        opts.method("POST");
        opts.body(Some(&body));

        let request = Request::new_with_str_and_init(url, &opts)?;
        request.headers()
            .set("Content-Type", "application/json")?;

        let window = web_sys::window().unwrap();
        let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;

        assert!(resp_value.is_instance_of::<Response>());
        let resp: Response = resp_value.dyn_into().unwrap();

        let json = JsFuture::from(resp.json()?).await?;

        let model: TRes = json.into_serde().unwrap();

        Ok(model)
    }
}
