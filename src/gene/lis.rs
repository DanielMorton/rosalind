fn find_min_greater(nums: &[u32], subseq_end: &[usize], element: u32) -> usize {
    let (mut low, mut high) = (0, subseq_end.len() - 1);
    let mut mid: usize;
    while low < high {
        mid = (low + high) / 2;
        if nums[subseq_end[mid]] < element {
            low = mid + 1
        } else {
            high = mid
        }
    }
    low
}
pub(crate) fn longest_increasing_sequence(nums: &[u32]) -> Vec<u32> {
    let mut subseq_end = Vec::new();
    let mut parent = vec![None; nums.len()];
    nums.iter().enumerate().for_each(|(i, &n)| {
        if subseq_end.is_empty() || n > nums[*subseq_end.last().unwrap()] {
            if !subseq_end.is_empty() {
                parent[i] = subseq_end.last().copied();
            }
            subseq_end.push(i);
        } else {
            let replace = find_min_greater(nums, &subseq_end, n);
            subseq_end[replace] = i;
            if replace > 0 {
                parent[i] = Some(subseq_end[replace - 1]);
            }
        }
    });
    let mut curr = subseq_end.last().copied();
    let mut lcs = Vec::new();
    while curr.is_some() {
        lcs.push(nums[curr.unwrap()]);
        curr = parent[curr.unwrap()];
    }
    lcs.reverse();
    lcs
}

pub(crate) fn longest_decreasing_sequence(nums: &[u32]) -> Vec<u32> {
    let reverse = nums.iter().rev().copied().collect::<Vec<_>>();
    longest_increasing_sequence(&reverse)
        .iter()
        .rev()
        .copied()
        .collect::<Vec<_>>()
}
