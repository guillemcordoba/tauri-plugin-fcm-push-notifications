use tauri::{
  plugin::{Builder, TauriPlugin},
  Manager, Runtime,
};

use std::{collections::HashMap, sync::Mutex};

pub use models::*;

#[cfg(desktop)]
mod desktop;
#[cfg(mobile)]
mod mobile;

mod commands;
mod error;
mod models;

pub use error::{Error, Result};

#[cfg(desktop)]
use desktop::PushNotifications;
#[cfg(mobile)]
use mobile::PushNotifications;

#[derive(Default)]
struct MyState(Mutex<HashMap<String, String>>);

/// Extensions to [`tauri::App`], [`tauri::AppHandle`] and [`tauri::Window`] to access the push-notifications APIs.
pub trait PushNotificationsExt<R: Runtime> {
  fn push_notifications(&self) -> &PushNotifications<R>;
}

impl<R: Runtime, T: Manager<R>> crate::PushNotificationsExt<R> for T {
  fn push_notifications(&self) -> &PushNotifications<R> {
    self.state::<PushNotifications<R>>().inner()
  }
}

/// Initializes the plugin.
pub fn init<R: Runtime>() -> TauriPlugin<R> {
  Builder::new("push-notifications")
    .invoke_handler(tauri::generate_handler![commands::execute])
    .setup(|app, api| {
      #[cfg(mobile)]
      let push_notifications = mobile::init(app, api)?;
      #[cfg(desktop)]
      let push_notifications = desktop::init(app, api)?;
      app.manage(push_notifications);

      // manage state so it is accessible by the commands
      app.manage(MyState::default());
      Ok(())
    })
    .build()
}
