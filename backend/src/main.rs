use actix_files::Files;
use actix_web::{
    error::InternalError, http::StatusCode, middleware, web, App, HttpRequest, HttpResponse,
    HttpServer,
};
use sailfish::TemplateOnce;

#[derive(sailfish_macros::TemplateOnce)]
#[template(path = "index.stpl")]
struct Index;

async fn index(_: HttpRequest) -> actix_web::Result<HttpResponse> {

    let body = Index
        .render_once()
        .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;

    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(body))
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .service(web::resource("/index.html").to(index))
            .service(web::resource("/").to(index))
            .service(Files::new("/pkg", "./pkg"))
            .service(Files::new("/static", "./static"))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
