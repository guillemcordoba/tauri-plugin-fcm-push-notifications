use serde::de::DeserializeOwned;
use tauri::{plugin::PluginApi, AppHandle, Runtime};

use crate::models::*;

pub fn init<R: Runtime, C: DeserializeOwned>(
  app: &AppHandle<R>,
  _api: PluginApi<R, C>,
) -> crate::Result<PushNotifications<R>> {
  Ok(PushNotifications(app.clone()))
}

/// Access to the push-notifications APIs.
pub struct PushNotifications<R: Runtime>(AppHandle<R>);

impl<R: Runtime> PushNotifications<R> {
  pub fn ping(&self, payload: PingRequest) -> crate::Result<PingResponse> {
    Ok(PingResponse {
      value: payload.value,
    })
  }
}
