use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, Response};

use crate::utils::map_value;
use async_trait::async_trait;
use rustic_http_client::{HttpClient, RusticHttpClient, HttpResponse};
use serde::de::DeserializeOwned;
use serde::Serialize;

#[derive(Debug, Clone)]
pub struct RusticWasmHttpClient;

impl RusticWasmHttpClient {
    pub const fn new() -> RusticHttpClient<RusticWasmHttpClient, WasmResponse> {
        RusticHttpClient {
            client: RusticWasmHttpClient,
            _marker: std::marker::PhantomData
        }
    }
}

#[derive(Clone)]
pub struct WasmResponse(Response);

// TODO: right now wasm is single threaded anyway
unsafe impl std::marker::Send for WasmResponse {}
unsafe impl std::marker::Sync for WasmResponse {}

#[async_trait(?Send)]
impl HttpResponse for WasmResponse {
    fn no_content(self) -> Result<(), failure::Error> {
        Ok(())
    }

    async fn json<TRes>(self) -> Result<TRes, failure::Error>
        where
            TRes: DeserializeOwned {
        let resp = self.0;
        let json = JsFuture::from(resp.json().unwrap())
            .await
            .map_err(map_value)?;

        let model: TRes = json.into_serde()?;

        Ok(model)
    }
}

impl From<Response> for WasmResponse {
    fn from(res: Response) -> Self {
        WasmResponse(res)
    }
}

#[async_trait(?Send)]
impl HttpClient<WasmResponse> for RusticWasmHttpClient {
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

    async fn post<TReq>(&self, url: &str, body: TReq) -> Result<WasmResponse, failure::Error>
    where
        TReq: Serialize,
    {
        let body = serde_json::to_string(&body)?.into();
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

        Ok(resp.into())
    }

    async fn put<TReq>(&self, url: &str, body: TReq) -> Result<WasmResponse, failure::Error>
        where
            TReq: Serialize,
    {
        let body = serde_json::to_string(&body)?.into();
        let mut opts = RequestInit::new();
        opts.method("PUT");
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

        Ok(resp.into())
    }

    async fn delete(&self, url: &str) -> Result<(), failure::Error> {
        let mut opts = RequestInit::new();
        opts.method("DELETE");

        let request = Request::new_with_str_and_init(url, &opts).map_err(map_value)?;

        let window = web_sys::window().unwrap();
        let resp_value = JsFuture::from(window.fetch_with_request(&request))
            .await
            .map_err(map_value)?;

        assert!(resp_value.is_instance_of::<Response>());
        let _: Response = resp_value.dyn_into().map_err(map_value)?;

        Ok(())
    }
}
