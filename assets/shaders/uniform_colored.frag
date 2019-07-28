#version 330 core

uniform vec4 uniformColor;
out vec4 Color;

void main()
{
    Color = uniformColor;
}