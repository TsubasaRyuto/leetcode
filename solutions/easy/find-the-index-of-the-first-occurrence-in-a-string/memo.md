# Problem
- Title: Find the Index of the First Occurrence in a String
- Difficulty:  Easy
- URL: https://leetcode.com/problems/find-the-index-of-the-first-occurrence-in-a-string/description/
- Accepted: true
- Runtime: 2 MS
- Memory: 2.2 MB

## Summary
### Input:
```rust
// case1
haystack = "sadbutsad", needle = "sad"

// case2
haystack = "leetcode", needle = "leeto"
```

### Output:
```rust
// case1
0

// case2
-1
```

### Constraints:
$1 \leq haystack.length, needle.length \leq 104$

haystack and needle consist of only lowercase English characters.

### Goal (One-line Summary):
Return the first index where the needle is found within the haystack, or -1 if it is not present.

myself: `If the needle is included in the haystack, return the starting index of the match.`

---

## My Approach
Sliding Window (Brute Force) approach

### My Insight:
Iterate through the haystack.
Extract a substring of the same length as the needle and compare them.
If they match, return the current index.
If the loop finishes without a match, return -1.

### Time Complexity:
$O(n \times m)$

### Space Complexity:
$O(1)$

### Learning:
#### マルチバイト文字（UTF-8）の安全性
RustのStringに対してi..jというスライスを行うと、文字の境界（境界がバイト単位であるため）でパニックを起こす可能性がある。
英数字のみであれば問題ないが、一般的にはバイト列（.as_bytes()）で比較するのが安全。
また、スライス範囲を `0..=(h_len - n_len)` とすることで無駄な計算を省ける。

---

## Insight
### The Core Essence of the Problem:
The "Essence" isn't just finding the word; it's about knowing how much of the haystack you can safely ignore after a mismatch occurs.
This is precisely what KMP optimizes.

## Other Approaches
### Summary:
- Runtime: ~0ms
- Memory: ~2.1MB

### Time Complexity:
$O(n + m)$

### Space Complexity:
$O(m)$

### Code:
```rust
pub fn str_str(haystack: String, needle: String) -> i32 {
  if needle.is_empty() { return 0 }

  let h_bytes = haystack.as_bytes();
  let n_bytes = needle.as_bytes();
  let n_len = n_bytes.len();
  let h_len = h_bytes.len();

  if n_len > h_len { return -1; }

  // Step 1: Precompute LPS (Longest Prefix Suffix) table
  let mut lps = vec![0; n_len];
  let mut prev_lps = 0;
  let mut i = 1;

  while i < n_len {
    if n_bytes[i] == n_bytes[prev_lps] {
      lps[i] = prev_lps + 1;
      prev_lps += 1;
      i += 1;
    } else if prev_lps == 0 {
      lps[i] = 0;
      i += 1;
    } else {
      prev_lps = lps[prev_lps - 1];      
    }
  }

  // Step 2: Search the haystack using the LPS table
  let mut h_idx = 0;
  let mut n_idx = 0;

  while h_idx < h_len {
    if h_bytes[h_idx] == n_bytes[n_idx] {
      h_idx += 1;
      n_idx += 1;
    } else {
      if n_idx == 0 {
        h_idx += 1;
      } else {
        // This is the Magic: we don't reset h_idx.
        // We only move n_idx back based on our LPS table.
        n_idx = lps[n_idx - 1];
      }
    }

    if n_idx == n_len {
      return (h_idx - n_len) as i32;
    }
  }

  -1
}
```

### Learning:
#### KMP Algorithm
KMP (Knuth-Morris-Pratt) minimizes redundant comparisons by precomputing a "Longest Prefix Suffix" (LPS) table. This table tells us the length of the longest proper prefix of `needle[0..i]` that is also a suffix of `needle[0..i]`. When a mismatch occurs at `n_idx`, we know that the first `lps[n_idx-1]` characters already match, so we can skip ahead without resetting the `h_idx`.

---

## Abstraction
### Computational Model:
Finite State Automaton (FSA). KMP can be viewed as building a state machine where each character in the needle is a state.

### Data Structure Perspective:
Using a precomputed array (LPS) as a lookup table to manage state transitions efficiently.

### Mathematical Perspective:
Exploiting the self-similarity (symmetry) within the `needle` string to avoid $O(n \times m)$ worst-case behavior.

