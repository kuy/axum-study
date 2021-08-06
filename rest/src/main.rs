use axum::prelude::*;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    // build our application with a route
    let app = route("/beans", get(handler));

    // run it
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn handler() -> String {
    let store = database::BeansInMemory {};
    let usecase = usecases::BeansUsecase::new(store);
    let beans = usecase.list().await;
    beans
        .into_iter()
        .map(|b| b.name)
        .reduce(|acc, b| format!("{}, {}", acc, b))
        .unwrap_or("<Nothing>".into())
}
