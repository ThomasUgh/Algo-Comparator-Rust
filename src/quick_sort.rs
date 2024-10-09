pub fn quick_sort(s: &mut [char]) {
    if s.len() <= 1 {
        return;
    }
    let pivot_index = partition(s);
    let (left, right) = s.split_at_mut(pivot_index);
    quick_sort(left);
    quick_sort(&mut right[1..]);
}

fn partition(s: &mut [char]) -> usize {
    let pivot = s[0];
    let mut i = 1;
    for j in 1..s.len() {
        if s[j] < pivot {
            s.swap(i, j);
            i += 1;
        }
    }
    s.swap(0, i - 1);
    i - 1
}