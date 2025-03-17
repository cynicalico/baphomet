#version 330
layout (location = 0) in vec3 aPos;

uniform float x_off;
uniform float y_off;
uniform mat4 mvp;

void main() {
    gl_Position = mvp * vec4(aPos.x + x_off, aPos.y + y_off, aPos.z, 1.0);
}
