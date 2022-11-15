pub struct Block {
    pub pos_x: i32,
    pub pos_y: i32,
    pub shape: [[i32; 4]; 4],
}

impl Block {
    pub fn new_square() -> Block {
        let block = Block {
            pos_x: 0,
            pos_y: 0,
            shape: [[1, 1, 0, 0], [1, 1, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0]],
        };

        block
    }
}
