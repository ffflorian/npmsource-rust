#[macro_use] extern crate rocket;
use rocket::serde::{Serialize, json::Json};
use rocket::response::Redirect;
use rocket::http::Status;
use regex::Regex;

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct ServerInfo {
    code: u8,
    commit: String,
    version: String
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct RawResult {
    code: u8,
    url: String
}

#[get("/_health")]
fn route_health() -> Status {
  Status::Ok
}

#[get("/_info")]
fn route_info() -> Json<ServerInfo> {
  Json(ServerInfo {
    code: 200,
    commit: String::from("14daa23b3e5be7644ee66c43b591b89d66357834"),
    version: String::from("0.1.0")
  })
}

#[get("/<package_name>")]
fn route_package_name(package_name: &str) -> Json<RawResult> {
  let re = Regex::new(r"^((?:@[A-z_][A-z-_]+/)?[A-z-_][A-z_]+)(?:@([A-z_][A-z-_]+))?$").unwrap();
  println!("Got request for package \"{}\" (version \"{}\").", package_name, "0.1.0");
  assert!(re.is_match(package_name));
  Json(RawResult {
    code: 200,
    url: format!("https://github.com/{}", package_name)
  })
}

#[get("/<scope>/<package_name>")]
fn route_scoped_package_name(scope: &str, package_name: &str) -> Json<RawResult> {
  route_package_name(&format!("{}/{}", scope, package_name).to_string())
}

#[get("/")]
fn route_default() -> Redirect {
  Redirect::temporary("https://github.com/ffflorian/npmsource-rust")
}

#[launch]
fn rocket() -> _ {
  rocket::build().mount("/", routes![
    route_health,
    route_info,
    route_package_name,
    route_scoped_package_name,
    route_default
  ])
}
