/*This function converts a string  to a vector of words split by space
*/
pub fn get_word_vector(buffer : &str) -> Vec<&str>{
    buffer.split_whitespace()
        .collect::<Vec<&str>>()
}