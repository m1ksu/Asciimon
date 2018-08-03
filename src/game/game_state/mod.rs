pub mod state_explore;

use game::UpdateResult;
use graphics::renderer::Renderer;
use game::console::Console;

pub trait GameState {
    fn tick (&mut self, input_args: &[&str], console: &mut Console) -> Option<UpdateResult>;
    fn draw (&mut self, renderer: &mut Renderer, console: &mut Console);
}
