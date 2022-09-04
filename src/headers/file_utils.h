/**
 *	\file file_utils.h
 *	\brief the header file for ec writing utils
 *	\author midugh
 *	\version 1.0
 *	\date September, 04 2022
 */

#ifndef DG2_FUTILS
#define DG2_FUTILS

#include "env.h"

#include <stdio.h>

/**
 * \fn	FILE * 	open_ec()
 * \brief 		opens the ec file in r+w binary mode	
 * \return 		the instance of the opened file or NULL
 */
FILE * 	open_ec();


/**
 * \fn RCODE 	write_ec(int key, unsigned char *)
 * \brief 			Writes the byte array at the offset given by key
 * \param handle	The file handle to the EC file
 * \param key 		The offset at which to write the byte array
 * \param value 	The byte array to write at offset
 * \param length 	The length of the value array
 * \return 			0 if succeeded, another RCODE else
 */
RCODE write_ec(FILE* handle, int key, unsigned char* value, int length);

/**
 * \fn RCODE close_ec(FILE *f)
 * \brief		closes the ec file
 * \param f		The file handle opened by open_ec
 * \return 		0 if succeeded, another RCODE else
 */ 
RCODE close_ec(FILE *f);



#endif
