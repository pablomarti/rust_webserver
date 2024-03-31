#[macro_use]
extern crate rocket;

use rocket::form::Form;
use rocket::http::ContentType;
use rocket_dyn_templates::{context, Template};
use serde::{Deserialize, Serialize};

mod kafka;

#[derive(FromForm, Serialize, Deserialize, Debug)]
struct Message {
    id: String,
    field: String,
    value: String,
}

#[get("/")]
fn index() -> (ContentType, Template) {
    (ContentType::HTML, Template::render("submit", context! {}))
}

#[post("/event", data = "<message>")]
async fn event(message: Form<Message>) -> (ContentType, Template) {
    let message_inner = message.into_inner();
    let json = match serde_json::to_string(&message_inner) {
        Ok(json) => json,
        Err(e) => return (
            ContentType::HTML,
            Template::render("error", context! { message: &e.to_string() }),
        ),
    };

    match kafka::producer::produce(&json, "events").await {
        Ok((partition, offset)) => println!(
            "Message produced successfully to partition {} at offset {}",
            partition, offset
        ),
        Err(e) => return (
            ContentType::HTML,
            Template::render("error", context! { message: &e.to_string() }),
        ),
    }

    (
        ContentType::HTML,
        Template::render("result", &message_inner),
    )
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index, event])
        .attach(Template::fairing())
}
