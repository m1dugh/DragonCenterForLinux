#include "./dragon_utils.h"

#include "../utils/file_utils.h"
#include <malloc.h>
#include <stdio.h>


#define ERROR(function, message, rc) fprintf(stderr, "%s: %s\n", function, message); return rc;

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
		ERROR("set_mode", "provided mode is not allowed", RC_FAILED)
	}

	RCODE res = write_ec(handle, FAN_MODE, &mode, 1);
	return res;
}

unsigned char get_mode(FILE * handle) {
	unsigned char result;
	RCODE res = read_ec(handle, FAN_MODE, &result, 1);
	if(res != RC_OK) {
		ERROR("get_mode", "an error occured while reading ec", -1);
	}

	return result;
}

RCODE set_cooler_boost(FILE * handle, unsigned char value) {
	if(value > 0x80)
		value = 0x80;
	if(write_ec(handle, COOLER_BOOST, &value, 1) != RC_OK) {
		ERROR("set_cooler_boost", "write_ec: an error occured", RC_FAILED);
	}

	return RC_OK;
}

unsigned char get_cooler_boost(FILE * handle) {
	unsigned char value;
	if(read_ec(handle, COOLER_BOOST, &value, 1) != RC_OK) {
		ERROR("get_cooler_boost", "error in read_ec", -1);
	}

	return value;
}


uint8_t get_gpu_temp(FILE * handle) {
	uint8_t result;
	if(read_ec(handle, GPU_TEMP, &result, 1) != RC_OK) {
		ERROR("get_gpu_temp", "error in read_ec", -1);
	}

	return result;
}


uint8_t get_cpu_temp(FILE * handle) {
	uint8_t result;
	if(read_ec(handle, CPU_TEMP, &result, 1) != RC_OK) {
		ERROR("get_cpu_temp", "error in read_ec", -1);
	}

	return result;
}

RCODE set_fan_mapping(FILE * handle, int fan, temp_mapper_t mapper) {
	size_t fan_count, temp_count;
	size_t temp_start_address, fan_start_address;
	if(fan == GPU_FAN) {
		fan_count = 7;
		temp_count = 6;
		temp_start_address = GPU1_TEMP;
		fan_start_address = GPU_FAN_ADDR;
	} else if (fan == CPU_FAN) {
		fan_count = 6;
		temp_count = 6;
		temp_start_address = CPU1_TEMP;
		fan_start_address = CPU_FAN_ADDR;
	} else {
		ERROR("set_fan_mapping", "invalid fan id", RC_FAILED);
	}

	if(write_ec(handle, temp_start_address, mapper.temps, temp_count) != RC_OK) {
		ERROR("set_fan_mapping", "error when setting temp addresses", RC_FAILED);
	}


	if(write_ec(handle, fan_start_address, mapper.fan_powers, fan_count) != RC_OK) {
		ERROR("set_fan_mapping", "error when setting fan values", RC_FAILED);
	}


	return RC_OK;

}

