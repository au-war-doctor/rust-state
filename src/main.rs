extern crate blog;

use blog::Post;

fn main() {

    // States: empty draft, review, published?
    let mut post = Post::new();

    post.add_text("I ate a salad for lunch, how interesting to my viewership");
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());

    post.approve();
    assert_eq!("I ate a salad for lunch, how interesting to my viewership", post.content());
}
