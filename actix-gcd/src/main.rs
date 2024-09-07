use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use serde::Deserialize;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(get_index))
            .service(hello)
            .service(echo)
            .service(post_gcd)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

#[get("/hello")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

async fn get_index() -> impl Responder {
    HttpResponse::Ok()
        .content_type("text/html")
        .body(
            r#"
                <h3> GCD Calculator </h3>
                <form action="/gcd" method="post">
                    <input type="text" name="n" />
                    <input type="text" name="m" />
                    <button type="submit">Compute GCD</button>
                </form> 
                
            "#,
        )
}

#[derive(Deserialize)]
struct GcdParameters {
    n: u64,
    m: u64
}

#[post("/gcd")]
async fn post_gcd(form: web::Form<GcdParameters>) -> impl Responder {
    if form.n == 0 || form.n == 0 {
        return HttpResponse::BadRequest()
        .content_type("text/html")
        .body("Computing the GCD with zero is boring.");
    }

    let gcd_result = find_gcd(form.n, form.m);
    let response = 
        format!("The gratest common divisor of the numbers {} and {} \
         is <b>{}</b> \n ",
            form.n, form.m, gcd_result);
        HttpResponse::Ok()
        .content_type("text/html")
        .body(response)
}

fn find_gcd(mut m: u64, mut n: u64) -> u64 {
    assert!(m !=0 && n!=0);
    while m != 0 {
        if m < n {
            let t = m;
            m = n;
            n = t;
        }
        m = m%n;
    }
    n
}