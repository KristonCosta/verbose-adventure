#version 330 core
out vec4 FragColor;

in vec2 TexCoord;
in vec4 BackgroundColor;
in vec4 ForegroundColor;

uniform sampler2D texture1;


void main()
{
    vec4 texColor = texture(texture1, TexCoord);
    if (texColor.a < 0.1) {
        if (BackgroundColor.a < 0.1) {
            discard;
        }
        FragColor = BackgroundColor;
    } else {
        if (ForegroundColor.a < 0.1) {
            FragColor = texColor;
        } else {
            FragColor = texColor * ForegroundColor;
        }

    }
}

