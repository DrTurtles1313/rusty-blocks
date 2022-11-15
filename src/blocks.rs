
pub enum BlockTypes {
    I,
    O,
    T,
    S,
    Z,
    L,
    J,
}

pub static I_ROTATIONS: [[[i32; 4]; 4]; 2] = [
    [[0, 0, 0, 0], [0, 0, 0, 0], [1, 1, 1, 1], [0, 0, 0, 0]],
    [[0, 0, 1, 0], [0, 0, 1, 0], [0, 0, 1, 0], [0, 0, 1, 0]],
];

pub static S_ROTATIONS: [[[i32; 4]; 4]; 2] = [
    [[0, 0, 0, 0], [0, 0, 0, 0], [0, 1, 1, 0], [1, 1, 0, 0]],
    [[0, 0, 0, 0], [1, 0, 0, 0], [1, 1, 0, 0], [0, 1, 0, 0]],
];

pub static Z_ROTATIONS: [[[i32; 4]; 4]; 2] = [
    [[0, 0, 0, 0], [0, 0, 0, 0], [1, 1, 0, 0], [0, 1, 1, 0]],
    [[0, 0, 0, 0], [0, 1, 0, 0], [1, 1, 0, 0], [1, 0, 0, 0]],
];

pub static O_ROTATIONS: [[[i32; 4]; 4]; 1] = [[[0, 0, 0, 0], [0, 1, 1, 0], [0, 1, 1, 0], [0, 0, 0, 0]]];

pub static T_ROTATIONS: [[[i32; 4]; 4]; 4] = [
    [[0, 0, 0, 0], [0, 1, 0, 0], [1, 1, 1, 0], [0, 0, 0, 0]],
    [[0, 0, 0, 0], [0, 1, 0, 0], [1, 1, 0, 0], [0, 1, 0, 0]],
    [[0, 0, 0, 0], [0, 0, 0, 0], [1, 1, 1, 0], [0, 1, 0, 0]],
    [[0, 0, 0, 0], [0, 1, 0, 0], [0, 1, 1, 0], [0, 1, 0, 0]],
];

pub static J_ROTATIONS: [[[i32; 4]; 4]; 4] = [
    [[0, 0, 0, 0], [0, 0, 0, 0], [1, 1, 1, 0], [0, 0, 1, 0]],
    [[0, 0, 0, 0], [0, 1, 0, 0], [0, 1, 0, 0], [1, 1, 0, 0]],
    [[0, 0, 0, 0], [1, 0, 0, 0], [1, 1, 1, 0], [0, 0, 0, 0]],
    [[0, 0, 0, 0], [0, 1, 1, 0], [0, 1, 0, 0], [0, 1, 0, 0]],
];

pub static L_ROTATIONS: [[[i32; 4]; 4]; 4] = [
    [[0, 0, 0, 0], [0, 0, 0, 0], [1, 1, 1, 0], [1, 0, 0, 0]],
    [[0, 0, 0, 0], [1, 1, 0, 0], [0, 1, 0, 0], [0, 1, 0, 0]],
    [[0, 0, 0, 0], [0, 0, 1, 0], [1, 1, 1, 0], [0, 0, 0, 0]],
    [[0, 0, 0, 0], [0, 1, 0, 0], [0, 1, 0, 0], [0, 1, 1, 0]],
];

pub struct Block {
    pub pos_x: i32,
    pub pos_y: i32,
    pub shape: [[i32; 4]; 4],
    pub current_rotation: i32,
    pub block_type: BlockTypes,
}

impl Block {
    pub fn new(block_type: BlockTypes) -> Block {
        let shape: [[i32; 4]; 4];

        match block_type {
            BlockTypes::I => shape = I_ROTATIONS[0],
            BlockTypes::O => shape = O_ROTATIONS[0],
            BlockTypes::T => shape = T_ROTATIONS[0],
            BlockTypes::S => shape = S_ROTATIONS[0],
            BlockTypes::Z => shape = Z_ROTATIONS[0],
            BlockTypes::J => shape = J_ROTATIONS[0],
            BlockTypes::L => shape = L_ROTATIONS[0],
        }

        let block = Block {
            pos_x: 0,
            pos_y: 0,
            shape,
            current_rotation: 0,
            block_type
        };

        block
    }

    pub fn rotate(&mut self, clock_wise: bool) {
        let value: i32;
        if clock_wise { value = 1 } else { value = -1 }

        self.current_rotation = wrapping_add(self.current_rotation, value, &self.block_type);
        match self.block_type {
            BlockTypes::I => self.shape = I_ROTATIONS[self.current_rotation as usize],
            BlockTypes::O => self.shape = O_ROTATIONS[self.current_rotation as usize],
            BlockTypes::T => self.shape = T_ROTATIONS[self.current_rotation as usize],
            BlockTypes::S => self.shape = S_ROTATIONS[self.current_rotation as usize],
            BlockTypes::Z => self.shape = Z_ROTATIONS[self.current_rotation as usize],
            BlockTypes::J => self.shape = J_ROTATIONS[self.current_rotation as usize],
            BlockTypes::L => self.shape = L_ROTATIONS[self.current_rotation as usize],
        }
    }
}


pub fn wrapping_add(x: i32, y: i32, block_type: &BlockTypes) -> i32 {
    let mut result: i32;

    match block_type {
            BlockTypes::I => {
                result = x + y;
                if result > 1 { result = 0 }
                if result < 0 { result = 1 }
            },
            BlockTypes::O => result = 0,
            BlockTypes::T => {
                result = x + y;
                if result > 3 { result = 0 }
                if result < 0 { result = 3 }
            },
            BlockTypes::S => {
                result = x + y;
                if result > 3 { result = 0 }
                if result < 0 { result = 3 }
            },
            BlockTypes::Z => {
                result = x + y;
                if result > 3 { result = 0 }
                if result < 0 { result = 3 }
            },
            BlockTypes::J => {
                result = x + y;
                if result > 3 { result = 0 }
                if result < 0 { result = 3 }
            },
            BlockTypes::L => {
                result = x + y;
                if result > 3 { result = 0 }
                if result < 0 { result = 3 }
            },
        }

    result
}
