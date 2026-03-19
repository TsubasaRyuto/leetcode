# Problem
- Title: Remove Duplicates from Sorted List
- Difficulty:  Easy
- URL: https://leetcode.com/problems/remove-duplicates-from-sorted-list/description/
- Accepted: true
- Runtime: 0 MS
- Memory: 2.2 MB

## Summary
### Input:
```rust
head = [1, 1, 2, 3, 3]
```

### Output:
```rust
[1, 2, 3]
```

### Constraints:
- The number of nodes in the list is in the range [0, 300].
- $-100 \leq Node.val \leq 100$
- The list is guaranteed to be sorted in ascending order.

### Goal (One-line Summary):
delete all duplicates node from sorted linked list.

---

## My Approach
### My Insight:
Iterate until the next node is None
If the current node's value equals the next node's value,
skip the next node by pointing to the "next of next" node.
Otherwise, move the current pointer to the next node.

### Time Complexity:
$O(N)$

### Space Complexity:
$O(1)$

### Learning:

#### Ownership
- `.unwrap()` はownershipがmoveする

#### Option
- `.as_ref()` はownershipを移動させずに参照する権利を得る
- `.as_mut()` はownershipを移動させずに変更する権利を得る
- `.tak()` は中身だけを取得し、元のメモリにはNoneを残す

