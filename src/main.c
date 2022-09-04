#include "headers/env.h"
#include "headers/file_utils.h"
#include "headers/dragon_utils.h"
#include <stdio.h>
#include <stdlib.h>

int main(int argc, char ** argv) {
	FILE * handle = open_ec();

	RCODE result = set_battery_threshold(handle, 100);
	printf("result is %d\n", result);

	close_ec(handle);
	return 0;
}

