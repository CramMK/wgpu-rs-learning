#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Uniform {
    view_projection: [[f32; 4]; 4],
}

impl Uniform {
    pub fn new() -> Self {
        use cgmath::SquareMatrix;
        Self {
            view_projection: cgmath::Matrix4::identity().into(),
        }
    }

    pub fn update_view_proj(&mut self, camera: &crate::camera::Camera) {
        self.view_projection = camera.build_view_projection_matrix().into();
    }
}
