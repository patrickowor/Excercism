pub fn square_of_sum(n: u32) -> u32 {
    (1..=n)
    .reduce(|acc, f|{ acc + f })
    .map(|total| { total * total})
    .unwrap()
}

pub fn sum_of_squares(n: u32) -> u32 {
    (1..=n)
    .reduce(|acc, f|{ acc + (f * f ) })
    .unwrap()
}

pub fn difference(n: u32) -> u32 {
    square_of_sum(n) - sum_of_squares(n)
}
