use std::io::{Read, Write};

use actix_web::{
    delete, get, post, put, rt, web, App, Error, HttpRequest, HttpResponse, HttpServer, Responder,
};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use tokio::sync::{
    broadcast::{channel, Sender},
    Mutex,
};

mod http_models;

const DATABASE_PATH: &str = "./database.json";
static DATABASE: Lazy<Mutex<SuperFancyUltraFastDatabase>> =
    Lazy::new(|| Mutex::new(SuperFancyUltraFastDatabase::load()));

#[derive(Debug)]
struct AppState {
    sender: Sender<SuperFancyUltraFastDatabase>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Article {
    id: uuid::Uuid,
    amount: usize,
    label: String,
    completed: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct SuperFancyUltraFastDatabase {
    articles: Vec<Article>,
}

impl SuperFancyUltraFastDatabase {
    fn load() -> Self {
        let path = std::path::PathBuf::from(DATABASE_PATH);
        if !path.exists() {
            return SuperFancyUltraFastDatabase { articles: vec![] };
        }
        let mut handle = std::fs::File::open(path).unwrap(); // TODO: unwrap
        let mut buf = String::new();
        handle.read_to_string(&mut buf).unwrap(); // TODO: unwrap

        serde_json::from_str(&buf).unwrap() // TODO: unwrap
    }

    fn save(&self) {
        let path = std::path::PathBuf::from(DATABASE_PATH);
        let mut handle = std::fs::File::create(path).unwrap(); // TODO: unwrap
        let buf = serde_json::to_string(&self).unwrap(); // TODO: unwrap
        handle.write_all(buf.as_bytes()).unwrap(); // TODO: unwrap
    }

    async fn get_all(&self) -> Vec<Article> {
        let db = DATABASE.lock().await;
        db.articles.clone()
    }

    async fn create(&mut self, article: http_models::ArticleCreate) {
        let new_article = Article {
            id: uuid::Uuid::now_v7(),
            amount: article.amount,
            label: article.label,
            completed: false,
        };
        self.articles.push(new_article);
        self.save();
    }

    async fn update(&mut self, id: uuid::Uuid, article_update: http_models::ArticleUpdate) {
        let article = self.articles.iter_mut().find(|a| a.id == id).unwrap(); // TODO: unwrap
        article.amount = article_update.amount;
        article.completed = article_update.completed;
        self.save();
    }

    async fn delete(&mut self, id: uuid::Uuid) {
        self.articles.retain(|a| a.id != id);
        self.save();
    }

    async fn clear(&mut self) {
        self.articles.clear();
        self.save();
    }
}

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Welcome, curious crawler! Here on my server, every GET counts, but be warned: if you dare to POST any mischief, I'll swiftly PUT you into a 403 Forbidden state, and your trail will vanish like a 404 error in the void. Tread carefully—this site serves only well-formed requests, or you'll be greeted with a cheeky 418 I'm a Teapot!")
}

#[get("/article")]
async fn get_articles() -> impl Responder {
    let db = DATABASE.lock().await;
    let articles = db.get_all().await;
    let json = serde_json::to_string(&articles).unwrap(); // TODO: unwrap
    HttpResponse::Ok()
        .content_type("application/json")
        .body(json)
}

#[post("/article")]
async fn create_article(
    state: web::Data<AppState>,
    article: web::Json<http_models::ArticleCreate>,
) -> impl Responder {
    let article = article.into_inner();
    let mut db = DATABASE.lock().await;
    db.create(article).await;
    state.sender.send(db.clone()).unwrap(); // TODO: unwrap
    HttpResponse::Created()
        .content_type("application/json")
        .body(r#"{"status": true}"#)
}

#[put("/article/{article_id}")]
async fn update_article(
    state: web::Data<AppState>,
    article_id: web::Path<uuid::Uuid>,
    article: web::Json<http_models::ArticleUpdate>,
) -> impl Responder {
    let mut db = DATABASE.lock().await;
    let article = article.into_inner();
    db.update(article_id.into_inner(), article).await;
    state.sender.send(db.clone()).unwrap(); // TODO: unwrap
    HttpResponse::Ok()
        .content_type("application/json")
        .body(r#"{"status": true}"#)
}

#[delete("/article/{article_id}")]
async fn delete_article(
    state: web::Data<AppState>,
    article_id: web::Path<uuid::Uuid>,
) -> impl Responder {
    let mut db = DATABASE.lock().await;
    db.delete(article_id.into_inner()).await;
    state.sender.send(db.clone()).unwrap(); // TODO: unwrap
    HttpResponse::Ok()
        .content_type("application/json")
        .body(r#"{"status": true}"#)
}

#[post("/article/delete_all")]
async fn delete_all_articles(state: web::Data<AppState>) -> impl Responder {
    let mut db = DATABASE.lock().await;
    db.clear().await;
    state.sender.send(db.clone()).unwrap(); // TODO: unwrap
    HttpResponse::Ok()
        .content_type("application/json")
        .body(r#"{"status": true}"#)
}

#[get("/ws/updates")]
async fn ws_updates(
    req: HttpRequest,
    stream: web::Payload,
    app_state: web::Data<AppState>,
) -> std::result::Result<HttpResponse, Error> {
    let state = app_state.clone();
    let (res, mut session, _) = actix_ws::handle(&req, stream)?;

    let mut storage_rx = state.sender.subscribe();

    rt::spawn(async move {
        while let Ok(storage) = storage_rx.recv().await {
            if let Err(e) = session.text(serde_json::to_string(&storage).unwrap()).await {
                eprintln!("Error sending ws message: {:?}", e);
                let _ = session.close(None).await;
                break;
            }
        }
    });

    Ok(res)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app_state = web::Data::new(AppState {
        sender: channel(10).0,
    });

    HttpServer::new(move || {
        App::new()
            // Pass the live update channel around
            .app_data(app_state.clone())
            // Register the route handlers
            .service(index)
            .service(create_article)
            .service(update_article)
            .service(delete_article)
            .service(delete_all_articles)
            // Ws for live updates
            .service(ws_updates)
    })
    .bind("127.0.0.1:30301")?
    .run()
    .await
}
