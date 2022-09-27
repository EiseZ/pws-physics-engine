use crate::vector::Vector;

pub struct PhysicsObject {
    pub object_type: PhysicsObjectType,
    pub pos: Vector<f32>,
    pub scale: Vector<f32>,
    pub rot: Vector<f32>,
    pub speed: Vector<f32>,
    pub accl: Vector<f32>,
}

pub enum PhysicsObjectType {
    Rect,
    Circ{ r: f32 },
}

impl PhysicsObject {
    pub fn new(object_type: PhysicsObjectType, pos: Vector<f32>, scale: Vector<f32>, rot: Vector<f32>, speed: Vector<f32>, accl: Vector<f32>) -> PhysicsObject {
        return PhysicsObject {
            object_type: object_type,
            pos: pos,
            scale: scale,
            rot: rot,
            speed: speed,
            accl: accl,
        }
    }

    pub fn set_pos(&mut self, pos: Vector<f32>) {
        self.pos = pos;
    }

    pub fn translate(&mut self, dpos: Vector<f32>) {
        self.pos += dpos;
    }

    pub fn set_scale(&mut self, scale: Vector<f32>) {
        self.scale = scale;
    }

    pub fn scale(&mut self, dscale: Vector<f32>) {
        self.scale += dscale;
    }

    pub fn set_rotation(&mut self, rot: Vector<f32>) {
        self.rot = rot;
    }

    pub fn rotate(&mut self, drot: Vector<f32>) {
        self.rot += drot;
    }

    pub fn set_acceleration(&mut self, accl: Vector<f32>) {
        self.accl = accl;
    }

    pub fn accelecare(&mut self, daccl: Vector<f32>) {
        self.accl += daccl;
    }

    fn update_speed(&mut self, dt: f32) {
        self.speed += self.accl * dt;
    }

    fn update_pos(&mut self, dt: f32) {
        self.pos += self.speed * dt * 100.0 /* 100 = because pixels is in cm */;
    }

    pub fn update_all(&mut self, dt: f32) {
        self.update_speed(dt);
        self.update_pos(dt);
    }
}