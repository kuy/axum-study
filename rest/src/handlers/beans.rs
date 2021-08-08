use crate::views;
use axum::{prelude::*, routing::BoxRoute};
use crossbeam_channel::{unbounded as channel, Receiver, Sender};
use std::{future::Future, task::Poll, thread};
use tokio::runtime::Runtime;
use usecases::BeansUsecase;

pub fn routes() -> BoxRoute<Body> {
    route("/", get(index_wrapper)).boxed()
}

async fn index(sender: Sender<String>) {
    let gateway = gateway::BeansGateway::new();
    let view = views::BeansRenderer::new(sender);
    let usecase = BeansUsecase::new(gateway, view);
    usecase.list().await;
}

fn index_wrapper() -> impl Future<Output = String> {
    let (sender, receiver) = channel::<String>();

    thread::spawn(move || {
        let rt = Runtime::new().unwrap();
        rt.block_on(async {
            index(sender).await;
        });
    });

    RespondLater::new(receiver)
}

struct RespondLater {
    receiver: Receiver<String>,
    pipe: (Sender<String>, Receiver<String>),
}

impl RespondLater {
    pub fn new(receiver: Receiver<String>) -> Self {
        Self {
            receiver,
            pipe: channel(),
        }
    }
}

impl Future for RespondLater {
    type Output = String;

    fn poll(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        if let Ok(body) = self.pipe.1.try_recv() {
            Poll::Ready(body)
        } else {
            let waker = cx.waker().clone();
            let sender = self.pipe.0.clone();
            let receiver = self.receiver.clone();
            thread::spawn(move || {
                if let Ok(body) = receiver.recv() {
                    sender.send(body).expect("should be sent");
                    waker.wake();
                }
            });
            Poll::Pending
        }
    }
}
