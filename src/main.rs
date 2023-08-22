#[macro_use]
extern crate rocket;
use serde_json;
use rocket::serde::{Deserialize, json::Json};
use rocket::serde::Serialize;




#[derive(Serialize,Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Team {
    team: String,
    poule:i32,
    is_removed: bool
}
use rocket::tokio::fs;
// use rocket_contrib::json;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[post("/users", format = "json", data = "<teamjson>")]
async fn new_user(teamjson: Json<Team>) -> String {
    let team = Team {
        team: teamjson.team.to_owned(),
        poule: teamjson.poule.to_owned(),
        is_removed: teamjson.is_removed.to_owned(),
    };
    // let json_data = serde_json::to_string(&team).unwrap();
    let text: String =serde_json::to_string_pretty(&team).unwrap();
    fs::write("/home/jcgouleau/IdeaProjects/tournoijson/foo.txt", text).await;
    format!("Hello, {}!", team.team)
}
#[get("/hello/<name>")]
async fn hello(name: &str) -> String {
    let val=format!("Hello, {}!", name);


    fs::write("/home/jcgouleau/IdeaProjects/tournoijson/foo.txt", val).await;
    format!("Hello, {}!", name)
}
#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index,hello,new_user])
}

