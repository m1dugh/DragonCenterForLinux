#include "headers/env.h"
#include "headers/file_utils.h"
#include <stdio.h>
#include <stdlib.h>

int main(int argc, char ** argv) {
	FILE * handle = open_ec();
	unsigned char * value = (unsigned char *) malloc(sizeof(char));
	value[0]=0x0;
	write_ec(handle, COOLER_BOOST, value, 1);
	close_ec(handle);
	return 0;
}

