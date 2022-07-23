use crate::v2f::V2F;
use crate::v3f::V3F;

#[derive(Clone, Copy, Debug)]
pub struct Vertex {
    pub position: V3F,
    pub normal: V3F,
    pub color: V3F,
    pub uv: V2F,
}
