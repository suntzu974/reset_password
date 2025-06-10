use tower_http::services::{ServeDir, ServeFile};
use axum::{
    extract::{Form, Query},
    Router,
    routing::{post, get},
    response::{Html, IntoResponse},
};
use serde::Deserialize;



#[derive(Deserialize)]
struct ForgotForm {
    email: String,
}

#[derive(Deserialize)]
struct ResetForm {
    token: String,
    password: String,
}

async fn forgot_password(Form(form): Form<ForgotForm>) -> impl IntoResponse {
    println!("Envoyer email à {}", form.email);
    Html("Lien envoyé si l’email existe")
}

async fn reset_password(Form(form): Form<ResetForm>) -> impl IntoResponse {
    println!("Reset avec token {}, nouveau mdp {}", form.token, form.password);
    Html("Mot de passe réinitialisé")
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{
    // Sert les fichiers de Yew, et renvoie index.html si non trouvé (SPA routing)
   let static_files = ServeDir::new("../frontend/dist");

    // Router sans `nest_service`, mais avec fallback
    let app = Router::new()
    .route("/api/forgot-password", post(forgot_password))
    .route("/api/reset-password", post(reset_password))
    .route("/index", axum::routing::get(|| async { "Hello from index!" }))
    .fallback_service(
            static_files
                .clone()
                .not_found_service(ServeFile::new("../frontend/dist/index.html")),
        );

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    println!("→ http://localhost:3000");

  axum::serve(listener, app).await?;
    Ok(())
}
// Note: Ensure you have the necessary dependencies in your Cargo.toml
