# Problem
- Title: Sqrt(x)
- Difficulty:  Easy
- URL: https://leetcode.com/problems/sqrtx/description/
- Accepted: false
- Runtime: - MS
- Memory: - MB

## Summary
### Input:
```rust
// case 1
x = 4

// case 2
x = 8
```

### Output:
```rust
// case 1
2

// case 2
2
```

### Constraints:
- $0 \leq x \leq 2^31 - 1$

### Goal (One-line Summary):
Calculate the integer square root of x without using built-in exponent functions.

---

## Insight
### The Core Essence of the Problem:

## Approaches 1
### Summary:
- Runtime: 0 MS
- Memory: 2.1 MB

### Time Complexity:
$O(\log x)$

### Space Complexity:
$O(1)$


### Code:
src/main.rs

### Learning:

#### Overflow Prevention
`mid * mid <= x` ではなく `mid <= x / mid` を使うことで i32 の境界を意識しなくて良い

##### Binary Search Pattern
条件を満たす最大値を探す際、`ans`変数に暫定値を保持して `left = mid + 1` するパターンは汎用的

※　Binary Searchを実装に落とし込む力が弱い

---

## Abstraction
### Computational Model:
Binary Search

### Data Structure Perspective:
Monotonic Function Search.
$f(k) = k^2$ は $k \ge 0$
で単調増加するため、二分探索が可能。

### Mathematical Perspective:
整数平方根 $\lfloor \sqrt{x} \rfloor$は、
実数空間の解を求めてから切り捨てるのではなく、整数空間内での二分探索で完結できる。

