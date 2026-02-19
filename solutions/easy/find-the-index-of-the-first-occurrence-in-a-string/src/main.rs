fn main() {
    println!("Hello, world!");
}

struct Solution {}

impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        let needle_size = needle.len();
        for i in 0..haystack.len() {
            if let Some(splited) = haystack.get(i..(needle_size + i)) {
                println!("{}", splited);
                if splited == needle {
                    return i as i32;
                }
            } else {
                return -1;
            }
        }

        return -1;
    }
}

// Iterate through the haystack.
// Extract a substring of the same length as the needle and compare them.
// If they match, return the current index.
// If the loop finishes without a match, return -1.
