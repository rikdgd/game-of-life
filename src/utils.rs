pub fn count_chars_in_str(str: &str, chr: char) -> usize {
    let mut counter: usize = 0;
    for i in str.chars() {
        if i == chr {
            counter += 1;
        }
    }
    counter
}
