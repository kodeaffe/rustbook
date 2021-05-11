enum State {
    Draft,
    PendingReview,
    Published,
}

pub struct Post {
    state: State,
    content: String,
}

impl Post {
    pub fn new() -> Post {
        Post {
            state: State::Draft,
            content: String::new(),
        }
    }

    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    pub fn content(&self) -> &str {
        match self.state {
            State::Published => &self.content,
            _ => "",
        }
    }

    pub fn request_review(&mut self) {
        match self.state {
            State::Draft => self.state = State::PendingReview,
            _ => (),
        }
    }

    pub fn approve(&mut self) {
        match self.state {
            State::PendingReview => self.state = State::Published,
            _ => (),
        }
    }
}


pub fn run() {
    println!("Running blog enum ...");

    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());

    post.approve();
    assert_eq!("I ate a salad for lunch today", post.content());
}
