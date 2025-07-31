#![feature(iter_map_windows)]
pub fn build_proverb(list: &[&str]) -> String {

    match list.len()  {
        0 => String::new(),
        _ => list.iter()
            .map_windows(|[x, y]| format!("For want of a {} the {} was lost.\n", x, y))
            .collect::<Vec<String>>().iter()
            .chain( [format!("And all for the want of a {}.", list[0])].iter())
            .flat_map(|s| s.chars())
            .collect::<String>()
    }
}
