use axum::{prelude::*, routing::BoxRoute};

pub fn routes() -> BoxRoute<Body> {
    route("/", get(index)).boxed()
}

async fn index() -> String {
    let store = database::BeansInMemory {};
    let usecase = usecases::BeansUsecase::new(store);
    usecase.list().await
}
