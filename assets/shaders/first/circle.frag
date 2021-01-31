#version 330 core

//uniform vec2 iResolution;
vec2 iResolution = vec2(1800.0, 1400.0);
vec3 outerColor = vec3(1.0, 0.0, 0.0);
vec3 innerColor = vec3(1.0);
float outerRadius = 0.3;
float innerRadius = 0.3;
float smoothness = 0.001;

out vec4 Color;

void main()
{
    vec2 uv = gl_FragCoord.xy / iResolution.xy;

    uv -= .5;

    uv.x *= iResolution.x / iResolution.y;

    float dist = length(uv);
	float outer = smoothstep(outerRadius + smoothness, outerRadius - smoothness, dist);
    float inner = smoothstep(innerRadius + smoothness, innerRadius - smoothness, dist);
    float alpha = smoothstep(outerRadius, outerRadius - smoothness, dist);
    
	vec3 currentColor = mix(outerColor, innerColor, inner) * outer;
    
    Color = vec4(currentColor, alpha);
}