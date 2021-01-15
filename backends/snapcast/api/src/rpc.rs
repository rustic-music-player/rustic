pub(crate) use self::http::HttpTransport;
pub(crate) use self::tcp::TcpTransport;
use crate::Result;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub(crate) enum SnapcastTransport {
    #[cfg(feature = "http")]
    Http(HttpTransport),
    #[cfg(feature = "tcp")]
    Tcp(TcpTransport),
}

impl SnapcastTransport {
    pub async fn request<TReq, TRes>(
        &self,
        method: &str,
        params: &TReq,
        request_id: u64,
    ) -> Result<TRes>
    where
        TReq: Serialize + std::fmt::Debug,
        TRes: DeserializeOwned,
    {
        let request = RpcRequest::new(request_id, method.to_string(), params);
        let res = match self {
            SnapcastTransport::Http(http) => http.request(request).await?,
            SnapcastTransport::Tcp(tcp) => tcp.request(request).await?,
        };
        match (res.result, res.error) {
            (Some(result), None) => Ok(result),
            (None, Some(err)) => Err(err.into()),
            (_, _) => unreachable!(),
        }
    }
}

#[derive(Debug, Clone, Serialize)]
struct RpcRequest<R> {
    id: u64,
    jsonrpc: String,
    method: String,
    params: R,
}

impl<TParams> RpcRequest<TParams> {
    fn new(request_id: u64, method: String, params: TParams) -> Self {
        RpcRequest {
            id: request_id,
            jsonrpc: "2.0".to_string(),
            method,
            params,
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
struct RpcResponse<R> {
    id: u64,
    jsonrpc: String,
    result: Option<R>,
    error: Option<RpcError>,
}

#[derive(Debug, Clone, Deserialize, Serialize, thiserror::Error)]
#[error("{data}")]
pub struct RpcError {
    pub code: i64,
    pub data: String,
    pub message: String,
}

mod http {
    use super::{RpcRequest, RpcResponse};
    use crate::Result;
    use serde::de::DeserializeOwned;
    use serde::Serialize;
    use surf::Body;

    #[derive(Debug)]
    pub(crate) struct HttpTransport {
        host: String,
    }

    impl HttpTransport {
        pub fn new(host: String) -> Self {
            HttpTransport { host }
        }

        pub(super) async fn request<TReq, TRes>(
            &self,
            req: RpcRequest<TReq>,
        ) -> Result<RpcResponse<TRes>>
        where
            TReq: Serialize + std::fmt::Debug,
            TRes: DeserializeOwned,
        {
            let url = format!("{}/jsonrpc", self.host);
            println!("POST {} {:?}", url, &req);
            let res = surf::post(url)
                .body(Body::from_json(&req)?)
                .recv_json()
                .await?;

            Ok(res)
        }
    }
}

mod tcp {
    use super::{RpcRequest, RpcResponse};
    use crate::Result;
    use serde::de::DeserializeOwned;
    use serde::Serialize;

    #[derive(Debug)]
    pub(crate) struct TcpTransport {}

    impl TcpTransport {
        pub fn new(_: String) -> Self {
            TcpTransport {}
        }

        pub(super) async fn request<TReq, TRes>(
            &self,
            req: RpcRequest<TReq>,
        ) -> Result<RpcResponse<TRes>>
        where
            TReq: Serialize,
            TRes: DeserializeOwned,
        {
            unimplemented!()
        }
    }
}
