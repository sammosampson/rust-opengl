#version 330 core

in VS_OUTPUT {
    vec2 TextureCoord;
} IN;

out vec4 Color;

void main()
{
    Color = vec4(IN.TextureCoord, 0.0, 1.0);
}