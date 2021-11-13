use serde::Deserialize;

#[derive(Deserialize)]
pub struct PushEvent<'a> {
    pub after: &'a str,
    pub sender: Sender<'a>,
    pub compare: &'a str,
    pub commits: Vec<Commit<'a>>,
}

#[derive(Deserialize)]
pub struct Sender<'a> {
    pub login: &'a str,
    pub avatar_url: &'a str,
}

#[derive(Deserialize)]
pub struct Commit<'a> {
    pub id: &'a str,
    pub message: &'a str,
    pub url: &'a str,
    pub author: Author<'a>,
}

#[derive(Deserialize)]
pub struct Author<'a> {
    pub username: &'a str,
}
