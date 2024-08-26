use crate::api::common::exceptions::UniverseError;

pub mod users;
pub mod storages;
pub mod files;
pub mod refresh;

pub mod trash;

#[flutter_rust_bridge::frb(opaque)]
#[derive(Clone)]
pub struct WlistClientManager {
    manager: wlist_native::core::client::WlistClientManager<String>,
}

impl WlistClientManager {
    pub async fn connect(addr: String) -> Result<Self, UniverseError> {
        let manager = wlist_native::core::client::WlistClientManager::new(addr).await?;
        Ok(Self { manager })
    }
}

macro_rules! define_func {
    ($func: ident($($para: ident: $ty: ty),*) -> $res: ty = $target: expr) => {
        pub async fn $func(client: Option<$crate::api::core::client::WlistClientManager>, $($para: $ty),*) -> Result<$res, $crate::api::common::exceptions::UniverseError> {
            let mut client = match &client {
                None => None, Some(manager) => Some(manager.manager.get().await?)
            };
            $target(&mut client.as_mut(), $($para.into()),*).await.map(Into::into).map_err(Into::into)
        }
    };
}
use define_func;
