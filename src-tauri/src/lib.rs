mod commands;

use commands::AppState;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  let app_state = AppState::new();

  tauri::Builder::default()
    .manage(app_state)
    .setup(|app| {
      if cfg!(debug_assertions) {
        app.handle().plugin(
          tauri_plugin_log::Builder::default()
            .level(log::LevelFilter::Info)
            .build(),
        )?;
      }
      Ok(())
    })
    .invoke_handler(tauri::generate_handler![
      commands::init_wallet,
      commands::get_balance,
      commands::get_new_address,
      commands::send_bitcoin,
      commands::sync_wallet,
      commands::list_transactions,
      commands::create_lightning_invoice,
      commands::pay_lightning_invoice,
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
