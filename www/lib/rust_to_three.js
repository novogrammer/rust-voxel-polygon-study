import { memory } from "rust-voxel-polygon-study-wasm/rust_voxel_polygon_study_wasm_bg";

import * as THREE from "three";


export function toBufferGeometry(universe,i){
  const bufferGeometry = new THREE.BufferGeometry();
  const vertex_length=universe.get_geometry_buffer_vertex_length(i);

  {
    const positionListPointer=universe.get_geometry_buffer_position_list_ptr(i);
    const positionList = new Float32Array(memory.buffer,positionListPointer,vertex_length * 3);
    const positionAttribute=new THREE.BufferAttribute(positionList,3);
    bufferGeometry.setAttribute("position",positionAttribute);

  }
  {
    const normalListPointer=universe.get_geometry_buffer_normal_list_ptr(i);
    const normalList =  new Float32Array(memory.buffer,normalListPointer,vertex_length * 3);
    const normalAttribute=new THREE.BufferAttribute(normalList,3);
    bufferGeometry.setAttribute("normal",normalAttribute);

  }
  {
    const colorListPointer=universe.get_geometry_buffer_color_list_ptr(i);
    const colorList =  new Float32Array(memory.buffer,colorListPointer,vertex_length * 3);
    const colorAttribute=new THREE.BufferAttribute(colorList,3);
    bufferGeometry.setAttribute("color",colorAttribute);
  }

  return bufferGeometry;
}