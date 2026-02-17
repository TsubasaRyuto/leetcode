# Problem
- Title: Remove Element
- Difficulty:  Easy
- URL: https://leetcode.com/problems/remove-element/description/
- Accepted: true
- Runtime: 0 MS
- Memory: 2.17 MB

## Summary
### Input:
```rust
nums = [3,2,2,3], val = 3
```

### Output:
```rust
the return value is 2
and update the nums array to [2,2,_,_]
```

### Constraints:
$0 \leq nums.length \leq 100$
$0 \leq nums[i] \leq 50$
$0 \leq val \leq 100$

### Goal (One-line Summary):
Filter the nums array in-place by overwriting elements that match the target val.

---

## My Approach
### My Insight:
Modifying the array in-place to "remove" the target value
1. Track the writer index
2. Iterate through the nums array:
   - if the nums[reader] is not equal val
      | This is a value we want to keep
      | so update the element at the writer index
      | nums[writer] = nums[reader]
      | and increment the `writer`
3. Finally, return `writer` as the new length of the modified array.

### Time Complexity:
$O(n)$

### Space Complexity:
$O(1)$

### Learning:
#### Handling the Empty Case
it's actually redundant. if the vector is empty, the 0..nums.len() range will be emppty, the loop won't execute and writer (0) will be returned naturally.

---

## Abstraction
### Computational Model:
2ポインタ法 (Read/Write分離), インプレース更新
1つの配列に対して、スキャン用の reader と書き込み用の writer という2つの異なる役割（ポインタ）を定義し、条件を満たす要素だけを前方に集約するモデル

### Data Structure Perspective:
連続メモリ領域, キャッシュ局所性
配列の「要素の削除には後続のシフトが必要で $O(N)$ かかる」という弱点を、有効な要素を上書き（Overwrite）していくことで克服

### Mathematical Perspective:
ループ不変条件 (Loop Invariant), 集合の分割
ループの各ステップにおいて、常に以下の条件（不変条件）が成立するように設計されています。「インデックス $0$ から $writer - 1$ までの部分配列には、削除対象の $val$ が一切含まれない」
