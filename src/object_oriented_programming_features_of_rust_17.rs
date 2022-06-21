/// The implementation using the state pattern is easy to extend to add more functionality. To see
/// the simplicity of maintaining code that uses the state pattern, try a few of these suggestions:
///
/// -    Add a reject method that changes the post’s state from PendingReview back to Draft.
/// -    Require two calls to approve before the state can be changed to Published.
/// -    Allow users to add text content only when a post is in the Draft state. Hint: have the state
/// object responsible for what might change about the content but not responsible for modifying the
/// Post.
#[allow(dead_code)]
pub mod state_pattern {
    pub struct Post {
        state: Option<Box<dyn State>>,
        content: String,
    }

    impl Post {
        pub fn new() -> Post {
            Post {
                state: Some(Box::new(Draft {})),
                content: String::new(),
            }
        }

        pub fn add_text(&mut self, text: &str) {
            if let Some(new_content) = self.state.as_ref().unwrap().modify_content(self, text) {
                self.content = new_content;
            }
        }

        pub fn content(&self) -> &str {
            self.state.as_ref().unwrap().content(self)
        }

        pub fn request_review(&mut self) {
            if let Some(s) = self.state.take() {
                self.state = Some(s.request_review())
            }
        }

        pub fn approve(&mut self) {
            if let Some(s) = self.state.take() {
                self.state = Some(s.approve())
            }
        }

        pub fn reject(&mut self) {
            if let Some(s) = self.state.take() {
                self.state = Some(s.reject())
            }
        }
    }

    #[allow(unused_variables)]
    trait State {
        fn request_review(self: Box<Self>) -> Box<dyn State>;
        fn approve(self: Box<Self>) -> Box<dyn State>;
        fn reject(self: Box<Self>) -> Box<dyn State>;

        fn content<'a>(&self, post: &'a Post) -> &'a str {
            ""
        }
        fn modify_content(&self, post: &Post, text: &str) -> Option<String> {
            None
        }
    }

    struct Draft {}

    impl State for Draft {
        fn request_review(self: Box<Self>) -> Box<dyn State> {
            Box::new(PendingReview { num_approvals: 0 })
        }

        fn approve(self: Box<Self>) -> Box<dyn State> {
            self
        }

        fn reject(self: Box<Self>) -> Box<dyn State> {
            self
        }

        fn modify_content(&self, post: &Post, text: &str) -> Option<String> {
            let mut content = post.content.clone();
            content.push_str(text);
            Some(content)
        }
    }

    struct PendingReview {
        num_approvals: u32,
    }

    impl State for PendingReview {
        fn request_review(self: Box<Self>) -> Box<dyn State> {
            self
        }

        fn approve(self: Box<Self>) -> Box<dyn State> {
            let num_approvals = self.num_approvals + 1;
            if num_approvals < 2 {
                Box::new(PendingReview { num_approvals })
            } else {
                Box::new(Published {})
            }
        }

        fn reject(self: Box<Self>) -> Box<dyn State> {
            Box::new(Draft {})
        }
    }

    struct Published {}

    impl State for Published {
        fn request_review(self: Box<Self>) -> Box<dyn State> {
            self
        }

        fn approve(self: Box<Self>) -> Box<dyn State> {
            self
        }

        fn reject(self: Box<Self>) -> Box<dyn State> {
            self
        }

        fn content<'a>(&self, post: &'a Post) -> &'a str {
            &post.content
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        /// Same test as the main of the documentation, but slightly modified to respect the new
        /// requirements.
        #[test]
        fn basic_test() {
            let mut post = Post::new();

            post.add_text("I ate a salad for lunch today");
            assert_eq!("", post.content());

            post.request_review();
            assert_eq!("", post.content());

            post.approve();
            post.approve(); // NEW: Added as required by the modification
            assert_eq!("I ate a salad for lunch today", post.content());
        }

        #[test]
        fn reject_test() {
            let mut post = Post::new();

            post.add_text("I ate a salad for lunch today");
            assert_eq!("", post.content());

            post.request_review();
            assert_eq!("", post.content());

            post.reject();
            assert_eq!("", post.content());

            // Checks that the state is still in the Draft state
            post.approve();
            post.approve();
            assert_eq!("", post.content());
        }

        #[test]
        fn test_two_approvals() {
            let mut post = Post::new();

            post.add_text("I ate a salad for lunch today");
            assert_eq!("", post.content());

            post.request_review();
            assert_eq!("", post.content());

            post.approve();
            assert_eq!("", post.content());

            post.approve();
            assert_eq!("I ate a salad for lunch today", post.content());
        }

        #[test]
        fn test_add_text() {
            let mut post = Post::new();

            post.add_text("I ate a salad");
            post.add_text(" for lunch today");
            assert_eq!("I ate a salad for lunch today", post.content);

            post.request_review();
            post.add_text("Extra text");
            assert_eq!("I ate a salad for lunch today", post.content);

            post.approve();
            post.add_text("Extra text");
            assert_eq!("I ate a salad for lunch today", post.content);

            post.add_text("Extra text");
            assert_eq!("I ate a salad for lunch today", post.content);
        }
    }
}

/// Try the tasks suggested for additional requirements that we mentioned at the start of this
/// section on the blog crate as it is after Listing 17-20 to see what you think about the design of
/// this version of the code. Note that some of the tasks might be completed already in this design.
#[allow(dead_code)]
pub mod state_pattern_2 {
    pub struct Post {
        content: String,
    }

    pub struct DraftPost {
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

    impl DraftPost {
        pub fn add_text(&mut self, text: &str) {
            self.content.push_str(text);
        }

        pub fn request_review(self) -> PendingReviewPost {
            PendingReviewPost {
                content: self.content,
                num_approvals: 0,
            }
        }
    }

    pub struct PendingReviewPost {
        content: String,
        num_approvals: u8,
    }

    impl PendingReviewPost {
        pub fn approve(&mut self) -> Option<Post> {
            self.num_approvals += 1;
            if self.num_approvals >= 2 {
                Some(Post {
                    content: self.content.clone(),
                })
            } else {
                None
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
        fn basic_test() {
            let mut post = Post::new();

            post.add_text("I ate a salad for lunch today");

            let mut post = post.request_review();

            post.approve();
            let post = post.approve().unwrap(); // NEW: Added as required by the modification

            assert_eq!("I ate a salad for lunch today", post.content());
        }

        #[test]
        fn reject_test() {
            let mut post = Post::new();

            post.add_text("I ate a salad for lunch today");

            let post = post.request_review();

            post.reject();
        }

        #[test]
        fn test_two_approvals() {
            let mut post = Post::new();

            post.add_text("I ate a salad for lunch today");

            let mut post = post.request_review();

            assert!(post.approve().is_none());
            let post = post.approve().unwrap();

            assert_eq!("I ate a salad for lunch today", post.content());
        }

        #[test]
        fn test_add_text() {
            let mut post = Post::new();

            post.add_text("I ate a salad");
            post.add_text(" for lunch today");

            let mut post = post.request_review();
            // post.add_text("Extra text"); // Syntax error: Method `add_text` is not available

            post.approve();
            let post = post.approve().unwrap();
            // post.add_text("Extra text"); // Syntax error: Method `add_text` is not available

            assert_eq!("I ate a salad for lunch today", post.content());
        }
    }
}
