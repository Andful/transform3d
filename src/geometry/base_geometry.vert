#version 450

layout(push_constant) uniform ConstData {
  mat4 camera_projection;
  vec3 camera_position;
  mat4 mesh_transformation;
  mat3 normal_transformation;
} PushConstant;

layout(location=0) in vec3 a_position;

void main() {
  gl_Position = vec4(a_position, 1.0);
}