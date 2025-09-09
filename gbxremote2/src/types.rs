use serde::{Deserialize, Serialize};

use crate::{ClientError, ServerClient};

#[derive(Debug, Serialize, Deserialize)]
pub struct WayPointEvent {
    #[serde(rename = "accountid")]
    account_id: String,
    login: String,
    time: u32,
    racetime: u32,
    laptime: u32,
    speed: f32,

    #[serde(rename = "checkpointinrace")]
    checkpoint_in_race: u32,
    #[serde(rename = "checkpointinlap")]
    checkpoint_in_lap: u32,
    #[serde(rename = "isendrace")]
    is_end_race: bool,
    #[serde(rename = "isendlap")]
    is_end_lap: bool,
    #[serde(rename = "isinfinitelaps")]
    is_infinite_laps: bool,
    #[serde(rename = "isindependentlaps")]
    is_independent_laps: bool,
    #[serde(rename = "curracecheckpoints")]
    current_race_checkpoints: Vec<u32>,
    #[serde(rename = "curlapcheckpoints")]
    current_lap_checkpoints: Vec<u32>,
    #[serde(rename = "blockid")]
    block_id: String,
}

pub trait ModeScriptCallbacks {
    fn on_way_point(&self, execute: impl Fn(WayPointEvent) + Send + Sync + 'static);
}

impl ModeScriptCallbacks for ServerClient {
    fn on_way_point(&self, execute: impl Fn(WayPointEvent) + Send + Sync + 'static) {
        self.on("Trackmania.Event.WayPoint", execute);
    }
}

#[allow(async_fn_in_trait)]
pub trait ModeScriptMethodsXmlRpc {
    async fn enable_callbacks(&self, enable: bool) -> Result<bool, ClientError>;
    async fn get_callbacks_list(&self, enable: bool) -> Result<bool, ClientError>;
    async fn get_callbacks_list_enabled(&self, enable: bool) -> Result<bool, ClientError>;
    async fn get_callbacks_list_disabled(&self, enable: bool) -> Result<bool, ClientError>;
    async fn block_callbacks(&self, enable: bool) -> Result<bool, ClientError>;
    async fn unblock_callbacks(&self, enable: bool) -> Result<bool, ClientError>;
    async fn get_callback_help(&self, enable: bool) -> Result<bool, ClientError>;
    async fn get_methods_list(&self, enable: bool) -> Result<bool, ClientError>;
    async fn get_method_help(&self, enable: bool) -> Result<bool, ClientError>;
    async fn get_doscumentation(&self, enable: bool) -> Result<bool, ClientError>;
    async fn set_api_version(&self, enable: bool) -> Result<bool, ClientError>;
    async fn get_api_version(&self, enable: bool) -> Result<bool, ClientError>;
    async fn get_all_api_versions(&self, enable: bool) -> Result<bool, ClientError>;
}

impl ModeScriptMethodsXmlRpc for ServerClient {
    ///Enable or disable mode script callbacks.
    async fn enable_callbacks(&self, enable: bool) -> Result<bool, ClientError> {
        self.call(
            "TriggerModeScriptEventArray",
            ("XmlRpc.EnableCallbacks", ["true"]),
        )
        .await
    }

    async fn get_callbacks_list(&self, enable: bool) -> Result<bool, ClientError> {
        todo!()
    }

    async fn get_callbacks_list_enabled(&self, enable: bool) -> Result<bool, ClientError> {
        todo!()
    }

    async fn get_callbacks_list_disabled(&self, enable: bool) -> Result<bool, ClientError> {
        todo!()
    }

    async fn block_callbacks(&self, enable: bool) -> Result<bool, ClientError> {
        todo!()
    }

    async fn unblock_callbacks(&self, enable: bool) -> Result<bool, ClientError> {
        todo!()
    }

    async fn get_callback_help(&self, enable: bool) -> Result<bool, ClientError> {
        todo!()
    }

    async fn get_methods_list(&self, enable: bool) -> Result<bool, ClientError> {
        todo!()
    }

    async fn get_method_help(&self, enable: bool) -> Result<bool, ClientError> {
        todo!()
    }

    async fn get_doscumentation(&self, enable: bool) -> Result<bool, ClientError> {
        todo!()
    }

    async fn set_api_version(&self, enable: bool) -> Result<bool, ClientError> {
        todo!()
    }

    async fn get_api_version(&self, enable: bool) -> Result<bool, ClientError> {
        todo!()
    }

    async fn get_all_api_versions(&self, enable: bool) -> Result<bool, ClientError> {
        todo!()
    }
}

#[allow(async_fn_in_trait)]
pub trait XmlRpcMethods {
    async fn kick(&self, player: String, message: Option<String>) -> Result<bool, ClientError>;

    async fn add_guest(&self, player: String) -> Result<bool, ClientError>;

    async fn auto_save_replays(&self, enable: bool) -> Result<bool, ClientError>;

    async fn is_auto_save_replays_enabled(&self) -> Result<bool, ClientError>;
}

impl XmlRpcMethods for ServerClient {
    async fn kick(&self, player: String, message: Option<String>) -> Result<bool, ClientError> {
        todo!()
    }

    async fn add_guest(&self, player: String) -> Result<bool, ClientError> {
        todo!()
    }

    async fn auto_save_replays(&self, enable: bool) -> Result<bool, ClientError> {
        self.call("AutoSaveReplays", enable).await
    }

    async fn is_auto_save_replays_enabled(&self) -> Result<bool, ClientError> {
        self.call("IsAutoSaveReplaysEnabled", ()).await
    }
}
