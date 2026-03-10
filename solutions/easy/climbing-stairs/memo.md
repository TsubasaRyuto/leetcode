# Problem
- Title: Climbing Stairs
- Difficulty:  Easy
- URL: https://leetcode.com/problems/climbing-stairs/description/
- Accepted: false
- Runtime: MS
- Memory: MB

## Summary
### Input:
```rust
// case 1
n = 2

// case 2
n = 3
```

### Output:
```rust
// case 1
2

// case 2
3
```

### Constraints:
$1 \leq n \leq 45$

### Goal (One-line Summary):
Return the number of combinations of ways to climb to the top

---

## My Approach
### My Insight:
1. Loop while 1..=n/2
-> Iterate from 1 up to n/2.
2. Add the combination result, ${}_{n-i} C_{i}$.
-> Sum the combinations  ${}_{n-i}C_i$
3. return result + 1
-> Return the total sum plus 1.
4. why it plus one at last, it is all one case.
-> Note: The final "+1" represents the case where only 1-step climbs are used.

### Time Complexity:
$O(N^2)$

### Space Complexity:
$O(1)$

### Learning:
TODO: オーバーフローにならないようにどうしたらいいかを思考する

---

## Insight
### The Core Essence of the Problem:

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
impl Solution {
  pub fn climb_stairs(n: i32) -> i32 {
    if n <= 2 { return n; }

    let mut a = 1;
    let mut b = 2;

    for _ in 3..=n {
      let temp = a + b;
      a = b;
      b = temp;
    }

    b
  }
}
```

### Learning:
#### Dynamic Programming
部分問題への分解と重複する計算の排除で計算量を減らすことができる

今回の場合、「$n$ 段目に登る」という大きな問題を、「$n-1$ 段目からの一歩」と「$n-2$ 段目からの一歩」という小さな問題に分解した

#### 状態遷移
アルゴリズム問題で「パターンの総数」を問われたとき、「ターゲット（$n$ 段目）に到達する直前の状態は何通りあるか？」を考えると大体DPに収まる

---

## Abstraction
### Computational Model:
DynamicProgramming (DP)

### Data Structure Perspective:
Implicit Directed Acyclic Graph (DAG)
(有効非巡回グラフ)

### Mathematical Perspective:
Linear Recurrence Relation (線形漸化式)
