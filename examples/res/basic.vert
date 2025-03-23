#version 330
in vec3 aPos;
in vec3 aColor;
in vec3 aRot;

out vec3 bColor;

uniform mat4 proj;

void main() {
    float c = cos(aRot.z);
    float s = sin(aRot.z);
    float xtr = -aRot.x * c + aRot.y * s + aRot.x;
    float ytr = -aRot.x * s - aRot.y * c + aRot.y;

    mat4 rot = mat4(
        vec4(c,   s,   0.0, 0.0),
        vec4(-s,  c,   0.0, 0.0),
        vec4(0.0, 0.0, 1.0, 0.0),
        vec4(xtr, ytr, 0.0, 1.0)
    );

    bColor = aColor;
    gl_Position = proj * rot * vec4(aPos, 1.0);
}
