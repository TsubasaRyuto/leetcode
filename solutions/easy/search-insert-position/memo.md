# Problem
- Title: Search Insert Position
- Difficulty:  Easy
- URL: https://leetcode.com/problems/search-insert-position/description/
- Accepted: true
- Runtime: 0 MS
- Memory: 2.2 MB

## Summary
### Input:
```rust
nums = [1,3,5,6], target = 5
```

### Output:
```rust
2
```

### Constraints:
- $1 \leq nums.length \leq 10^4$
- $-10^4 \leq nums[i] \leq 10^4$
- nums contains distinct values sorted in ascending order.
- $-10^4 \leq target \leq 10^4$

### Goal (One-line Summary):
Search the insert position of target value from sorted array.

---

## My Approach
### My Insight:
Divide the nums array into halves to find the insertion position.
1. Divide the sorted array into two halves.
2. Compare the target with the last element of the first half.
3. If the target is greater, narrow the search to the second half;
   otherwise, proceed with the first half.
4. Repeat until a single element remains to determine the insertion index.

### Time Complexity:
$O(log(n))$

### Space Complexity:
$O(1)$

### Learning:
#### データの所有と参照
Vec<i32> vs &[i32]

誤認：
nums という変数の中身を、スライスした新しい配列で「上書き」できると考えていた。

学び：
Rustでは、**データそのもの（Vec）**と、**データの一部を指し示すポインタ（スライス）**を明確に区別する
探索中に nums = ... と更新したいなら、変数 nums は最初からスライス型（&[i32]）として定義しておく

#### Zero-based とHalf-open range
誤認:
「最後の要素まで含めるには -1 が必要」という、他の言語や数学的な直感。

学び:
Rustの範囲指定 a..b は、**「aは含むが、bは含まない」**
0..nums.len() と書けば、自動的にインデックス 0 から len - 1 までが含まる。

逆に 0..nums.len() - 1 と書くと、最後の要素が計算から漏れる

---

## Insight
### The Core Essence of the Problem:
Binary Search

## Other Approaches
### Summary:
- Standard iterative binary search using `left` and `right` pointers to define the search range.
- Runtime: 0 MS
- Memory: 2.1 MB

### Time Complexity:
$O(log(n))$

### Space Complexity:
$O(1)$

### Code:
```rust
fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
  let (mut left, mut right) = (0, nums.len() as i32);

  while left < right {
    let mid = left + (right - left) / 2;
    if nums[mid as usize] < target {
      left = mid + 1;
    } else {
      right = mid;
    }
  }

  left
}
```

### Learning:

---

## Abstraction
### Computational Model:
Binary Search

### Data Structure Perspective:
静的・ソート済み配列

### Mathematical Perspective:
ある条件を満たす最小のインデックスを求める写像

