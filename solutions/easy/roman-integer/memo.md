# Problem
- Title: Roman to Integer
- Difficulty:  Easy
- URL: https://leetcode.com/problems/roman-to-integer
- Accepted: true
- Runtime: 0 MS
- Memory: 2 MB

## Summary
### Input:
```rust
Input: s = "III"
```

### Output:
```rust
Output: 3
```

### Constraints:
$1 \leq s.length \leq 15$

$s contains only the characters ('I', 'V', 'X', 'L', 'C', 'D', 'M')$

It is guaranteed that s is a valid roman numeral in the range [1, 3999]

### Goal (One-line Summary):
ローマ数字をアラビア数字に変換する

---

## My Approach
### My Insight:
1. 文字列を1文字ずつ走査する
2. 対応する数値を取得する
3. 「減算則」の判定と計算
   現在の数値が「直前の数値」よりも大きい場合（例：IV, IXなど）、減算する
    * 合計値から「直前の数値」を引き、さらに「現在の数値と直前の数値を引いた差分」を合計値に加算
4. 通常の加算
   現在の数値が直前の数値以下の場合、そのまま合計値に現在の数値を加算
5. 直前の数値を更新
   次の文字の比較に備え、現在の数値を「直前の数値（before_number）」として保持

### Time Complexity:
$O(n)$

### Space Complexity:
$O(1)$

### Learning:
#### match式の活用
HashMapはヒープ割り当てが発生し、ハッシュ計算も必要。
HashMap を使うより、ローマ数字の種類は固定（7種）なので、match を使う方が高速でメモリ効率も良く、Rustらしい

```rust
for c in s.chars {
  let curr_val = match c {
    'I' => 1,
    'V' => 5,
    'X' => 10,
    'L' => 50,
    'C' => 100,
    'D' => 500,
    'M' => 1000,
    _ => 0,
  };
  ....
}
```

#### 計算式
before
```rust
tmp_number -= before_number;
tmp_number += current_number - before_number;
```
after
```rust
tmp_number += current_number - 2 * before_number;
```

---

## Insight
### The Core Essence of the Problem:
「順序（Order）」が「意味（Value）」を変える
というデータ構造を、いかに効率的なループと条件分岐で処理するか

## Other Approaches
### Summary:
- Runtime: 
- Memory: 

### Time Complexity:
$O(n)$

### Space Complexity:
$O(1)$

### Code:
```rust
impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let mut total = 0;
        let mut prev_val = 0;

        // bytes() で回すと、UTF-8チェックをスキップして高速にアクセスできる
        // 後ろからスキャン (rev)
        for b in s.bytes().rev() {
            let curr_val = match b {
                b'I' => 1,
                b'V' => 5,
                b'X' => 10,
                b'L' => 50,
                b'C' => 100,
                b'D' => 500,
                b'M' => 1000,
                _ => 0,
            };

            if curr_val < prev_val {
                total -= curr_val;
            } else {
                total += curr_val;
            }
            prev_val = curr_val;
        }

        total
    }
}

```

### Learning:
リバースして後ろからスキャンすることで、余計な加算・減算が不要になる

---

## Abstraction
### Computational Model:
決定性有限オートマトンの一種

状態遷移：
文字を一つ読むごとに合計値を更新し、次の状態へと遷移する

文脈依存性：
ローマ文字には $IV$（4）や $IX$（9）といった減算ルールがあるため、一つ前の状態が現在の文字の解釈に影響を与える。

### Data Structure Perspective:
連想配列と順序付きシーケンス

連想配列：
Mapping (文字 $\rightarrow$ 数値)

順序付きシーケンス：
入力文字列は単なる集合ではなく、「位置（インデックス）」が重要な意味を持つ列

### Mathematical Perspective:
