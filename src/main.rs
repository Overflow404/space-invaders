use std::sync::Arc;

use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::infrastructure::{bevy_renderer::BevyRenderer, renderer::Renderer};

pub mod domain;
pub mod infrastructure;

fn setup_tracing_subscriber() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| "info".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();
}

fn main() {
    setup_tracing_subscriber();
    render_game_with(Arc::new(BevyRenderer::new()));
}

fn render_game_with(renderer: Arc<BevyRenderer>) {
    renderer.start_game_loop();
}
