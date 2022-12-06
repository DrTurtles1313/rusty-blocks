use raylib::prelude::*;

mod blocks;
use blocks::*;

//Size of screen in tiles
const TILE_RESOLUTION_X: i32 = 10;
const TILE_RESOLUTION_Y: i32 = 16;
//Resolution of tiles
const RES_MULTIPLIER: i32 = 32;

pub struct GameState {
    landed: [[bool; 16]; 10],
    block: Block,
    time: f32,
    current_tick_time: f32,
}

impl GameState {
    fn new() -> GameState {
        GameState {
            landed: [[false; 16]; 10],
            block: Block::new(BlockTypes::I),
            time: 0.0,
            current_tick_time: 1.5,
        }
    }
}

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(
            TILE_RESOLUTION_X * RES_MULTIPLIER,
            TILE_RESOLUTION_Y * RES_MULTIPLIER,
        )
        .title("Rusty Blocks")
        .build();

    let mut game_state = GameState::new();
    rl.set_target_fps(60);

    while !rl.window_should_close() {
        render(&mut rl, &thread, &game_state);
        tick(&mut game_state, &mut rl);
    }
}

/// Main render function
fn render(rl: &mut RaylibHandle, thread: &RaylibThread, game_state: &GameState) {
    let mut d = rl.begin_drawing(&thread);

    d.clear_background(Color::WHITE);
    //Draw landed tiles
    for (x, row) in game_state.landed.iter().enumerate() {
        for (y, col) in row.iter().enumerate() {
            if *col {
                d.draw_rectangle(
                    x as i32 * RES_MULTIPLIER,
                    y as i32 * RES_MULTIPLIER,
                    RES_MULTIPLIER,
                    RES_MULTIPLIER,
                    Color::BLUE,
                );
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

/// Main game tick functions
fn tick(game_state: &mut GameState, rl: &mut RaylibHandle) {
    //Handle input
    match rl.get_key_pressed() {
        None => (),
        Some(x) => match x {
            KeyboardKey::KEY_D => try_move(game_state, 1, 0),
            KeyboardKey::KEY_A => try_move(game_state, -1, 0),
            KeyboardKey::KEY_J => try_rotate(game_state, false),
            KeyboardKey::KEY_K => try_rotate(game_state, true),
            _ => (),
        },
    }
    
    game_state.time += rl.get_frame_time();

    if game_state.time >= game_state.current_tick_time {
        game_state.time = 0.0;
        try_move(game_state, 0, 1)
    }
}

/// Attempt to move block, landing if hitting the bottom, or moving block if not
fn try_move(game_state: &mut GameState, dx: i32, dy: i32) {
    let new_x = game_state.block.pos_x + dx;
    let new_y = game_state.block.pos_y + dy;

    for (x, row) in game_state.block.shape.iter().enumerate() {
        for (y, col) in row.iter().enumerate() {
            if *col == 1 {
                //Prevent Blocks from being moved off the edge of the screen
                //Also restricts new position values to -1 as lowest, preventing overflow errors
                if new_x + (x as i32) > 9 || new_x + (x as i32) < 0 {
                    return;
                }
                if new_y + y as i32 > 15 {
                    land_block(game_state);
                    return;
                }
                //Positions can be negative, so juggle type casts to prevent overflows
                //This is ok because new_x will never be less than -1
                if game_state.landed[(x as i32 + new_x) as usize][(y as i32 + new_y) as usize] {
                    land_block(game_state);
                    return;
                }
            }
        }
    }

    game_state.block.pos_y += dy;
    game_state.block.pos_x += dx;
}

fn try_rotate(game_state: &mut GameState, clock_wise: bool) {
        let value: i32;
        if clock_wise { value = 1 } else { value = -1 }

        game_state.block.current_rotation = wrapping_add(game_state.block.current_rotation, value, &game_state.block.block_type);
        match game_state.block.block_type {
            BlockTypes::I => game_state.block.shape = I_ROTATIONS[game_state.block.current_rotation as usize],
            BlockTypes::O => game_state.block.shape = O_ROTATIONS[game_state.block.current_rotation as usize],
            BlockTypes::T => game_state.block.shape = T_ROTATIONS[game_state.block.current_rotation as usize],
            BlockTypes::S => game_state.block.shape = S_ROTATIONS[game_state.block.current_rotation as usize],
            BlockTypes::Z => game_state.block.shape = Z_ROTATIONS[game_state.block.current_rotation as usize],
            BlockTypes::J => game_state.block.shape = J_ROTATIONS[game_state.block.current_rotation as usize],
            BlockTypes::L => game_state.block.shape = L_ROTATIONS[game_state.block.current_rotation as usize],
        }
    }


/// Adds block to landed playfield, generates new block
fn land_block(game_state: &mut GameState) {
    for (x, row) in game_state.block.shape.iter().enumerate() {
        for (y, col) in row.iter().enumerate() {
            if *col == 1 {
                //Juggle number types to prevent overflow
                game_state.landed[(game_state.block.pos_x + x as i32) as usize]
                    [game_state.block.pos_y as usize + y] = true;
            }
        }
    }

    game_state.block = Block::new(BlockTypes::O);
}
