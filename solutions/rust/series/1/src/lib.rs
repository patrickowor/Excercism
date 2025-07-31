pub fn series(digits: &str, len: usize) -> Vec<String> {
    let my_string = String::from(digits) ;
    let mut results : Vec<String> = vec![];
    if my_string.len() < len { return vec![]; }
    let max_iter = my_string.len() - len + 1;
    for i in 0..max_iter{
        results.push(String::from(&my_string[i..(i + len)])) ;
    } 
    results
}
