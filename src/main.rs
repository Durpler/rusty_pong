use ggez::{self, Context, ContextBuilder, GameResult}; 
use ggez::graphics::{self, Color, Text, Font, Scale};
use ggez::event::{self, EventHandler};
use ggez::nalgebra as na; 

use ggez::input::keyboard::{self, KeyCode};
use rand::{self, thread_rng, Rng};

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
const BALL_SPEED : f32 = 150.0;


// clamp function to hold players in screen
fn clamp(value: &mut f32, low: f32, high: f32){
    if *value < low{
        *value = low; 
    }
    else if *value > high{
        *value = high; 
    }
}


fn poll_movement_racket(pos : &mut na::Point2<f32>, keycode: KeyCode, y_dir : f32, ctx : &mut Context){
    let screen_h = graphics::drawable_size(ctx).1;
    let dt = ggez::timer::delta(ctx).as_secs_f32(); 
    if keyboard::is_key_pressed(ctx, keycode){
        pos.y += y_dir * PLAYER_SPEED * dt; 
    }
    clamp(&mut pos.y, RACKET_HEIGHT_HALF, screen_h - RACKET_HEIGHT_HALF); 
}
// set ball to middle of the screen and randomize velocity
fn reset_ball (ball_pos :&mut na::Point2<f32>, ball_velocity : &mut na::Vector2<f32>, ctx : &mut Context){
    let (screen_w, screen_h) = graphics::drawable_size(ctx);
    ball_pos.x = screen_w * 0.5;
    ball_pos.y = screen_h * 0.5;
    randomize_vector(ball_velocity, BALL_SPEED, BALL_SPEED); 
}

fn randomize_vector(vec : &mut na::Vector2<f32>, x : f32, y : f32){
    let mut rng = thread_rng(); 
    vec.x = match rng.gen_bool(0.5){
        true => x, 
        false => -y,
    };
    vec.y = match rng.gen_bool(0.5){
        true => y, 
        false => -y,
    };
}


struct MainState{
    player_1_pos: na::Point2<f32>, 
    player_2_pos: na::Point2<f32>,
    ball_pos: na::Point2<f32>,
    ball_velocity : na::Vector2<f32>, 
    player_1_score : i32, 
    player_2_score : i32,
}

impl MainState{
    // constructor / new-operator
    pub fn new(ctx: &mut Context) -> MainState{
        let (screen_w, screen_h) = graphics::drawable_size(ctx); 
        let (screen_w_half, screen_h_half) = (screen_w * 0.5, screen_h * 0.5);
        
        let mut ball_velocity = na::Vector2::new(0.0, 0.0); 
        randomize_vector(&mut ball_velocity, 50.0, 50.0); 

        MainState{           
            player_1_pos : na::Point2::new(RACKET_WIDTH_HALF, screen_h_half),
            player_2_pos : na::Point2::new(screen_w - RACKET_WIDTH_HALF, screen_h_half),
            ball_pos : na::Point2::new(screen_w_half, screen_h_half),
            ball_velocity,
            player_1_score : 0,
            player_2_score : 0, 
        }
    }
}

impl EventHandler for MainState{
    fn update(&mut self, ctx: &mut Context) -> GameResult<()>{
        // poll racket movement
        poll_movement_racket(&mut self.player_1_pos,KeyCode::W, -1.0, ctx);
        poll_movement_racket(&mut self.player_1_pos,KeyCode::S, 1.0, ctx);
        poll_movement_racket(&mut self.player_2_pos,KeyCode::Up, -1.0, ctx);
        poll_movement_racket(&mut self.player_2_pos,KeyCode::Down, 1.0, ctx);

        // update ball
        let dt = ggez::timer::delta(ctx).as_secs_f32(); 
        self.ball_pos += self.ball_velocity * dt; 

        let (screen_w, screen_h) = graphics::drawable_size(ctx);
        // ball check for score
        if self.ball_pos.x < 0.0 {
            reset_ball(&mut self.ball_pos, &mut self.ball_velocity, ctx);
            self.player_2_score += 1; 
        }
        if self.ball_pos.x > screen_w{
            reset_ball(&mut self.ball_pos, &mut self.ball_velocity, ctx); 
            self.player_1_score +=1; 
        }

        //ball bounce 
        if self.ball_pos.y < 0.0 {
            self.ball_velocity = self.ball_velocity.abs(); 
        } else if self.ball_pos.y > screen_h{
            self.ball_velocity = -self.ball_velocity.abs(); 
        }

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

        // draw scoreboard
        let mut score_text = graphics::Text::new(format!("{}  :  {}", self.player_1_score, self.player_2_score));
        score_text.set_font(Font::default(), Scale::uniform(100.0));
        let (screen_w, screen_h) = graphics::drawable_size(ctx);
        let (screen_w_half, screen_h_half) = (screen_w * 0.5, screen_h * 0.5);
        let score_pos = na::Point2::new(screen_w_half - (score_text.width(ctx)/2) as f32, screen_h_half * 0.25 - (score_text.height(ctx)/2) as f32);
        draw_params.dest = score_pos.into(); 

        graphics::draw(ctx, &score_text, draw_params)?; 
        // finalize drawing
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
