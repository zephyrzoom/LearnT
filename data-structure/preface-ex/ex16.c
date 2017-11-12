#include <stdio.h>
void descent(int x, int y, int z);

int main() {
	int x = 0;
	int y = 0;
	int z = 0;
	printf("input three numbers:");
	scanf("%d %d %d", &x, &y, &z);
	descent(x, y, z);
}

void descent(int x, int y, int z) {
	if (x < y) {
		if (x < z) {
			if (y < z) {
				printf("descent: %d %d %d\n", z, y, x);
			} else {
				printf("descent: %d %d %d\n", y, z, x);
			}
		} else {
			printf("descent: %d %d %d\n", y, x, z);
		}
	} else {
		if (x < z) {
			printf("descent: %d %d %d\n", z, x, y);
		} else {
			if (y < z) {
				printf("descent: %d %d %d\n", x, z, y);
			} else {
				printf("descent: %d %d %d\n", x, y, z);
			}
		}
	}
}
