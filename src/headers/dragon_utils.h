#ifndef DG2_DRAGON_UTILS_H
#define DG2_DRAGON_UTILS_H

#define GPU_FAN 0
#define CPU_FAN 1

#include "env.h"

/**
 * \fn RCODE *	set_fan_rpm(FILE * handle, int fan_id, int rpm)
 * \brief	sets the rpm for the specified fans
 * \param handle	The file handle for ec
 * \param fan_id	The fan id
 * \param rpm		The RPM to set for the fan
 *
 */
RCODE *	set_fan_rpm(FILE * handle, int fan, int rpm);
int 	get_fan_rpm(FILE * handle, int fan);

RCODE * set_cooler_boost(FILE * handle, int value);
int 	get_coolet_boost(FILE * handle);

RCODE * set_mode(FILE * handle, int mode);
int 	get_mode(FILE * handle);

RCODE *	set_battery_threshold(FILE * handle, char threshold);
char 	get_battery_threshold(FILE * handle);


#endif
