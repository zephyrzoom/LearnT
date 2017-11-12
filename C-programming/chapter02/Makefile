out = 01 02 03 04 05 06 07 08

compile: $(out)

01: 01.c
	gcc 01.c -o 01

02: 02.c
	gcc 02.c -o 02

03: 03.c
	gcc 03.c -o 03

04: 04.c
	gcc 04.c -o 04

05: 05.c
	gcc 05.c -o 05

06: 06.c
	gcc 06.c -o 06

07: 07.c
	gcc 07.c -o 07

08: 08.c
	gcc 08.c -o 08

.PHONY: clean
clean:
	rm $(out)