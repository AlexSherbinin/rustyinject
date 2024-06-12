use std::{future::Future, pin::Pin};
use tokio::sync::RwLock;

pub trait PostsRepository: Send + Sync {
    fn create<'a>(
        &'a self,
        title: String,
        text: String,
    ) -> Pin<Box<dyn Future<Output = usize> + Send + 'a>>;

    fn like<'a>(&'a self, id: usize) -> Pin<Box<dyn Future<Output = ()> + Send + 'a>>;

    fn show<'a>(&'a self, id: usize) -> Pin<Box<dyn Future<Output = Option<String>> + Send + 'a>>;
}

#[derive(Clone)]
pub struct Post {
    title: String,
    text: String,
    likes_count: u32,
}

#[derive(Default)]
pub(super) struct InMemoryPostsRepository(RwLock<Vec<Post>>);

impl PostsRepository for InMemoryPostsRepository {
    fn create<'a>(
        &'a self,
        title: String,
        text: String,
    ) -> Pin<Box<dyn Future<Output = usize> + Send + 'a>> {
        Box::pin(async move {
            let mut posts = self.0.write().await;
            let post_id = posts.len();
            posts.push(Post {
                title,
                text,
                likes_count: 0,
            });
            post_id
        })
    }

    fn like<'a>(&'a self, id: usize) -> Pin<Box<dyn Future<Output = ()> + Send + 'a>> {
        Box::pin(async move {
            let mut posts = self.0.write().await;
            if let Some(post) = posts.get_mut(id) {
                post.likes_count += 1;
            }
        })
    }

    fn show<'a>(&'a self, id: usize) -> Pin<Box<dyn Future<Output = Option<String>> + Send + 'a>> {
        Box::pin(async move {
            let posts = self.0.read().await;
            let post = posts.get(id)?;

            Some(format!(
                "{}\n\n{}\n\nLikes: {}",
                post.title, post.text, post.likes_count
            ))
        })
    }
}
