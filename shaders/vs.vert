#version 140

in vec3 position;

uniform mat4 matrix;
uniform mat4 perspective;

void main() {
    gl_Position = perspective * matrix * vec4(position, 1.0);
}