# Problem
- Title: Remove Duplicates from Sorted Array
- Difficulty:  Easy
- URL: https://leetcode.com/problems/remove-duplicates-from-sorted-array/
- Accepted: true
- Runtime: 0 MS
- Memory: 2.2 MB

## Summary
### Input:
```rust
[0,0,1,1,1,2,2,3,3,4]
```

### Output:
```rust
return 5

and update nums = [0,1,2,3,4,_,_,_,_,_]
```

### Constraints:
$1 \leq nums.length \leq 3 * 104$

$-100 \leq nums[i] \leq 100$

nums is sorted in non-decreasing order.

### Goal (One-line Summary):
Update the element and return unique elements.

---

## My Approach
### My Insight:
Modifying the array in-place to "remove" duplicates
1. Track the value of the last unique element found.
2. Track the index where the next unique element should be placed.
3. Iterate through the nums array:
   - if the current num matches latest_new_num: it's a duplicate, so skip it.
   - if they don't match: Update the element at the next index in nums,
     then update latest_new_num and increment latest_new_num_index.
4. Finally, return latest_new_num_index + 1 as the count of unique elements.

### Time Complexity:
$O(n)$

### Space Complexity:
$O(1)$

### Learning:
#### review-1
Use the `is_empty()` method instead of checking `len() == 0`. It is more idiomatic and readable in Rust.

before
```rust
if nums.len() == 0 {}
```

after
```rust
if nums.is_empty() {}
```

#### review-2
There is no need to manually track the last unique element in a separate variable. I can directly access it from the nums array using the current index.

#### Vec::first() return Option<&T>
`Vec::first()` method returns the `Option<&T>` type, taking into account the case when the array is empty.

so, I should use `array[index]`.

---

## Insight
### The Core Essence of the Problem:
The essence is data partitioning. I'm partitioning the array into two zones
Indices 0 to writer - 1: The clean, unique, sorted elements.
Indices Writer to end: Garbage/Leftover data that we no longer care about.

---

## Abstraction
### Computational Model:
2ポインタ法, RAMモデル

$O(N)$ 時間と $O(1)$ 空間で効率的に処理する機械的手順

### Data Structure Perspective:
連続メモリ

上書き配列の欠点（削除のコスト）を回避し、キャッシュ効率を活かす

### Mathematical Perspective:
多重集合, 単調性, 不変条件

ソートの性質を利用し、重複判定を隣接比較のみに簡略化する
