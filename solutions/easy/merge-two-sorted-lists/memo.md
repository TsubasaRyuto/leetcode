# Problem
- Title: Merge two sorted lists
- Difficulty:  Easy
- URL: https://leetcode.com/problems/merge-two-sorted-lists
- Accepted: false
- Runtime: 0 MS
- Memory: 2.06 MB

## Summary
### Input:
```rust
list1 = [1,2,4], list2 = [1,3,4]
list1: Option<Box<ListNode>>
list2: Option<Box<ListNode>>

// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
```

### Output:
```rust
[1,1,2,3,4,4]
// ListNode
```

### Constraints:
The number of nodes in both lists is in the range [0, 50]

$-100 \leq Node.val \leq 100$

Both list1 and list2 are sorted in non-decreasing order

### Goal (One-line Summary):
二つのソート済みlistをマージする

---

## My Approach
### My Insight:
1. ダミーノード（空のスタート地点）を一つ用意する。
2. 現在の位置（tail）をダミーノードにセットする。
3. list1 と list2 の両方が空でない間、以下の処理を繰り返す：
   - list1 の値と list2 の値を比べる。
   - 小さい方のノードを tail.next に繋ぐ。
   - 選ばれた方のリストを一つ進める。
   - tail を新しく繋いだノードに進める。
4. どちらかのリストが空になったら、残っている方のリストをそのまま tail.next に繋ぐ（すでにソートされているため）。
5. 最後に、ダミーノードの次（dummy.next）を返す。

### Time Complexity:
$O(N + M)$;

### Space Complexity:
$O(1)$;

### Learning:

#### Box の役割
データをヒープ領域において、アドレスだけを持つこと
LinkedListでは必須

無限に続く再起構造をポインタに置き換えてサイズを確定する
Boxは指してる先のデータを所有してるので、Boxが捨てられるとデータ自体もメモリ解放する

#### as_ref / as_mut
as_refは所有権を奪わずに中身を参照する(不変参照)

as_mutは可変参照

```rust
// 可変参照し、take()で所有権毎nextに格納されているListNodeをnext変数へ移動する
// するとlist1のnextはNoneとなる
let next = list1.as_mut().unwrap().next.take();

// list1 = {
//   val: i32
//   next: None
// }
// な状態ととなる
tail.next = list1;

// list1にはnextを格納する
list1 = next;
```

---

## Insight
### The Core Essence of the Problem:
新しいリストを作るのではなく、既存の2本の鎖を一度バラして、値の小さい順に1本の鎖に編み直す

---

## Abstraction
### Computational Model:
two pointer merge method

### Data Structure Perspective:
Singly Linked List

### Mathematical Perspective:


