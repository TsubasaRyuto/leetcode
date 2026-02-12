fn main() {
    println!("Hello, world!");
}

struct Solution {}
impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let mut prefix_chars: Vec<char> = strs.first().unwrap().chars().collect();

        for s in strs.iter() {
            if prefix_chars.len() > s.len() {
                prefix_chars.truncate(s.len());
            }

            for (i, c) in s.chars().enumerate() {
                if prefix_chars.len() > i && prefix_chars[i] != c {
                    prefix_chars.truncate(i);
                }
            }

            if prefix_chars.is_empty() {
                return String::new();
            }
        }

        prefix_chars.into_iter().collect()
    }
}

// 1. 初期化
//    * 1番目の文字列を「共通接頭辞の候補」として保持する
// 2. 逐次更新
//    * 2番目以降の文字列を順に比較し、候補を「より短い一致部分」へと更新していく。
//    * 長さの調整: 比較する文字列が候補より短い場合、その時点で候補を文字列の長さに切り詰める。
//    * 不一致での切り詰め: 1文字ずつ比較し、一致しなくなった瞬間にそれ以降を切り捨てる。
// 3. 終了条件
//    * 完了: 全ての文字列を確認し終えたら、残った候補が答え。
//    * 早期終了: 途中で候補が「空」になったら、共通部分は存在しないため即座に終了。
//
