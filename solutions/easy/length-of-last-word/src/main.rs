fn main() {
    println!("Hello, world!");
}

struct Solution {}
impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        let splited_words: Vec<&str> = s.split(' ').into_iter().filter(|x| *x != "").collect();
        if splited_words.is_empty() {
            return 0;
        };
        let target_word = splited_words[splited_words.len() - 1];
        target_word.trim().len() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_length_of_last_word_case1() {
        assert_eq!(Solution::length_of_last_word("Hello World".to_string()), 5);
    }

    #[test]
    fn test_length_of_last_word_case2() {
        assert_eq!(
            Solution::length_of_last_word("   fly me   to   the moon  ".to_string()),
            4
        );
    }

    #[test]
    fn test_length_of_last_word_case3() {
        assert_eq!(
            Solution::length_of_last_word("luffy is still joyboy".to_string()),
            6
        );
    }

    #[test]
    fn test_length_of_last_word_case4() {
        assert_eq!(Solution::length_of_last_word("    ".to_string()), 0);
    }

    #[test]
    fn test_length_of_last_word_case5() {
        assert_eq!(Solution::length_of_last_word("     1".to_string()), 1);
    }

    #[test]
    fn test_length_of_last_word_case6() {
        assert_eq!(
            Solution::length_of_last_word(
                "aaa   aa             a          a                     ".to_string()
            ),
            1
        );
    }
}

// Algorithm: Length of Last Word
// 1. Split the string by spaces and filter out any empty substrings.
// 2. Identify the last element in the resulting list.
// 3. Return the length of the identified world.

// Code review by AI
// - Avoid unnecessary allocations(.collect())
//   Calling `.collect::<Vec<&str>>()` allocates memory on the heap to store all the words. Since you only need the last word, you don't need to store the rest of them in an array.
//
// - Use split_whitespace()
//   Instead of manually splitting by ' ' and then filtering out empty strings, Rust has a builtin method called .split_whitespace().
//
// - Use .last() instead of indexing
//   Iterators in Rust have a handy .last() method that grabs the final item. This is safer and cleaner than doing.
//
// Refactored solution
pub fn length_of_last_word(s: String) -> i32 {
    s.split_whitespace()
        .last()
        .map_or(0, |word| word.len() as i32)
}
