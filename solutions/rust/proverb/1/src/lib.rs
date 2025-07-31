#![feature(iter_map_windows)]

pub fn build_proverb(list: &[&str]) -> String {
    if (list.len() > 0){
            let mut res = list
        .iter()
        .map_windows(|[x, y]| format!("For want of a {} the {} was lost.\n", x, y))
        .collect::<Vec<String>>();
    res.push(format!("And all for the want of a {}.", list[0]));
    let final_result : String= res.iter().flat_map(|s| s.chars())
                          .collect();
    println!("{}", final_result);
    final_result
    } else {
        String::new()
    }

}
