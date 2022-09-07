#include "../headers/dragon_utils.h"

#include "../headers/file_utils.h"
#include <malloc.h>
#include <stdio.h>


#define ERROR(function, message) fprintf(stderr, "%s: %s\n", function, message);

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

RCODE set_mode(FILE * handle, unsigned char mode) {
	if(mode != FAN_MODE_ADVANCED &&
			mode!= FAN_MODE_BASIC &&
			mode != FAN_MODE_AUTO &&
			mode != FAN_MODE_CURRENT) {
		ERROR("set_mode", "provided mode is not allowed");
		return RC_FAILED;
	}

	RCODE res = write_ec(handle, FAN_MODE, &mode, 1);
	return res;
}

unsigned char get_mode(FILE * handle) {
	unsigned char result;
	RCODE res = read_ec(handle, FAN_MODE, &result, 1);
	if(res != RC_OK) {
		ERROR("get_mode", "an error occured while reading ec");
		return -1;
	}

	return result;
}

RCODE set_cooler_boost(FILE * handle, unsigned char value) {
	if(value > 0x80)
		value = 0x80;
	if(write_ec(handle, COOLER_BOOST, &value, 1) != RC_OK) {
		ERROR("set_cooler_boost", "write_ec: an error occured");
		return RC_FAILED;
	}

	return RC_OK;
}

unsigned char get_cooler_boost(FILE * handle) {
	unsigned char value;
	if(read_ec(handle, COOLER_BOOST, &value, 1) != RC_OK) {
		ERROR("get_cooler_boost", "error in read_ec");
		return -1;
	}

	return value;
}


uint8_t get_gpu_temp(FILE * handle) {
	uint8_t result;
	if(read_ec(handle, GPU_TEMP, &result, 1) != RC_OK) {
		ERROR("get_gpu_temp", "error in read_ec");
		return -1;
	}

	return result;
}


uint8_t get_cpu_temp(FILE * handle) {
	uint8_t result;
	if(read_ec(handle, CPU_TEMP, &result, 1) != RC_OK) {
		ERROR("get_cpu_temp", "error in read_ec");
		return -1;
	}

	return result;
}

