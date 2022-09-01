// TestLibRtmp.cpp : 此文件包含 "main" 函数。程序执行将在此处开始并结束。
//

#include <iostream>
#include "librtmp_send264.h"

FILE* fp_send1;

int read_buffer1(unsigned char* buf, int buf_size) {
	if (!feof(fp_send1)) {
		int true_size = fread(buf, 1, buf_size, fp_send1);
		return true_size;
	}
	else {
		return -1;
	}
}

int main()
{
    std::cout << "Hello World!\n";

	fp_send1 = fopen("../../../data/test-25fps.h264", "rb");
	RTMP264_Connect("rtmp://localhost:1935/live/movie");
	RTMP264_Send(read_buffer1);
	RTMP264_Close();

	return 0;
}

