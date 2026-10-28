use state_pattern_OOP_blog::Post;

fn main() {
    let mut post = Post::new();

    post.add_text("I ate as salad for lunch today");
    assert_eq!("", post.content());
    post.request_review();
    assert_eq!("", post.content());
    post.approve();
    assert_eq!("I ate a salad for lunch to day", post.content());
}
