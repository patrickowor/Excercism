pub fn is_armstrong_number(num: u32) -> bool {
    let   num_string = format!("{}", num);
    let length = num_string.len() as u32;
    let mut result = num_string
    .chars()
    .collect::<Vec<char>>()
    .into_iter()
    .map(|a| a.to_digit(10).unwrap().pow(length) as usize )
    .collect::<Vec<usize>>()
    .into_iter()
    .reduce(|acc, value|  acc + value )
    .unwrap();
    if result == num as usize { true } else { false }
}