#![feature(slice_flatten)]

use ggez::event;
use ggez::event::EventHandler;
use ggez::graphics::{Canvas, Color, DrawMode, DrawParam, Mesh, MeshBuilder, Rect};
use ggez::{Context, ContextBuilder, GameResult};
use road_to_lenia::lenias::StandardLenia;
use road_to_lenia::{self, Lenia, Simulator};

struct MainState<L: Lenia> {
    fps: u32,
    screen_size: f32,
    shape: (usize, usize),
    game: Simulator<L>,
}

impl<L: Lenia> MainState<L> {
    pub fn new(screen_size: f32, fps: u32, shape: (usize, usize)) -> Self {
        let (w, h) = shape;
        let game = Simulator::<L>::new(&[w, h]);
        MainState {
            game,
            shape,
            screen_size,
            fps,
        }
    }
}

impl<L: Lenia> EventHandler for MainState<L> {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        while ctx.time.check_update_time(self.fps) {
            self.game.iterate();
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = Canvas::from_frame(ctx, Color::BLACK);
        let mut builder = MeshBuilder::new();

        let cells = self.game.get_channel_as_ref(0);
        let (w, h) = self.shape;

        // Draw cells
        cells
            .iter()
            .enumerate()
            .filter(|(_, &x)| x > 0.)
            .for_each(|(i, &x)| {
                let pos_x = i % w;
                let pos_y = i / h;
                let cell_size = self.screen_size / w as f32;
                let color = Color::new(0., 1., 0., x as f32); // Green
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
    let shape = (64, 64);

    // Set configuration
    let state = MainState::<StandardLenia>::new(screen_size, fps, shape);

    // Setup ggez stuff
    let cb = ContextBuilder::new("game_of_life", "Zoran")
        .window_mode(ggez::conf::WindowMode::default().dimensions(screen_size, screen_size));
    let (ctx, event_loop) = cb.build()?;
    ctx.gfx.set_window_title("Game of life");
    // Setup game state -> game loop
    event::run(ctx, event_loop, state)
}
