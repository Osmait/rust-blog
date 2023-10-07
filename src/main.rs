use actix_cors::Cors;
use serde::Deserialize;
use serde::Serialize;
use std::sync::{Arc, Mutex};

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

#[derive(Debug, Serialize, Deserialize)]
struct Service {
    list: Vec<Product>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Product {
    id: String,
    name: String,
    description: String,
    img: String,
}

impl Service {
    // Método para agregar un elemento a la lista
    fn add_item(&mut self, item: Product) {
        self.list.push(item);
    }

    // Método para obtener una referencia a la lista
    fn get_list(&self) -> &Vec<Product> {
        &self.list
    }
}
#[post("/")]
async fn add_data(
    item: web::Json<Product>,
    service: web::Data<Arc<Mutex<Service>>>,
) -> impl Responder {
    let mut service = service.lock().unwrap();
    service.add_item(item.into_inner());
    "Item agregado a la lista".to_string()
}
#[get("/")]
async fn get_list(service: web::Data<Arc<Mutex<Service>>>) -> impl Responder {
    let service = service.lock().unwrap();
    let list = service.get_list();

    HttpResponse::Ok().json(list)
}

#[get("/one/{id}")]
async fn get_one(
    path: web::Path<String>,
    service: web::Data<Arc<Mutex<Service>>>,
) -> impl Responder {
    let service = service.lock().unwrap();
    let list = service.get_list();
    let post = list.iter().find(|x| x.id == path.as_str());

    HttpResponse::Ok().json(post)
}
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let service = Arc::new(Mutex::new(Service { list: Vec::new() }));

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header();

        App::new()
            .wrap(cors)
            .app_data(web::Data::new(service.clone()))
            .service(get_list)
            .service(add_data)
            .service(get_one)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
