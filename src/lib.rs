struct Post {
    state: Option<Box<dyn State>>,
    content: String
}

impl Post {
    fn new() -> Post {

    }
}

trait State {}

struct Draft {}

impl State for Draft {}