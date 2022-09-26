pub struct Rect {
    x: u32,
    y: u32,
    z: u32,
    // s = size
    s_x: u32,
    s_y: u32,
}

impl Rect {
    pub fn new(x: u32, y: u32, z: u32, s_x: u32, s_y: u32) -> Rect {
        Rect {
            x: x,
            y: y,
            z: z,
            s_x: s_x,
            s_y: s_y,
        }
    }
}