
const { execSync } = require('child_process');

const size="1k";
const material_list=[
  "coast_sand_rocks_02",
  "metal_plate",
  "red_brick_03",
  "stone_brick_wall_001",
  "rock_boulder_cracked",
];
const texture_params_list=[
  {
    name:"diff",
    ext:"jpg",
  },
  // {
  //   name:"disp",
  //   ext:"png",
  // },
  {
    name:"nor_gl",
    ext:"exr",
  },
  // {
  //   name:"rough",
  //   ext:"exr",
  // },
];


for (let texture_params of texture_params_list){
  const files=material_list.map((material)=>`src/${material}_${size}/${material}_${texture_params.name}_${size}.${texture_params.ext}`);
  let option_for_ext="";
  switch(texture_params.ext){
    case "exr":
      option_for_ext="-compress dwaa";
      break;
  }
  const result = execSync(`montage -tile 4x4 -geometry 1024x1024 ${files.join(" ")} ${option_for_ext} dist/${texture_params.name}.${texture_params.ext}`);
  console.log(result.toString());
}


