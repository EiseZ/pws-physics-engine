pub struct PhysicsObject {
    pub object_type: PhysicsObjectType,
    pub x: f32,
    pub y: f32,
    pub scale_x: f32,
    pub scale_y: f32,
    pub scale_z: f32,
    rot_x: f32,
    rot_y: f32,
    rot_z: f32,
}

pub enum PhysicsObjectType {
    Rect,
    Circ{ r: f32 },
}

impl PhysicsObject {
    pub fn new(object_type: PhysicsObjectType, x: f32, y: f32, scale_x: f32, scale_y: f32, scale_z: f32) -> PhysicsObject {
        return PhysicsObject {
            object_type: object_type,
            x: x,
            y: y,
            scale_x: scale_x,
            scale_y: scale_y,
            scale_z: scale_z,
            rot_x: 0.0,
            rot_y: 0.0,
            rot_z: 0.0,
        }
    }

    pub fn set_pos(&mut self, x: f32, y: f32) {
        self.x = x;
        self.y = y;
    }

    pub fn translate(&mut self, dx: f32, dy: f32) {
        self.x += dx;
        self.y += dy;
    }
}