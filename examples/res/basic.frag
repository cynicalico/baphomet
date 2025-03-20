#version 330 core
in vec3 bColor;

out vec4 FragColor;

void main() {
    FragColor = vec4(bColor, 1.0f);
}
