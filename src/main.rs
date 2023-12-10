use candle_core::{DType, Shape};
use candle_core::{Device, Tensor};
use ggez::event;
use ggez::event::EventHandler;
use ggez::graphics::{Canvas, Color, DrawMode, DrawParam, Mesh, MeshBuilder, Rect};
use ggez::{Context, ContextBuilder, GameResult};
use itertools::Itertools;
use rand::Rng;
use utils::ones;

mod utils;

/// Config for the start of the game
#[derive(Debug, Clone)]
pub struct Config {
    pub grid_width: usize,
    pub grid_height: usize,
    pub cell_size: f32,
    pub t: f32,
    pub r: usize,
    pub fps: u32,
}

struct MainState {
    config: Config,
    cells: Vec<f32>,
    kernel: Tensor,
    ones: Tensor,
    zeros: Tensor,
    shape: Shape,
    conv_shape: Shape,
}

impl MainState {
    pub fn new(config: Config, initial_state: Vec<f32>) -> Self {
        let shape = Shape::from((config.grid_width, config.grid_height));
        let conv_shape = Shape::from((1, 1, config.grid_width, config.grid_height));

        let mut filter = ones(config.r * 2 + 1, config.r * 2 + 1);
        filter[config.r][config.r] = 0.;
        let filter = filter.into_iter().flatten().collect_vec();
        let sum = filter.iter().sum::<f32>();
        let filter_norm = filter.iter().map(|x| x / sum).collect();
        let filter_shape = Shape::from((1, 1, config.r * 2 + 1, config.r * 2 + 1));
        let kernel = Tensor::from_vec(filter_norm, filter_shape, &Device::Cpu).unwrap();

        let zeros = Tensor::zeros(shape.clone(), DType::F32, &Device::Cpu).unwrap();
        let ones = Tensor::ones(shape.clone(), DType::F32, &Device::Cpu).unwrap();

        MainState {
            config,
            cells: initial_state,
            kernel,
            ones,
            zeros,
            shape,
            conv_shape,
        }
    }

    fn growth((x, y): (&f32, f32), t: f32) -> f32 {
        x + t * (f32::from((y >= 0.12) & (y <= 0.15)) - f32::from((y <= 0.12) | (y > 0.15)))
    }
}

impl EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        while ctx.time.check_update_time(self.config.fps) {
            let image = Tensor::from_vec(self.cells.clone(), self.conv_shape.clone(), &Device::Cpu)
                .unwrap();
            let res = image.conv2d(&self.kernel, self.config.r, 1, 1, 1).unwrap();

            let res_flatten = res.flatten_to(3).unwrap().to_vec1::<f32>().unwrap();
            let cells_grown = self
                .cells
                .iter()
                .zip(res_flatten)
                .map(|(x, y)| Self::growth((x, y), 1. / self.config.t))
                .collect();

            let grown_tensor =
                Tensor::from_vec(cells_grown, self.shape.clone(), &Device::Cpu).unwrap();
            let res = grown_tensor.clamp(&self.zeros, &self.ones).unwrap();
            let res_flatten = res.flatten_all().unwrap().to_vec1().unwrap();
            self.cells = res_flatten;
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
            .filter(|(_, x)| **x > 0.)
            .for_each(|(i, x)| {
                let pos_x = i % self.config.grid_width;
                let pos_y = i / self.config.grid_height;
                let color = Color::new(0., 1., 0., *x); // Green
                let draw_mode = DrawMode::fill();
                let rect = Rect::new(
                    pos_x as f32 * self.config.cell_size,
                    pos_y as f32 * self.config.cell_size,
                    self.config.cell_size,
                    self.config.cell_size,
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
    let screen_size = (1000., 1000.);
    let grid_size = (100, 100);
    let cell_size = 10.;
    let t = 10.;
    let r = 10;
    let fps = 20;

    let mut rng = rand::thread_rng();
    let initial_state = (0..grid_size.0 * grid_size.1)
        .into_iter()
        .map(|_| rng.gen())
        .collect::<Vec<f32>>();

    // Set configuration
    let config: Config = Config {
        grid_width: grid_size.0,
        grid_height: grid_size.1,
        cell_size,
        t,
        r,
        fps,
    };
    let state = MainState::new(config, initial_state);

    // Setup ggez stuff
    let cb = ContextBuilder::new("game_of_life", "Zoran")
        .window_mode(ggez::conf::WindowMode::default().dimensions(screen_size.0, screen_size.1));
    let (ctx, event_loop) = cb.build()?;
    ctx.gfx.set_window_title("Game of life");
    // Setup game state -> game loop
    event::run(ctx, event_loop, state)
}
