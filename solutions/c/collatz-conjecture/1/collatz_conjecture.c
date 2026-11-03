#include "collatz_conjecture.h"

int steps(int start){
    if (start <= 0) return ERROR_VALUE;
    int steps = 0;
    int curr = start;
    while (curr > 1){
        if (curr % 2 == 0) curr /= 2;
        else curr = (curr * 3) + 1;
        steps++;
    }
    return steps;
}