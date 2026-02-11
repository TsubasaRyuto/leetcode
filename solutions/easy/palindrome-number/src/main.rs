struct Solution {}
impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        if (x < 0) {
            return false;
        }

        let num_to_chars: Vec<char> = x.to_string().chars().collect();
        let num_string_size = num_to_chars.len();
        let mid = num_string_size / 2;
        let (first_half, mut second_half) = num_to_chars.split_at(mid);
        let first_string: String = first_half.iter().collect();
        let mut second_string: String = second_half.iter().rev().collect();

        if (num_string_size % 2 == 0) {
            return first_string == second_string;
        } else {
            second_string.remove(second_string.len() - 1);

            return first_string == second_string;
        }
    }
}

// [memo]
// 負数の時点で false
// 数値文字列の長さが偶数の場合
// 文字列の半分にして、後半の文字列を反転させて、
// 先頭と一致する場合、true
//
// 奇数の場合、
// 文字列を半分にして、後半の文字列を反転させて、最後の文字列を取り除き、
// 先頭と一致する場合、true
