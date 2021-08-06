use axum::{prelude::*, routing::BoxRoute};

pub fn routes() -> BoxRoute<Body> {
    route("/", get(index)).boxed()
}

async fn index() -> String {
    let store = database::BeansInMemory {};
    let usecase = usecases::BeansUsecase::new(store);
    let beans = usecase.list().await;
    beans
        .into_iter()
        .map(|b| b.name)
        .reduce(|acc, b| format!("{}, {}", acc, b))
        .unwrap_or("<Nothing>".into())
}
