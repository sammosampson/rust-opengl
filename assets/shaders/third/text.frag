#version 330 core

uniform samplerBuffer inner_colour_buffer;
uniform sampler2DArray font_buffer;
float smoothness = 0.002;

in GM_OUT 
{
    vec2 texture_coord;
    vec3 extra_data;
} fs_in;
out vec4 Color;

float median(float r, float g, float b) {
    return max(min(r, g), min(max(r, g), b));
}

void main()
{
    vec3 colour = texelFetch(inner_colour_buffer, 0).rgb;
    vec3 sample = texture(font_buffer, vec3(fs_in.texture_coord, fs_in.extra_data.r)).rgb;
    float dist = median(sample.r, sample.g, sample.b);
    float width = fwidth(dist);
    float opacity = smoothstep(0.5 - width, 0.5 + width, dist);
    Color = vec4(colour, opacity);
    //Color = vec4(sample, 1.0);
}