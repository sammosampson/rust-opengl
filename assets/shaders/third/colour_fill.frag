#version 330 core

in GM_OUT 
{
    vec2 texture_coord;
    vec4 inner_colour;
    vec4 outer_colour;
    vec2 extra_data;
} fs_in;

out vec4 Color;

void main()
{
    Color = fs_in.inner_colour;
}