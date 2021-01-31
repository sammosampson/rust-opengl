#version 330 core

layout(points) in;
layout(triangle_strip, max_vertices = 4) out;

in VS_OUT
{
    vec2 dimensions;
    vec3 extra_data;
} gm_in[];

out GM_OUT 
{
    vec2 texture_coord;
    vec3 extra_data;
} gm_out;

void createVertex(vec2 pos, vec2 scale, vec2 corner, float u, float v) {
    vec2 scaled = scale * corner;
    vec2 transformed = pos + scaled;
    gl_Position = vec4(transformed, 0.0, 1.0);
    gm_out.texture_coord = vec2(u, v);
    EmitVertex();
}

void main()
{
    vec2 pos = gl_in[0].gl_Position.xy;;
    vec2 size = gm_in[0].dimensions; 

    mat3 scale = mat3(
        size.x, 0.0, 0.0,
        0.0, size.y, 0.0,
        0.0, 0.0, 1.0
    );

    float one = 1.0;
    vec2 bottomLeft = vec2(-one, -one);
    vec2 bottomRight = vec2(one, -one);
    vec2 topLeft = vec2(-one, one);
    vec2 topRight = vec2(one, one);
    
    gm_out.extra_data = gm_in[0].extra_data;
    createVertex(pos, size, bottomLeft, 0.0, 1.0);
    gm_out.extra_data = gm_in[0].extra_data;
    createVertex(pos, size, bottomRight, 1.0, 1.0);
    gm_out.extra_data = gm_in[0].extra_data;
    createVertex(pos, size, topLeft, 0.0, 0.0);
    gm_out.extra_data = gm_in[0].extra_data;
    createVertex(pos, size, topRight, 1.0, 0.0);

    EndPrimitive();
}