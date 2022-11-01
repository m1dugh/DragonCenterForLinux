CC=gcc
CFLAGS=`pkg-config --cflags gtk+-3.0` -g -Wall -c

LD=gcc
LDFLAGS=`pkg-config --libs gtk+-3.0` -rdynamic

BINDIR=./bin
TARGET=$(BINDIR)/DragonCenter2
SRCDIR=./src
SOURCES=main.c utils/file_utils.c dragon_utils/dragon_utils.c
OBJDIR=./obj
OBJS=$(addprefix $(OBJDIR)/, $(SOURCES:.c=.o))

.PHONY: clean build all run

all: $(TARGET)


run: $(TARGET)
	./$^


$(TARGET): $(OBJS)
	@mkdir -p $(BINDIR)
	$(LD)  $^ -o $@ $(LDFLAGS)


build: $(OBJS)

$(OBJDIR)/%.o: $(SRCDIR)/%.c
	@mkdir -p $(shell echo $@ | grep -Eo "(\w+/)+")
	@# vim color scheme debug "
	$(CC) $(CFLAGS) $< -o $@

	
clean:
	rm -rf $(OBJDIR)
	rm -rf $(BINDIR)

