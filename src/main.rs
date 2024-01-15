#![feature(slice_flatten)]

use games::lenia::Lenia;
use games::Game;
use ggez::event;
use ggez::event::EventHandler;
use ggez::graphics::{Canvas, Color, DrawMode, DrawParam, Mesh, MeshBuilder, Rect};
use ggez::{Context, ContextBuilder, GameResult};
use nalgebra::{Complex, DMatrix};

mod creatures;
mod games;
mod kernels;

struct MainState {
    fps: u32,
    screen_size: f32,
    game: Lenia,
    cells: DMatrix<Complex<f64>>,
}

impl MainState {
    pub fn new(screen_size: f32, fps: u32) -> Self {
        let game = Lenia::new();
        let initial_state = Lenia::initial_state();
        MainState {
            game,
            cells: initial_state,
            screen_size,
            fps,
        }
    }
}

impl EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        while ctx.time.check_update_time(self.fps) {
            self.cells = self.game.update(self.cells.clone());
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = Canvas::from_frame(ctx, Color::BLACK);
        let mut builder = MeshBuilder::new();

        // Draw cells
        self.cells
            .iter()
            .enumerate()
            .filter(|(_, x)| x.re > 0.)
            .for_each(|(i, x)| {
                let pos_x = i % Lenia::SIZE;
                let pos_y = i / Lenia::SIZE;
                let cell_size = self.screen_size / Lenia::SIZE as f32;
                let color = Color::new(0., 1., 0., x.re as f32); // Green
                let draw_mode = DrawMode::fill();
                let rect = Rect::new(
                    pos_x as f32 * cell_size,
                    pos_y as f32 * cell_size,
                    cell_size,
                    cell_size,
                );
                builder.rectangle(draw_mode, rect, color).unwrap();
            });

        let mesh = builder.build();
        let mesh = Mesh::from_data(ctx, mesh);
        canvas.draw(&mesh, DrawParam::default());
        canvas.finish(ctx)
    }
}

fn main() -> GameResult {
    let screen_size = 1000.;
    let fps = 20;

    // Set configuration
    let state = MainState::new(screen_size, fps);

    // Setup ggez stuff
    let cb = ContextBuilder::new("game_of_life", "Zoran")
        .window_mode(ggez::conf::WindowMode::default().dimensions(screen_size, screen_size));
    let (ctx, event_loop) = cb.build()?;
    ctx.gfx.set_window_title("Game of life");
    // Setup game state -> game loop
    event::run(ctx, event_loop, state)
}
