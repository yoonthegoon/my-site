use crate::blog::models::Post;
use askama_axum::Template;

#[derive(Template)]
#[template(path = "blog/post_list.html")]
pub struct PostListTemplate {}

#[derive(Template)]
#[template(path = "blog/post_detail.html")]
pub struct PostDetailTemplate {
    pub post: Post,
}
