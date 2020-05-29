use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, Response};

use crate::utils::map_value;
use async_trait::async_trait;
use rustic_http_client::{HttpClient, RusticHttpClient};
use serde::de::DeserializeOwned;
use serde::Serialize;

#[derive(Debug, Clone)]
pub struct RusticWasmHttpClient;

impl RusticWasmHttpClient {
    pub const fn new() -> RusticHttpClient<RusticWasmHttpClient> {
        RusticHttpClient {
            client: RusticWasmHttpClient,
        }
    }
}

#[async_trait(?Send)]
impl HttpClient for RusticWasmHttpClient {
    async fn get<T>(&self, url: &str) -> Result<T, failure::Error>
    where
        T: DeserializeOwned,
    {
        let mut opts = RequestInit::new();
        opts.method("GET");

        let request = Request::new_with_str_and_init(url, &opts).map_err(map_value)?;

        let window = web_sys::window().unwrap();
        let resp_value = JsFuture::from(window.fetch_with_request(&request))
            .await
            .map_err(map_value)?;

        assert!(resp_value.is_instance_of::<Response>());
        let resp: Response = resp_value.dyn_into().map_err(map_value)?;

        let json = JsFuture::from(resp.json().unwrap())
            .await
            .map_err(map_value)?;

        let model: T = json.into_serde()?;

        Ok(model)
    }

    async fn post<TReq, TRes>(&self, url: &str, body: TReq) -> Result<TRes, failure::Error>
    where
        TRes: DeserializeOwned,
        TReq: Serialize,
    {
        let body = JsValue::from_serde(&body)?;
        let mut opts = RequestInit::new();
        opts.method("POST");
        opts.body(Some(&body));

        let request = Request::new_with_str_and_init(url, &opts).map_err(map_value)?;
        request
            .headers()
            .set("Content-Type", "application/json")
            .unwrap();

        let window = web_sys::window().unwrap();
        let resp_value = JsFuture::from(window.fetch_with_request(&request))
            .await
            .map_err(map_value)?;

        assert!(resp_value.is_instance_of::<Response>());
        let resp: Response = resp_value.dyn_into().map_err(map_value)?;

        let json = JsFuture::from(resp.json().unwrap())
            .await
            .map_err(map_value)?;

        let model: TRes = json.into_serde()?;

        Ok(model)
    }
}
