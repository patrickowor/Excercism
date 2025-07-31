pub fn eggCount(number: usize) usize {
    var result : usize = 0;
    var current = number;
    while(true){
        if (current == 0){ 
            return result;
        }
        if (current % 2 == 1 ){
            result += 1;
        }
        current /= 2;
    }
}
