import {Universe,V3F} from "rust-voxel-polygon-study-wasm";

import * as THREE from "three";
import {OrbitControls} from "three/examples/jsm/controls/OrbitControls.js";
import {EXRLoader} from "three/examples/jsm/loaders/EXRLoader.js";

import { updateBufferGeometry } from "./rust_to_three";
import Stats from "stats.js"

interface Ball{
  mesh:THREE.Mesh,
  fill:number,
  radius:number,
  velocity:THREE.Vector3,
};

export default class App{
  setupPromise:Promise<void>;
  voxel?:{
    universe:Universe;
  }
  three?:{
    renderer:THREE.WebGLRenderer,
    scene:THREE.Scene,
    camera:THREE.PerspectiveCamera,
    material:THREE.Material,
    bufferGeometryList:THREE.BufferGeometry[],
    controls:OrbitControls,
    meshList:Array<THREE.Mesh>,
    ballAir:Ball,
    ballBrick:Ball,
  };
  stats?:Stats;
  modes:{
    isDebug:boolean,
    isAuto:boolean,
    isBall:boolean,
  };
  previousTime:number;
  constructor(){
    this.modes={
      isDebug:true,
      isAuto:true,
      isBall:false,
    };
    this.previousTime=0;
    this.setupPromise=this.setupAsync();
  }
  async setupStatsAsync(){
    const stats=new Stats();
    document.body.appendChild(stats.dom);
    stats.dom.style.display=this.modes.isDebug?"block":"none";
    this.stats=stats;

  }
  async setupVoxelAsync(){
    const universe=Universe.new();
    universe.update(0.0,V3F.new(Infinity,Infinity,Infinity));
    universe.draw();

    this.voxel={
      universe,
    }
  }
  async setupThreeAsync(){
    const renderer=new THREE.WebGLRenderer({
      // ogp画像を作成するときだけ有効化する
      // preserveDrawingBuffer: true,
      antialias: true,
      canvas:document.querySelector("#View") as HTMLCanvasElement,
    });
    renderer.shadowMap.enabled = true;
    renderer.setSize(window.innerWidth,window.innerHeight);
    renderer.setPixelRatio(window.devicePixelRatio);
    renderer.outputEncoding = THREE.sRGBEncoding;
    
    const scene = new THREE.Scene();
    const camera = new THREE.PerspectiveCamera(75, window.innerWidth / window.innerHeight, 0.01, 1000);
    camera.position.y = 5;
    camera.position.z = 50;
    const controls = new OrbitControls(camera, renderer.domElement);
    controls.autoRotate=this.modes.isAuto;
    controls.autoRotateSpeed=1.0;
  
    const ambientLight = new THREE.AmbientLight(0xffffff, 0.5);
    scene.add(ambientLight);
    const spotLight = new THREE.SpotLight(0xffffff, 1.2,1000,THREE.MathUtils.degToRad(45));
    spotLight.castShadow=true;
    spotLight.position.set(50, 50, 50);
    spotLight.lookAt(0,0,0);
    scene.add(spotLight);
  

    const loadEXRTextureAsync=(baseDir:string,filename:string)=>{
      return new Promise<THREE.Texture>((resolve)=>{
        new EXRLoader().setPath(baseDir).load(filename,(texture)=>{
          texture.encoding=THREE.LinearEncoding;
          texture.needsUpdate=true;
          resolve(texture);
        });
      });
    };
    const loadTextureAsync=(baseDir:string,filename:string,isLinear:boolean=true)=>{
      return new Promise<THREE.Texture>((resolve)=>{
        new THREE.TextureLoader().setPath(baseDir).load(filename,(texture)=>{
          texture.encoding=isLinear?THREE.LinearEncoding:THREE.sRGBEncoding;
          texture.needsUpdate=true;
          resolve(texture);
        });
      });
    };
    const setupTexture=(t:THREE.Texture)=>{
      // t.minFilter= THREE.NearestFilter;
      // t.magFilter= THREE.NearestFilter;
      // t.minFilter= THREE.LinearFilter;
      // t.magFilter= THREE.LinearFilter;
      t.generateMipmaps=true;
      t.needsUpdate=true;
    };


    const material=await(async()=>{
      const baseDir="./textures/packed/";
      const diff=await loadTextureAsync(baseDir,"packed_diff.jpg",false);
      // const disp=await loadTextureAsync(baseDir,"packed_disp.png");
      // const nor=await loadEXRTextureAsync(baseDir,"packed_nor_gl.exr");
      const nor=await loadTextureAsync(baseDir,"packed_nor_gl.png");
      // const rough=await loadEXRTextureAsync(baseDir,"packed_rough.exr");
      const rough=await loadTextureAsync(baseDir,"packed_rough.jpg");
      const metal=await loadTextureAsync(baseDir,"packed_metal.png");

      [diff,nor,rough,metal].forEach(setupTexture);
    
      const material=new THREE.MeshStandardMaterial({
        map:diff,
        roughnessMap:rough,
        metalnessMap:metal,
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
    const meshList=[];
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
      mesh.frustumCulled=false;
      mesh.receiveShadow=true;
      mesh.castShadow=true;
      const origin=universe.get_chunk_origin(i);
      mesh.position.set(origin.get_x(),origin.get_y(),origin.get_z());
      scene.add(mesh);
      meshList.push(mesh);

      const chunkSize=universe.get_chunk_size(i);
      const boundingBox=new THREE.Box3(new THREE.Vector3(0,0,0),new THREE.Vector3(chunkSize.get_x(),chunkSize.get_y(),chunkSize.get_z()));
      const boundingBoxHelper=new THREE.Box3Helper(boundingBox,new THREE.Color(0xff00ff));
      boundingBoxHelper.visible=this.modes.isDebug;
      mesh.add(boundingBoxHelper);
    }
    for(let i=0;i<l;i++){
      const bufferGeometry=bufferGeometryList[i];
  
      updateBufferGeometry(universe,i,bufferGeometry);
      console.log(bufferGeometry);
    }
  
    const loadingTextElement=document.querySelector(".loading-text") as HTMLElement;
    if(loadingTextElement){
      loadingTextElement.style.display="none";
    }

    const radius=5;
    const ballAir:Ball={
      mesh:new THREE.Mesh(new THREE.SphereGeometry(radius,32,16),new THREE.MeshStandardMaterial({
        color:0xff0000,
        metalness:0.5,
        roughness:0.5,
      })),
      fill:0xff,
      radius,
      velocity:new THREE.Vector3(13,11,17).multiplyScalar(1),
    };
    ballAir.mesh.visible=this.modes.isBall;
    scene.add(ballAir.mesh);

    const ballBrick:Ball={
      mesh:new THREE.Mesh(new THREE.SphereGeometry(radius,32,16),new THREE.MeshStandardMaterial({
        color:0x0000ff,
        metalness:0.5,
        roughness:0.5,
      })),
      fill:2,
      radius,
      velocity:new THREE.Vector3(-17,13,11).multiplyScalar(1),
    };
    ballBrick.mesh.visible=this.modes.isBall;
    scene.add(ballBrick.mesh);


    this.three={
      renderer,
      scene,
      camera,
      material,
      controls,
      bufferGeometryList,
      meshList,
      ballAir,
      ballBrick,
    };
  }
  async setupEventsAsync():Promise<void>{
    if(!this.three){
      throw new Error("this.three is null");
    }
    const {renderer}=this.three;

    renderer.setAnimationLoop( this.onRender.bind(this) );

    window.addEventListener("resize",this.onResize.bind(this));

    window.addEventListener("keydown",this.onKeyDown.bind(this));
    window.addEventListener("keyup",this.onKeyUp.bind(this));

    document.querySelector(".button--auto")?.addEventListener("click",()=>{
      this.toggleAuto();
    });
    document.querySelector(".button--debug")?.addEventListener("click",()=>{
      this.toggleDebug();
    });
    document.querySelector(".button--ball")?.addEventListener("click",()=>{
      this.toggleBall();
    });

  }
  async setupAsync():Promise<void>{
    await this.setupStatsAsync();
    await this.setupVoxelAsync();
    await this.setupThreeAsync();
    await this.setupEventsAsync();
  }
  onRender(time: DOMHighResTimeStamp, frame: XRFrame){
    if(!this.stats){
      throw new Error("this.stats is null");
    }
    this.stats.begin();
    if(!this.voxel){
      throw new Error("this.voxel is null");
    }
    const {universe}=this.voxel;
    if(!this.three){
      throw new Error("this.three is null");
    }

    const deltaTime=Math.min(time-this.previousTime,1000/60);
    const {renderer,scene,camera,bufferGeometryList,controls,ballAir,ballBrick}=this.three;

    if(this.modes.isBall){

      const universeSize=universe.get_size();
      const balls=[ballAir,ballBrick];
      for(let ball of balls){
        ball.mesh.position.add(ball.velocity.clone().multiplyScalar(deltaTime/1000));
        if( universeSize.get_x() * 0.5 < ball.mesh.position.x + ball.radius){
          ball.mesh.position.x =universeSize.get_x() * 0.5 - ball.radius;
          ball.velocity.x*=-1;
        }
        if( ball.mesh.position.x - ball.radius < universeSize.get_x() * -0.5){
          ball.mesh.position.x =universeSize.get_x() * - 0.5 + ball.radius;
          ball.velocity.x*=-1;
        }
        if( universeSize.get_y() * 0.5 < ball.mesh.position.y + ball.radius){
          ball.mesh.position.y =universeSize.get_y() * 0.5 - ball.radius;
          ball.velocity.y*=-1;
        }
        if( ball.mesh.position.y - ball.radius < universeSize.get_y() * -0.5){
          ball.mesh.position.y =universeSize.get_y() * - 0.5 + ball.radius;
          ball.velocity.y*=-1;
        }
        if( universeSize.get_z() * 0.5 < ball.mesh.position.z + ball.radius){
          ball.mesh.position.z =universeSize.get_z() * 0.5 - ball.radius;
          ball.velocity.z*=-1;
        }
        if( ball.mesh.position.z - ball.radius < universeSize.get_z() * -0.5){
          ball.mesh.position.z =universeSize.get_z() * - 0.5 + ball.radius;
          ball.velocity.z*=-1;
        }
        universe.update_add_sphere(V3F.new(ball.mesh.position.x,ball.mesh.position.y,ball.mesh.position.z),ball.radius,ball.fill);

      }
    }

    controls.update();
    universe.update(time * 0.001,V3F.new(camera.position.x,camera.position.y,camera.position.z));
    universe.draw();

    const l=universe.get_chunk_list_length();
    for(let i=0;i<l;i++){
      const bufferGeometry=bufferGeometryList[i];
  
      updateBufferGeometry(universe,i,bufferGeometry);
      // console.log(bufferGeometry);
    }
    renderer.render(scene,camera);
    this.stats.end();
    this.previousTime=time;
  }
  toggleAuto(){
    if(!this.three){
      throw new Error("this.three is null");
    }
    const {controls}=this.three;
    this.modes.isAuto=!this.modes.isAuto;
    controls.autoRotate=this.modes.isAuto;
  }
  toggleDebug(){
    if(!this.stats){
      throw new Error("this.stats is null");
    }
    if(!this.three){
      throw new Error("this.three is null");
    }
    this.modes.isDebug=!this.modes.isDebug;

    const {stats}=this;
    stats.dom.style.display=this.modes.isDebug?"block":"none";

    const {meshList}=this.three;
    for(let mesh of meshList){
      const boundingBoxHelper=mesh.children[0];
      if(boundingBoxHelper){
        boundingBoxHelper.visible=this.modes.isDebug;
      }
    }

  }
  toggleBall(){
    if(!this.three){
      throw new Error("this.three is null");
    }
    this.modes.isBall=!this.modes.isBall;
    const {ballAir,ballBrick}=this.three;

    ballAir.mesh.visible=this.modes.isBall;
    ballBrick.mesh.visible=this.modes.isBall;

  }
  onKeyDown(event:KeyboardEvent){
    switch(event.key){
      case "a":
        this.toggleAuto();
        break;
      case "d":
        this.toggleDebug();
        break;
      case "b":
        this.toggleBall();
        break;
    }
  }
  onKeyUp(event:KeyboardEvent){

  }
  onResize(event:UIEvent){
    if(this.three){
      const {camera,renderer}=this.three;
      renderer.setSize(window.innerWidth,window.innerHeight);
      renderer.setPixelRatio(window.devicePixelRatio);
      camera.aspect=window.innerWidth/window.innerHeight;
      camera.updateProjectionMatrix();

    }
  }
  async destroyStatsAsync(){
    const {stats}=this;
    if(stats){
      document.body.removeChild(stats.dom);
    }

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
    await this.destroyStatsAsync();
  }
}