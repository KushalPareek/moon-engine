#version 300 es

layout (location = 0) in vec3 aPosition;
layout (location = 1) in vec3 aColor;
layout (location = 2) in vec2 aTexCoord;
uniform float uTime;

out vec2 vTexCoord;
out vec3 vColor;

void main() {
    gl_Position = vec4(aPosition, 1.0);
    gl_Position.x += sin(uTime) * 0.1;
    gl_Position.y += cos(uTime) * 0.1;
    vColor = aColor;
    vTexCoord = aTexCoord;
    vTexCoord.y = 1.0 - vTexCoord.y;
}