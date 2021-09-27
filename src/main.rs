use ggez::{Context, ContextBuilder, GameResult}; 
use ggez::graphics::{self, Color};
use ggez::event::{self, EventHandler};


struct MainState{

}

impl MainState{
    // constructor / new-operator
    pub fn new(_ctx: &mut Context) -> MainState{
        MainState{}
    }
}

impl EventHandler for MainState{
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()>{
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()>{
        // define color white, since it can't be found? 
        let _white : Color = Color::new(1.0,1.0,1.0,1.0); 
        let black : Color = Color::new(0.0,0.0,0.0,0.0); 
        graphics::clear(ctx, black); 
        graphics::present(ctx)
        .expect("Could not present the rendered frame"); 
        Ok(())
    }
}

fn main() -> GameResult<()> {
    // create a context
    let (mut ctx,mut event_loop) = ContextBuilder::new("MyPong", "Durpler")
    .build()
    .expect("Could not initialize the game : geez"); 

    graphics::set_window_title(&ctx, "My Pong!");

    let mut game = MainState::new(&mut ctx); 
    event::run(&mut ctx,&mut event_loop, &mut game)
    .expect("Could not run main loop"); 
    Ok(())
}
