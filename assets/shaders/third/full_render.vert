#version 330 core

uniform vec2 uResolution;

layout (location = 0) in ivec2 Position;
layout (location = 1) in ivec2 Dimensions;
layout (location = 2) in vec4 InnerColour;
layout (location = 3) in vec4 OuterColour;
layout (location = 4) in ivec2 Identification;
layout (location = 5) in vec4 ExtraData1;
layout (location = 6) in vec4 ExtraData2;

out VS_OUT
{
    vec2 dimensions;
    vec4 inner_colour;
    vec4 outer_colour;
    flat ivec2 identification;
    vec4 extra_data_1;
    vec4 extra_data_2;
} vs_out;

vec2 toClipSpace(vec2 resolution, vec2 from)
{
    return vec2(
        from.x / (resolution.x / 2.0) - 1.0,
        1.0 - (from.y / (resolution.y / 2.0))
    );
}

void main()
{
    gl_Position = vec4(toClipSpace(uResolution, vec2(Position)), 0.0, 1.0);
    vs_out.dimensions = Dimensions;
    vs_out.inner_colour = InnerColour;
    vs_out.outer_colour = OuterColour;
    vs_out.identification = Identification;
    vs_out.extra_data_1 = ExtraData1;
    vs_out.extra_data_2 = ExtraData2;
}