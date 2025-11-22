use std::sync::Arc;

use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::infrastructure::{bevy::bevy_renderer::BevyRenderer, renderer::Renderer};

pub mod domain;
pub mod infrastructure;

const DEFAULT_LOG_LEVEL: &str = "ERROR";

fn setup_tracing_subscriber() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| DEFAULT_LOG_LEVEL.into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();
}

fn main() {
    setup_tracing_subscriber();

    let renderer = Arc::new(BevyRenderer::new());
    renderer.render();
}
