use askama::Template;
use askama_web::WebTemplate;
use axum::extract::Query;
use serde::Deserialize;

#[derive(Debug, Deserialize, Default)]
pub struct Param {
    pub fdid: Option<String>,
}

#[derive(Template, WebTemplate, Debug, Default)]
#[template(path = "fda02.html")]
pub struct WebTemp {
    //fdid: String,
    name: String,
}

pub async fn page(_para: Query<Param>) -> WebTemp {
    WebTemp::default()
}
