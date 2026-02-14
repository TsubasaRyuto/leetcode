# Problem
- Title: Valid Parentheses
- Difficulty:  Easy
- URL: https://leetcode.com/problems/valid-parentheses
- Accepted: true
- Runtime: 0 MS
- Memory: 2.24 MB

## Summary
### Input:
```rust
"()[]{}"
```

### Output:
```rust
true
```

### Constraints:
$1 \leq s.length \leq 104$

s consists of parentheses only '()[]{}'.

### Goal (One-line Summary):
'()[]{}'開き括弧と閉じ括弧の種類と順序が正しく対応しているかを判定する

---

## My Approach
### My Insight:
スタックを利用して、直近の開き括弧と現在の閉じ括弧が正しく対応しているかを順次確認する。
1. 文字列の長さが奇数の場合は、ペアが成立し得ないため即座に `false` を返す。
2. 開き括弧 `(`, `{`, `[` の場合はスタックに積む。
3. 閉じ括弧 `)`, `}`, `]` の場合はスタックから直近の要素をポップし、対応する開き括弧か判定する。
4. 全ての走査が終わった時点で、スタックが空であれば全ての括弧が正しく閉じられた（`true`）と判断する。

### Time Complexity:
$O(n)$

### Space Complexity:
$O(n)$

### Learning:

---

## Insight
### The Core Essence of the Problem:
この問題の本質は、括弧の入れ子構造が持つ「後入れ先出し（LIFO）」の性質を捉えることです。最後に開いた括弧が最初に閉じられる必要があるため、スタックデータ構造がこの整合性チェックに最適であるという点にあります。

## Other Approaches
### Summary:
- Runtime: 
- Memory: 

### Time Complexity:
$O(n)$

### Space Complexity:
$O(n)$

### Code:
```rust

```

### Learning:

---

## Abstraction
### Computational Model:
プッシュダウン・オートマトン (PDA)

### Data Structure Perspective:
LIFOによる「未解決問題」の管理

### Mathematical Perspective:

