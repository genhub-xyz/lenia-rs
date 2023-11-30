mod cell;
mod grid;

use ggez::event;
use ggez::event::EventHandler;
use ggez::graphics;
use ggez::{Context, ContextBuilder, GameResult};
use grid::{Grid, Point};
use rand::Rng;

/// Config for the start of the game
#[derive(Debug, Clone)]
pub struct Config {
    pub grid_width: usize,
    pub grid_height: usize,
    pub cell_size: f32,
    pub screen_size: (f32, f32),
    pub fps: u32,
    pub initial_state: Vec<Point>,
}

struct MainState {
    grid: Grid,
    config: Config,
}

impl MainState {
    pub fn new(config: Config) -> Self {
        // Initialize the grid based on configuration
        let mut grid = Grid::new(config.grid_width, config.grid_height);
        // Convert the starting states into a vector of points
        grid.set_state(&config.initial_state);
        MainState { grid, config }
    }
}

impl EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        while ctx.time.check_update_time(self.config.fps) {
            self.grid.update();
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, graphics::Color::BLACK);
        let mut builder = graphics::MeshBuilder::new();

        // Draw cells
        for (idx, cell) in self.grid.cells.iter().enumerate() {
            if cell.is_alive() {
                let pos = self.grid.index_to_coords(idx);
                let color = graphics::Color::GREEN; // Green
                builder.rectangle(
                    graphics::DrawMode::fill(),
                    graphics::Rect::new(
                        pos.x as f32 * self.config.cell_size,
                        pos.y as f32 * self.config.cell_size,
                        self.config.cell_size,
                        self.config.cell_size,
                    ),
                    color,
                )?;
            }
        }
        let mesh = builder.build();
        let mesh = graphics::Mesh::from_data(ctx, mesh);
        canvas.draw(&mesh, graphics::DrawParam::default());
        canvas.finish(ctx)
    }
}

fn main() -> GameResult {
    let screen_size = (2000., 1500.);
    let grid_size = (200, 150);
    let cell_size = 10.;
    let fps = 20;

    let mut start_cells_coords = Vec::new();
    let mut rng = rand::thread_rng();
    for i in 0..grid_size.0 {
        for j in 0..grid_size.1 {
            if rng.gen::<bool>() {
                start_cells_coords.push((i, j));
            }
        }
    }

    // Set configuration
    let config: Config = Config {
        grid_width: grid_size.0,
        grid_height: grid_size.1,
        cell_size,
        screen_size,
        fps,
        initial_state: start_cells_coords
            .iter()
            .map(|&p| p.into())
            .collect::<Vec<Point>>(),
    };

    // Setup ggez stuff
    let cb = ContextBuilder::new("game_of_life", "Zoran")
        .window_mode(ggez::conf::WindowMode::default().dimensions(screen_size.0, screen_size.1));
    let (ctx, event_loop) = cb.build()?;
    ctx.gfx.set_window_title("Game of life");
    // Setup game state -> game loop
    let state = MainState::new(config);
    event::run(ctx, event_loop, state)
}
