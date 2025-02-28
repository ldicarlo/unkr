#version 460

struct bufferPod{
    vec4 chars;
};

struct outBufferPod{
    vec4 chars;
};

layout(local_size_x = 4, local_size_y = 1, local_size_z = 1) in;

layout(set = 0, binding = 0) readonly buffer BufferIn  {
    bufferPod[100] in_data;
};

layout(set = 0, binding = 1) buffer BufferOut  {
    outBufferPod[100] out_data;
};

void main() {
    uint idx = gl_GlobalInvocationID.x;
    int a[]=in_data[idx].chars;
    int b[]= out_data[idx].chars;
    out_data[idx].chars = in_data[idx].chars;

}
