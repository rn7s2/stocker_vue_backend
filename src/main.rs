mod controllers;
mod dao;
mod models;

use controllers::item;
use dao::db::setup_db;
use rocket::{
    catch, catchers,
    serde::json::{json, Value},
};
use rocket_okapi::{
    openapi_get_routes,
    swagger_ui::{make_swagger_ui, SwaggerUIConfig},
};

#[catch(404)]
fn not_found() -> Value {
    json!({
      "error": {
        "code": 404,
        "reason": "Resource Not Found",
        "description": "Error finding resource you requested."
      }
    })
}

#[rocket::main]
async fn main() {
    let db = match setup_db().await {
        Ok(db) => db,
        Err(err) => panic!("Database error: {}.", err.to_string()),
    };

    let launch_result = rocket::build()
        .manage(db)
        .register("/", catchers![not_found])
        .mount(
            "/api",
            openapi_get_routes![
                item::get_items,
                item::create_item,
                item::modify_item,
                item::delete_item
            ],
        )
        .mount(
            "/api/swagger-ui",
            make_swagger_ui(&SwaggerUIConfig {
                url: "../openapi.json".to_owned(),
                ..Default::default()
            }),
        )
        .launch()
        .await;

    match launch_result {
        Ok(_) => println!("Shutdown successfully."),
        Err(err) => println!("Rocket had an error: {}.", err),
    }
}
