use macroquad::prelude::*;

#[warn(deprecated)]
const PLAYER_SPEED: f32 = 700f32;
const BALL_SPEED: f32 = 400f32;

const PLAYER_SIZE: Vec2 = const_vec2!([150f32, 40f32]);
const BLOCK_SIZE: Vec2 = const_vec2!([100f32, 30f32]);
const BALL_SIZE: Vec2 = const_vec2!([30f32, 30f32]);

struct Player{
    rect: Rect,
}

impl Player{
    pub fn new() -> Self{
        Self{
            rect: Rect::new(
                screen_width() * 0.5f32 - PLAYER_SIZE.x * 0.5f32,
                screen_height() - 100f32,
                PLAYER_SIZE.x,
                PLAYER_SIZE.y
            )
        }
    }

    pub fn update(&mut self, dt: f32){
        let mut x_move = 0f32;
        if is_key_down(KeyCode::A){
            x_move -= 1f32;
        }
        if is_key_down(KeyCode::D){
            x_move += 1f32;
        }
        self.rect.x += x_move * dt * PLAYER_SPEED;

        if self.rect.x < 0f32{
            self.rect.x = 0f32;
        }
        else if self.rect.x > screen_width() - self.rect.w{
            self.rect.x = screen_width() - self.rect.w;
        }

    }

    pub fn draw(&self){
        draw_rectangle(self.rect.x, self.rect.y, self.rect.w, self.rect.h, GREEN);
    }
}

struct Block{
    rect: Rect,
    lives: i32,
}

impl Block{
    pub fn new(pos: Vec2) -> Self{
        Self{
            rect: Rect::new(pos.x, pos.y, BLOCK_SIZE.x, BLOCK_SIZE.y),
            lives: 3
        }
    }

    pub fn draw(&self){
        let color = match self.lives {
            3 => YELLOW,
            2 => ORANGE,
            _ => RED
        };

        draw_rectangle(self.rect.x, self.rect.y, self.rect.w, self.rect.h, color);
    }
}

pub struct Ball{
    rect: Rect,
    vel: Vec2
}

impl Ball{
    pub fn new(pos: Vec2) -> Self{
        Self{
            rect: Rect::new(pos.x, pos.y, BALL_SIZE.x, BALL_SIZE.y),
            vel: vec2(rand::gen_range(-1f32, 1f32), 1f32).normalize()
        }
    }

    pub fn update(&mut self, dt: f32){
        self.rect.x += self.vel.x * dt * BALL_SPEED;
        self.rect.y += self.vel.y * dt * BALL_SPEED;

        if self.rect.x > screen_width() - self.rect.w{
            self.vel.x = -1f32;
        }
        if self.rect.x < 0f32{
            self.vel.x = 1f32;
        }
        if self.rect.y < 0f32{
            self.vel.y = 1f32;
        }
    }

    pub fn draw(&self){
        draw_rectangle(self.rect.x, self.rect.y, self.rect.w, self.rect.h, BLACK);
    }
}

fn resolve_collission(a: &mut Rect, vel: &mut Vec2, b: &Rect) -> bool{
    let intersection = match a.intersect(*b){
        Some(intersection) => intersection,
        None => return false,
    };

    let a_center = a.point() + a.size() * 0.5f32;
    let b_center = b.point() + b.size() * 0.5f32;
    let to = b_center - a_center;
    let to_signum = to.signum();
    match intersection.w > intersection.h{
        true =>{
            a.y -= to_signum.y * intersection.h;
            match to_signum.y > 0f32{
                true => vel.y = -vel.y.abs(),
                false => vel.y = vel.y.abs(),
            }
        }

        false => {
            a.x -= to_signum.x * intersection.w;
            match to_signum.x < 0f32{
                true => vel.x = vel.x.abs(),
                false => vel.y = -vel.x.abs(),
            }
        }
    }
    true
}

#[macroquad::main("Breakout")]
async fn main(){
    let font = load_ttf_font("res/Roboto-Italic.ttf").await.unwrap();
    let mut score = 0;
    let mut player_lives = 3;

    let mut player: Player = Player::new();
    let mut blocks = Vec::new();
    let mut balls = Vec::new();
    let (block_count_x, block_count_y) = (6, 6);

    let padding = 5f32;
    let total_block_size = BLOCK_SIZE + vec2(padding, padding);

    let board_start_pos = vec2(
        (screen_width() - (total_block_size.x * block_count_x as f32)) * 0.5f32,
        50f32
    );

    for i in 0..block_count_x * block_count_y{
        let block_x = (i % block_count_x) as f32 * total_block_size.x;
        let block_y = (i / block_count_x) as f32 * total_block_size.y;
        blocks.push(Block::new(board_start_pos + vec2(block_x, block_y)));
    }

    balls.push(Ball::new(
        vec2(screen_width() * 0.5f32, screen_height() * 0.5f32)
    ));

    loop{
        player.update(get_frame_time());
        for ball in balls.iter_mut() {
            ball.update(get_frame_time());
        }
        for ball in balls.iter_mut(){
            resolve_collission(&mut ball.rect, &mut ball.vel, &player.rect);
            for block in blocks.iter_mut(){
                if resolve_collission(&mut ball.rect, &mut ball.vel, &block.rect){
                    block.lives -= 1;
                    if block.lives <= 0{
                        score += 10;
                    }
                }
            }
        }
        let balls_len = balls.len();
        let was_last_ball = balls_len == 1;
        balls.retain(|ball| ball.rect.y < screen_height());
        let removed_balls = balls_len - balls.len();
        if removed_balls > 0 && was_last_ball{
            player_lives -= 1;
        }
        blocks.retain(|block| block.lives > 0);

        clear_background(WHITE);
        player.draw();
        for block in blocks.iter(){
            block.draw();
        }
        for ball in balls.iter(){
            ball.draw();
        }

        let score_text = format!("score: {}", score);
        let score_text_dim = measure_text(&score_text, Some(font), 30u16, 1.0);
        draw_text_ex(
            &score_text,
            screen_width() * 0.5f32 - score_text_dim.width * 0.5f32,
            40.0,
            TextParams{font, font_size: 30u16, color: BLACK, ..Default::default() }
        );

        draw_text_ex(
            &format!("lives: {}", player_lives),
            30.0,
            40.0,
            TextParams{font, font_size: 30u16, color: BLACK, ..Default::default()}
        );

        next_frame().await        
    }
}