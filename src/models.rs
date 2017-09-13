#[derive(Debug, Deserialize)]
pub struct App {
    pub id: String,
}

#[derive(Debug, Deserialize)]
pub struct Category {
    pub id: String,
}

#[derive(Debug, Serialize)]
pub struct Release {
    pub download: String,
    pub signature: String,
    pub nightly: bool,
}
