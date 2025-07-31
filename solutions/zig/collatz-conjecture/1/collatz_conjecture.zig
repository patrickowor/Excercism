// Please implement the `ComputationError.IllegalArgument` error.

pub const ComputationError = error {
	IllegalArgument,
};

pub fn steps(number: usize) anyerror!usize {
    if (number <= 0){
        return ComputationError.IllegalArgument;
    }
    var no_of_steps: usize = 0;
    var current_value: usize = number;

    while (true) {
        if (current_value == 1){
            break;
        } else if (current_value % 2 == 0){
            current_value /= 2;
        } else {
            current_value = (current_value * 3) + 1;
        }
        no_of_steps += 1;
    }
    return no_of_steps;
}
