#include <stdio.h>
int main() {
	int i = 1;
	int k = 0;
	int count = 0;
	int n = 100;
	int x = 0;
	int delta = 3;
	int y = 0;
	int j = 0;

	/* first ex */
	while (i <= n - 1) {
		k += 10 * i;
		count++;
		i++;
	}
	printf("1 count: %d\n", count);

	/* second ex */
	i = 1;
	k = 0;
	count = 0;
	do {
		k += 10 * i;
		count++;
		i++;
	} while (i <= n - 1);
	printf("2 count: %d\n", count);

	/* 3rd ex */
	i = 1;
	k = 0;
	count = 0;
	while (i <= n - 1) {
		i++;
		k += 10 * i;
		count++;
	}		
	printf("3 count: %d\n", count);

	/* 4th ex */
	k = 0;
	count = 0;
	for (i = 1; i <= n; i++) {
		for (j = i; j <= n; j++) {
			k++;
			count++;
		}
	}
	printf("4 count: %d\n", count);

	/* 5th ex */
	count = 0;
	for (i = 1; i <= n; i++) {
		for (j = 1; j <= i; j++) {
			for (k = 1; k <= j; k++) {
				x += delta;
				count++;
			}
		}
	}
	printf("5 count: %d\n", count);

	/* 6th ex */
	i = 1;
	j = 0;
	count = 0;
	while (i + j <= n) {
		if (i > j) {
			j++;
			count++;
		} else {
			i++;
		}
	}
	printf("6 count: %d\n", count);

	/* 7th ex */
	x = n;
	y = 0;
	count = 0;
	while (x >= (y + 1) * (y + 1)) {
		y++;
		count++;
	}
	printf("7 count: %d\n", count);

	/* 8th ex */
	x = 91;
	y = 100;
	count = 0;
	while (y > 0) {
		if (x > 100) {
			x -= 10;
			y--;
			count++;
		} else {
			x++;
		}
	}
	printf("8 count: %d\n", count);
}
