use bevy::math::{Vec2, Vec3};
use extend::ext;

#[ext]
impl Vec2 {
    fn from_vec3(v: Vec2) -> Self {
        Vec2::new(v.x, v.y)
    }
    fn to_vec3(&self) -> Vec3 {
        Vec3::new(self.x, self.y, 0.0)
    }
}

#[ext]
impl Vec3 {
    fn from_vec2(v: Vec2) -> Self {
        Vec3::from((v, 0.0))
    }
    fn to_vec2(&self) -> Vec2 {
        Vec2::new(self.x, self.y)
    }
}
