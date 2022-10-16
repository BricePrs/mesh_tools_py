#version 450 core

layout (location = 0) in vec3 a_pos;

out vec2 uv;

void main() {
    gl_Position = vec4(a_pos.x, a_pos.y, -1., 1.);
    uv = vec2(a_pos.x, a_pos.y)*.5+.5;
}
