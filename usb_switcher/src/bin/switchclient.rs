use std::{env, io, io::Write, net};

use actix_files as fs;
use actix_web::http::{header, Method, StatusCode};
use actix_web::{
    error, get, guard, middleware, web, App, Error, HttpRequest, HttpResponse, HttpServer, Result,
};
use bytes::Bytes;
use futures::unsync::mpsc;
use futures::{future::ok, Future, Stream};

/// 404 handler
fn p404() -> Result<fs::NamedFile> {
    Ok(fs::NamedFile::open("static/404.html")?.set_status_code(StatusCode::NOT_FOUND))
}

/// handler with path parameters like `/user/{name}/`
fn with_param(req: HttpRequest, path: web::Path<(u8)>) -> HttpResponse {
    println!("{:?}", path);
    let data = path.into_inner();
    let mut client = net::TcpStream::connect("127.0.0.1:7424").unwrap();
    client.write(std::slice::from_ref(&data)).unwrap();
    HttpResponse::Ok()
        .content_type("text/plain")
        .body(format!("Hello {}!", data))
}

fn main() -> io::Result<()> {
    env::set_var("RUST_LOG", "actix_web=debug");
    env_logger::init();
    let sys = actix_rt::System::new("basic-example");

    HttpServer::new(|| {
        App::new()
            // enable logger - always register actix-web Logger middleware last
            .wrap(middleware::Logger::default())
            // with path parameters
            .service(web::resource("/mount/{number}").route(web::get().to(with_param)))
            // default
            .default_service(
                // 404 for GET request
                web::resource("")
                    .route(web::get().to(p404))
                    // all requests that are not `GET`
                    .route(
                        web::route()
                            .guard(guard::Not(guard::Get()))
                            .to(|| HttpResponse::MethodNotAllowed()),
                    ),
            )
    })
    .bind("127.0.0.1:8080")?
    .start();

    println!("Starting http server: 127.0.0.1:8080");
    sys.run()
}
