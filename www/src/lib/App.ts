import {Universe,V3F} from "rust-voxel-polygon-study-wasm";

import * as THREE from "three";
import {OrbitControls} from "three/examples/jsm/controls/OrbitControls.js";
import {EXRLoader} from "three/examples/jsm/loaders/EXRLoader.js";

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
    bufferGeometryList:THREE.BufferGeometry[],
    controls:OrbitControls,
  };
  constructor(){
    this.setupPromise=this.setupAsync();
  }
  async setupVoxelAsync(){
    const universe=Universe.new();
    universe.update(0.0);
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
  

    const loadEXRTextureAsync=(baseDir:string,filename:string)=>{
      return new Promise<THREE.Texture>((resolve)=>{
        new EXRLoader().setPath(baseDir).load(filename,(texture)=>{
          texture.encoding=THREE.sRGBEncoding;
          texture.needsUpdate=true;
          resolve(texture);
        });
      });
    };
    const loadTextureAsync=(baseDir:string,filename:string)=>{
      return new Promise<THREE.Texture>((resolve)=>{
        new THREE.TextureLoader().setPath(baseDir).load(filename,(texture)=>{
          texture.encoding=THREE.sRGBEncoding;
          texture.needsUpdate=true;
          resolve(texture);
        });
      });
    };
    const setupTexture=(t:THREE.Texture)=>{
      t.minFilter= THREE.NearestFilter;
      t.magFilter= THREE.NearestFilter;
      // t.minFilter= THREE.LinearFilter;
      // t.magFilter= THREE.LinearFilter;
      t.generateMipmaps=true;
      t.needsUpdate=true;
    };

    // const material= await (async()=>{
    //   const baseDir="./textures/coast_sand_rocks_02_1k/";
    //   const diff=await loadTextureAsync(baseDir,"coast_sand_rocks_02_diff_1k.jpg");
    //   // const disp=await loadTextureAsync(baseDir,"coast_sand_rocks_02_disp_1k.png");
    //   const nor=await loadEXRTextureAsync(baseDir,"coast_sand_rocks_02_nor_gl_1k.exr");
    //   const rough=await loadEXRTextureAsync(baseDir,"coast_sand_rocks_02_rough_1k.exr");

    //   const material=new THREE.MeshStandardMaterial({
    //     map:diff,
    //     roughnessMap:rough,
    //     metalness:0,
    //     normalMap:nor,
    //     // displacementMap:disp,
    //     vertexColors:true,
    //   });
    //   return material;
  
    // })();

    // const material=await(async()=>{
    //   const baseDir="./textures/metal_plate_1k/";
    //   const diff=await loadTextureAsync(baseDir,"metal_plate_diff_1k.jpg");
    //   // const disp=await loadTextureAsync(baseDir,"metal_plate_disp_1k.png");
    //   const nor=await loadEXRTextureAsync(baseDir,"metal_plate_nor_gl_1k.exr");
    //   const rough=await loadTextureAsync(baseDir,"metal_plate_rough_1k.jpg");
    
    //   const material=new THREE.MeshStandardMaterial({
    //     map:diff,
    //     roughnessMap:rough,
    //     metalness:0,
    //     normalMap:nor,
    //     // displacementMap:disp,
    //     vertexColors:true,
    //   });
    //   return material;
    // })();

    // const material=await(async()=>{
    //   const baseDir="./textures/rock_boulder_cracked_1k/";
    //   const diff=await loadTextureAsync(baseDir,"rock_boulder_cracked_diff_1k.jpg");
    //   // const disp=await loadTextureAsync(baseDir,"rock_boulder_cracked_disp_1k.png");
    //   const nor=await loadEXRTextureAsync(baseDir,"rock_boulder_cracked_nor_gl_1k.exr");
    //   const rough=await loadEXRTextureAsync(baseDir,"rock_boulder_cracked_rough_1k.exr");
    
    //   const material=new THREE.MeshStandardMaterial({
    //     map:diff,
    //     roughnessMap:rough,
    //     metalness:0,
    //     normalMap:nor,
    //     // displacementMap:disp,
    //     vertexColors:true,
    //   });
    //   return material;
    // })();

    // const material=await(async()=>{
    //   const baseDir="./textures/stone_brick_wall_001_1k/";
    //   const diff=await loadTextureAsync(baseDir,"stone_brick_wall_001_diff_1k.jpg");
    //   // const disp=await loadTextureAsync(baseDir,"stone_brick_wall_001_disp_1k.png");
    //   const nor=await loadEXRTextureAsync(baseDir,"stone_brick_wall_001_nor_gl_1k.exr");
    //   const rough=await loadTextureAsync(baseDir,"stone_brick_wall_001_rough_1k.jpg");
    
    //   const material=new THREE.MeshStandardMaterial({
    //     map:diff,
    //     roughnessMap:rough,
    //     metalness:0,
    //     normalMap:nor,
    //     // displacementMap:disp,
    //     vertexColors:true,
    //   });
    //   return material;
    // })();

    // const material=await(async()=>{
    //   const baseDir="./textures/red_brick_03_1k/";
    //   const diff=await loadTextureAsync(baseDir,"red_brick_03_diff_1k.jpg");
    //   // const disp=await loadTextureAsync(baseDir,"red_brick_03_disp_1k.png");
    //   const nor=await loadEXRTextureAsync(baseDir,"red_brick_03_nor_gl_1k.exr");
    //   const rough=await loadTextureAsync(baseDir,"red_brick_03_rough_1k.jpg");
    
    //   const material=new THREE.MeshStandardMaterial({
    //     map:diff,
    //     roughnessMap:rough,
    //     metalness:0,
    //     normalMap:nor,
    //     // displacementMap:disp,
    //     vertexColors:true,
    //   });
    //   return material;
    // })();

    const material=await(async()=>{
      const baseDir="./textures/packed/";
      const diff=await loadTextureAsync(baseDir,"packed_diff.jpg");
      // const disp=await loadTextureAsync(baseDir,"packed_disp.png");
      const nor=await loadEXRTextureAsync(baseDir,"packed_nor_gl.exr");
      const rough=await loadEXRTextureAsync(baseDir,"packed_rough.exr");

      [diff,nor,rough].forEach(setupTexture);
    
      const material=new THREE.MeshStandardMaterial({
        map:diff,
        roughnessMap:rough,
        metalness:0,
        normalMap:nor,
        // displacementMap:disp,
        vertexColors:true,
      });
      return material;
    })();

    // const material=new THREE.MeshBasicMaterial({
    //   color:0xffffff,
    //   vertexColors:true,
    // });
    // const material=new THREE.MeshNormalMaterial({
    //   color:0xffffff,
    //   vertexColors:true,
    // });

    if(!this.voxel){
      throw new Error("this.voxel is null");
    }
    const {universe}=this.voxel;
  
    
    const l=universe.get_chunk_list_length();
    // const maxGeometryVertexLength=universe.get_max_geometry_vertex_length();
    const initialGeometryVertexLength=universe.get_initial_geometry_vertex_length();
    const bufferGeometryList=[];
    for(let i=0;i<l;i++){
      const bufferGeometry = new THREE.BufferGeometry();
      bufferGeometry.userData.vertexLength=initialGeometryVertexLength;
      {
        const positionList=new Float32Array(initialGeometryVertexLength*3);
        const positionAttribute=new THREE.BufferAttribute(positionList,3);
        positionAttribute.setUsage(THREE.DynamicDrawUsage);
        bufferGeometry.setAttribute("position",positionAttribute);
        bufferGeometry.userData.positionList=positionList;
      }
      {
        const normalList=new Float32Array(initialGeometryVertexLength*3);
        const normalAttribute=new THREE.BufferAttribute(normalList,3);
        normalAttribute.setUsage(THREE.DynamicDrawUsage);
        bufferGeometry.setAttribute("normal",normalAttribute);
        bufferGeometry.userData.normalList=normalList;
      }
      {
        const colorList=new Float32Array(initialGeometryVertexLength*3);
        const colorAttribute=new THREE.BufferAttribute(colorList,3);
        colorAttribute.setUsage(THREE.DynamicDrawUsage);
        bufferGeometry.setAttribute("color",colorAttribute);
        bufferGeometry.userData.colorList=colorList;
      }
      {
        const uvList=new Float32Array(initialGeometryVertexLength*2);
        const uvAttribute=new THREE.BufferAttribute(uvList,2);
        uvAttribute.setUsage(THREE.DynamicDrawUsage);
        bufferGeometry.setAttribute("uv",uvAttribute);
        bufferGeometry.userData.uvList=uvList;
      }

      bufferGeometryList.push(bufferGeometry);
      const mesh=new THREE.Mesh(bufferGeometry,material);
      const origin=universe.get_chunk_origin(i);
      mesh.position.set(origin.get_x(),origin.get_y(),origin.get_z());
      scene.add(mesh);
    }
    for(let i=0;i<l;i++){
      const bufferGeometry=bufferGeometryList[i];
  
      updateBufferGeometry(universe,i,bufferGeometry);
      console.log(bufferGeometry);
    }
  


    this.three={
      renderer,
      scene,
      camera,
      material,
      controls,
      bufferGeometryList,
    };
  }
  async setupEventsAsync():Promise<void>{
    if(!this.three){
      throw new Error("this.three is null");
    }
    const {renderer}=this.three;

    renderer.setAnimationLoop( this.onRender.bind(this) );

  }
  async setupAsync():Promise<void>{
    await this.setupVoxelAsync();
    await this.setupThreeAsync();
    await this.setupEventsAsync();
  }
  onRender(time: DOMHighResTimeStamp, frame: XRFrame){
    if(!this.voxel){
      throw new Error("this.voxel is null");
    }
    const {universe}=this.voxel;
    if(!this.three){
      throw new Error("this.three is null");
    }
    const {renderer,scene,camera,bufferGeometryList}=this.three;
    universe.update(time * 0.001);
    universe.draw();

    const l=universe.get_chunk_list_length();
    for(let i=0;i<l;i++){
      const bufferGeometry=bufferGeometryList[i];
  
      updateBufferGeometry(universe,i,bufferGeometry);
      // console.log(bufferGeometry);
    }


    renderer.render(scene,camera);

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