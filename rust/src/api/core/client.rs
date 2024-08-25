use crate::api::common::exceptions::UniverseError;

pub mod users;
pub mod storages;
pub mod files;
pub mod refresh;

pub mod trash;

#[flutter_rust_bridge::frb(opaque)]
pub struct WlistClientManager {
    pool: deadpool::managed::Pool<wlist_native::core::client::WlistClientManager<String>>,
}

#[flutter_rust_bridge::frb(opaque)]
pub struct WlistClient {
    client: deadpool::managed::Object<wlist_native::core::client::WlistClientManager<String>>,
}

impl WlistClientManager {
    pub async fn connect(addr: String) -> Result<Self, UniverseError> {
        let pool = wlist_native::core::client::WlistClientManager::new(addr).await?;
        Ok(Self { pool })
    }

    pub async fn get(&self) -> Result<WlistClient, UniverseError> {
        let client = self.pool.get().await.map_err(anyhow::Error::from)?;
        Ok(WlistClient { client })
    }
}

impl WlistClient {
    fn inner(&mut self) -> &mut wlist_native::core::client::WlistClient {
        &mut self.client
    }
}

macro_rules! define_func {
    ($func: ident($($para: ident: $ty: ty),*) -> $res: ty = $target: expr) => {
        pub async fn $func(mut client: Option<$crate::api::core::client::WlistClient>, $($para: $ty),*) -> Result<$res, $crate::api::common::exceptions::UniverseError> {
            $target(&mut client.as_mut().map(|c| c.inner()), $($para.into()),*).await.map(Into::into).map_err(Into::into)
        }
    };
}
use define_func;
