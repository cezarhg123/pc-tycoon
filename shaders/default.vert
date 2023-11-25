#version 460

layout(location = 0) in vec2 v_pos;
layout(location = 1) in vec2 v_uv;

layout(location = 0) out vec3 f_color;
layout(location = 1) out vec2 f_uv;

layout(binding = 1) uniform Rect {
    vec2 position;
    vec3 color;
    vec2 size;
};

void main() {
    vec2 actual_position = vec2((v_pos * size) + position);
    gl_Position = vec4(actual_position, 0.0, 1.0);
    f_color = color;
    f_uv = v_uv;
}
