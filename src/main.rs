use std::fs;
use actix_web::{get, web, App, HttpRequest, HttpServer, Responder};
use actix_files::{Files, NamedFile};

#[get("/home")]
async fn home(req: HttpRequest) -> impl Responder {
    let content = fs::read_to_string("static/home.html")
        .unwrap_or_else(|_| "<h1>404: File Not Found</h1>".to_string());
    actix_web::HttpResponse::Ok().content_type("text/html").body(content)
}

#[get("/keyboard")]
async fn keyboard(req: HttpRequest) -> impl Responder {
    let content = fs::read_to_string("static/keyboard.html")
        .unwrap_or_else(|_| "<h1>404: File Not Found</h1>".to_string());
    actix_web::HttpResponse::Ok().content_type("text/html").body(content)
}
#[get("/radio")]
async fn radio(req: HttpRequest) -> impl Responder {
    let content = fs::read_to_string("static/radio.html")
        .unwrap_or_else(|_| "<h1>404: File Not Found</h1>".to_string());
    actix_web::HttpResponse::Ok().content_type("text/html").body(content)
}
#[get("/software")]
async fn software(req: HttpRequest) -> impl Responder {
    let content = fs::read_to_string("static/software.html")
        .unwrap_or_else(|_| "<h1>404: File Not Found</h1>".to_string());
    actix_web::HttpResponse::Ok().content_type("text/html").body(content)
}
#[get("/theory")]
async fn theory(req: HttpRequest) -> impl Responder {
    let content = fs::read_to_string("static/theory.html")
        .unwrap_or_else(|_| "<h1>404: File Not Found</h1>".to_string());
    actix_web::HttpResponse::Ok().content_type("text/html").body(content)
}

async fn index(req: HttpRequest) -> impl Responder {
    let content = fs::read_to_string("static/index.html")
        .unwrap_or_else(|_| "<h1>404: File Not Found</h1>".to_string());
    actix_web::HttpResponse::Ok().content_type("text/html").body(content)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(home)
            .service(keyboard)
            .service(radio)
            .service(software)
            .service(theory)
            .service(web::resource("/").to(index))
            .service(Files::new("/", "./../style/output.css").show_files_listing())
            .service(Files::new("/", "./../static/images/").show_files_listing())
    })
        .bind(("0.0.0.0", 10000))?
        .run()
        .await
}
