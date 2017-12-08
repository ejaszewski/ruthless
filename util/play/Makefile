CC          = g++
CFLAGS      = -std=c++11 -Wall -pedantic -ggdb

all: testgame

testgame: testgame.o
	$(CC) -o $@ $^

%.o: %.cpp
	$(CC) -c $(CFLAGS) -x c++ $< -o $@

clean:
	rm -f *.o testgame
