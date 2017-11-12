#include <stdio.h>
int main() {
	int n = 512;
	printf("%d\n", Time(n));	
}

int Time (int n) {
	int count = 0;
	int x = 2;
	while (x < n / 2) {
		x *= 2;
		count++;
	}
	return count;
}
