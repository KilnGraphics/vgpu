use serde::Deserialize;

#[derive(Deserialize)]
pub struct PushEvent<'a> {
    pub after: &'a str,
    pub sender: Sender<'a>,
    pub compare: &'a str,
    pub head_commit: HeadCommit<'a>,
}

#[derive(Deserialize)]
pub struct Sender<'a> {
    pub login: &'a str,
    pub avatar_url: &'a str,
}

#[derive(Deserialize)]
pub struct HeadCommit<'a> {
    pub message: &'a str,
}
