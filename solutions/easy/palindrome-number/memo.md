# Problem
- Title: 9. Palindrome Number
- Difficulty:  Easy
- URL: https://leetcode.com/problems/palindrome-number/
- Accepted: true
- Runtime: 4 MS
- Memory: 2.2 MB

## Summary
### Input:
```rust
x = 121
```

### Output:
```rust
true
```

### Constraints:
-231 <= x <= 231 - 1

### Goal (One-line Summary):
与えられた数値が逆から読んでも同じ場合、trueを返す、異なる場合、falseを返す

---

## My Approach
### My Insight:
与えられた数値を文字列に変換し、文字列のサイズが奇数の場合と偶数の場合で場合分けした上で、
偶数の場合、文字列を半分にし、片方を反転させて一致してるかを判定する
奇数の場合、文字列を半分にし、1文字多い方を反転させ最後の文字を除いた上で一致してるかを判定する

### Time Complexity:
$O(n)$ (or $O(\log_{10} x)$)

### Space Complexity:
$O(n)$

### Learning:

#### Vec<char>へ変換せずに文字列を分割する

```rust
let num_to_chars: Vec<char> = x.to_string().chars().collect();
let num_string_size = num_to_chars.len();
let mid = num_string_size / 2;
let (first_half, mut second_half) = num_to_chars.split_at(mid);
```
一度Vec<char>に変換した上で、split_atしてるが、変換せずに分割することでメモリ消費を抑えることができる

```rust
let s = x.to_string();
let mid = s.len() / 2;
let (first_half, second_half) = s.split_at(mid);
```

#### `eq` vs `==`
before
```rust
let first_string: String = first_half.iter().collect();
let mut second_string: String = second_half.iter().rev().collect();

first_string == second_string;
```

after
```rust
first_half.chars().eq(second_half.chars().rev())
```

iterの`eq`を使用することでメモリ割り当て（アロケーション）を回避することできる。

`==`の場合、Stringの実態をヒープ領域に作成する必要があるが、iterの`eq`の場合、元の文字列を参照する。

また、短絡評価によるパフォーマンス向上の可能性もある。
`eq`の場合、1文字ずつ評価しながら処理が進むため、最初の1文字で不一致の場合その時点で処理が終了し、反転処理等も行われない。


---

## Insight
### The Core Essence of the Problem:
文字列変換せずに、数値のまま対称性を判定すること

## Other Approaches
### Summary:
- Runtime: 1 ms
- Memory: 2.1 MB

### Time Complexity:
$O(\log_{10} n)$

### Space Complexity:
$O(1)$

### Code:
```rust
impl Solution {
  pub fn is_palindrome(x: i32) -> bool {
    // 1. 負数は回文にならない
    // 2. 末尾が0の場合、その数字が0でない限り回文にならない
    if (x < 0 || (x % 10 == 0 && x != 0)) {
      return false;     
    }

    let mut x = x;
    let mut reverted_number = 0;

    // 数値の半分を反転させる
    while x > reverted_number {
      reverted_number = reverted_number * 10 + x % 10;
      x /= 10;
    }

    x == reverted_number || x == reverted_number / 10
  }
}

```

### Learning:


---

## Abstraction
### Computational Model:
Iterative State Transformation (反復的状態遷移)
入力状態 $S_{initial} = (x, 0)$ から開始し、終了条件$(x \leq reversed)$ を満たすまで $S_{t} = (x_{t}, reversed_{t})$を更新し続ける

Time Complexity = $O(\log_{10} n)$
Space Complexity = $O(1)$

### Data Structure Perspective:
Implicit Array via Positional Notation (位取り記数法による暗黙的配列)

### Mathematical Perspective:
Polynomial Coefficient Extraction & Reconstruction (多項式係数の抽出と再構築)

定義:ある整数 $x$ は、基数 $b=10$ において以下のように表現される
$$x = \sum_{i=0}^{n} a_i \cdot 10^i \quad (0 \le a_i < 10)$$

ex)
$1221 = \mathbf{1} \cdot 10^3 + \mathbf{2} \cdot 10^2 + \mathbf{2} \cdot 10^1 + \mathbf{1} \cdot 10^0$


$係数 = (1, 2, 2, 1)$が対称である = 回文である

アルゴリズムの数学的表現:
以下の漸化式を用いて、数値の下半分を抽出して新たな数値 $R$ （反転数）を構築する。

抽出 (Extraction): $d = x \pmod{10}$

削減 (Reduction): $x_{new} = \lfloor x_{old} / 10 \rfloor$

再構築 (Reconstruction): $R_{new} = R_{old} \cdot 10 + d$

最終的に、元の数値 $x$ （縮小された前半部分）と $R$ （構築された後半部分）が等しいか、あるいは $x = \lfloor R / 10 \rfloor$ （桁数が奇数の場合）であれば、回文であると証明されている。
