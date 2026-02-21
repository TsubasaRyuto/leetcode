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

// code review
//
// impl Solution {
//     pub fn str_str(haystack: String, needle: String) -> i32 {
//         let h_bytes = haystack.as_bytes();
//         let n_bytes = needle.as_bytes();
//         let h_len = h_bytes.len();
//         let n_len = n_bytes.len();
////
//         // needleがhaystackより長い場合は見つかるはずがない
//         if n_len > h_len {
//             return -1;
//         }
//
//         // ループ範囲を最適化： needleが収まる範囲まで
//         for i in 0..=(h_len - n_len) {
//             // スライスで比較（バイト列なので安全かつ高速）
//             if &h_bytes[i..i + n_len] == n_bytes {
//                 return i as i32;
//             }
//         }
//
//         -1
//     }
// }
