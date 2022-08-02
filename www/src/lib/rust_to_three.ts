import { memory } from "rust-voxel-polygon-study-wasm/rust_voxel_polygon_study_wasm_bg.wasm";
import type { Universe } from "rust-voxel-polygon-study-wasm";
import * as THREE from "three";
import { BufferAttribute } from "three";

function resizeIfNeeded(bufferGeometry:THREE.BufferGeometry,size:number){
  if(size<=bufferGeometry.userData.vertexLength){
    return;
  }

  // 多めに確保する
  const nextSize=size*2;
  console.log(`resize ${bufferGeometry.userData.vertexLength} -> ${nextSize}`)
  bufferGeometry.userData.vertexLength=nextSize;
  // 古いリソースの削除が必要？
  {
    const positionList=new Float32Array(nextSize*3);
    const positionAttribute=new THREE.BufferAttribute(positionList,3);
    positionAttribute.setUsage(THREE.DynamicDrawUsage);
    bufferGeometry.setAttribute("position",positionAttribute);
    bufferGeometry.userData.positionList=positionList;
  }
  {
    const normalList=new Float32Array(nextSize*3);
    const normalAttribute=new THREE.BufferAttribute(normalList,3);
    normalAttribute.setUsage(THREE.DynamicDrawUsage);
    bufferGeometry.setAttribute("normal",normalAttribute);
    bufferGeometry.userData.normalList=normalList;
  }
  {
    const colorList=new Float32Array(nextSize*3);
    const colorAttribute=new THREE.BufferAttribute(colorList,3);
    colorAttribute.setUsage(THREE.DynamicDrawUsage);
    bufferGeometry.setAttribute("color",colorAttribute);
    bufferGeometry.userData.colorList=colorList;
  }
  {
    const uvList=new Float32Array(nextSize*2);
    const uvAttribute=new THREE.BufferAttribute(uvList,2);
    uvAttribute.setUsage(THREE.DynamicDrawUsage);
    bufferGeometry.setAttribute("uv",uvAttribute);
    bufferGeometry.userData.uvList=uvList;
  }
}

export function updateBufferGeometry(universe:Universe,i:number,bufferGeometry:THREE.BufferGeometry){

  const version = universe.get_geometry_version(i);
  // console.log(`bufferGeometry.userData.version:${bufferGeometry.userData.version}`);
  // console.log(`version:${version}`);
  if(bufferGeometry.userData.version == version){
    return;
  }

  const vertex_length=universe.get_geometry_buffer_vertex_length(i);

  resizeIfNeeded(bufferGeometry,vertex_length);
  

  {
    const positionListPointer=universe.get_geometry_buffer_position_list_ptr(i);
    const positionList = new Float32Array(memory.buffer,positionListPointer,vertex_length * 3);
    // bufferGeometry.userData.positionList.fill(0);
    bufferGeometry.userData.positionList.set(positionList);
    bufferGeometry.getAttribute("position").needsUpdate=true;

  }
  {
    const normalListPointer=universe.get_geometry_buffer_normal_list_ptr(i);
    const normalList =  new Float32Array(memory.buffer,normalListPointer,vertex_length * 3);
    // bufferGeometry.userData.normalList.fill(0);
    bufferGeometry.userData.normalList.set(normalList);
    bufferGeometry.getAttribute("normal").needsUpdate=true;
  }
  {
    const colorListPointer=universe.get_geometry_buffer_color_list_ptr(i);
    const colorList =  new Float32Array(memory.buffer,colorListPointer,vertex_length * 3);
    // bufferGeometry.userData.colorList.fill(0);
    bufferGeometry.userData.colorList.set(colorList);
    bufferGeometry.getAttribute("color").needsUpdate=true;
  }
  {
    const uvListPointer=universe.get_geometry_buffer_uv_list_ptr(i);
    const uvList =  new Float32Array(memory.buffer,uvListPointer,vertex_length * 2);
    // bufferGeometry.userData.uvList.fill(0);
    bufferGeometry.userData.uvList.set(uvList);
    bufferGeometry.getAttribute("uv").needsUpdate=true;
  }
  bufferGeometry.setDrawRange( 0, vertex_length );
  bufferGeometry.userData.version=version;
}