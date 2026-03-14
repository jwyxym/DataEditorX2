use rusqlite::params;
use tokio_rusqlite::Connection;
use anyhow::{Result, Error, anyhow};
use std::{collections::BTreeMap, env, sync::OnceLock, path::{Path, PathBuf}};
use walkdir::{WalkDir, DirEntry};
use tokio::{
	sync::{RwLock, RwLockReadGuard, RwLockWriteGuard}, task::{JoinHandle, spawn}
};

pub static DB: OnceLock<RwLock<Dbs>> = OnceLock::new();

type Db = BTreeMap<u32, (Vec<u32>, Vec<String>)>;
type Dbs = BTreeMap<String, Db>;

pub async fn init () -> Result<(), Error> {
	let mut db: Dbs = BTreeMap::new();
	let cwd: PathBuf = env::current_dir()?;
	let args: Vec<String> = env::args()
		.skip(1)
		.filter_map(|i: String| {
			Path::new(&cwd)
				.join(&i)
				.as_os_str()
				.to_str()
				.map(|i: &str| String::from(i))
		})
		.collect();
	let tasks: Vec<JoinHandle<Result<(String, Db), Error>>> = args
		.into_iter()
		.map(|i| spawn(async {
			let db: Db = read_db(&i).await?;
			Ok((String::from(i), db))
		}))
		.collect();
	for task in tasks {
		if let Ok(i) = task.await {
			if let Ok(i) = i {
				db.insert(i.0, i.1);
			}
		}
	}
	let db: RwLock<Dbs> = RwLock::new(db);
	let _ = DB.set(db).map_err(|_| anyhow!("read cdb error"));
	Ok(())
}

pub async fn read (path: String) -> Result<(), Error> {
	let db: &RwLock<Dbs> = DB.get().ok_or(anyhow!(""))?;
	let mut db: RwLockWriteGuard<'_, Dbs> = db.write().await;
	let cdb: Db = read_db(&path).await?;
	db.insert(path, cdb);
	Ok(())
}

pub async fn write (path: String, code: u32, cdb: (Vec<u32>, Vec<String>)) -> Result<(), Error> {
	let db: &RwLock<Dbs> = DB.get().ok_or(anyhow!(""))?;
	let mut db: RwLockWriteGuard<'_, Dbs> = db.write().await;
	let db: &mut Db = db.get_mut(&path).ok_or(anyhow!("no such cdb"))?;
	if cdb.0[0] != code && db.contains_key(&cdb.0[0]) {
		return Err(anyhow!("change code error"));
	}
	if cdb.0[0] != code {
		db.remove(&code);
		del_db(&path, code).await?;
	}
	db.insert(cdb.0[0], cdb.clone());
	write_db(&path, cdb.clone()).await?;
	Ok(())
}

pub async fn del (path: String, code: u32) -> Result<(), Error> {
	let db: &RwLock<Dbs> = DB.get().ok_or(anyhow!(""))?;
	let mut db: RwLockWriteGuard<'_, Dbs> = db.write().await;
	let db: &mut Db = db.get_mut(&path).ok_or(anyhow!("no such cdb"))?;
	db.remove(&code);
	del_db(&path, code).await?;
	Ok(())
}

pub async fn get (path: String, code: u32) -> Result<(String, (Vec<u32>, Vec<String>)), Error> {
	let db: &RwLock<Dbs> = DB.get().ok_or(anyhow!(""))?;
	let db: RwLockReadGuard<'_, Dbs> = db.read().await;
	let db: (Vec<u32>, Vec<String>) = db
		.get(&path)
		.ok_or(anyhow!("no such db"))?
		.get(&code)
		.ok_or(anyhow!("no such code"))
		.unwrap_or(&(Vec::new(), Vec::new()))
		.clone();
	let pic: String = (|| -> Result<String, Error> {
		let path: PathBuf = Path::new(&path)
			.parent()
			.ok_or(anyhow!("path error"))?
			.join("pics");
		let pic: DirEntry = WalkDir::new(path)
			.into_iter()
			.filter_map(|i| i.ok())
			.find(|i| {
				let name: &str = i.path()
					.file_name()
					.and_then(|n| n.to_str())
					.unwrap_or("");
				let ext: &str = i.path()
					.extension()
					.and_then(|n| n.to_str())
					.unwrap_or("");
				name.starts_with(&format!("{}.", code))
					&& ["jpg", "jpeg", "png", "webp"].contains(&ext)
			})
			.ok_or(anyhow!("no such path"))?;
		Ok(pic
			.path()
			.as_os_str()
			.to_str()
			.ok_or(anyhow!("no such path"))?
			.to_string())
	})().unwrap_or(String::new());
	Ok((pic, db))
}

pub async fn get_list (path: String) -> Result<Vec<(u32, String)>, Error> {
	let db: &RwLock<Dbs> = DB.get().ok_or(anyhow!(""))?;
	let db: RwLockReadGuard<'_, Dbs> = db.read().await;
	Ok(db
		.get(&path)
		.ok_or(anyhow!("no such db"))?
		.into_iter()
		.map(|i: (&u32, &(Vec<u32>, Vec<String>))| (*i.0, i.1.1[0].clone()))
		.collect())
}

pub async fn get_dbs () -> Result<Vec<String>, Error> {
	let db: &RwLock<Dbs> = DB.get().ok_or(anyhow!(""))?;
	let db: RwLockReadGuard<'_, Dbs> = db.read().await;
	Ok(db.keys()
		.map(|i| i.clone())
		.collect())
}

pub async fn create (path: String) -> Result<(), Error> {
	let db: &RwLock<Dbs> = DB.get().ok_or(anyhow!(""))?;
	let mut db: RwLockWriteGuard<'_, Dbs> = db.write().await;
	db.insert(path.clone(), BTreeMap::new());
	create_db(path).await
}

pub async fn close (path: String) -> Result<(), Error> {
	let db: &RwLock<Dbs> = DB.get().ok_or(anyhow!(""))?;
	let mut db: RwLockWriteGuard<'_, Dbs> = db.write().await;
	db.remove(&path);
	Ok(())
}

async fn read_db<P: AsRef<Path>> (path: P) -> Result<Db, Error> {
	let conn: Connection = Connection::open(path).await?;
	let result: Vec<(u32, (Vec<u32>, Vec<String>))> = conn
		.call(|conn| {
			let mut stmt = conn.prepare("SELECT
				datas.id,
				datas.ot,
				datas.alias,
				datas.setcode,
				datas.type,
				datas.atk,
				datas.def,
				datas.level,
				datas.race,
				datas.attribute,
				datas.category,
				texts.id,
				texts.name,
				texts.desc,
				texts.str1,
				texts.str2,
				texts.str3,
				texts.str4,
				texts.str5,
				texts.str6,
				texts.str7,
				texts.str8,
				texts.str9,
				texts.str10,
				texts.str11,
				texts.str12,
				texts.str13,
				texts.str14,
				texts.str15,
				texts.str16
				FROM datas, texts WHERE datas.id = texts.id")?;
			
			let result = stmt.query_map([], |row| {
				let int: Vec<u32> = (0..12)
					.map(|i| row.get::<_, u32>(i))
					.collect::<Result<Vec<u32>, _>>()?;
				
				let string: Vec<String> = (12..30)
					.map(|i| row.get::<_, String>(i))
					.collect::<Result<Vec<String>, _>>()?;
				
				Ok((int[0], (int, string)))
			})?;

			Ok::<Vec<(u32, (Vec<u32>, Vec<String>))>, Error>(result
				.filter_map(Result::ok)
				.collect()
			)
		})
		.await
		.map_err(|e| anyhow!("{}", e))?;

	let result: Db = result.into_iter().collect();
	Ok(result)
}

async fn del_db<P: AsRef<Path>> (path: P, code: u32) -> Result<(), Error> {
	let conn: Connection = Connection::open(path).await?;
	conn.call(move |conn| {
		let tx = conn.transaction()?;
		
		let mut datas = tx.prepare("DELETE FROM datas WHERE id = ?")?;
		let mut texts = tx.prepare("DELETE FROM texts WHERE id = ?")?;
		
		let _ = datas.execute([code])?;
		let _ = texts.execute([code])?;

		drop(datas);
		drop(texts);
		
		tx.commit()?;
		Ok::<(), Error>(())
	}).await
		.map_err(|e| anyhow!("{}", e))?;
	Ok(())
}
async fn write_db<P: AsRef<Path>> (path: P, db : (Vec<u32>, Vec<String>)) -> Result<(), Error> {
	let conn: Connection = Connection::open(path).await?;
	conn.call(move |conn| {
		let tx = conn.transaction()?;
		let mut datas = tx.prepare(
			"INSERT OR REPLACE INTO datas (
				id, ot, alias, setcode, type, atk, def, 
				level, race, attribute, category
			) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"
		)?;
		let mut texts = tx.prepare(
			"INSERT OR REPLACE INTO texts (
				id, name, desc, 
				str1, str2, str3, str4, str5, str6, str7, str8,
				str9, str10, str11, str12, str13, str14, str15, str16
			) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"
		)?;
		let _ = datas.execute(params![db.0[0], db.0[1], db.0[2], db.0[3], db.0[4], db.0[5], db.0[6], db.0[7], db.0[8], db.0[9], db.0[10]])?;
		let _ = texts.execute(params![db.0[0], db.1[0], db.1[1], db.1[2], db.1[3], db.1[4], db.1[5], db.1[6], db.1[7], db.1[8], db.1[9], db.1[10], db.1[11], db.1[12], db.1[13], db.1[14], db.1[15], db.1[16], db.1[17]])?;
		drop(datas);
		drop(texts);
		tx.commit()?;
		Ok::<(), Error>(())
	}).await
		.map_err(|e| anyhow!("{}", e))?;
	Ok(())
}

async fn create_db<P: AsRef<Path>> (path: P) -> Result<(), Error> {
	let conn: Connection = Connection::open(path).await?;
	conn.call(move |conn| {
		conn.execute("CREATE TABLE datas(id integer primary key,ot integer,alias integer,setcode integer,type integer,atk integer,def integer,level integer,race integer,attribute integer,category integer);", [])?;
		conn.execute("CREATE TABLE texts(id integer primary key,name text,desc text,str1 text,str2 text,str3 text,str4 text,str5 text,str6 text,str7 text,str8 text,str9 text,str10 text,str11 text,str12 text,str13 text,str14 text,str15 text,str16 text);", [])
	}).await?;
	Ok(())
}