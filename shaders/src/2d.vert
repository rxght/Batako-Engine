#version 450

layout(location = 0) in vec2 position;

layout( push_constant ) uniform Pc {
    vec2 scaling;
};

layout( set = 0, binding = 0 ) uniform GlobalUbo {
    mat4 view_projection;
};

void main()
{
    gl_Position =  view_projection * vec4(scaling * position, 0.0f, 1.0f);
}