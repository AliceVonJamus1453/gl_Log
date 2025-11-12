#version 330 core

const float PI = 3.14159265359;
layout (location = 0) in vec3 Position;
layout (location = 1) in vec3 Color;

out VS_OUTPUT {
    vec3 Color;
} OUT;

mat3 Rodrigues(vec3 axis, float angle) {
    mat3 axis_mat = mat3(
        axis.x, 0.0, 0.0,
        axis.y, 0.0, 0.0,
        axis.z, 0.0, 0.0
    );
    mat3 result = mat3(0.0);
    result += cos(angle) * mat3(1.0);
    result += (1 - cos(angle)) * axis_mat * transpose(axis_mat);
    result += sin(angle) * mat3(
        0, -axis.z, axis.y,
        axis.z, 0, -axis.x,
        -axis.y, axis.x, 0
    );
    return result;
}

void main()
{
    // mat3 rotation = Rodrigues(vec3(0.0, 0.0, 0.0), PI);
    // vec3 NewPosition = rotation * Position;
    gl_Position = vec4(Position, 1.0);
    OUT.Color = Color;
}
