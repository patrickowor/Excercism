#include "difference_of_squares.h"

unsigned int square_of_sum(unsigned int value){
    unsigned result  = 0;
    for (unsigned i = 1; i <= value; i++){
        result += i;
    }
    return result * result;
}

unsigned int sum_of_squares(unsigned int value){
    unsigned result  = 0;
    for (unsigned i = 1; i <= value; i++){
        result += (i * i);
    }
    return result;
}

unsigned int difference_of_squares(unsigned int value){
    return square_of_sum(value) - sum_of_squares(value);
}