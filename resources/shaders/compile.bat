glslangValidator.exe -V .\planetgen.vert -o planetgen.vert.spv
glslangValidator.exe -V .\planetgen.frag -o planetgen.frag.spv

glslangValidator.exe -V ..\..\src\modes\playgame\render\passes\terrain_pass\terrain.frag -o ..\..\src\modes\playgame\render\passes\terrain_pass\terrain.frag.spv
glslangValidator.exe -V ..\..\src\modes\playgame\render\passes\terrain_pass\terrain.vert -o ..\..\src\modes\playgame\render\passes\terrain_pass\terrain.vert.spv

glslangValidator.exe -V ..\..\src\modes\playgame\render\passes\model_pass\models.frag -o ..\..\src\modes\playgame\render\passes\model_pass\models.frag.spv
glslangValidator.exe -V ..\..\src\modes\playgame\render\passes\model_pass\models.vert -o ..\..\src\modes\playgame\render\passes\model_pass\models.vert.spv

glslangValidator.exe -V ..\..\src\modes\playgame\render\passes\grass_pass\grass.frag -o ..\..\src\modes\playgame\render\passes\grass_pass\grass.frag.spv
glslangValidator.exe -V ..\..\src\modes\playgame\render\passes\grass_pass\grass.vert -o ..\..\src\modes\playgame\render\passes\grass_pass\grass.vert.spv

glslangValidator.exe -V ..\..\src\modes\playgame\render\passes\vox_pass\vox.frag -o ..\..\src\modes\playgame\render\passes\vox_pass\vox.frag.spv
glslangValidator.exe -V ..\..\src\modes\playgame\render\passes\vox_pass\vox.vert -o ..\..\src\modes\playgame\render\passes\vox_pass\vox.vert.spv

glslangValidator.exe -V ..\..\src\modes\playgame\render\passes\lighting_pass\lighting.frag -o ..\..\src\modes\playgame\render\passes\lighting_pass\lighting.frag.spv
glslangValidator.exe -V ..\..\src\modes\playgame\render\passes\lighting_pass\lighting.vert -o ..\..\src\modes\playgame\render\passes\lighting_pass\lighting.vert.spv

glslangValidator.exe -V ..\..\src\modes\playgame\render\passes\cursors_pass\cursors.frag -o ..\..\src\modes\playgame\render\passes\cursors_pass\cursors.frag.spv
glslangValidator.exe -V ..\..\src\modes\playgame\render\passes\cursors_pass\cursors.vert -o ..\..\src\modes\playgame\render\passes\cursors_pass\cursors.vert.spv
