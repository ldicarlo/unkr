#version 460

struct bufferPod{
    uvec2 chars;
};

layout(local_size_x = 1, local_size_y = 1, local_size_z = 1) in;

layout(set = 0, binding = 0)  buffer BufferPod  {
    bufferPod data[];
} buf;


void main() {
    uint idx = gl_GlobalInvocationID.x;
    // uvec2 newstr[4];

    // newstr[1].x = 1;
    // newstr[1].y = 1;
    //uvec2 q = uvec2(1,1);

    //  for (int i = 0; i < 4; i++){

     buf.data[idx].chars.x += 1; // buf.data[idx].string[4-i-1].x;


    //}

    //outpod.data[idx] = inputPod (newstr);
}
