use raylib::prelude::*;

const PIXEL_RESOLUTION_X: i32 = 10;
const PIXEL_RESOLUTION_Y: i32 = 16;
const RES_MULTIPLIER: i32 = 16;

struct GameState {
    landed: [[bool; 16]; 10],
    block: Block,
}

impl GameState {
    fn new() -> GameState {
        GameState {
            landed: [[false; 16]; 10],
            block: Block::new_square(),
        }
    }
}

struct Block {
    pos_x: i32,
    pos_y: i32,
    shape: [[i32; 4]; 4],
}

impl Block {
    fn new_square() -> Block {
        let block = Block {
            pos_x: 0,
            pos_y: 0,
            shape: [[1, 1, 0, 0], [1, 1, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0]],
        };

        block
    }
}

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(
            PIXEL_RESOLUTION_X * RES_MULTIPLIER,
            PIXEL_RESOLUTION_Y * RES_MULTIPLIER,
        )
        .title("Rusty Blocks")
        .build();

    let mut game_state = GameState::new();
    game_state.landed[0][15] = true;

    while !rl.window_should_close() {
        render(&mut rl, &thread, &game_state);
        
    }
}

fn render(rl: &mut RaylibHandle, thread: &RaylibThread, game_state: &GameState) {
    let mut d = rl.begin_drawing(&thread);

    d.clear_background(Color::WHITE);
    //Draw landed tiles
    for (x, row) in game_state.landed.iter().enumerate() {
        for (y, col) in row.iter().enumerate() {
            if *col {
                d.draw_rectangle((x * 16) as i32, (y * 16) as i32, 16, 16, Color::BLUE);
            }
        }
    }

    //Draw current block
    for (x, row) in game_state.block.shape.iter().enumerate() {
        for (y, col) in row.iter().enumerate() {
            if *col == 1 {
                d.draw_rectangle(
                    (x as i32 + game_state.block.pos_x) * RES_MULTIPLIER,
                    (y as i32 + game_state.block.pos_y) * RES_MULTIPLIER,
                    RES_MULTIPLIER,
                    RES_MULTIPLIER,
                    Color::BLACK,
                );
            }
        }
    }
}
