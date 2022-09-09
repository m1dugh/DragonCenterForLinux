#include "headers/env.h"
#include "headers/file_utils.h"
#include "headers/dragon_utils.h"
#include <stdio.h>
#include <stdlib.h>

int main(int argc, char ** argv) {
	FILE * handle = open_ec();

	// unsigned short rpm = get_fan_rpm(handle, GPU_FAN);
	// printf("rpm: %d\n", rpm);
	printf("cpu temp: %d\n", get_cpu_temp(handle));
	set_cooler_boost(handle, 0x02);

	close_ec(handle);
	return 0;
}

