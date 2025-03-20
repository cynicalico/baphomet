#version 330
in vec3 aPos;
in vec3 aColor;

out vec3 bColor;

uniform mat4 mvp;

void main() {
    bColor = aColor;
    gl_Position = mvp * vec4(aPos.x, aPos.y, aPos.z, 1.0);
}
