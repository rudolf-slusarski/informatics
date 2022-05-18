fn insertion_sort(arr: &mut Vec<i32>) -> Vec<i32> {
    for i in 1..arr.len() {
        let mut j: usize = i;
        while j > 0 && arr[j - 1] > arr[j] {
            arr.swap(j - 1, j);
            j -= 1;
        }
    }
    arr.to_vec()
}

#[cfg(test)]
mod tests {
    use rand::{distributions::Uniform, Rng};

    use crate::algorithms::insertion_sort::insertion_sort;

    #[test]
    fn insertion_sort_random_10_000() {
        let range = Uniform::from(0..=1_000_000);
        let mut random_10_000: Vec<i32> = rand::thread_rng()
            .sample_iter(&range)
            .take(10_000)
            .collect();
        assert!(insertion_sort(&mut random_10_000).is_sorted());
    }
}
