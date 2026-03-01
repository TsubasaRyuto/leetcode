# Problem
- Title: Add Binary
- Difficulty:  Easy
- URL: https://leetcode.com/problems/add-binary/description/
- Accepted: false
- Runtime: - MS
- Memory: - MB

## Summary
### Input:
```rust
a = "11", b = "1"
```

### Output:
```rust
"100"
```

### Constraints:
- $1 \leq a.length, b.length \leq 10^4$
- a and b consist only of '0' or '1' characters.
- Each string does not contain leading zeros except for the zero itself.

### Goal (One-line Summary):
Given two binary strings a and b, return their sum as a binary string.

---

## My Approach
### My Insight:
1. Pseudo-Conversion to a Numeric Value
   interpret a binary string as if it were a decimal number(e.g. "11")

2. Batch Addition
   Perform adition treating the values as decimal numbers.
   Create a number where each digit represents the sum of the corresponding digits

3. Digit-by-Digit "binary Conversion" Adjustment
  Starting from the lowest digit, check each position.
  Whenever a digits is "2" or there is acarry, rewrite it according to binary rules(i.e., 2 -> 0 with a carry of 1 to the next higher digit), updating one character at a time.

### Time Complexity:
$O(N)$
- $N$ is the length of the longer string. Parsing and the adjustment loop both scale linearly.

### Space Complexity:
$O(N)$
- Requires space for the intermediate sum string and the final result.

### Learning:
- **Critical Limitation (Overflow):** 
  - My initial approach used `a.parse::<i64>().unwrap()`, which interprets "11" as the decimal number eleven.
  - While this works for tiny strings, `i64` can only hold up to $\approx 9 \times 10^{18}$ (19 digits).
  - The problem constraint is $10^4$ characters. This approach will **panic** due to overflow for any input longer than 19 bits.
  - **Lesson:** Always check constraints ($10^4$) against the capacity of primitive types. For "Big Integer" problems, bit-by-bit processing is mandatory in Rust unless using a `BigInt` library.

---

## Insight
### The Core Essence of the Problem:
1. The Full Adder Logic
2. The Ripple Effect
3. Beyond the string

## Other Approaches
### Summary:
- Runtime: 
- Memory: 

### Time Complexity:
$O(N)$

### Space Complexity:
$O(N)$

### Code:
```rust
pub fn add_binary(a: String, b: String) -> String {
  let mut result = String::new();
  let mut a_bytes = a.as_bytes().iter().rev();
  let mut b_bytes = b.as_bytes().iter().rev();
  let mut carry = 0;

  while carry > 0 || a_bytes.clone().next().is_some() || b_bytes.clone().next().is_some() {
    let sum = carry
      + a_bytes.next().map_or(0, |&bit| (bit - b'0') as u32)
      + b_bytes.next().map_or(0, |&bit| (bit - b'0') as u32);

    result.push(if sum % 2 == 0 { '0' } else { '1' });
    carry = sum / 2; 
  }
  result.chars().rev().collect()
}

```

### Learning:
#### Algorithm Essentials / アルゴリズムの要点
- **Iterate from the tail / 末尾からの走査**
  - Process strings from the last character to the first.
- **Separation of sum and carry / 和と繰り上がりの分離**
  - **Column Sum / 各桁の合計**: $S = digit_a + digit_b + previous\_carry$
  - **Current Digit / 現在の桁**: $S \pmod 2$ (remainder)
  - **Carry to Next / 次への繰り上がり**: $S \div 2$ (quotient)
- **Final Reversal / 最後に反転**
  - The result is built in reverse order, so a final `rev().collect()` is needed.

#### Rust Syntax / Rust構文
##### `map_or(default, f)`
**Definition / 定義:** Returns the provided default result (if None), or applies a function to the contained value (if Some).
値があれば関数を適用して加工し、なければデフォルト値を返す。

```rust
let val = some_option.map_or(default_value, |v| transformation_logic);
```

**Example / 例:**
```rust
fn main() {
    let s = Some("Rust");
    let empty: Option<&str> = None;

    // When Some(s): return s.len() => 4
    let len1 = s.map_or(0, |x| x.len());
    // When None: return 0
    let len2 = empty.map_or(0, |x| x.len());

    println!("len1: {}", len1); // out: 4
    println!("len2: {}", len2); // out: 0
}
```

##### `next()`
**Definition / 定義:** Advances the iterator and returns the next value inside `Option`.
イテレータの内部状態（カーソル）を更新しながら、現在の要素を `Option` 型で取得する。
`Iterator` トレイトの `fn next(&mut self) -> Option<Self::Item>` という定義が示す通り、**イテレータ自身を可変 (`mut`) として扱う必要がある**のが大きな特徴。

1. `next()` を呼ぶたびに、イテレータ内部の「指し示している位置（カーソル）」が一つ進む（状態が変化する）。
2. `iter()` を使っている場合、元のコレクションの所有権は奪わず、内部カーソルのみが進む。
3. 全要素を返しきると `None` を返す。

image
```txt
iter = [1, 2, 3] // イテレータは mutable である必要がある: let mut iter = ...

iter.next() // カーソルが 2 に進む
→ Some(&1)

iter.next() // カーソルが 3 に進む
→ Some(&2)

iter.next() // カーソルが終端に進む
→ Some(&3)

iter.next()
→ None
```
Rust の `iter.next()`（`iter()` メソッドから生成された場合）の実際の挙動は、値そのものではなく**要素への参照**を返す。
そのため、計算に値を使用する場合は、パターンマッチング (`|&bit|`) やデリファレンス (`*`) で中身を取り出す必要がある。

---

## Abstraction
### Computational Model:
- Iterative Process
- Full Adder simulator

### Data Structure Perspective:
- String Dinamic Array
- LIFO

### Mathematical Perspective:
- 剰余と商の利用
- Positional Notation(位置取り記法)
