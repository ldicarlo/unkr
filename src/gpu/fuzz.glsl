#version 460

struct bufferPod{
    uint[4] chars;
};

struct outBufferPod{
    uint[4] chars;
};

layout(local_size_x = 1, local_size_y = 1, local_size_z = 1) in;

layout(set = 0, binding = 0)  buffer BufferIn  {
    bufferPod data[100];
} buf_in;

layout(set = 0, binding = 1) buffer BufferOut  {
    outBufferPod data[100];
} buf_out;

void main() {
    uint idx = gl_GlobalInvocationID.x;
     for (uint i = 0; i<4; i++){
        buf_out.data[idx].chars[i] = buf_in.data[idx].chars[i+1];
     }
}
