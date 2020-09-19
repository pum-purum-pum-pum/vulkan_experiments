use glam::{Mat4, Vec3,};

pub struct Camera {
    pub view: Mat4,
    pub projection: Mat4,
    pub zoom: f32,
}

impl Camera {
    pub fn new(width: f32, height: f32) -> Self {
        let zoom = 1.;
        let view = Mat4::look_at_rh(
            Vec3::new(0.0, 0.0, 1.0),
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::new(0.0, 1.0, 0.0),
        );
        let w = 1. / zoom;
        let h = (height / width) /zoom;
        let projection = Mat4::orthographic_rh(
            -w/2., 
            w/2., 
            -h/2., 
            h/2., 
            10.,
            -10.,
        );
        Camera {
            zoom,
            view,
            projection
        }
    }

    pub fn update(&mut self, width: f32, height: f32) {
        let w = 1. / self.zoom;
        let h = (height / width) /self.zoom;
        self.projection = Mat4::orthographic_rh(
            -w/2., 
            w/2., 
            -h/2., 
            h/2., 
            10.,
            -10.,
        );
    }
}