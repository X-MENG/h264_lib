#pragma once
#ifdef __cplusplus
extern "C"
{
#endif

int RTMP264_Connect(const char* url);    

int RTMP264_Send(int (*read_buffer)(unsigned char *buf, int buf_size));
  
void RTMP264_Close();  

#ifdef __cplusplus
}
#endif
