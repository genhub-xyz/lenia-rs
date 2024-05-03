#![feature(slice_flatten)]

use ggez::event;
use ggez::event::EventHandler;
use ggez::graphics::{Canvas, Color, DrawMode, DrawParam, Mesh, MeshBuilder, Rect};
use ggez::{Context, ContextBuilder, GameResult};
use ndarray::Array2;
use rand::{thread_rng, Rng};
use road_to_lenia::lenias::StandardLenia;
use road_to_lenia::{self, load_from_png, Lenia, Simulator};

#[macro_use]
extern crate ndarray;

struct MainState<L: Lenia> {
    fps: u32,
    screen_size: f32,
    shape: usize,
    game: Simulator<L>,
}

impl<L: Lenia> MainState<L> {
    pub fn new(screen_size: f32, fps: u32, shape: usize, initial_state: Array2<f64>) -> Self {
        let mut game = Simulator::<L>::new(&[shape, shape]);
        game.fill_channel(&initial_state.into_dyn(), 0);
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
        let cell_size = self.screen_size / self.shape as f32;

        // Draw cells
        cells
            .iter()
            .enumerate()
            .filter(|(_, &x)| x > 0.)
            .for_each(|(i, &x)| {
                let pos_x = (i % self.shape) as f32;
                let pos_y = (i / self.shape) as f32;
                let color = Color::new(0., 1., 0., x as f32); // Green
                let draw_mode = DrawMode::fill();
                let rect = Rect::new(pos_x * cell_size, pos_y * cell_size, cell_size, cell_size);
                builder.rectangle(draw_mode, rect, color).unwrap();
            });

        let mesh = builder.build();
        let mesh = Mesh::from_data(ctx, mesh);
        canvas.draw(&mesh, DrawParam::default());
        canvas.finish(ctx)
    }
}

fn main() -> GameResult {
    let resolution = 1800.;
    let fps = 60;
    let shape = 300;

    // let rng = &mut thread_rng();
    let mut initial_state = Array2::<f64>::zeros([shape, shape]);
    // initial_state.map_mut(|x| *x = rng.gen::<f64>());

    let glider = load_from_png("./images/glider.png");
    initial_state.slice_mut(s![..100, ..100]).assign(&glider);
    initial_state.slice_mut(s![..100, 100..200]).assign(&glider);
    initial_state.slice_mut(s![..100, 200..300]).assign(&glider);
    initial_state.slice_mut(s![100..200, ..100]).assign(&glider);
    initial_state
        .slice_mut(s![100..200, 100..200])
        .assign(&glider);
    initial_state
        .slice_mut(s![100..200, 200..300])
        .assign(&glider);
    initial_state.slice_mut(s![200..300, ..100]).assign(&glider);
    initial_state
        .slice_mut(s![200..300, 100..200])
        .assign(&glider);
    initial_state
        .slice_mut(s![200..300, 200..300])
        .assign(&glider);

    // Set configuration
    let state = MainState::<StandardLenia>::new(resolution, fps, shape, initial_state);

    // Setup ggez stuff
    let cb = ContextBuilder::new("game_of_life", "Zoran")
        .window_mode(ggez::conf::WindowMode::default().dimensions(resolution, resolution));
    let (ctx, event_loop) = cb.build()?;
    ctx.gfx.set_window_title("Game of life");
    // Setup game state -> game loop
    event::run(ctx, event_loop, state)
}
