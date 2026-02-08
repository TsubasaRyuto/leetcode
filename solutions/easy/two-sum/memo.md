# Problem
- Title: TwoSum
- Difficulty: Easy
- URL: https://leetcode.com/problems/two-sum/
- Accepted: true
- Runtime: 27 ms
- Memory: 2.1 MB

## Summary
### Input:
```rust
nums = [2,7,11,15], target = 9
```

### Output
```
[0,1]
```

### Constraints:
2 <= nums.length <= 104
-109 <= nums[i] <= 109
-109 <= target <= 109

### Goal (One-line Summary):
与えられた配列の中から合計したら与えられた数字になるペアのindexを配列で返す

---

## My Approach
### My Insight
numsの値を順番に見ていき、target - nums[i]と一致する値がnumsのなかに存在するか判定する

### Time Complexity:
O(n^2)

### Space Complexity:
O(1)

### Learning

#### イテレータによるループ
自分のコード
```rust
for i in 0..nums.len() {
```

Idiomaticなコード
```rust
for (i, &v1) in nums.iter().enumerate() {
```

- indexと同時にその値を直接取得することが可能
- イテレータは「最初から最後まで要素が存在すること」が保証されているため、実行時の境界チェックを省略する最適化
- インデックスの範囲外アクセスが発生する余地がなくなる

#### skip
```rust
for j in (i + 1)..nums.len() {
```
i + 1 から nums.len() - 1 までの「数字」を生成し、それを jに代入
これをiteretorを使って実装する場合

```rust
for (j, &v2) in nums.iter().enumerate().skip(i + 1) {
```
のように、skip(i + 1)で最初のi + 1を読み飛ばすことができる。
結果的に同じ処理となる

---

## Inskjjight
### The Core Essence of the Problem:
何度も繰り返される検索コストを、ハッシュマップ記憶に置き換えることで、処理時間を極限まで削ぎ落とすこと

## Other Approaches
### Summary
- Runtime: 0 ms
- Memory: 2.4 MB

### Time Complexity:
O(n)

### Space Complexity
O(n)

### Code

```rust
use std::collections::HashMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut seen = HashMap::with_capacity(nums.len());

        for (i, &num) in nums.iter().enumerate() {
            let complement = target - num;

            if let Some(&j) = seen.get(&complement) {
                return vec![j as i32, i as i32];
            }

            seen.insert(num, i);
        }

        vec![]
    }
}
```

### Learning

#### HashMapによるメモ化
数学において、写像 $f: A \to B$ は集合 $A$ の各要素に集合 $B$の要素を対応させる
本来のVecのデータ構造はインデックス $\to$ 値であるが、
値 $\to$ インデックスという写像 $f(v) = i$を、ループを回しながら動的に作成(HashMap化)
```rust
// num -> i
seen.insert(num, i);
```
これによりに値からインデックスを逆引きできるようになる

何をメモ化しているのか
既知集合をメモ化してる
target - num = 補数が既知集合に所属しているか判定するアルゴリズムにすることで、
O(n)を実現することができる

#### HashMapを初期化について
HashMap.newをの場合は、要素が増えるたびにメモリの再確保処理が走る可能性があるが、
HashMap.with_capacityを使うことで、あらかじめ指定した分の容量を確保するため、実行時のオーバーヘッドを減らすことができる

---

## Abstraction
### Computational Model:
Time-Space Trade-off（時間と空間の負の相関）

### Data Structure Perspective:
HashTableによる定数時間アクセス

### Mathematical Perspective:
写像による補数(Additive Complement)の所属判定
