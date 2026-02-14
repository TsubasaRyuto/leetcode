fn main() {
    println!("Hello, world!");
}

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

struct Solution {}
impl Solution {
    pub fn merge_two_lists(
        list1: Option<Box<ListNode>>,
        list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut list1 = list1;
        let mut list2 = list2;
        let mut dummy = ListNode::new(0);
        let mut tail = &mut dummy;

        while list1.is_some() && list2.is_some() {
            if list1.as_ref().unwrap().val <= list2.as_ref().unwrap().val {
                let next = list1.as_mut().unwrap().next.take();
                tail.next = list1;
                list1 = next;
            } else {
                let next = list2.as_mut().unwrap().next.take();
                tail.next = list2;
                list2 = next;
            }

            tail = tail.next.as_mut().unwrap();
        }

        tail.next = if list1.is_some() { list1 } else { list2 };

        // 最初に0のListNodeを作ってるため、nextを返す
        dummy.next
    }
}

// 1. ダミーノード（空のスタート地点）を一つ用意する。
// 2. 現在の位置（tail）をダミーノードにセットする。
// 3. list1 と list2 の両方が空でない間、以下の処理を繰り返す：
//    - list1 の値と list2 の値を比べる。
//    - 小さい方のノードを tail.next に繋ぐ。
//    - 選ばれた方のリストを一つ進める。
//    - tail を新しく繋いだノードに進める。
// 4. どちらかのリストが空になったら、残っている方のリストをそのまま tail.next に繋ぐ（すでにソートされているため）。
// 5. 最後に、ダミーノードの次（dummy.next）を返す。
