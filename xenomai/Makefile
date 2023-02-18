CC = gcc
SKIN = native
CFLAGS = -std=gnu99 $(shell xeno-config --skin=$(SKIN) --cflags) -Wl,--no-as-needed -ldl 
LDFLAGS = $(shell xeno-config --skin=$(SKIN) --ldflags)

EXE = pidrone


all: libxenomai.c gpio.h libgpio.a
	$(CC) $^ $(CFLAGS)  $(LDFLAGS)   -o $(EXE)


