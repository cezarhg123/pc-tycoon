#version 460 core

layout (location = 0) in vec2 aPos;
layout (location = 1) in vec2 aUV;

out vec2 UV;

void main() {
    UV = aUV;

    // program draws from the top-left of the screen in pixels
    // these calculations change that to the standard of opengl (draws from the center of the screen from -1.0 to 1.0)
    vec2 pos;
    pos.x = aPos.x / 960.0;
    pos.x -= 1.0;

    pos.y = aPos.y / 540.0;
    pos.y -= 1.0;
    pos.y *= -1.0;

    gl_Position = vec4(pos, 0.0, 1.0);
}
