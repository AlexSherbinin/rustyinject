use std::{ops::Deref, sync::Arc};

use analytics::{AnalyticsRepository, InMemoryAnalyticRepository};
use app::App;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Redirect,
    routing::{get, post},
    Json, Router,
};
use posts::{InMemoryPostsRepository, PostsRepository};
use rustyinject::{
    injector::{
        containers::{RefConstructorFactoryContainer, SingletonContainer},
        Injector,
    },
    DependencyContainer,
};
use serde::Deserialize;
use tokio::net::TcpListener;

mod analytics;
mod app;
mod posts;

type Container<'a> = DependencyContainer<
    (),
    (
        RefConstructorFactoryContainer<App<'a>>,
        (
            SingletonContainer<Box<dyn AnalyticsRepository>>,
            (SingletonContainer<Box<dyn PostsRepository>>, ()),
        ),
    ),
>;

#[tokio::main]
async fn main() {
    let posts_repository: Box<dyn PostsRepository> = Box::<InMemoryPostsRepository>::default();
    let analytics_repository: Box<dyn AnalyticsRepository> =
        Box::<InMemoryAnalyticRepository>::default();
    let container: Container = DependencyContainer::default()
        .with_singleton(posts_repository)
        .with_singleton(analytics_repository)
        .with_ref_constructor_factory::<App>();

    let routes = Router::new()
        .route("/posts", post(create_post))
        .route("/posts/:id", get(show_post))
        .route("/posts/:id/like", post(like_post))
        .route("/analytics", get(show_analytics))
        .with_state(Arc::new(container));

    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
    axum::serve(listener, routes).await.unwrap();
}

#[derive(Deserialize)]
struct CreatePost {
    title: String,
    text: String,
}

async fn create_post(
    State(container): State<Arc<Container<'_>>>,
    Json(create_post): Json<CreatePost>,
) -> Redirect {
    let app: App = container.deref().inject();
    let post_id = app.create_post(create_post.title, create_post.text).await;

    Redirect::to(&format!("/posts/{post_id}"))
}

async fn show_post(
    State(container): State<Arc<Container<'_>>>,
    Path(id): Path<usize>,
) -> (StatusCode, String) {
    let app: App = container.deref().inject();
    let post = app.show_post(id).await;

    if let Some(post) = post {
        (StatusCode::OK, post)
    } else {
        (StatusCode::NOT_FOUND, String::new())
    }
}

async fn like_post(State(container): State<Arc<Container<'_>>>, Path(id): Path<usize>) -> Redirect {
    let app: App = container.deref().inject();
    app.like_post(id).await;

    Redirect::to(&format!("/posts/{id}"))
}

async fn show_analytics(State(container): State<Arc<Container<'_>>>) -> String {
    let app: App = container.deref().inject();
    app.show_analytics().await
}
