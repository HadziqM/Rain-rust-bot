pub mod websocket;



use std::{path::Path, sync::Arc, fmt::Debug, process::Command};
use axum::{routing::get, Router, extract::State, response::{IntoResponse, Html}};
use tera::{Tera, Context};
use tower_http::services::ServeDir;

#[derive(Clone)]
pub struct AppState {
    tera: Arc<Tera>
}

impl AppState {
    fn new() -> Self {
        let path = Path::new("./rain-lab/templates").join("**/*.html");
        let tera = Arc::new(Tera::new(path.to_str().unwrap()).unwrap());
        AppState {tera}
    }
    pub fn render(&self,name:&str,context:&Context) -> Result<Html<String>,WebErr> {
        Ok(Html(self.tera.render(name, context)?))
    }
}

#[derive(Debug)]
pub enum WebErr {
    Tera(tera::Error)
}

impl IntoResponse for WebErr {
    fn into_response(self) -> axum::response::Response {
        println!("got an error: {:?}",&self);
        self.to_string().into_response()
    }
}

impl std::error::Error for WebErr {
    // add code here
}

impl std::fmt::Display for WebErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Tera(x) => std::fmt::Display::fmt(&x, f)
        }
    }
}

impl From<tera::Error> for WebErr {
    fn from(value: tera::Error) -> Self {
        WebErr::Tera(value)
    }
}


async fn test_index(State(data): State<AppState>) -> Result<impl IntoResponse,WebErr> {
    Ok(data.render("index.html", &Context::new())?)
}

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let state = AppState::new();
    let router = Router::new()
        .route("/", get(test_index))
        .with_state(state)
        .fallback_service(ServeDir::new("./rain-lab/public"));

    // build tailwind script
    println!("start run script");
    let cmd = Command::new("sh")
        .args(&["-c","cd rain-lab&&npx tailwindcss -i ./tailwind.css -o ./public/index.css"]).spawn().expect("cant run the script");
    cmd.wait_with_output().expect("stuck in infinite loop");

    Ok(router.into())
}
