fn main() {
    println!("Hello, world!");
}

use std::collections::HashMap;
struct Solution {}

impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let mut roman_int_map = HashMap::with_capacity(7);
        roman_int_map.insert('I', 1);
        roman_int_map.insert('V', 5);
        roman_int_map.insert('X', 10);
        roman_int_map.insert('L', 50);
        roman_int_map.insert('C', 100);
        roman_int_map.insert('D', 500);
        roman_int_map.insert('M', 1000);

        let mut tmp_number = 0;
        let mut before_number = 0;

        for v in s.chars() {
            let current_number = *roman_int_map.get(&v).unwrap_or(&0);

            if before_number < current_number {
                tmp_number -= before_number;
                tmp_number += current_number - before_number;
            } else {
                tmp_number += current_number;
            }
            before_number = current_number;
        }

        tmp_number
    }
}

// s = "III"
//
// =>3
// 文字列をchar[]に変換し、繰り返し処理を行う
// roman文字にマッチする数値を取得する
// もし取得した値がtmpNumberに入ってる桁数より多い数字が来た場合、
// 該当する桁数値を現在の数値より減算する
// その値をtmpNumberに足していく
//
// tmpNumberに入っている桁数より多い数が来たかどうかはbefore_numberで管理する
//
// ex)
// MM = 1000 + 1000
//
// MMCM
// 1000
// 1000 + 1000
// 2000 + 100
// 2000 - 100 + 1000 - 100
