#![feature(decl_macro)]
#[macro_use] extern crate rocket;
//#[macro_use] extern crate sqlx;

use std::process::id;

use rocket::response::content::{RawJson, self};
use rocket_dyn_templates::Template;

use serde::Serialize;
use crate::sqlx::mysql::MySqlRow;

use rocket_db_pools::{sqlx, Database, Connection};
use rocket_db_pools::sqlx::Row;

/// The function you're interressed into : it shows the CPU List
/// it uses templating (handlebars, file : ./templates/cpu_list)
/// URL : http://127.0.0.1:8000/
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

    rocket::response::content::RawHtml(Template::render("cpu_list", result))
    
}

#[get("/<id>")]
async fn Only_one(mut db: Connection<Nubedian>, id: &str) -> content::RawHtml<Template> {
    let rq = "SELECT * FROM cpu_list WHERE id = ".to_owned()+id;
    let q = sqlx::query(&rq);

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

    rocket::response::content::RawHtml(Template::render("cpu_details", result))
    
}

/// A function that will insert something in the table test
/// URL : http://127.0.0.1:8000/post/{the thing to insert}
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

/// A random JSON test
/// URL : http://127.0.0.1:8000/hello
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

#[derive(Database)]
#[database("nubedian")]
struct Nubedian(sqlx::MySqlPool);

/// The main Rocket function
#[launch]
fn rocket() -> _ {
   rocket::build()
    .attach(Template::fairing())
    .attach(Nubedian::init())
    .mount("/", routes![testjson, posttest, list, Only_one])
}

/// I intentionnaly didn't respected snake case for the variables to match names with the database
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