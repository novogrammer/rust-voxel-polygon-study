import {Universe,V3F} from "rust-voxel-polygon-study-wasm";

import * as THREE from "three";
import { toBufferGeometry } from "./lib/rust_to_three";





async function main(){
  const universe=Universe.new();
  const l=universe.get_chunk_list_length();

  universe.update();
  {
    const p=V3F.new(0,0,0);
    universe.draw(p);
    p.free();
  }

  const renderer=new THREE.WebGLRenderer({
    antialias: true,
    canvas:document.querySelector("#View"),
  });
  renderer.setSize(window.innerWidth,window.innerHeight);
  renderer.setPixelRatio(window.devicePixelRatio);
  renderer.outputEncoding = THREE.sRGBEncoding;
  
  const scene = new THREE.Scene();
  const camera = new THREE.PerspectiveCamera(75, window.innerWidth / window.innerHeight, 0.01, 1000);
  camera.position.z = 5;

  const material=new THREE.MeshNormalMaterial({
  });
  for(let i=0;i<l;i++){
    const bufferGeometry=toBufferGeometry(universe,i);
    console.log(bufferGeometry);
    const mesh=new THREE.Mesh(bufferGeometry,material);
    const origin=universe.get_chunk_origin(i);
    mesh.position.set(origin.get_x(),origin.get_y(),origin.get_z());
    scene.add(mesh);
  }
  renderer.render(scene,camera);

  // universe.free();
}



main();

