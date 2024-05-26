use crate::render_gl::data;

#[derive(VertexAttribPointers, Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct LinePoint {
    #[location = "0"]
    pub pos: data::f32_f32_f32,
    #[location = "1"]
    pub color: data::u2_u10_u10_u10_rev_float,
}
