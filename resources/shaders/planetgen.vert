#version 450

layout(location=0) in vec3 a_position;
layout(location=1) in vec4 a_color;

layout(location=0) out vec4 v_color;

uniform mat4 u_view_proj;
uniform float rot_angle;

mat4 rotation3d(vec3 axis, float angle) {
  axis = normalize(axis);
  float s = sin(angle);
  float c = cos(angle);
  float oc = 1.0 - c;

  return mat4(
		oc * axis.x * axis.x + c,           oc * axis.x * axis.y - axis.z * s,  oc * axis.z * axis.x + axis.y * s,  0.0,
        oc * axis.x * axis.y + axis.z * s,  oc * axis.y * axis.y + c,           oc * axis.y * axis.z - axis.x * s,  0.0,
        oc * axis.z * axis.x - axis.y * s,  oc * axis.y * axis.z + axis.x * s,  oc * axis.z * axis.z + c,           0.0,
		0.0,                                0.0,                                0.0,                                1.0
	);
}

void main() {
    gl_Position = u_view_proj * rotation3d(vec3(0.0, 1.0, 0.0), rot_angle) * vec4(a_position, 1.0);
    v_color = a_color;
}