mod game_state;
mod player;

pub mod world;
pub mod console;

use graphics::renderer::Renderer;
use util::vector::Vector2D;

use self::game_state::{state_explore::StateExplore, GameState};
use self::console::Console;

use std::io::{stdin, stdout, Write};

pub const GAME_AREA_SIZE: Vector2D<i32> = Vector2D { x: 81, y: 45 };
pub const GAME_AREA_CENTRE: Vector2D<i32> = Vector2D {
    x: GAME_AREA_SIZE.x / 2,
    y: GAME_AREA_SIZE.y / 2,
};

mod colours {
    use graphics::colour::Colour;
    define_colour!(LOGO, 50, 255, 200);
    define_colour!(TEXT, 255, 255, 255);
    define_colour!(GAME_BACKGROUND, 0, 0, 0);
}

pub const LOGO: &str = r"
                   _ _
    /\            (_|_)
   /  \   ___  ___ _ _ _ __ ___   ___  _ __
  / /\ \ / __|/ __| | | '_ ` _ \ / _ \| '_ \
 / ____ \ __ \ (__| | | | | | | | (_) | | | |
/_/    \_\___/\___|_|_|_| |_| |_|\___/|_| |_|
";

#[allow(dead_code)]
pub enum UpdateResult {
    Redraw,
    StatePush(Box<GameState>),
    StatePop,
    Exit,
}

pub struct Game {
    renderer: Renderer,
    state_stack: Vec<Box<GameState>>,
    is_running: bool,
    needs_redraw: bool,
    console: Console
}

impl Game {
    pub fn run_game() {
        let mut game = Game {
            renderer: Renderer::new(Vector2D::new(113, 52)),
            state_stack: Vec::new(),
            is_running: true,
            needs_redraw: true,
            console: Console::new()
        };
        //ui::init(&mut game.renderer);

        //Yay for magic numbers
        game.renderer
            .add_render_section("game", Vector2D::new(0, 7), GAME_AREA_SIZE);
        game.renderer
            .add_render_section("logo", Vector2D::new(0, 0), Vector2D::new(50, 6));
        game.renderer.add_render_section(
            "input",  Vector2D::new(50, 0), Vector2D::new(GAME_AREA_SIZE.x - 50, 6),
        );
        game.renderer.add_render_section(
            "console", Vector2D::new(GAME_AREA_SIZE.x + 1, 0), Vector2D::new(32, 52)
        );

        game.renderer.create_border("game");
        game.renderer.create_border("logo");
        game.renderer.create_border("input");
        game.renderer.create_border("console");
        Game::draw_logo(&game.renderer);

        game.renderer
            .clear_section("game", &colours::GAME_BACKGROUND);

        game.run();
    }

    fn run(&mut self) {
        self.state_stack.push(Box::new(StateExplore::new()));
        //Main loop!
        while self.is_running {
            match self.tick() {
                Some(UpdateResult::Redraw) => self.needs_redraw = true,
                Some(UpdateResult::StatePush(state)) => {
                    self.state_stack.push(state);
                    self.needs_redraw = true;
                }
                Some(UpdateResult::StatePop) => {
                    self.state_stack.pop();
                    if self.state_stack.is_empty() {
                        self.is_running = false;
                    }
                    self.needs_redraw = true;
                }
                Some(UpdateResult::Exit) => self.is_running = false,
                None => {}
            }
        }
    }

    fn tick(&mut self) -> Option<UpdateResult> {
        if let Some(current_state) = self.state_stack.last_mut() {
            //Drawing happens first because the input is blocking, so nothing would be drawn until input has been
            //got on the first loop
            if self.needs_redraw {
                self.renderer
                    .clear_section("game", &colours::GAME_BACKGROUND);

                current_state.draw(&mut self.renderer, &mut self.console);
                self.needs_redraw = false;

                //Ensure what has been drawn is flushed to stdout before getting input/updating
                stdout()
                    .flush()
                    .expect("Could not buffer the terminal output!");
            }
            self.console.draw(&mut self.renderer);
            self.renderer.create_border("input");

            if let Some(input) = Game::get_user_input(&self.renderer) {
                let input_args: Vec<&str> = input.trim().split(' ').collect();

                match &input_args[..] {
                    ["exit"] | ["quit"] => Some(UpdateResult::Exit),
                    input => current_state.tick(input, &mut self.console),
                }
            } else {
                return Some(UpdateResult::Exit);
            }
        } else {
            return Some(UpdateResult::Exit);
        }
    }

    fn get_user_input(renderer: &Renderer) -> Option<String> {
        Renderer::set_text_colour(&colours::TEXT);
        renderer.clear_section("input", renderer.default_clear_colour());
        renderer.draw_string("input", "Enter Input Here:", Vector2D::new(0, 0));
        renderer.draw_string("input", "> ", Vector2D::new(0, 2));

        stdout()
            .flush()
            .expect("Could not buffer the terminal output!");

        renderer.set_cursor_render_section("input", Vector2D::new(2, 2));
        let mut input = String::new();
        match stdin()
            .read_line(&mut input)
            .expect("Failed to get user input")
        {
            0 => None,
            _ => Some(input),
        }
    }

    fn draw_logo(renderer: &Renderer) {
        Renderer::set_text_colour(&colours::LOGO);
        for (line_num, line) in LOGO.lines().enumerate() {
            renderer.draw_string("logo", line, Vector2D::new(1, line_num as i32 - 1));
        }
        Renderer::set_text_colour(&colours::TEXT);
    }
}
