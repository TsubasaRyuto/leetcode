# Problem
- Title: Plus One
- Difficulty:  Easy
- URL: https://leetcode.com/problems/plus-one/description/
- Accepted: true
- Runtime: 0 MS
- Memory: 2.2 MB

## Summary
### Input:
```rust
// case 1
digits = [1, 2, 3]

// case 2
digits = [9]
```

### Output:
```rust
// case 1
[1, 2, 4]

// case 2
[1, 0]
```

### Constraints:
- $1 \leq digits.length \leq 100$
- $0 \leq digits[i] \leq 9$
- digits does not contain any leading 0's.

### Goal (One-line Summary):
Return a new array representing the result of adding one to an integer stored as an array of digits.

---

## My Approach
### My Insight:
1. Iterate backwards through the array.
2. if digits[current_index] + 1 is less than 10:
   increment the current digit and return the array.
3. if digits[current_index] + 1 equals 10:
   set the current digit to 0 and continue to the next iteration.
4. if the loop finishes without returning, prepend 1 to the array and return it.

### Time Complexity:
$O(N)$

### Space Complexity:
$O(1)$ (In-place mutation).
Note: `digits.insert(0, 1)` takes $O(N)$ time and may trigger a reallocation, but it is only done once at most.

### Learning:
- Rust Iterators:
Using `digits.iter_mut().rev()` is more idiomatic than manual indexing with `i32` and `usize` conversions.
It eliminates bounds checking and makes the intent clearer.

- In-place Mutation:
Mutating the input vector in-place is efficient ($O(1)$ additional space beyond the input) unless the vector needs to grow.

---

## Insight
### The Core Essence of the Problem:
The problem is a simplified simulation of base-10 addition with carry propagation.
Since we only add 1, the carry only continues as long as we encounter the digit `9`.

---

## Abstraction
### Computational Model:
- **Finite State Machine / Carry Propagation:**
The process starts at the least significant digit (LSD) and propagates a "carry" state to the left until a digit less than 9 is found or the beginning of the array is reached.

### Data Structure Perspective:
- **BigInt representation:**
The array is essentially a custom "BigInt" representation.

- **Prepend Cost:**
Inserting at the front of a `Vec` (`digits.insert(0, 1)`) is $O(N)$. For this specific problem, it only occurs once (at most) when the number is all 9s (e.g., 999 -> 1000).

### Mathematical Perspective:
- Adding 1 to $x$ where $x = \sum_{i=0}^{n-1} digits[i] \cdot 10^{n-1-i}$.
- If $digits[n-1] < 9$, then $digits[n-1] \to digits[n-1] + 1$.
- If $digits[n-1] = 9$, then $digits[n-1] \to 0$ and recursively apply "Plus One" to the more significant digits.

