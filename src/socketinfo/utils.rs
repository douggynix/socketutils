/*This function converts a string  to a vector of words split by space
*/
pub fn split_text_by_words(buffer : &str) -> Vec<&str>{
    buffer.split_whitespace()
        .collect::<Vec<&str>>()
}

pub fn isdigit(text : &&str) -> bool {
    text.parse::<usize>().is_ok()
}
