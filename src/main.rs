//mod data;
//mod board;
//mod actions;
//mod setup_cards;
//
//use data::SetupCard;
//use board::setup_game;
//
//fn main() {
//    let test_setup: SetupCard = setup_cards::two_player_frontiers();
//
//    let inital_game_state = setup_game(&test_setup);
//
//    print!("{:?}", inital_game_state.systems.iter().map(|x|{
//        match x {
//            data::System::Unused => "Unused".to_string(),
//            data::System::Used {controlled_by: Some(_), ..} => "Controlled".to_string(),
//            data::System::Used {controlled_by: None, ..} => "None".to_string()
//        }
//    }).collect::<Vec<String>>())
//}

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

#[get("/")]
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

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}