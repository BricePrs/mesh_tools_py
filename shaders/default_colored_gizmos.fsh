#version 330 core

uniform vec3 u_camPosition;

in vec3 VertexColor;
in vec3 VertexPos;

out vec4 FragColor;

void main () {

    float fade_dist = 0.;
    float fade_sigma = 50.;

    float fade = smoothstep(fade_dist+fade_sigma, fade_dist, length(u_camPosition+VertexPos));
    FragColor = vec4(VertexColor, fade);
}
