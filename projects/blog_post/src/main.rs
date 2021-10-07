use blog_post::Post;

fn main() {
    let mut post = Post::new();

    post.add_text("hello");
    assert_eq!(post.content(), "");

    post.request_review();
    assert_eq!(post.content(), "");

    post.approve();
    assert_eq!(post.content(), "");

    post.approve();
    assert_eq!(post.content(), "hello");
}
