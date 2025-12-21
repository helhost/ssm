use glam::{Quat, Vec3};

use crate::visuals::camera::Camera;

pub struct CameraController {
    pub yaw: f32,
    pub pitch: f32,
    pub radius: f32,
    pub target: Vec3,
}

impl CameraController {
    pub fn new(radius: f32, target: Vec3) -> Self {
        Self {
            yaw: 0.0,
            pitch: -0.6,
            radius,
            target,
        }
    }

    pub fn on_mouse_drag(&mut self, dx: f32, dy: f32) {
        let sensitivity = 0.005;
        self.yaw -= dx * sensitivity;
        self.pitch -= dy * sensitivity;

        let limit = 1.5;
        self.pitch = self.pitch.clamp(-limit, limit);
    }

    pub fn on_scroll(&mut self, delta: f32) {
        let zoom = 1.0 - delta * 0.1;
        self.radius = (self.radius * zoom).clamp(1.0, 500.0);
    }

    pub fn apply(&self, camera: &mut Camera) {
        let rot_yaw = Quat::from_rotation_z(self.yaw);
        let rot_pitch = Quat::from_rotation_x(self.pitch);
        let rot = rot_yaw * rot_pitch;

        let offset = rot * Vec3::new(0.0, -self.radius, 0.0);
        camera.eye = self.target + offset;
        camera.target = self.target;
    }
}
