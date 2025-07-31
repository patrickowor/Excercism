use std::collections::HashSet;


pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let mut myVec : Vec<&str> = vec![];
    'outer:  for single in possible_anagrams {
        if word.to_lowercase() == single.to_lowercase(){
            continue;
        }
        let mut s = String::from(single.clone()).to_lowercase();
        for i in word.to_lowercase().chars().collect::<Vec<char>>() {
            if s.contains(i) { 
                s = s.replacen(i, "", 1); 
            } else {break}
            
             if s == "" {
                myVec.push(single);
                continue 'outer; 
            }
        }
    }
    {
        println!("{myVec:?}, {word}")
    }
    myVec.into_iter().collect::<HashSet<&'a str>>()   
}
