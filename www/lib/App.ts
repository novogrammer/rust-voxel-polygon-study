import {Universe,V3F} from "rust-voxel-polygon-study-wasm";

import * as THREE from "three";
import {OrbitControls} from "three/examples/jsm/controls/OrbitControls.js";

import { updateBufferGeometry } from "./rust_to_three";



export default class App{
  setupPromise:Promise<void>;
  voxel?:{
    universe:Universe;
  }
  three?:{
    renderer:THREE.WebGLRenderer,
    scene:THREE.Scene,
    camera:THREE.Camera,
    material:THREE.Material,
    controls:OrbitControls,
  };
  constructor(){
    this.setupPromise=this.setupAsync();
  }
  async setupVoxelAsync(){
    const universe=Universe.new();
    universe.update();
    universe.draw();

    this.voxel={
      universe,
    }
  }
  async setupThreeAsync(){
    const renderer=new THREE.WebGLRenderer({
      antialias: true,
      canvas:document.querySelector("#View") as HTMLCanvasElement,
    });
    renderer.setSize(window.innerWidth,window.innerHeight);
    renderer.setPixelRatio(window.devicePixelRatio);
    renderer.outputEncoding = THREE.sRGBEncoding;
    
    const scene = new THREE.Scene();
    const camera = new THREE.PerspectiveCamera(75, window.innerWidth / window.innerHeight, 0.01, 1000);
    camera.position.z = 40;
    const controls = new OrbitControls(camera, renderer.domElement);
  
    const ambientLight = new THREE.AmbientLight(0xffffff, 0.5);
    scene.add(ambientLight);
    const spotLight = new THREE.SpotLight(0xffffff, 1.2,1000,THREE.MathUtils.degToRad(30));
  
    spotLight.position.set(50, 50, 50);
    spotLight.lookAt(0,0,0);
    scene.add(spotLight);
  
  
    // const material=new THREE.MeshBasicMaterial({
    //   color:0xffffff,
    //   vertexColors:true,
    // });
    const material=new THREE.MeshStandardMaterial({
      color:0xffffff,
      roughness:0.5,
      metalness:0.3,
      vertexColors:true,
    });
    // const material=new THREE.MeshNormalMaterial({
    //   color:0xffffff,
    //   vertexColors:true,
    // });

    if(!this.voxel){
      throw new Error("this.voxel is null");
    }
    const {universe}=this.voxel;
  
    
    const l=universe.get_chunk_list_length();
    for(let i=0;i<l;i++){
      const bufferGeometry = new THREE.BufferGeometry();
  
      updateBufferGeometry(universe,i,bufferGeometry);
      console.log(bufferGeometry);
      const mesh=new THREE.Mesh(bufferGeometry,material);
      const origin=universe.get_chunk_origin(i);
      mesh.position.set(origin.get_x(),origin.get_y(),origin.get_z());
      scene.add(mesh);
    }
  


    this.three={
      renderer,
      scene,
      camera,
      material,
      controls,
    };
  }
  async setupEventsAsync():Promise<void>{
    if(!this.three){
      throw new Error("this.three is null");
    }
    const {renderer,scene,camera}=this.three;

    function render() {
      renderer.render(scene,camera);
    }
  
    renderer.setAnimationLoop( render );

  }
  async setupAsync():Promise<void>{
    await this.setupVoxelAsync();
    await this.setupThreeAsync();
    await this.setupEventsAsync();
  }

  async destroyVoxelAsync():Promise<void>{
    if(!this.voxel){
      return;
    }
    const {universe}=this.voxel;
    universe.free();
    this.voxel=undefined;
  }
  async destroyThreeAsync():Promise<void>{
  }
  async destroyEventsAsync():Promise<void>{
    if(!this.three){
      throw new Error("this.three is null");
    }
    const {renderer}=this.three;
    renderer.setAnimationLoop(null);
  }
  async destroyAsync():Promise<void>{
    await this.setupPromise;
    await this.destroyEventsAsync();
    await this.destroyThreeAsync();
    await this.destroyVoxelAsync();
  }
}