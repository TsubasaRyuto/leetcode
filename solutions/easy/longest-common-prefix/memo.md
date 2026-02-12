# Problem
- Title: Longest Common Prefix
- Difficulty:  Easy
- URL: https://leetcode.com/problems/longest-common-prefix
- Accepted: true
- Runtime: 0 MS
- Memory: 2.29 MB

## Summary
### Input:
```rust
strs = ["flower","flow","flight"]
```

### Output:
```rust
"fl"
```

### Constraints:
$1 \leq strs.length \leq 200$

$0 \leq strs[i].length \leq 200$

strs[i] consists of only lowercase English letters if it is non-empty.

### Goal (One-line Summary):
与えられた文字列配列の中から、共通する最長の接頭語を抽出し出力する

---

## My Approach
### My Insight:
1. 初期化
   * 1番目の文字列を「共通接頭辞の候補」として保持する
2. 逐次更新
   * 2番目以降の文字列を順に比較し、候補を「より短い一致部分」へと更新していく。
   * 長さの調整: 比較する文字列が候補より短い場合、その時点で候補を文字列の長さに切り詰める。
   * 不一致での切り詰め: 1文字ずつ比較し、一致しなくなった瞬間にそれ以降を切り捨てる。
3. 終了条件
   * 完了: 全ての文字列を確認し終えたら、残った候補が答え。
   * 早期終了: 途中で候補が「空」になったら、共通部分は存在しないため即座に終了。

### Time Complexity:
$O(N \times M)$
文字列の数（$N$）
文字列の平均長（$M$）

### Space Complexity:
$O(n)$

### Learning:
#### skipで最初の要素に比較を避ける
```rust
for s in strs.iter().skip(1)
```
すでに最初の要素をprefixとして登録しているので、最初の要素をskipした状態から繰り返し処理を開始すると効率性が上がる

#### zipを使ったindex管理排除
```rust
let mismatch_index = prefix_chars.iter() // 接頭辞の文字
    .zip(s.chars())                      // 比較対象の文字とペアにする
    .position(|(p, c)| *p != c);         // 異なる箇所のインデックスを探す
```
zipとpositionを使って、prefix_charsと現在の文字列 s をペアで回し不一致を探す

```markdown
比較対象: prefix='flower' vs s='flight'

--- [zip] の視覚化 ---
インデックス | (prefix, s)
-------------+-------------
     0       | ('f', 'f')
     1       | ('l', 'l')
     2       | ('o', 'i')
     3       | ('w', 'g')
     4       | ('e', 'h')
     5       | ('r', 't')

※ 'flower'の最後の 'r' は、'flight' の長さと合わないため zip で無視されました。

--- [position] の挙動 ---
チェック中: ('f', 'f') -> 一致 (false)
チェック中: ('l', 'l') -> 一致 (false)
チェック中: ('o', 'i') -> 不一致！ (true)

結果: mismatch_index = Some(2)
```
不一致が見つかった場合に、切り詰める処理を挟むようにする
```rust
if let Some(i) = mismatch_index {
  prefix_chars.truncate(i);
}
```

---

## Insight
### The Core Essence of the Problem:

## Other Approaches
### Summary:
- Runtime: 
- Memory: 

### Time Complexity:
$O(S)$
$S$ は全文字列に含まれる文字の総数

### Space Complexity:
$O(L)$ または $O(1)$
結果として返す接頭辞の長さを $L$ とすると、新しい文字列を生成するために $O(L)$ のメモリが必要

### Code:
```rust
pub fn longest_common_prefix(strs: Vec<String>) -> String {
    if strs.is_empty() {
        return String::new();
    }

    // 1番目の文字列を基準として、1文字ずつインデックス i で走査する
    let first_str = &strs[0];

    for (i, char_to_match) in first_str.chars().enumerate() {
        // 残りの文字列 (strs[1..]) を確認
        for s in &strs[1..] {
            // 文字列が i 番目に到達していない（sの方が短い）
            // または、i 番目の文字が一致しない場合
            // そこまでの部分文字列を返して終了
            if i >= s.len() || s.chars().nth(i) != Some(char_to_match) {
                return first_str[..i].to_string();
            }
        }
    }

    // すべてのチェックを通過した場合は、最初の文字列全体が接頭辞
    strs[0].clone()
}
```

### Learning:
#### 垂直走査
自分の書いたコードは水平走査で、文字列一つずつを現在の暫定prefixと比較しながら繰り返しをこなう。
一方で垂直走査はは全文字列の1文字目は一致してる？次に全文字列の2文字目は一致してる？を繰り返し、一致しなくなった時点で判定終了させる。

垂直の方が良い理由
ex) [longest_common_xxx, longest_common_xxx, ..., la]
最後の文字列だけlaで終わってる場合でも水平の場合全て走査した上で l だけがprefixであることが判定される。
一方で、垂直の場合、数回の比較だけで処理が終了する。

条件設定:
- 文字列の数: 1,000個
- 文字列の長さ: 全員 10,000文字（ただし、最後の1個だけ「3文字」で終わっている、あるいは3文字目で違う文字になっている）

1. 水平走査（Horizontal）
「隣同士を比較して、勝った方（共通部分）を次に持ち越す」方式
- 1個目 vs 2個目: 10,000文字すべて一致 $\rightarrow$ 10,000回比較
- 暫定1位 vs 3個目: 10,000文字すべて一致 $\rightarrow$ 10,000回比較
- ... (これが998回続く) ...
- 暫定1位 vs 1,000個目: ここでやっと「あ、3文字しか合わない」となる
- 総比較回数: $10,000 \times 999 \approx$ 9,990,000回（約1000万回）
- 無駄: 99.9% の計算が無駄

2. 垂直走査（Vertical）
「全員の1文字目、全員の2文字目…」と進む方式
- 1文字目: 1,000個全員チェック $\rightarrow$ 1,000回比較
- 2文字目: 1,000個全員チェック $\rightarrow$ 1,000回比較
- 3文字目: 1,000個全員チェック $\rightarrow$ 1,000回比較
- 4文字目: 1,000個目の文字列がもう無い（または違う）ので、即終了！
- 総比較回数: $1,000 \times 3 =$ 3,000回
- 結果: 3,330倍速い です

---

## Abstraction
### Computational Model:
畳み込みと走査

- 水平スキャン＝畳み込み（Reduce / Fold）
$S_1$ と $S_2$ の共通項を出す。その結果と $S_3$ の共通項を出す……というように、前の計算結果を次の入力に使う。

- 垂直スキャン＝走査（Scanning / Zip）
全文字列の0文字目をチェック。全員同じなら次へ。全文字列の1文字目をチェック……。誰か1人でも違う文字が出てきたら、その手前までが答え。

### Data Structure Perspective:
TrieTree(Prefix Tree)
全文字列をツリーにした際、「根（ルート）から最初の枝分かれが起こるまでの距離」を測る問題と言い換えられる。


### Mathematical Perspective:
半格子と代数的性質
共通集合（Intersection）: 全ての文字列集合が共有している、先頭からの連続した要素を抽出する操作。
