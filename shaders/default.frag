#version 460

layout(location = 0) in vec4 f_color;
layout(location = 1) in vec2 f_uv;

layout(location = 0) out vec4 frag_color;

layout(binding = 0) uniform sampler2D tex;

void main() {
    vec4 color = texture(tex, f_uv) * f_color;
    
    frag_color = color;
}
