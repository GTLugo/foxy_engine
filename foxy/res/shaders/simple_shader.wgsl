struct VertexInput {
  [[location(0)]] position: vec3<f32>;
  [[location(1)]] color: vec4<f32>;
};

struct FragmentInput {
  [[builtin(position)]] position: vec4<f32>;
  [[location(0)]] color: vec4<f32>;
};

[[stage(vertex)]]
fn vs_main(
  vs_in: VertexInput
) -> FragmentInput {
  var vs_out: FragmentInput;
  //vs_out.position = vs_in.position;
  vs_out.position = vec4<f32>(vs_in.position, 1.0);
  vs_out.color = vs_in.color;
  return vs_out;
}

[[stage(fragment)]]
fn fs_main(
  fs_in: FragmentInput
) -> [[location(0)]] vec4<f32> {
  return fs_in.color;
}