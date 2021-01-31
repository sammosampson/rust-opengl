#version 330 core

vec3 outerColor = vec3(0.0, 0.0, 0.0);
vec3 innerColor = vec3(0.5, 0.5, 0.5);
float innerRadius = 0.45;
float smoothness = 0.002;

in VS_OUTPUT {
    vec2 TextureCoord;
} IN;

out vec4 Color;

void main()
{
    float outerRadius = 0.5;
    vec2 uv = IN.TextureCoord - 0.5;
    float dist = length(uv);
	float outer = smoothstep(outerRadius + smoothness, outerRadius - smoothness, dist);
    float inner = smoothstep(innerRadius + smoothness, innerRadius - smoothness, dist);
    float alpha = smoothstep(outerRadius, outerRadius - smoothness, dist);
    
	vec3 currentColor = mix(outerColor, innerColor, inner) * outer;
    
    Color = vec4(currentColor, alpha);
}