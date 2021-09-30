use ggez::{self, Context, ContextBuilder, GameResult}; 
use ggez::graphics::{self, Color};
use ggez::event::{self, EventHandler};
use ggez::nalgebra as na; 

use ggez::input::keyboard::{self, KeyCode};


///
/// CONSTANT VALUES 
/// 

const PLAYER_SPEED: f32 = 350.0; 

const RACKET_HEIGHT: f32 = 100.0; 
const RACKET_WIDTH: f32 = 20.0; 
const RACKET_WIDTH_HALF:f32 = RACKET_WIDTH * 0.5;
const RACKET_HEIGHT_HALF:f32 = RACKET_HEIGHT * 0.5; 

const BALL_SIZE:f32 = 30.0;
const BALL_SIZE_HALF:f32 = BALL_SIZE * 0.5; 


// clamp function to hold players in screen
fn clamp(value: &mut f32, low: f32, high: f32){
    if *value < low{
        *value = low; 
    }
    else if *value > high{
        *value = high; 
    }
}


struct MainState{
    player_1_pos: na::Point2<f32>, 
    player_2_pos: na::Point2<f32>,
    ball_pos: na::Point2<f32>, 
}

impl MainState{
    // constructor / new-operator
    pub fn new(ctx: &mut Context) -> MainState{
        let (screen_w, screen_h) = graphics::drawable_size(ctx); 
        let (screen_w_half, screen_h_half) = (screen_w * 0.5, screen_h * 0.5);
        MainState{           
            player_1_pos : na::Point2::new(RACKET_WIDTH_HALF, screen_h_half),
            player_2_pos : na::Point2::new(screen_w - RACKET_WIDTH_HALF, screen_h_half),
            ball_pos : na::Point2::new(screen_w_half, screen_h_half),
        }
    }
}

impl EventHandler for MainState{
    fn update(&mut self, ctx: &mut Context) -> GameResult<()>{
        // get delta time
        let dt = ggez::timer::delta(ctx).as_secs_f32(); 
        let screen_h = graphics::drawable_size(ctx).1;
        if keyboard::is_key_pressed(ctx, KeyCode::W){
            self.player_1_pos.y -= (PLAYER_SPEED * dt);
        }
        if keyboard::is_key_pressed(ctx, KeyCode::S){
            self.player_1_pos.y += (PLAYER_SPEED * dt); 
        }
        // note if player 2 -> AI 

        if keyboard::is_key_pressed(ctx, KeyCode::Up){
            self.player_2_pos.y -= (PLAYER_SPEED * dt);
        }
        if keyboard::is_key_pressed(ctx, KeyCode::Down){
            self.player_2_pos.y += (PLAYER_SPEED * dt); 
        }

        // clamp both players
        clamp(&mut self.player_1_pos.y, RACKET_HEIGHT_HALF, screen_h - RACKET_HEIGHT_HALF);
        clamp(&mut self.player_2_pos.y, RACKET_HEIGHT_HALF, screen_h - RACKET_HEIGHT_HALF);

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()>{
        // define color white, since it can't be found? What the heck?!
        let white : Color = Color::new(1.0,1.0,1.0,1.0); 
        let black : Color = Color::new(0.0,0.0,0.0,0.0); 
        graphics::clear(ctx, black); 

        //create player rect
        let racket_rect = graphics::Rect::new(-RACKET_WIDTH_HALF, -RACKET_HEIGHT_HALF, RACKET_WIDTH, RACKET_HEIGHT); 
        let racket_mesh = graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), racket_rect, white)?;
       
        //create ball rect
        let ball_rect = graphics::Rect::new(-BALL_SIZE_HALF,-BALL_SIZE_HALF, BALL_SIZE_HALF, BALL_SIZE_HALF);
        let ball_mesh = graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), ball_rect, white)?; 


        let mut draw_params = graphics::DrawParam::default(); 

        draw_params.dest = self.player_1_pos.into(); 
        graphics::draw(ctx, &racket_mesh, draw_params)?; 

        // Draw Player 2
        draw_params.dest = self.player_2_pos.into(); 
        graphics::draw(ctx, &racket_mesh, draw_params)?; 

        // Draw Ball
        draw_params.dest = self.ball_pos.into(); 
        graphics::draw(ctx,&ball_mesh, draw_params)?; 
        

        graphics::present(ctx)?; 
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
