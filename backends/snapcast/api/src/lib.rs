use self::models::*;
pub use crate::error::*;
use crate::rpc::{HttpTransport, SnapcastTransport, TcpTransport};
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::sync::atomic::{AtomicU64, Ordering};

mod error;
mod rpc;

#[derive(Debug)]
pub struct SnapcastClient {
    transport: SnapcastTransport,
    request_id: AtomicU64,
}

impl SnapcastClient {
    #[cfg(feature = "http")]
    pub fn http(url: String) -> Self {
        SnapcastClient {
            transport: SnapcastTransport::Http(HttpTransport::new(url)),
            request_id: AtomicU64::new(0),
        }
    }

    #[cfg(feature = "tcp")]
    pub fn tcp(url: &str) -> Self {
        SnapcastClient {
            transport: SnapcastTransport::Tcp(TcpTransport::new(url.to_string())),
            request_id: AtomicU64::new(0),
        }
    }

    pub async fn get_client_status(&self, id: String) -> Result<Client> {
        let response = self
            .request("Client.GetStatus", &GetClientStatusRequest { id })
            .await?;

        Ok(response)
    }

    /// Returns Stream ID
    pub async fn add_stream(&self, stream_uri: String) -> Result<String> {
        let response = self
            .request::<_, StreamId>("Stream.AddStream", &AddStreamRequest { stream_uri })
            .await?;

        Ok(response.id)
    }

    /// Returns Stream ID
    pub async fn remove_stream(&self, stream_id: String) -> Result<String> {
        let response = self
            .request::<_, StreamId>("Stream.RemoveStream", &StreamId { id: stream_id })
            .await?;

        Ok(response.id)
    }

    async fn request<TReq, TRes>(&self, method: &str, params: &TReq) -> Result<TRes>
    where
        TReq: Serialize + std::fmt::Debug,
        TRes: DeserializeOwned,
    {
        self.transport
            .request(
                method,
                params,
                self.request_id.fetch_add(1, Ordering::Relaxed),
            )
            .await
    }
}

pub mod models {
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Clone, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub(crate) struct AddStreamRequest {
        pub stream_uri: String,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub(crate) struct StreamId {
        pub id: String,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub(crate) struct GetClientStatusRequest {
        pub id: String,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub(crate) struct GetStatusResponse {
        pub client: Client,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Client {
        pub config: Config,
        pub connected: bool,
        pub host: Host,
        pub id: String,
        #[serde(rename = "lastSeen")]
        pub last_seen: LastSeen,
        pub snapclient: Snapclient,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Config {
        pub instance: i64,
        pub latency: i64,
        pub name: String,
        pub volume: Volume,
    }

    #[derive(Debug, Clone, Copy, Serialize, Deserialize)]
    pub struct Volume {
        pub muted: bool,
        pub percent: i64,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Host {
        pub arch: String,
        pub ip: String,
        pub mac: String,
        pub name: String,
        pub os: String,
    }

    #[derive(Debug, Clone, Copy, Serialize, Deserialize)]
    pub struct LastSeen {
        pub sec: i64,
        pub usec: i64,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Snapclient {
        pub name: String,
        #[serde(rename = "protocolVersion")]
        pub protocol_version: i64,
        pub version: String,
    }
}
