#version 330 core

uniform vec2 uResolution;
//vec2 uResolution = vec2(1800.0, 1400.0);

layout (location = 0) in vec2 Position;
layout (location = 1) in vec2 Dimensions;
layout (location = 2) in vec3 ExtraData;

out VS_OUT
{
    vec2 dimensions;
    vec3 extra_data;
} vs_out;

vec2 toClipSpace(vec2 from)
{
    return vec2(
        from.x / (uResolution.x / 2.0) - 1.0,
        1.0 - (from.y / (uResolution.y / 2.0))
    );
}

void main()
{
    gl_Position = vec4(toClipSpace(Position), 0.0, 1.0);
    vs_out.dimensions = Dimensions / uResolution.xy;
    vs_out.extra_data = ExtraData;
}