use ggez;
use ggez::{ Context, GameResult };
use ggez::graphics;
use ggez::event;

struct MainState {

}

impl MainState {
    pub fn new() -> Self {
        MainState {}
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, graphics::BLACK);
        let rect = graphics::Rect::new(10.0, 10.0, 300.0, 150.0);
        let rect_mesh = graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), rect, graphics::WHITE)?;
        graphics::draw(ctx, &rect_mesh, graphics::DrawParam::default())?;
        graphics::present(ctx)?;
        Ok(())
    }
}

fn main() -> GameResult {
    let cb = ggez::ContextBuilder::new("ping_pong", "Ismael");
    let (ctx, event_loop) = &mut cb.build()?;
    graphics::set_window_title(&ctx, "Ping Pong");

    let mut state = MainState::new();
    event::run(ctx, event_loop, &mut state)
}
 