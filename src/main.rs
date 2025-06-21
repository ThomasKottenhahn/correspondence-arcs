mod data;
mod board;
mod actions;

use data::setup_cards;
use board::setup_game_with_set_seed;

fn main() {
    let test_setup = setup_cards::two_player_frontiers();

    let mut found_seed = None;
    for i in 0..10000 {
        let inital_game_state = setup_game_with_set_seed(&test_setup, i);
        let has_mass_uprising = inital_game_state.court.iter().any(|c| match c {
            data::court_cards::CourtCard::VoxCard { vox, .. } => vox.title == "Mass Uprising",
            _ => false,
        });
        if has_mass_uprising {
            found_seed = Some(i);
            break;
        }
    }
    if let Some(seed) = found_seed {
        println!("Found seed with Mass Uprising: {}", seed);
    } else {
        println!("No seed found with Mass Uprising in initial court.");
    }

    for i in 0..50 {
        let inital_game_state = setup_game_with_set_seed(&test_setup,i);

        println!("{:?}: {:?}", i, inital_game_state.court.iter().map(|c| match c {
            data::court_cards::CourtCard::VoxCard { vox, .. } => vox.title.clone(),
            data::court_cards::CourtCard::GuildCard { guild, .. } => guild.title.clone(),
        }).collect::<Vec<String>>())
    }
    
}

//use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
//
//#[get("/")]
//async fn hello() -> impl Responder {
//    HttpResponse::Ok().body("Hello world!")
//}
//
//#[post("/echo")]
//async fn echo(req_body: String) -> impl Responder {
//    HttpResponse::Ok().body(req_body)
//}
//
//async fn manual_hello() -> impl Responder {
//    HttpResponse::Ok().body("Hey there!")
//}
//
//#[actix_web::main]
//async fn main() -> std::io::Result<()> {
//    HttpServer::new(|| {
//        App::new()
//            .service(hello)
//            .service(echo)
//            .route("/hey", web::get().to(manual_hello))
//    })
//    .bind(("127.0.0.1", 8080))?
//    .run()
//    .await
//}