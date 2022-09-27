#version 330 core

uniform vec3 u_camPosition;

in vec3 VertexColor;
in vec3 VertexPos;

out vec4 FragColor;

void main () {

    float fade_sigma = 10.*min(abs(u_camPosition.y/5.)+2., 5.);

    float fade = smoothstep(fade_sigma, 0., length(u_camPosition.xz+VertexPos.xz));
    FragColor = vec4(VertexColor, fade);
}
