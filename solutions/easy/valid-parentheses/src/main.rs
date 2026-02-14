fn main() {
    println!("Hello, world!");
}

struct Solution {}

impl Solution {
    pub fn is_valid(s: String) -> bool {
        if s.len() % 2 != 0 {
            return false;
        }

        let mut stack: Vec<char> = Vec::new();

        for c in s.chars() {
            match (c) {
                ']' => {
                    if stack.pop() == Some('[') {
                        continue;
                    } else {
                        return false;
                    }
                }
                '}' => {
                    if stack.pop() == Some('{') {
                        continue;
                    } else {
                        return false;
                    }
                }
                ')' => {
                    if stack.pop() == Some('(') {
                        continue;
                    } else {
                        return false;
                    }
                }
                _ => {
                    stack.push(c);
                }
            }
        }

        if stack.len() == 0 {
            return true;
        }

        false
    }
}

// sが奇数の場合falseを返す
//
// sをcharsに変換し、繰り返す
// ({[のばい、スタックに入れる
// もし]})の場合、スタックから取り出し判定する
// 判定結果がfalseの場合、その時点でreturn false
// もし最後まで処理が行われた場合、return true
