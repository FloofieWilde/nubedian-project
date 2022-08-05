#![feature(decl_macro)]
#[macro_use] extern crate rocket;
//#[macro_use] extern crate sqlx;

use rocket::response::content::{RawJson, self};
use rocket_dyn_templates::Template;
use std::fmt::Debug;
use crate::rocket::futures::FutureExt;

use rocket::Request;
use serde::Serialize;
use crate::sqlx::mysql::MySqlRow;

use rocket_db_pools::{sqlx, Database, Connection};
use rocket_db_pools::sqlx::Row;

// let mut rq = pool.prepare("SELECT * FROM cpu_list").unwrap();

//#[get("/")]
/*async fn cpu_list(mut db: Connection<Nubedian>) -> Option<RawJson<String>> {
  let var = sqlx::query("SELECT * FROM cpu_list")
  .map(|r : _| RawJson(CpuList { ID: r.ID, Price: r.Price, CPUMark: r.CPUMark, Name: r.Name, Platform: r.Platform, Socket: r.Socket, Clockspeed: r.Clockspeed, Turbospeed: r.Turbospeed, Cores: r.Cores, Threads: r.Threads, TDP: r.TDP, ReleaseDate: r.ReleaseDate,}))
  .await
  .ok();

    return var;
}*/

#[get("/temp/<name>/<last>")]
fn index(name : String, last : String) -> content::RawHtml<Template> {
  #[derive(Serialize)]
  struct Context {
    first_name: String,
    last_name: String
  }

  let context = Context {
    first_name: name,
    last_name: last
  };

  rocket::response::content::RawHtml(Template::render("home", context))
}

#[get("/post")]
async fn postt(mut db: Connection<Nubedian>) -> String {
  sqlx::query("INSERT into test(Oui) value ('Test')")
    .execute(&mut *db)
    .await;

return String::from("true");
}

#[get("/post/<test>")]
async fn posttest(mut db: Connection<Nubedian>, test : String) -> RawJson<String> {
  let rq = String::from("INSERT into test(Oui) values (\'".to_owned()+&test+"\')");

  sqlx::query(&rq)
  .execute(&mut *db)
  .await;

  let json = "{
    \"Var\": \"".to_owned()+&test+&"\",
    \"Q\": \"".to_owned()+&rq+"\",
    \"Status\": \"Posted\"
  }";
  RawJson(json)
}

#[get("/")]
async fn list(mut db: Connection<Nubedian>) -> content::RawHtml<Template> {
    let q = sqlx::query("SELECT * FROM cpu_list");

    let result: Vec<CpuList> = q
    .map(|r: MySqlRow| CpuList{
      ID: r.get("ID"), 
      Price: r.get("Price"), 
      CPUMark: r.get("CPUMark"), 
      Name: r.get("Name"), 
      Platform: r.get("Platform"), 
      Socket: r.get("Socket"), 
      Clockspeed: r.get("Clockspeed"), 
      Turbospeed: r.get("Turbospeed"), 
      Cores: r.get("Cores"), 
      Threads: r.get("Threads"), 
      TDP: r.get("TDP"), 
      ReleaseDate: r.get("ReleaseDate")
    })
    .fetch_all(&mut *db)
    .await
    .unwrap();

    //info!("{:?}",  result[0]);

    //return result[0].CPUMark.expect("REASON").to_string();
    //return result[0].Name.as_ref().expect("REASON").to_string();

    rocket::response::content::RawHtml(Template::render("cpu_list", result))
    
}

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
    .attach(Template::fairing())
    .attach(Nubedian::init())
    .mount("/", routes![hello, testjson, world, posttest, postt, index,list])
}

#[derive(Serialize)]
struct CpuList{
  ID: i32,
  Price: Option<String>,
  CPUMark: Option<i32>,
  Name: Option<String>,
  Platform: Option<String>,
  Socket: Option<String>,
  Clockspeed: Option<String>,
  Turbospeed: Option<String>,
  Cores: Option<i32>,
  Threads: Option<i32>,
  TDP: Option<String>,
  ReleaseDate: Option<String>,
}