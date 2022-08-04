#![feature(decl_macro)]
#[macro_use] extern crate rocket;
//#[macro_use] extern crate sqlx;

//use rocket::response::content::Json;
use rocket::response::content::RawJson;
use rocket_contrib::templates::Template;
use std::fmt::Debug;

//use rocket_db_pools::{Database, Connection};
//use rocket_db_pools::sqlx::{self, Row};
use rocket_db_pools::{sqlx, Database, Connection};
use rocket_db_pools::sqlx::Row;

//use sqlx::mysql::MySqlPoolOptions;
//use sqlx::MySqlPool;
//use sqlx::query;

// let mut rq = pool.prepare("SELECT * FROM cpu_list").unwrap();

#[get("/")]
async fn cpu_list(mut db: Connection<Nubedian>) -> Option<String> {
  sqlx::query("SELECT * FROM cpu_list")
      .fetch_one(&mut *db).await
      .and_then(|r| Ok(r.try_get(0)?))
      .ok()
}

/*#[get("/")]
async fn list(mut db: Connection<Db>) -> Result<Json<Vec<i64>>> {
    let ids = sqlx::query!("SELECT id FROM posts")
        .fetch(&mut *db)
        .map_ok(|record| record.id)
        .try_collect::<Vec<_>>()
        .await?;

    Ok(Json(ids))
}*/


#[get("/world")]
fn world() -> &'static str {
    "Hello, world!"
}

/*#[get("/test")]
async fn test(mut db: Connection<Nubedian>) -> RawJson<&'static str> {
  let ids = sqlx::query("SELECT * FROM cpu_list")
      .fetch(&mut *db)
      .map(|row: PgRow|)
      .try_collect::<Vec<_>>()
      .await?;

  Ok(RawJson(ids))
}*/

#[get("/hello")]
fn testjson() -> RawJson<&'static str> {
  RawJson("{
    \"firstName\": \"Rack's\",
    \"lastName\": \"Jackon\",
    \"gender\": \"man\",
    \"age\": 24,
    \"address\": {
      \"streetAddress\": \"126\",
      \"city\": \"San Jone\",
      \"state\": \"CA\",
      \"postalCode\": \"394221\"
    },
    \"phoneNumbers\": [
      {
        \"type\": \"home\",
        \"number\": \"7383627627\"
      }
    ]
  }")
}

#[get("/hello/<name>")]
fn hello(name: String) -> RawJson<String> {
  let json = "{
    \"firstName\": \"".to_owned()+&name+"\",
    \"lastName\": \"Jackon\",
    \"gender\": \"man\",
    \"age\": 24,
    \"address\": {
      \"streetAddress\": \"126\",
      \"city\": \"San Jone\",
      \"state\": \"CA\",
      \"postalCode\": \"394221\"
    },
    \"phoneNumbers\": [
      {
        \"type\": \"home\",
        \"number\": \"7383627627\"
      }
    ]
  }";
  RawJson(json)
}

#[derive(Database)]
#[database("nubedian")]
struct Nubedian(sqlx::MySqlPool);

#[launch]
fn rocket() -> _ {
   rocket::build()
    .attach(Nubedian::init())
    .mount("/", routes![hello, cpu_list, testjson,world])
}

struct CpuList{
  ID: i8,
  Price: String,
  CPUMark: i8,
  Name: String,
  Platform: String,
  Socket: String,
  Clockspeed: String,
  Turbospeed: String,
  Cores: i8,
  Threads: i8,
  TDP: String,
  ReleaseDate: String,
}
