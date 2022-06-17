use super::{Context, Gpu};

pub struct RenderGraph {
}

impl RenderGraph {
    pub fn create() -> Self {

        RenderGraph {

        }
    }

    pub fn draw(&self, c: &Context, gpus: &[Gpu]) {
        let g = &gpus[0];
    }
}