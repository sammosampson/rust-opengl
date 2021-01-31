#version 330 core

uniform samplerBuffer inner_colour_buffer;
uniform samplerBuffer outer_colour_buffer;
uniform samplerBuffer inner_radius_buffer;
float smoothness = 0.002;

in vec2 textureCoord;

out vec4 Color;

float circle_signed_dist(vec2 position, float radius) 
{
    return length(position) - radius;
}

void main()
{
    vec3 inner_colour = texelFetch(inner_colour_buffer, 0).rgb;
    vec3 outer_colour = texelFetch(outer_colour_buffer, 0).rgb;
    float stroke_width = texelFetch(inner_radius_buffer, 0).r;

    float outer_radius = 0.5;
    float dist = circle_signed_dist(textureCoord - 0.5, outer_radius);
	
    float outer = smoothstep(smoothness, -smoothness, dist);
    float inner = smoothstep(-stroke_width + smoothness, -stroke_width - smoothness, dist);
    float alpha = smoothstep(0.00, -smoothness, dist);
    
	vec3 current_colour = mix(outer_colour, inner_colour, inner) * outer;
    
    Color = vec4(current_colour, alpha);
}