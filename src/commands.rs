use tauri::{command, AppHandle, Runtime, State, Window};

use crate::Result;

#[command]
pub(crate) async fn execute<R: Runtime>(_app: AppHandle<R>, _window: Window<R>) -> Result<String> {
    Ok("success".to_string())
}
