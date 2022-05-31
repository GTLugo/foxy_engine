use std::env;
//use shaderc::{CompilationArtifact, CompileOptions, ShaderKind, SourceLanguage};

fn main() {
  println!("{:?}", env::current_dir());
  //println!("cargo:rerun-if-changed=res/shaders/simple_shader.vert.hlsl");
  //println!("cargo:rerun-if-changed=res/shaders/simple_shader.frag.hlsl");
  //compile_shader_to_file(ShaderKind::Vertex, "simple_shader.vert.hlsl");
  //compile_shader_to_file(ShaderKind::Fragment, "simple_shader.frag.hlsl");
}

// fn compile_shader(shader_kind: ShaderKind, shader_file: &str) -> CompilationArtifact {
//   let shader_code = fs::read_to_string(shader_file).unwrap();
//
//   let compiler = shaderc::Compiler::new().unwrap();
//   let mut compile_options = CompileOptions::new().unwrap();
//   compile_options.set_source_language(SourceLanguage::HLSL);
//   let compiled_shader = compiler.compile_into_spirv(
//     shader_code.as_str(),
//     shader_kind,
//     shader_file,
//     "main",
//     Some(&compile_options),
//   ).expect(&*("Failed to compile shader: ".to_string() + shader_file));
//
//   compiled_shader
// }

// fn compile_shader_to_file(shader_kind: ShaderKind, shader_file_name: &str) {
//   let out_dir = env::var_os("OUT_DIR").unwrap();
//   let res_shaders = Path::new("res").join("shaders");
//   let shaders_out_dir = Path::new(&out_dir).join(&res_shaders);
//   fs::create_dir_all(shaders_out_dir).unwrap();
//
//   let shader_file_dir = Path::new(&res_shaders).join(shader_file_name);
//   let shader_file_out_name =
//     Path::new(shader_file_name).file_stem().unwrap().to_str().unwrap().to_string() + ".spv";
//   let shader_file_out_dir = Path::new(&out_dir).join(&res_shaders).join(shader_file_out_name);
//
//   // println!("out_dir: {:?}", out_dir);
//   // println!("shaders_resource_path: {:?}", shaders_resource_path);
//   // println!("shaders_out_path: {:?}", shaders_out_path);
//   // println!("shader_file_path: {:?}", shader_file_path);
//   // println!("shader_out_file_path: {:?}", shader_out_file_path);
//
//   let compiled = compile_shader(shader_kind, shader_file_dir.to_str().unwrap());
//   fs::write(&shader_file_out_dir, compiled.as_binary_u8()).expect(
//     &("Failed to write to file: ".to_string()
//     + shader_file_out_dir.to_str().unwrap())
//   );
// }