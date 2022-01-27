use crate::models::post::Post;

pub fn post_fixture(id: usize) -> Post {
    Post {
        id: id as i32,
        user_id: 1,
        category_id: 1,
        title: String::from("post1"),
        content: String::from("content1"),
    }
}

pub fn posts_fixture(num: usize) -> Vec<Post> {
    let mut posts = vec![];
    for i in 1..num + 1 {
        posts.push(post_fixture(i));
    }
    posts
}
