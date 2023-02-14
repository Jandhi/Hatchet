CC = g++
CFLAGS = -Wall
LDFLAGS =
OBJFILES = \
	src/main.o \
	src/values/value.o
TARGET = hatchet

all: $(TARGET)

$(TARGET): $(OBJFILES)
	$(CC) $(CFLAGS) -o $(TARGET) $(OBJFILES) $(LDFLAGS)
clean:
	rm -f $(OBJFILES) $(TARGET) *~