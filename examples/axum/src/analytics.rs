use std::{fmt::Display, fmt::Write, future::Future, pin::Pin, time::Instant};
use tokio::sync::RwLock;

pub trait AnalyticsRepository: Send + Sync {
    fn post_created<'a>(&'a self, id: usize) -> Pin<Box<dyn Future<Output = ()> + Send + 'a>>;
    fn post_liked<'a>(&'a self, id: usize) -> Pin<Box<dyn Future<Output = ()> + Send + 'a>>;
    fn show<'a>(&'a self) -> Pin<Box<dyn Future<Output = String> + Send + 'a>>;
}

pub enum AnalyticsKind {
    PostCreated(usize),
    PostLiked(usize),
}

pub struct AnalyticsItem {
    time: Instant,
    kind: AnalyticsKind,
}

impl Display for AnalyticsItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}:", self.time.elapsed())?;

        match &self.kind {
            AnalyticsKind::PostCreated(id) => write!(f, "{id} post created"),
            AnalyticsKind::PostLiked(id) => write!(f, "{id} post liked"),
        }
    }
}

#[derive(Default)]
pub(super) struct InMemoryAnalyticRepository(RwLock<Vec<AnalyticsItem>>);

impl InMemoryAnalyticRepository {
    async fn push_analytics(&self, analytics_record: AnalyticsKind) {
        let mut analytics = self.0.write().await;
        analytics.push(AnalyticsItem {
            time: Instant::now(),
            kind: analytics_record,
        });
    }
}

impl AnalyticsRepository for InMemoryAnalyticRepository {
    fn post_created<'a>(&'a self, id: usize) -> Pin<Box<dyn Future<Output = ()> + Send + 'a>> {
        Box::pin(async move {
            self.push_analytics(AnalyticsKind::PostCreated(id)).await;
        })
    }

    fn post_liked<'a>(&'a self, id: usize) -> Pin<Box<dyn Future<Output = ()> + Send + 'a>> {
        Box::pin(async move {
            self.push_analytics(AnalyticsKind::PostLiked(id)).await;
        })
    }

    fn show<'a>(&'a self) -> Pin<Box<dyn Future<Output = String> + Send + 'a>> {
        Box::pin(async move {
            let analytics = self.0.read().await;
            analytics.iter().fold(String::new(), |mut string, item| {
                let _ = writeln!(string, "{item}");
                string
            })
        })
    }
}
