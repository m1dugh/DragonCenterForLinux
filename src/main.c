#include "headers/env.h"
#include "headers/file_utils.h"
#include "headers/dragon_utils.h"
#include <stdio.h>
#include <stdlib.h>

int main(int argc, char ** argv) {
	FILE * handle = open_ec();

	unsigned char bt = get_battery_threshold(handle);
	printf("result is %d\n", bt);
	printf("mode is %d\n", get_mode(handle));
	set_mode(handle, FAN_MODE_ADVANCED);

	bt = get_cooler_boost(handle);
	double pr = ((double)bt)/((double)0x80);
	printf("cooler boost: %f\n", pr);

	close_ec(handle);
	return 0;
}

