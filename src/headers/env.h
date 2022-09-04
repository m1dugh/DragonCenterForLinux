#ifndef DG2_ENV	
#define DG2_ENV

// definitions for embedded controller registers
#define EC_FILE "/sys/kernel/debug/ec/ec0/io"

#define BATTERY_THRESOLD 0xEF

#define CPU_TEMP 0x68
#define CPU1_TEMP 0x6A
#define CPU2_TEMP CPU1_TEMP+0x1
#define CPU3_TEMP CPU1_TEMP+0x2
#define CPU4_TEMP CPU1_TEMP+0x3
#define CPU5_TEMP CPU1_TEMP+0x4
#define CPU6_TEMP CPU1_TEMP+0x5

#define GPU_TEMP 0x80
#define GPU1_TEMP 0x82
#define GPU2_TEMP GPU1_TEMP+0x1
#define GPU3_TEMP GPU1_TEMP+0x2
#define GPU4_TEMP GPU1_TEMP+0x3
#define GPU5_TEMP GPU1_TEMP+0x4
#define GPU6_TEMP GPU1_TEMP+0x5

#define CPU_FAN_RPM 0xCA
#define GPU_FAN_RPM 0xCC

#define COOLER_BOOST 0x98

#define FAN_MODE 0xF4
#define FAN_MODE_ADVANCED 0x8C
#define FAN_MODE_BASIC 0x4C
#define FAN_MODE_AUTO 0xC

// type definitions

typedef char RCODE;

#define RC_OK 0
#define RC_FAILED -1

#endif
