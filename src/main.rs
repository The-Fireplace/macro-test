use actix_web::{post, HttpResponse, Responder};
use std::collections::HashMap;

#[macro_export]
macro_rules! hashmap {
    ($($key:expr => $value:expr),*) => {{
        let mut map = HashMap::new();
        $(map.insert($key, $value);)*
        map
    }};
}

#[actix_web::main]
fn main() {
    // Not needed for this example
}

#[post("/")]// This is the key to the problem - if we remove it, the macro result type will be correctly inferred
async fn my_responder() -> impl Responder {
    let map = hashmap! {
        "one" => 1,
        "two" => 2,
        "three" => 3
    };
    let some_value = map.get("two");// Extract Method on map.get("two") with vs without this method being a #[post("/")]
    let value = some_value.unwrap_or_else(|| &0);
    HttpResponse::Ok().body(format!("The value is: {}", value))
}