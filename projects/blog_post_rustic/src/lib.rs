pub struct Post {
    content: String,
}

impl Post {
    pub fn new() -> DraftPost {
        DraftPost {
            content: String::new(),
        }
    }

    pub fn content(&self) -> &str {
        &self.content
    }
}

pub struct DraftPost {
    content: String,
}

impl DraftPost {
    pub fn add_text(&mut self, new_text: &str) {
        self.content.push_str(new_text);
    }

    pub fn request_review(self) -> PendingReview {
        PendingReview {
            content: self.content,
        }
    }
}

pub struct PendingReview {
    content: String,
}

impl PendingReview {
    pub fn approve(self) -> Post {
        Post {
            content: self.content,
        }
    }

    pub fn reject(self) -> DraftPost {
        DraftPost {
            content: self.content,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_flow() {
        let mut post = Post::new();

        post.add_text("Hello there");

        let post = post.request_review();

        let post = post.approve();

        assert_eq!("Hello there", post.content());
    }
}
