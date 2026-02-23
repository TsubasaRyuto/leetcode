# Problem
- Title: Length of Last word
- Difficulty:  Easy
- URL: https://leetcode.com/problems/length-of-last-word/description/
- Accepted: true
- Runtime: 0 MS
- Memory: 2.2 MB

## Summary
### Input:
```rust
// case 1
s = "Hello World"

// case 2
s = "   fly me   to   the moon  "
```

### Output:
```rust
// case 1
5

// case 2
4
```

### Constraints:
- $1 \leq s.length \leq 10^4$
- s consists of only English letters and spaces ' '.
- There will be at least one word in s.

### Goal (One-line Summary):
Given a string s consisting of words and spaces, return the length of the last word in the string.

---

## My Approach
### My Insight:
Algorithm: Length of Last Word
1. Split the string by spaces and filter out any empty substrings.
2. Identify the last element in the resulting list.
3. Return the length of the identified world.

### Time Complexity:
$O(N)$

### Space Complexity:
$O(N)$

### Learning:

#### `.split_whitespace` method

My approach:
```rust
s.split(' ').into_iter().filter(|x| *x != "")
```

Rust's standard library
```rust
s.split_whitespace()
```

this method is built specifically to handle these common parsing tasks safely and efeciently.

difference between the following codes;
```rust
fn main() {
    // A string with multiple spaces, a tab (\t), and a newline (\n)
    let s = "Rust  is   \tvery\nfast  ";

    let manual_split: Vec<&str> = s.split(' ')
        .filter(|x| *x != "")
        .collect();

    let whitespace_split: Vec<&str> = s.split_whitespace()
        .collect();

    println!("Manual Split:     {:?}", manual_split);
    // => Manual Split:     ["Rust", "is", "\tvery\nfast"]

    println!("split_whitespace: {:?}", whitespace_split);
    // => split_whitespace: ["Rust", "is", "very", "fast"]
}
```

#### Option<T>による安全なインデックス
My approach:
Checked if `splited_words.is_empty()` and then used `splited_words[splited_words.len() - 1]`.

Good code:
Direct indexing (using brackets[]) in Rust whill cause the program to panic (crash) if the index is out of bounds.
While your if check prevented a crash, Rust prefers handling missing data through the Option enum. methods like `.last()` return an `Option<&str>`

---

## Insight
### The Core Essence of the Problem:
いかに効率的かつ堅牢に末尾の空白を処理するか

## Other Approaches
### Summary:
- Runtime: 
- Memory: 

### Time Complexity:
$O(N)$

### Space Complexity:
$O(1)$

### Code:
```rust

pub fn lenght_of_last_word(s: String) -> i32 {
  s.trim_end()
   .split_whitespace()
   .next_back()
   .map(|w| w.len() as i32)
   .unwrap_or(0)
}
```

### Learning:

---

## Abstraction
### Computational Model:
Finite State Automaton

### Data Structure Perspective:

### Mathematical Perspective:
Function Mapping

Mathematically, we can define the problem as a function $f$ that maps a string $S$ from an alphabet $\Sigma$ to a non-negative integer.

Let $S$ be a sequence of characters $c_1, c_2, \dots, c_n$.
Let $I$ be the set of indices of non-space characters: $I = \{i \mid c_i \neq \text{' '}\}$.If $I = \emptyset$, then $f(S) = 0$.
Otherwise, let $k$ be the maximum index in $I$ (the end of the last word):
$$k = \max(I)$$

The "Length of the Last Word" is the number of contiguous non-space characters ending at $k$.
If we define $j$ as the largest index less than $k$ such that $c_j$ is a space:
$$j = \max(\{i \mid i < k \land c_i = \text{' '}\} \cup \{0\})$$

Then the length is simply:
$$f(S) = k - j$$
