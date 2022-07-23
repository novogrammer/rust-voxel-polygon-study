use crate::v2f::V2F;
use crate::v3f::V3F;
pub struct GeometryBuffer {
    pub position_list: Vec<V3F>,
    pub normal_list: Vec<V3F>,
    pub color_list: Vec<V3F>,
    pub uv_list: Vec<V2F>,
}
