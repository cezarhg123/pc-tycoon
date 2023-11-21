#version 460

layout(location = 0) in vec2 v_pos;
layout(location = 1) in vec3 v_color;
layout(location = 2) in vec2 v_uv;

layout(location = 0) out vec3 f_color;
layout(location = 1) out vec2 f_uv;

void main() {
    gl_Position = vec4(v_pos, 0.0, 1.0);
    f_color = v_color;
    f_uv = v_uv;
}
