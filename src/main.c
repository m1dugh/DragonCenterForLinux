#include "headers/env.h"
#include "headers/file_utils.h"
#include "headers/dragon_utils.h"
#include <stdio.h>
#include <stdlib.h>

int main(int argc, char ** argv) {
	FILE * handle = open_ec();

	unsigned char bt = get_battery_threshold(handle);
	printf("result is %d\n", bt);

	close_ec(handle);
	return 0;
}

