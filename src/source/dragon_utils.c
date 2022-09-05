#include "../headers/dragon_utils.h"

#include "../headers/file_utils.h"
#include <malloc.h>



RCODE	set_battery_threshold(FILE * handle, unsigned char threshold) {
	if(threshold > 100)
		threshold = 100;

	unsigned char effective_threshold = 0x80 + threshold;
	unsigned char * value = (unsigned char *)malloc(sizeof(char));
	*value = effective_threshold;
	RCODE res = write_ec(handle, BATTERY_THRESHOLD, value, 1);
	free(value);
	return res;
}

unsigned char get_battery_threshold(FILE * handle) {
	unsigned char value;
	RCODE res = read_ec(handle, BATTERY_THRESHOLD, &value, 1);
	if(res != RC_OK) 
		return -1;
	return value - 0x80;
}

