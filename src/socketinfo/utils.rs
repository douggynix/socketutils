/*This function converts a string  to a vector of words split by space
*/
pub fn split_text_by_words(buffer : &str) -> Vec<&str>{
    buffer.split_whitespace()
        .collect::<Vec<&str>>()
}

pub fn isdigit(text : &&str) -> bool {
    text.parse::<usize>().is_ok()
}

pub fn truncate(str : &String, new_len: usize) -> String{
    let mut trunc_str = format!("{}", str);
    trunc_str.truncate(new_len);
    return trunc_str
}

pub fn remove_non_printable_chars(dirty_string: &String) -> String{
    dirty_string.chars()
        .map( |c| { //removing non printable characters
            match c {
                '\0' => ' ',
                _ => c
            }
        })
        .collect::<String>()
}
