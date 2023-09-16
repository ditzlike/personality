use axum::{routing::get, Router};
use std::net::SocketAddr;
use rand::Rng;

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(move || async { 
        let questions: [&'static str;3]  = ["Was ist dein name\n", "Wann ist Bubatz auf\n", "Wie alt bist du\n"];
        let num = rand::thread_rng().gen_range(0..3);
        questions[num]
    }));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("listening on {}", addr);
    axum_server::bind(addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
