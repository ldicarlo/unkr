#version 460

struct inputPod{
    uvec2 string[4] ;
};


layout(local_size_x = 64, local_size_y = 1, local_size_z = 1) in;

layout(set = 0, binding = 0)  buffer InputPod  {
    inputPod data[];
} buf;

void main() {
    uint idx = gl_GlobalInvocationID.x;
    // uvec2 newstr[4];

    // newstr[1].x = 1;
    // newstr[1].y = 1;


  //  for (int i = 0; i < 4; i++){

    buf.data[idx].string[0].y = 1; // buf.data[idx].string[4-i-1].x;
    buf.data[idx].string[1].y = 1;
    buf.data[idx].string[2].y = 1;
    buf.data[idx].string[3].y = 1;
    //}

    //outpod.data[idx] = inputPod (newstr);
}
