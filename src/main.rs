use candle_core::{DType, Shape};
use candle_core::{Device, Tensor};
use ggez::event;
use ggez::event::EventHandler;
use ggez::graphics::{Canvas, Color, DrawMode, DrawParam, Mesh, MeshBuilder, Rect};
use ggez::{Context, ContextBuilder, GameResult};
use itertools::Itertools;
use rand::Rng;
use utils::{norm, range};

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
    pub fn new(config: Config) -> Self {
        let mut rng = rand::thread_rng();
        let initial_state = (0..config.grid_width * config.grid_height)
            .into_iter()
            .map(|_| rng.gen())
            .collect::<Vec<f32>>();

        let shape = Shape::from((config.grid_width, config.grid_height));
        let conv_shape = Shape::from((1, 1, config.grid_width, config.grid_height));

        let filter = range(-9., 1., 20).iter().map(|x| vec![*x]).collect_vec();
        let norm = norm(filter);
        let filter = norm
            .into_iter()
            .flatten()
            .map(|x| x / config.r as f32)
            .collect_vec();
        let filter = Self::k(filter);
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

    // fn growth(y: f32) -> f32 {
    //     f32::from((y >= 0.12) & (y <= 0.15)) - f32::from((y <= 0.12) | (y > 0.15))
    // }

    fn growth(y: f32) -> f32 {
        let m = 0.135;
        let s = 0.015;
        Self::bell(y, m, s) * 2. - 1.
    }

    fn bell(x: f32, m: f32, s: f32) -> f32 {
        f32::exp(-((x - m) / s).powf(2.) / 2.)
    }

    fn k(d: Vec<f32>) -> Vec<f32> {
        d.iter()
            .map(|x| f32::from(*x < 1.) * Self::bell(*x, 0.5, 0.15))
            .collect_vec()
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
                .map(|(x, y)| x + 1. / self.config.t * Self::growth(y))
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

    // Set configuration
    let config: Config = Config {
        grid_width: grid_size.0,
        grid_height: grid_size.1,
        cell_size,
        t,
        r,
        fps,
    };
    let state = MainState::new(config);

    // Setup ggez stuff
    let cb = ContextBuilder::new("game_of_life", "Zoran")
        .window_mode(ggez::conf::WindowMode::default().dimensions(screen_size.0, screen_size.1));
    let (ctx, event_loop) = cb.build()?;
    ctx.gfx.set_window_title("Game of life");
    // Setup game state -> game loop
    event::run(ctx, event_loop, state)
}
