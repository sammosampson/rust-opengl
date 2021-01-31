#version 330 core

uniform vec2 iResolution;

layout (location = 0) in vec3 Position;
layout (location = 1) in vec4 Color;

void main()
{
    vec3 ClipSpacePosition = vec3(
        Position.x / (iResolution.x / 2.0) - 1.0,
        1.0 - (Position.y / (iResolution.y / 2.0)),
        Position.z);

    gl_Position = vec4(ClipSpacePosition, 1.0);
}