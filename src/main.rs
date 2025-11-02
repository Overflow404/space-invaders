use std::sync::Arc;

use crate::infrastructure::{bevy_renderer::BevyRenderer, renderer::Renderer};

pub mod domain;
pub mod infrastructure;

fn main() {
    render_game_with(Arc::new(BevyRenderer::new()));
}

fn render_game_with(renderer: Arc<BevyRenderer>) {
    renderer.start_game_loop();
}
