CC=gcc

LIBS=gtk+-3.0

COMMON_FLAGS=-Wall -Wextra -Wno-format-security `pkg-config --cflags $(LIBS)`
CFLAGS=$(COMMON_FLAGS) -O3
DFLAGS=$(COMMON_FLAGS) -O0 -g -fsanitize=address

LD=gcc
LDFLAGS=`pkg-config --libs $(LIBS)` -rdynamic
DEBUG_LDFLAGS=-fsanitize=address

BINARY=DragonCenter2

ROOT_TARGET=./bin
TARGET_DIR=$(ROOT_TARGET)/Release
TARGET=$(TARGET_DIR)/$(BINARY)

DEBUG_TARGET_DIR=$(ROOT_TARGET)/Debug
DEBUG_TARGET=$(DEBUG_TARGET_DIR)/$(BINARY).debug

SRC_DIR=./src

SRC=main.c utils/file_utils.c dragon_utils/dragon_utils.c ui.c
HEADERS=utils/file_utils.h utils/config_parser.h dragon_utils/dragon_utils.h env.h ui.h

DEPS=$(addprefix $(SRC_DIR)/, $(HEADERS))

OBJ_ROOT=./obj
OBJ_DIR=$(OBJ_ROOT)/Release
OBJS=$(addprefix $(OBJ_DIR)/, $(SRC:.c=.o))

DEBUG_OBJ_DIR=$(OBJ_ROOT)/Debug
DEBUG_OBJS=$(addprefix $(DEBUG_OBJ_DIR)/, $(SRC:.c=.o))

.PHONY: all debug build prod dbuild clean

all: prod

prod: $(TARGET)

$(TARGET): $(OBJS)
	@mkdir -p $(TARGET_DIR)
	$(LD) -o $@ $^ $(LDFLAGS)

debug: $(DEBUG_TARGET)

$(DEBUG_TARGET): $(DEBUG_OBJS)
	@mkdir -p $(DEBUG_TARGET_DIR)
	$(LD) $^ -o $@ $(LDFLAGS) $(DEBUG_LDFLAGS)

dbuild: $(DEBUG_OBJS)

build: $(OBJS)

$(OBJ_DIR)/%.o: $(SRC_DIR)/%.c $(DEPS)
	@mkdir -p $(shell echo $@ | grep -Eo "(\w+/)+")
	@# vim color scheme debug "
	$(CC) $(CFLAGS) -c $< -o $@

$(DEBUG_OBJ_DIR)/%.o: $(SRC_DIR)/%.c $(DEPS)
	@mkdir -p $(shell echo $@ | grep -Eo "(\w+/)+")
	@# vim color scheme debug "
	$(CC) $(DFLAGS) -c $< -o $@

clean:
	rm -rf ./bin
	rm -rf ./obj
