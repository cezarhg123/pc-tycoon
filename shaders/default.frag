#version 460 core

in vec2 UV;
out vec4 FragColor;

uniform sampler2D tex;

void main() {
    vec4 color = texture(tex, UV);
    if (color.a <= 0.1) {
        discard;
    }
    FragColor = color;
}
