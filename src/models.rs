#[derive(Debug, Deserialize)]
pub struct App {
    pub id: String,
    pub authors: Vec<Author>,
    pub categories: Vec<String>,
    pub isFeatured: bool,
}

#[derive(Debug, Deserialize)]
pub struct Author {
    pub name: String,
    pub mail: String,
    pub homepage: String,
}

#[derive(Debug, Deserialize)]
pub struct Category {
    pub id: String,
}

#[derive(Debug, Serialize)]
pub struct NewRelease {
    pub download: String,
    pub signature: String,
    pub nightly: bool,
}
