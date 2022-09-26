#version 330 core

layout (location = 0) in vec3 a_Pos;
layout (location = 1) in vec3 a_Color;

uniform mat4 u_view;
uniform mat4 u_projection;

out vec3 VertexPos;
out vec3 VertexColor;

void main () {
    VertexPos = a_Pos;
    VertexColor = a_Color;
    gl_Position = u_projection * u_view * vec4(a_Pos, 1.);
}
