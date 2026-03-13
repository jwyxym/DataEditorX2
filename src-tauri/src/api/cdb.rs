use rusqlite::params;
use tokio_rusqlite::Connection;
use anyhow::{Result, Error, anyhow};
use std::{collections::BTreeMap, env, sync::OnceLock, path::{Path, PathBuf}};
use tokio::{
	join, sync::{RwLock, RwLockReadGuard, RwLockWriteGuard}, task::{JoinHandle, spawn, spawn_blocking, JoinError}
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

pub async fn write (path: String, cdb: Vec<(Vec<u32>, Vec<String>)>) -> Result<(), Error> {
	let db: &RwLock<Dbs> = DB.get().ok_or(anyhow!(""))?;
	let mut db: RwLockWriteGuard<'_, Dbs> = db.write().await;
	let i: (Result<(), Error>, Result<Result<(), Error>, JoinError>) = join!(
		write_db(path.clone(), cdb.clone()),
		spawn_blocking(move || -> Result<(), Error> {
			let db: &mut Db = db.get_mut(&path).ok_or(anyhow!("no such cdb"))?;
			Ok(cdb
				.into_iter()
				.for_each(|i| {
					db.insert(i.0[0], i.clone());
				}))
		})
	);
	i.0?;
	i.1??;
	Ok(())
}

pub async fn get (path: String, code: u32) -> Result<(Vec<u32>, Vec<String>), Error> {
	let db: &RwLock<Dbs> = DB.get().ok_or(anyhow!(""))?;
	let db: RwLockReadGuard<'_, Dbs> = db.read().await;
	Ok(db
		.get(&path)
		.ok_or(anyhow!("no such db"))?
		.get(&code)
		.ok_or(anyhow!("no such code"))
		.unwrap_or(&(Vec::new(), Vec::new()))
		.clone()
	)
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

async fn write_db<P: AsRef<Path>> (path: P, db : Vec<(Vec<u32>, Vec<String>)>) -> Result<(), Error> {
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
			) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"
		)?;
		db.iter().for_each(|i| {
			let _ = datas.execute(params![i.0[0], i.0[1], i.0[2], i.0[3], i.0[4], i.0[5], i.0[6], i.0[7], i.0[8], i.0[9], i.0[10], i.0[11]]);
			let _ = texts.execute(params![i.0[0], i.1[0], i.1[1], i.1[2], i.1[3], i.1[4], i.1[5], i.1[6], i.1[7], i.1[8], i.1[9], i.1[10], i.1[11], i.1[12], i.1[13], i.1[14], i.1[15], i.1[16], i.1[17]]);
		});
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