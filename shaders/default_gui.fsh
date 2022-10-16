#version 450 core

#define texture_count 20

in vec2 uv;

out vec4 FragColor;

uniform sampler2D u_textures[texture_count];
uniform ivec2 u_texture_pos[texture_count];

uniform ivec2 u_resolution;

void main() {
    FragColor = vec4(0.);
    ivec2 iCoords = ivec2(u_resolution * uv);
    for (int i = 0; i < texture_count; i++) {
        ivec2 tex_size = textureSize(u_textures[i], 0);
        ivec2 delta_coords = (iCoords - u_texture_pos[i]);
        if (delta_coords/tex_size == ivec2(0)) {
            FragColor = texelFetch(u_textures[i], delta_coords, 0);
        }
    }
}
