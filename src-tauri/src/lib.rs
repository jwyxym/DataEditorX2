mod api;

use tauri::{
	Builder,
	generate_handler,
	AppHandle,
	Emitter,
	Manager
};
use std::path::Path;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
	let mut builder = Builder::default();
	#[cfg(desktop)]
    {
        builder = builder
			.plugin(tauri_plugin_single_instance::init(|app: &AppHandle, args: Vec<String>, cwd: String| {
				let _ = app.get_webview_window("main")
					.expect("no main window")
					.set_focus();
				let path: Vec<String> = args
					.into_iter()
					.skip(1)
					.filter_map(|i: String| {
						Path::new(&cwd)
							.join(&i)
							.as_os_str()
							.to_str()
							.map(|i: &str| String::from(i))
					})
					.collect();
				let _ = app.emit("new_db", path);
			}))
    }

    builder
		.plugin(tauri_plugin_opener::init())
		.plugin(tauri_plugin_dialog::init())
		.invoke_handler(generate_handler![
			api::get_config,
			api::init,
			api::read_db,
			api::write_db,
			api::del_db,
			api::get_db,
			api::get_dbs,
			api::get_list,
			api::search_list,
			api::create_db,
			api::close_db,
			api::save_lua,
		])
		.run(tauri::generate_context!())
		.expect("error while running tauri application");
}