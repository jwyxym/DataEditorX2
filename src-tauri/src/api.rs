mod cdb;
mod config;
use config::Info;
use std::path::PathBuf;
use bincode::{encode_to_vec, config::{standard, Configuration}};
use tauri::{
	ipc::Response,
	path::BaseDirectory,
	AppHandle,
	Manager
};
use tauri_plugin_os::{OsType, type_};

static CONFIG : Configuration = standard();

#[tauri::command]
pub async fn get_config (app: AppHandle) -> Response {
	app.path().resolve("./", {
		match type_() {
			OsType::Android => BaseDirectory::Public,
			_ => BaseDirectory::Resource
		}
	})
		.ok()
		.and_then(|i: PathBuf| encode_to_vec(Info::new(i.join("config.toml")).to_array(), CONFIG).ok())
		.map(Response::new)
		.unwrap_or_else(|| Response::new(Vec::new()))
}

#[tauri::command]
pub async fn init () -> Result<(), String> {
	cdb::init().await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn read_db (path: String) -> Result<(), String> {
	cdb::read(path).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn write_db (path: String, cdb: Vec<(Vec<u32>, Vec<String>)>) -> Result<(), String> {
	cdb::write(path, cdb).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_db (path: String, code: u32) -> Response {
	cdb::get(path, code).await
		.ok()
		.and_then(|i| encode_to_vec(i, CONFIG).ok())
		.map(Response::new)
		.unwrap_or_else(|| Response::new(Vec::new()))
}

#[tauri::command]
pub async fn get_list (path: String) -> Response {
	cdb::get_list(path).await
		.ok()
		.and_then(|i| encode_to_vec(i, CONFIG).ok())
		.map(Response::new)
		.unwrap_or_else(|| Response::new(Vec::new()))
}

#[tauri::command]
pub async fn get_dbs () -> Result<Vec<String>, String> {
	cdb::get_dbs().await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn create_db (path: String) -> Result<(), String> {
	cdb::create(path).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn close_db (path: String) -> Result<(), String> {
	cdb::close(path).await.map_err(|e| e.to_string())
}