#version 450

layout( push_constant ) uniform ConstData {
  mat4 camera_projection;
  vec3 camera_position;
  mat4 mesh_transformation;
  mat3 normal_transformation;
} PushConstant;

layout(location=0) out vec4 f_color;

layout(binding=2)
uniform Uniforms {
    vec3 color;
};

void main() {
    f_color = vec4(color, 1.0);
}