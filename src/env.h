#ifndef DG2_ENV	
#define DG2_ENV

#include <stdint.h>

// definitions for embedded controller registers
#define EC_FILE "/sys/kernel/debug/ec/ec0/io"

#define BATTERY_THRESHOLD 0xEF
#define BATTERY_MOBILITY 100
#define BATTERY_HYBRID 80
#define BATTERY_BATTERY 60

#define CPU_TEMP 0x68
#define CPU_FAN_ADDR 0x72
#define CPU1_TEMP 0x6A

#define GPU_TEMP 0x80
#define GPU_FAN_ADDR 0x8A
#define GPU1_TEMP 0x82

#define CPU_FAN_RPM 0xCA
#define GPU_FAN_RPM 0xCC

#define COOLER_BOOST 0x98

#define FAN_MODE 0xF4
#define FAN_MODE_ADVANCED 0x8C
#define FAN_MODE_BASIC 0x4C
#define FAN_MODE_AUTO 0xC
#define FAN_MODE_CURRENT 0x0

// type definitions

typedef char RCODE;

#define RC_OK 0
#define RC_FAILED -1

#endif
