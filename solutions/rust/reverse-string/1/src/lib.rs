pub fn reverse(input: &str) -> String {
    let theString = String::from(input);
    let mut theStringVec = theString.chars().collect::<Vec<char>>();
    theStringVec.reverse();
    theStringVec.into_iter().collect::<String>()
}
