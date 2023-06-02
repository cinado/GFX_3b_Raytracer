pub struct Camera {
    pub position: Point,
    pub lookat: Vec3,
    pub up: Vec3,
    pub horizontal_fov: f32,
    pub resolution_horizontal: u32,
    pub resolution_vertical: u32,
    pub max_bounces: u32,
}