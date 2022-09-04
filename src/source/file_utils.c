#include "../headers/file_utils.h"
#include "../headers/env.h"

#include <malloc.h>


#define ERROR(function, message) fprintf(stderr, "%s: %s\n", function, message);


FILE * 	open_ec() {
	FILE * ec_file = fopen(EC_FILE, "rb+");
	if(ec_file == NULL) {
		ERROR("open_ec", "could not open file");
	}

	return ec_file;
}

RCODE close_ec(FILE * handle) {
	if(handle == NULL) {
		ERROR("close_ec", "handle is NULL");
		return RC_FAILED;
	}

	fclose(handle);
	return RC_OK;
}


RCODE write_ec(FILE * handle, int key, unsigned char * value, size_t length) {
	if(handle == NULL) {
		ERROR("write_ec", "handle is NULL");
		return RC_FAILED;
	}
	
	int result;
	if((result = fseek(handle, key, SEEK_SET)) != 0) {
		char * err_msg = (char*) malloc(32 * sizeof(char));
		sprintf(err_msg, "fseek failed: returned %d", result);
		ERROR("write_ec", err_msg);
		free(err_msg);
		return RC_FAILED;
	}

	size_t written = fwrite(value, length * sizeof(char), 1, handle);
	if(written < length) {
		ERROR("write_ec", "fwrite: an error occured while reading");
		return RC_FAILED;
	}

	return RC_OK;
	
}

RCODE read_ec(FILE * handle, int key, unsigned char * value, size_t length) {
	if(handle == NULL) {
		ERROR("read_ec", "handle is NULL");
		return RC_FAILED;
	}

	int result;
	if((result = fseek(handle, key, SEEK_SET)) != 0) {
		char * err_msg = (char*) malloc(32 * sizeof(char));
		sprintf(err_msg, "fseek failed: returned %d", result);
		ERROR("read_ec", err_msg);
		free(err_msg);
		return RC_FAILED;
	}

	size_t read_bytes = fread(value, length * sizeof(char), 1, handle);
	if(read_bytes < length) {
		ERROR("read_ec", "fread: an error occured while reading");
		return RC_FAILED;
	}

	return RC_OK;

}


