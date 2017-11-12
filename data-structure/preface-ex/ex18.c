/*
 * 田径运动得分统计
 */
#include <stdio.h>
#include <stdlib.h>
#define FILE_NAME "scoreItems.txt"
typedef struct ScoreItem {
    char items[20];
    char gender[5];
    char university;
    int score;
    int result;
} ScoreItem;

void readInfo(ScoreItem * items);

int main() {
    ScoreItem items[12];
    int i;
    int bsum[4] = {0};
    int gsum[4] = {0};
    int sum[4] = {0};
    readInfo(items);
    
    for (i = 0; i < 12; i++) {
	switch (items[i].university) {
	    case 'A': sum[0] += items[i].result;
		      if (strcmp(items[i].gender, "男") == 0) {
			  bsum[0] += items[i].result;
		      } else {
			  gsum[0] += items[i].result;
		      }
		      break;
	    case 'B':sum[1] += items[i].result;
		      if (strcmp(items[i].gender, "男") == 0) {
			  bsum[1] += items[i].result;
		      } else {
			  gsum[1] += items[i].result;
		      }
		      break;

	    case 'C':sum[2] += items[i].result;
		      if (strcmp(items[i].gender, "男") == 0) {
			  bsum[2] += items[i].result;
		      } else {
			  gsum[2] += items[i].result;
		      }
		      break;

	    case 'D': sum[3] += items[i].result;
		      if (strcmp(items[i].gender, "男") == 0) {
			  bsum[3] += items[i].result;
		      } else {
			  gsum[3] += items[i].result;
		      }
		      break;
	    default: break;
	}
    }
    
    for (i = 0; i < 4; i++) {
	printf("university %c: \nboy result is %d\ngirl result is %d\nsum result is %d\n", "ABCD"[i], bsum[i], gsum[i], sum[i]);
    }
    return 0;
}

void readInfo(ScoreItem * items) {
    FILE * fp;
    int i;
    if ((fp = fopen(FILE_NAME, "r")) == NULL) {	
        printf("can't open the file %s\n", FILE_NAME);	
        exit(EXIT_FAILURE);
    }
    for (i = 0; i < 12; i++) {
        if (fscanf(fp, "%s\t", items[i].items) != 1) {
	    printf("read file error");
        }
        if (fscanf(fp, "%s\t", items[i].gender) != 1) {
	    printf("read file error");
        }
	if (fscanf(fp, "%c\t", &items[i].university) != 1) {
	    printf("read file error");
        }
	if (fscanf(fp, "%i\t", &items[i].score) != 1) {
	    printf("read file error");
        }
	if (fscanf(fp, "%i\t", &items[i].result) != 1) {
	    printf("read file error");
        }
    }
}
