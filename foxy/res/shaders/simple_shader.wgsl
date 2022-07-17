struct FragmentInput {
  [[builtin(position)]] position: vec4<f32>;
  [[location(0)]] color: vec4<f32>;
};

[[stage(vertex)]]
fn vert_main(
  [[location(0)]] position: vec3<f32>,
  [[location(1)]] color: vec4<f32>,
) -> FragmentInput {
  var vs_out: FragmentInput;
  vs_out.position = vec4<f32>(position, 1.0);
  vs_out.color = color;
  return vs_out;
}

[[stage(fragment)]]
fn frag_main(
  fs_in: FragmentInput,
) -> [[location(0)]] vec4<f32> {
  return fs_in.color;
}