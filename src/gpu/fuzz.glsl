#version 460

struct bufferPod{
    int[4] chars;
};

struct outBufferPod{
    int[4] chars;
};

layout(local_size_x = 4, local_size_y = 1, local_size_z = 1) in;

layout(set = 0, binding = 0)  buffer BufferIn  {
    bufferPod[100] in_data;
} ;

layout(set = 0, binding = 1) buffer BufferOut  {
    outBufferPod[100] out_data;
} ;

void main() {
    uint idx = gl_GlobalInvocationID.x;
    out_data[idx].chars = in_data[idx].chars;

}
