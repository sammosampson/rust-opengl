#version 330 core

uniform samplerBuffer inner_colour_buffer;
uniform samplerBuffer outer_colour_buffer;
uniform samplerBuffer stroke_width_buffer;
uniform samplerBuffer corner_radii_buffer;
float smoothness = 0.002;

in GM_OUT 
{
    vec2 texture_coord;
    vec3 extra_data;
} fs_in;
out vec4 Color;

float box_signed_dist(in vec2 position, in vec4 corner_radii)
{
    vec2 bounds = vec2(0.5);
    vec2 quadrant_position = step(vec2(0.5), position);
    int corner_radius_index = int(quadrant_position.x) + int(quadrant_position.y) * 2;
    float corner_radius = corner_radii[corner_radius_index];

    vec2 centred_position = position - 0.5;    
    vec2 offset = abs(centred_position) - bounds + corner_radius;
    return min(max(offset.x, offset.y), 0.0) + length(max(offset, 0.0)) - corner_radius;
}

void main()
{
    vec3 inner_colour = texelFetch(inner_colour_buffer, 0).rgb;
    vec3 outer_colour = texelFetch(outer_colour_buffer, 0).rgb;
    float stroke_width = texelFetch(stroke_width_buffer, 0).r;
    vec4 corner_radii = texelFetch(corner_radii_buffer, 0).rgba;

    float dist = box_signed_dist(fs_in.texture_coord, corner_radii);
	
    float outer = smoothstep(smoothness, -smoothness, dist);
    float inner = smoothstep(-stroke_width + smoothness, -stroke_width - smoothness, dist);
    float alpha = smoothstep(0.0, -smoothness, dist);
    
	vec3 current_colour = mix(outer_colour, inner_colour, inner) * outer;
    
    Color = vec4(current_colour, alpha);
}