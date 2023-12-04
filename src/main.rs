
use ntex::web;

#[web::get("/")]
async fn hello() -> impl web::Responder {
    web::HttpResponse::Ok().body("<h1>Hello Winter! Hello Ntex!</h1>")
}

#[web::post("/echo")]
async fn echo(req_body: String) -> impl web::Responder {
    web::HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl web::Responder {
    web::HttpResponse::Ok().body("<h1>Hey Winter! Hey Ntex!</h1>")
}

#[ntex::main]
async fn main() -> std::io::Result<()> {
    println!("\nHTTP server at http://0.0.0.0:8080/\n");
    web::HttpServer::new(|| {
        web::App::new()
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
