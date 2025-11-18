use std::sync::Arc;

use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::infrastructure::{bevy::bevy_renderer::BevyRenderer, renderer::Renderer};

pub mod domain;
pub mod infrastructure;

fn setup_tracing_subscriber() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();
}

fn main() {
    setup_tracing_subscriber();
    let renderer = Arc::new(BevyRenderer::default());
    renderer.render();
}
