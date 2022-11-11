use raylib::prelude::*;

struct GameState
{
    landed: [[bool; 10]; 16]
}

impl GameState
{
    fn new() -> GameState
    {
        GameState { landed: [[false; 10]; 16] }
    }
}

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(640, 480)
        .title("Rusty Blocks")
        .build();

    let mut game_state = GameState::new();
    game_state.landed[0][0] = true;

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::WHITE);
        for (x, row) in game_state.landed.iter().enumerate()
        {
            for (y, col) in row.iter().enumerate()
            {
                if *col
                {
                    d.draw_rectangle((x * 16) as i32, (y * 16) as i32, 16, 16, Color::BLUE);
                }
            }
        }
    }
}
