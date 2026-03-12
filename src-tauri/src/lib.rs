mod api;

use std::path::PathBuf;
use tauri_plugin_os::{OsType, type_};
use tauri::{
	Builder,
	generate_handler,
	path::BaseDirectory,
	Manager
};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
	Builder::default()
		.plugin(tauri_plugin_opener::init())
		.plugin(tauri_plugin_dialog::init())
		.invoke_handler(generate_handler![
			api::get_config,
			api::init,
			api::read_db,
			api::write_db,
			api::get_db,
			api::get_dbs,
			api::get_list,
			api::create_db,
			api::close_db,
		])
		.setup(|app| {
			let path: PathBuf = app.path().resolve("./", {
				match type_() {
					OsType::Android => BaseDirectory::Public,
					_ => BaseDirectory::Resource
				}
			})?;
			let _ = api::PATH.set(path);
			Ok(())
		})
		.run(tauri::generate_context!())
		.expect("error while running tauri application");
}