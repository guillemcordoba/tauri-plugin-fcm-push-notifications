use serde::{de::DeserializeOwned, Serialize};
use serde_json::{Map, Value};
use tauri::{
    ipc::{Channel, InvokeBody},
    plugin::{PluginApi, PluginHandle},
    AppHandle, Manager, Runtime,
};

use crate::models::*;

#[cfg(target_os = "android")]
const PLUGIN_IDENTIFIER: &str = "studio.darksoil.pushnotifications";

#[cfg(target_os = "ios")]
tauri::ios_plugin_binding!(init_plugin_push - notifications);

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EventHandler {
    pub handler: Channel,
}

// initializes the Kotlin or Swift plugin classes
pub fn init<R: Runtime, C: DeserializeOwned>(
    app_handle: &AppHandle<R>,
    api: PluginApi<R, C>,
) -> crate::Result<PushNotifications<R>> {
    #[cfg(target_os = "android")]
    let handle = api.register_android_plugin(PLUGIN_IDENTIFIER, "PushNotificationsPlugin")?;
    #[cfg(target_os = "ios")]
    let handle = api.register_ios_plugin(init_plugin_push - notifications)?;

    let app_handle = app_handle.clone();

    Ok(PushNotifications(handle))
}

/// Access to the push-notifications APIs.
pub struct PushNotifications<R: Runtime>(PluginHandle<R>);

impl<R: Runtime> PushNotifications<R> {
    pub fn register_push_notification_handler<F>(&self, handler: F) -> crate::Result<()>
    where
        F: Fn(Map<String, Value>) + Sync + Send + 'static,
    {
        self.0.run_mobile_plugin::<()>(
            "registerPushNotificationHandler",
            EventHandler {
                handler: Channel::new(move |event| {
                    let data = match event {
                        InvokeBody::Json(payload) => payload.get("data").map(|s| s.to_owned()),
                        _ => None,
                    };
                    if let Some(Value::Object(object_data)) = data {
                        println!("emiitiing{object_data:?}");
                        handler(object_data);
                    }
                    Ok(())
                }),
            },
        )?;
        Ok(())
    }
}
