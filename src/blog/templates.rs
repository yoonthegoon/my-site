use askama::Template;

#[derive(Template)]
#[template(path = "blog/index.html")]
pub struct IndexTemplate {}
