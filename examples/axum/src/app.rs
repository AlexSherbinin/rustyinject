use std::ops::Deref;

use crate::{analytics::AnalyticsRepository, posts::PostsRepository};
use rustyinject::injector::factories::RefConstructorFactory;

pub struct App<'a> {
    posts_repository: &'a dyn PostsRepository,
    analytics_repository: &'a dyn AnalyticsRepository,
}

impl<'a> RefConstructorFactory<'a> for App<'a> {
    type Dependencies = (
        &'a Box<dyn PostsRepository>,
        (&'a Box<dyn AnalyticsRepository>, ()),
    );

    fn build(dependencies: Self::Dependencies) -> Self {
        Self {
            posts_repository: dependencies.0.deref(),
            analytics_repository: dependencies.1 .0.deref(),
        }
    }
}

impl<'a> App<'a> {
    pub async fn create_post(&self, title: String, text: String) -> usize {
        let post_id = self.posts_repository.create(title, text).await;
        self.analytics_repository.post_created(post_id).await;
        post_id
    }

    pub async fn show_post(&self, id: usize) -> Option<String> {
        self.posts_repository.show(id).await
    }

    pub async fn like_post(&self, id: usize) {
        self.posts_repository.like(id).await;
        self.analytics_repository.post_liked(id).await;
    }

    pub async fn show_analytics(&self) -> String {
        self.analytics_repository.show().await
    }
}
