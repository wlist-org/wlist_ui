use crate::api::common::exceptions::UniverseError;

pub mod users;

#[flutter_rust_bridge::frb(opaque)]
pub struct WlistServer {
    server: wlist_native::core::server::WlistServer,
}

impl WlistServer {
    pub fn local_addr(&self) -> String {
        self.server.local_addr().to_string()
    }

    pub async fn start(addr: String) -> Result<Self, UniverseError> {
        let server = wlist_native::core::server::WlistServer::start(addr).await?;
        Ok(Self { server })
    }

    pub async fn stop(self) -> Result<(), UniverseError> {
        self.server.stop().await.map_err(Into::into)
    }
}
