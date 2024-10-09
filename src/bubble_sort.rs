pub fn bubble_sort(s: &mut Vec<char>) {
    let n = s.len();
    for i in 0..n {
        for j in 0..n - i - 1 {
            if s[j] > s[j + 1] {
                s.swap(j, j + 1);
            }
        }
    }
}