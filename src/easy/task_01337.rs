// 1337. The K Weakest Rows in a Matrix
// https://leetcode.com/problems/the-k-weakest-rows-in-a-matrix/

use crate::Solution;

impl Solution {
    pub fn k_weakest_rows(mat: Vec<Vec<i32>>, k: i32) -> Vec<i32> {
        let mut arr = mat
            .iter()
            .map(|v| v.iter().take_while(|&&x| x == 1).count())
            .enumerate()
            .collect::<Vec<_>>();

        arr.sort_unstable_by_key(|(ind, n)| (*n, *ind));
        arr.iter().map(|&x| x.0 as i32).take(k as usize).collect()
    }

    pub fn k_weakest_rows_heap(mat: Vec<Vec<i32>>, k: i32) -> Vec<i32> {
        use std::cmp::Reverse;
        use std::collections::BinaryHeap;

        let mut heap = BinaryHeap::with_capacity(mat.len());
        for (i, row) in mat.iter().enumerate() {
            let soldiers = row.iter().take_while(|&&e| e == 1).count();
            heap.push(Reverse((soldiers, i)));
        }

        let mut res = Vec::with_capacity(k as usize);
        for _ in 0..k {
            if let Some(Reverse(entry)) = heap.pop() {
                res.push(entry.1 as i32);
            }
        }
        res
    }

    // Solution from @seandewar
    // https://leetcode.com/problems/the-k-weakest-rows-in-a-matrix/discuss/1066813/Rust%3A-0ms-Iterator-and-Heap-Solutions-%2B-Comments
    pub fn k_weakest_rows_heap_optimal(mat: Vec<Vec<i32>>, k: i32) -> Vec<i32> {
        // as BinaryHeap is a max-heap by default, we use Reverse() to reverse the ordering of our
        // tuples, so that our heap becomes a min-heap (so we sort in ascending order)
        let row_sums_to_idxs = mat
            .into_iter()
            .enumerate()
            .map(|(i, r)| std::cmp::Reverse((r.into_iter().sum::<i32>(), i)))
            .collect::<Vec<_>>();

        // BinaryHeap::from() uses "heapification", which has time complexity O(n) (not the
        // O(n*logn) you'd typically expect if we push() each element into the heap manually)
        let mut heap = std::collections::BinaryHeap::from(row_sums_to_idxs);

        // BinaryHeap::into_iter_sorted() would be much nicer, but it hasn't been stabilized yet,
        // so we'll need to pop the k elements out manually (if we use into_sorted_vec(), we'll end
        // up popping out all n elements, which would defeat the purpose of our optimization)
        let mut result = Vec::with_capacity(k as _);
        for _ in 0..k {
            result.push((heap.pop().unwrap().0).1 as i32);
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            vec![2, 0, 3],
            Solution::k_weakest_rows(
                vec![
                    vec![1, 1, 0, 0, 0],
                    vec![1, 1, 1, 1, 0],
                    vec![1, 0, 0, 0, 0],
                    vec![1, 1, 0, 0, 0],
                    vec![1, 1, 1, 1, 1]
                ],
                3
            )
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            vec![0, 2],
            Solution::k_weakest_rows(
                vec![
                    vec![1, 0, 0, 0],
                    vec![1, 1, 1, 1],
                    vec![1, 0, 0, 0],
                    vec![1, 0, 0, 0]
                ],
                2
            )
        );
    }
}
