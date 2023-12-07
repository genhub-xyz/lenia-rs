use candle_core::{DType, Shape};
use candle_core::{Device, Tensor};
use ggez::event;
use ggez::event::EventHandler;
use ggez::graphics::{Canvas, Color, DrawMode, DrawParam, Mesh, MeshBuilder, Rect};
use ggez::{Context, ContextBuilder, GameResult};
use rand::Rng;

/// Config for the start of the game
#[derive(Debug, Clone)]
pub struct Config {
    pub grid_width: usize,
    pub grid_height: usize,
    pub cell_size: f32,
    pub num_states: f32,
    pub fps: u32,
}

struct MainState {
    config: Config,
    cells: Vec<f32>,
}

impl MainState {
    pub fn new(config: Config, initial_state: Vec<f32>) -> Self {
        MainState {
            config,
            cells: initial_state,
        }
    }

    fn growth((x, y): (&f32, f32)) -> f32 {
        x + f32::from((y >= 20.) & (y <= 24.)) - f32::from((y <= 18.) | (y > 32.))
    }

    fn map(x: f32, a: f32, b: f32, c: f32, d: f32) -> f32 {
        (x - a) / (b - a) * (d - c) + c
    }
}

impl EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        while ctx.time.check_update_time(self.config.fps) {
            let shape = Shape::from((self.config.grid_width, self.config.grid_height));
            let conv_shape = Shape::from((1, 1, self.config.grid_width, self.config.grid_height));
            let image =
                Tensor::from_vec(self.cells.clone(), conv_shape.clone(), &Device::Cpu).unwrap();

            let filter = vec![1f32, 1., 1., 1., 0., 1., 1., 1., 1.];
            let filter_shape = Shape::from((1, 1, 3, 3));
            let kernel = Tensor::from_vec(filter, filter_shape, &Device::Cpu).unwrap();
            let res = image.conv2d(&kernel, 1, 1, 1, 1).unwrap();

            let res_flatten = res.flatten_to(3).unwrap().to_vec1::<f32>().unwrap();
            let cells_grown = self
                .cells
                .iter()
                .zip(res_flatten)
                .map(Self::growth)
                .collect();

            let zeros = Tensor::zeros(shape.clone(), DType::F32, &Device::Cpu).unwrap();
            let max_state =
                vec![self.config.num_states; self.config.grid_width * self.config.grid_height];
            let max = Tensor::from_vec(max_state, shape.clone(), &Device::Cpu).unwrap();
            let grown_tensor = Tensor::from_vec(cells_grown, shape, &Device::Cpu).unwrap();
            let res = grown_tensor.clamp(&zeros, &max).unwrap();
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
                let max_value = self.config.num_states;
                let alpha = Self::map(*x, 0., max_value, 0., 1.);
                let color = Color::new(0., 1., 0., alpha); // Green
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
    let num_states = 12.;
    let fps = 20;

    let mut rng = rand::thread_rng();
    let initial_state = (0..grid_size.0 * grid_size.1)
        .into_iter()
        .map(|_| rng.gen_range(0..12u8).into())
        .collect::<Vec<f32>>();

    // Set configuration
    let config: Config = Config {
        grid_width: grid_size.0,
        grid_height: grid_size.1,
        cell_size,
        num_states,
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
