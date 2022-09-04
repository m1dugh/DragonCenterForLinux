CC=gcc
CFLAGS=-g -Wall -c

LD=gcc
LFLAGS=

BINDIR=./bin
TARGET=$(BINDIR)/DragonCenter2
SRCDIR=./src
SOURCES=main.c
OBJDIR=./obj
OBJS=$(addprefix $(OBJDIR)/, $(SOURCES:.c=.o))

.PHONY: clean build all run

all: $(TARGET)


run: $(TARGET)
	./$^


$(TARGET): $(OBJS)
	@mkdir -p $(BINDIR)
	$(LD) $(LDFLAGS) $^ -o $@


build: $(OBJS)

$(OBJDIR)/%.o: $(SRCDIR)/%.c
	@mkdir -p $(OBJDIR)
	$(CC) $(CFLAGS) $< -o $@
	
clean:
	rm -rf $(OBJDIR)
	rm -rf $(BINDIR)

