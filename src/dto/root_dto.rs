#[derive(serde::Deserialize)]
pub struct Message {
    pub message: Option<String>,
}

#[derive(serde::Serialize)]
pub struct Response {
    pub message: Option<String>,
}