#version 330 core
layout (location = 0) in vec3 aPos;
layout (location = 1) in vec2 aTexCoord;
layout (location = 2) in vec4 backgroundColor;
layout (location = 3) in vec4 foregroundColor;

out vec2 TexCoord;
out vec4 BackgroundColor;
out vec4 ForegroundColor;

void main()
{
    gl_Position = vec4(aPos, 1.0);
    TexCoord = aTexCoord;
    BackgroundColor = backgroundColor;
    ForegroundColor = foregroundColor;
}