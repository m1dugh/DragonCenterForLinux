#ifndef DG2_DRAGON_UTILS_H
#define DG2_DRAGON_UTILS_H

#define GPU_FAN 0
#define CPU_FAN 1

#include "env.h"
#include <stdio.h>

/**
 * \fn RCODE 	set_fan_rpm(FILE * handle, int fan_id, int rpm)
 * \brief	sets the rpm for the specified fans
 * \param handle	The file handle for ec
 * \param fan_id	The fan id
 * \param rpm		The RPM to set for the fan
 * \return			RC_OK id succeeded, RC_FAILED otherwise.
 *
 */
RCODE 	set_fan_rpm(FILE * handle, int fan, int rpm);

/**
 * \fn int	get_fan_rpm(FILE * handle, int fan_id)
 * \brief	fetches the rpm for the given fan
 * \param handle	The file handle for the EC.
 * \param fan_id	The id of the fan.
 * \return 			The fan RPM, -1 if failed.
 */
int 	get_fan_rpm(FILE * handle, int fan);

/**
 * \fn RCODE 		set_cooler_boost(FILE * handle, int value)
 * \brief			Sets the rpm for the cooler boost fan.
 * \param handle	The file handle for ec
 * \param value		The value to set for the fan, 0x00 -> 0x80.
 * \return			RC_OK id succeeded, RC_FAILED otherwise.
 *
 */
RCODE	set_cooler_boost(FILE * handle, int value);

/**
 * \fn int			get_cooler_boost(FILE * handle)
 * \brief			Fetches the rpm for the cooler boost fan.
 * \param handle	The file handle for the EC.
 * \return 			The fan RPM, -1 if failed.
 */
int 	get_coolet_boost(FILE * handle);

/**
 * \fn RCODE 		set_mode(FILE * handle, int mode)
 * \brief			Sets the mode for the power.
 * \param handle	The file handle for EC.
 * \param mode		The mode to set, FAN_MODE_ADVANCED, FAN_MODE_BASIC, FAN_MODE_AUTO. 
 * \return			RC_OK id succeeded, RC_FAILED otherwise.
 *
 */
RCODE	set_mode(FILE * handle, int mode);

/**
 * \fn int			get_mode(FILE * handle)
 * \brief			Fetches the power mode of the embedded controller.
 * \param handle	The file handle for the EC.
 * \return 			The mode, -1 if failed.
 */
int 	get_mode(FILE * handle);

/**
 * \fn RCODE 		set_battery_threshold(FILE * handle, int mode)
 * \brief			Sets the charging threshold in % for the battery.
 * \param handle	The file handle for EC.
 * \param threshold	The threshold in % at which the battery should stop charging.
 * \return			RC_OK id succeeded, RC_FAILED otherwise.
 *
 */
RCODE 	set_battery_threshold(FILE * handle, unsigned char threshold);

/**
 * \fn byte			get_battery_threshold(FILE * handle)
 * \brief			Fetches the battery threshold.
 * \param handle	The file handle for the EC.
 * \return 			The battery threshold in %, -1(255) if failed.
 */
unsigned char 	get_battery_threshold(FILE * handle);


#endif
