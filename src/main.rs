use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use std::sync::Arc;
use actix_web::web::Data;
use java_bridge::JniExecutor;

mod java_bridge;

#[post("/incident/create")]
async fn create_incident_handler(
    executor: web::Data<Arc<JniExecutor>>,
) -> impl Responder {
    match executor
        .get_ref()
        .create_incident("Title", "Description", "High")
    {
        Ok(id) => HttpResponse::Ok().body(id),
        Err(e) => HttpResponse::InternalServerError().body(format!("JNI error: {}", e)),
    }
}

#[post("/incident/change_status")]
async fn change_status_handler(
    executor: web::Data<Arc<JniExecutor>>,
) -> impl Responder {
    match executor
        .get_ref()
        .change_status("1", "Open", "User", "Comment")
    {
        Ok(status) => HttpResponse::Ok().body(status),
        Err(e) => HttpResponse::InternalServerError().body(format!("JNI error: {}", e)),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Инициализация JVM (предполагается, что это уже сделано в java_bridge)
    // В реальном коде JVM должна быть инициализирована до этого

    // Получаем JVM из java_bridge
    let jvm = java_bridge::get_jvm();

    // Создаем JniExecutor с использованием JVM
    let executor = java_bridge::JniExecutor(jvm)
        .expect("Failed to create JniExecutor");

    // Оборачиваем в Arc для безопасного разделения между потоками
    let executor_arc = std::sync::Arc::new(executor);

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(executor_arc.clone()))
            .service(create_incident_handler)
            .service(change_status_handler)
            .service(get_incident_handler)
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}