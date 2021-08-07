use std::{future::Future, task::Poll};

use crate::views;
use axum::{prelude::*, routing::BoxRoute};
use usecases::BeansUsecase;

pub fn routes() -> BoxRoute<Body> {
    route("/", get(index_wrapper)).boxed()
}

async fn index(out: &'static mut impl std::io::Write) {
    let store = database::BeansInMemory {};
    let view = views::BeansRenderer::new(out);
    let mut usecase = BeansUsecase::new(store, view);
    let x = usecase.list();
}

fn index_wrapper() -> impl Future<Output = String> {
    RespondLater {}
}

struct RespondLater;

impl Future for RespondLater {
    type Output = String;

    fn poll(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        Poll::Ready("Response!!!".into())
    }
}
