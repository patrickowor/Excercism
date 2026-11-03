#include "grains.h"


uint64_t square(uint8_t num) {
    if (num == 0 || num > 64 ) return 0;
    uint64_t total = 1;
    for (int i = 2; i <= num; ++i){
        total *= 2;
    }
    return total;
}

uint64_t total(){
    uint64_t total = 0;
    for (int i = 1; i <= 64; i++){
        total += square(i);
    }
    return total;
}