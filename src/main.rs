#[macro_use] extern crate rocket;
use rocket::serde::{Serialize, json::Json};

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct ServerInfo<'r> {
    code: u8,
    commit: &'r str,
    version: &'r str
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct RawResult {
    code: u8,
    url: String
}

#[get("/_health")]
fn health() -> String {
  format!("OK")
}

#[get("/_info")]
fn info() -> Json<ServerInfo<'static>> {
  Json(ServerInfo {
    code: 200,
    commit: "14daa23b3e5be7644ee66c43b591b89d66357834",
    version: "0.1.0"
  })
}

#[get("/<package_name>")]
fn package_name(package_name: &str) -> Json<RawResult> {
  Json(RawResult {
    code: 200,
    url: format!("https://github.com/{}", package_name)
  })
}

#[launch]
fn rocket() -> _ {
  rocket::build().mount("/", routes![health, info, package_name])
}
