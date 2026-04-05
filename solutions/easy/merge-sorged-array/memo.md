# Problem
- Title: Merge Sorted Array
- Difficulty:  Easy
- URL: https://leetcode.com/problems/merge-sorted-array/description/
- Accepted: true
- Runtime: 0 MS
- Memory: 2.4 MB

## Summary
### Input:
```rust
// Case1: Input
nums1 = [1,2,3,0,0,0], m = 3, nums2 = [2,5,6], n = 3


// Case2: Input
nums1 = [1], m = 1, nums2 = [], n = 0
```

### Output:
```rust
// Case1: Output
[1,2,2,3,5,6]

// Case2: Input
[1]
```

### Constraints:
* `nums1.length` $== m + n$
* `nums2.length` $== n$
* $0 \le m, n \le 200$
* $1 \le m + n \le 200$
* $-10^9 \le nums1[i], nums2[j] \le 10^9$

### Goal (One-line Summary):
Merge nums2 into nums1 and sort the result.

---

## My Approach
### My Insight:
Iterate through nums2 from index 0 to n-1. For each element, update the value at nums1[index + m]
with the current value from nums2.
Finally, sorting nums1 will produce the merged result.

### Time Complexity:
$$O(n) + O((m+n) \log (m+n)) = \mathbf{O((m+n) \log (m+n))}$$

（Rustの sort()）の時間計算量は、対象の要素数を $N$ とすると $O(N \log N)$に引っ張られる

### Space Complexity:
O(1)

### Learning:
* Using `sort()` is a simple and intuitive way to solve this problem, but it doesn't take full advantage of the fact that the input arrays are already sorted.

---

## Insight
### The Core Essence of the Problem:
How to merge two sorted arrays in $O(m+n)$ time without using extra space.

## Other Approaches
### Summary:
- Runtime: 0 MS
- Memory: 2.4 MB

### Time Complexity:
$O(m + n)$

### Space Complexity:
$O(1)$

### Code:
```rust
impl Solution {
  pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
    let mut i = m - 1; // nums1の有効な数字の末尾
    let mut j = n - 1; // nums2の末尾
    let mut k = (m + n - 1) as usize; // nums1の本当の末尾（書き込み先）

    // nums2を使い切るまで繰り返す
    while j >= 0 {
      if i >= 0 && nums1[i as usize] > nums2[j as usize] {
        nums1[k] = nums1[i as usize];
        i -= 1;
      } else {
        nums1[k] = nums2[j as usize];
        j -= 1;
      }

      if k == 0 { break; }
      k -= 1;
    }
  }
}
```

sort が不要になる

### Learning:
* **「後ろから埋める」という逆転の発想:** `nums1` の後ろ半分に空きスペースがあるという制約を活かし、大きな値から順に後ろから埋めていくことで、既存の要素（`nums1` の前方の要素）を上書きしたり退避させたりすることなく $O(1)$ の追加スペースでマージが可能になる。
* 配列操作において、前方からの操作で要素のシフトが発生する場合は、後方からの操作が可能か検討するのが定石。

---

## Abstraction
### Computational Model:

### Data Structure Perspective:

### Mathematical Perspective:

