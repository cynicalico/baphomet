#version 330
layout (location = 0) in vec3 aPos;
layout (location = 1) in vec3 aColor;

out vec3 bColor;

uniform mat4 mvp;

void main() {
    bColor = aColor;
    gl_Position = mvp * vec4(aPos.x, aPos.y, aPos.z, 1.0);
}
