use tauri::Manager;
use tauri_plugin_notification::Channel;
use tauri_plugin_notification::NotificationExt;
use tauri_plugin_notification::PermissionState;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    println!("RUNING");
    tauri::Builder::default()
        .plugin(tauri_plugin_push_notifications::init())
        .plugin(tauri_plugin_notification::init())
        .setup(|app| {
            let mut permissions_state = app.notification().permission_state()?;
            if let PermissionState::Unknown = permissions_state {
                permissions_state = app.notification().request_permission()?;
            }
            let h = app.handle().clone();

            if let PermissionState::Granted = permissions_state {
                h.notification()
                    .create_channel(Channel::builder("test", "test").build())
                    .expect("Failed to create channel");
                #[cfg(mobile)]
                app.listen_global("push-notification-received", move |event| {
                    println!("notifiactionreceived{event:?}");
                    h.notification()
                        .builder()
                        .channel_id("test")
                        .title("Hey!")
                        .show()
                        .expect("Failed to send notification");
                });
            }

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
