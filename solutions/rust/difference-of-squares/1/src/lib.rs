pub fn square_of_sum(n: u32) -> u32 {
    let mut total = 0;
    for i in 1..=n {
        total += i;
    }
  total * total
}

pub fn sum_of_squares(n: u32) -> u32 {
    let mut total = 0;
    for i in 1..=n {
        let square = i * i;
        total += square ;
    }
    
  total
}

pub fn difference(n: u32) -> u32 {
    square_of_sum(n) - sum_of_squares(n)
}
