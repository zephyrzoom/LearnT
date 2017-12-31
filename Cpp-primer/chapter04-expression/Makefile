compile: 4.4.o \
    	 4.21.o \
	 4.22.o \
	 4.28.o \
	 4.29.o
	g++ -o 4.4 4.4.o
	g++ -o 4.21 4.21.o
	g++ -o 4.22 4.22.o
	g++ -o 4.28 4.28.o
	g++ -o 4.29 4.29.o

4.4.o: 4.4.cpp
	g++ -c 4.4.cpp -std=c++11

4.21.o: 4.21.cpp
	g++ -c 4.21.cpp -std=c++11

4.22.o: 4.22.cpp
	g++ -c 4.22.cpp -std=c++11

4.28.o: 4.28.cpp
	g++ -c 4.28.cpp -std=c++11

4.29.o: 4.29.cpp
	g++ -c 4.29.cpp -std=c++11

.PHONY: clean
clean:
	rm 4.4 4.4.o \
	   4.21 4.21.o \
	   4.22 4.22.o \
	   4.28 4.28.o \
	   4.29 4.29.o
