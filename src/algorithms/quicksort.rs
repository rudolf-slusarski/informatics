#![allow(clippy::ptr_arg)]
use rand::{distributions::Uniform, Rng};
use std::{
    io::Error,
    time::{Duration, Instant},
};

fn quicksort(arr: &mut Vec<isize>, p: isize, r: isize) -> Vec<isize> {
    if p < r {
        let q = partition(arr, p, r);
        quicksort(arr, p, q);
        quicksort(arr, q + 1, r);
    }
    arr.clone()
}

fn partition(arr: &mut Vec<isize>, p: isize, r: isize) -> isize {
    let x = arr[r as usize];
    let mut i = p as isize - 1;
    for (_, j) in (p..=r).enumerate() {
        if arr[j as usize] <= x {
            i += 1;
            arr.swap(i as usize, j as usize);
        }
    }
    if i < r {
        i
    } else {
        i - 1
    }
}

pub fn run() -> Result<(), Error> {
    let total_time = Instant::now();
    let range = Uniform::from(0..=1_000_000);
    let descending: Vec<isize> = (1..=100_000).rev().collect();

    println!(
        "{:^7} | {:^12} | {:^12}",
        "sample", "qs (random)", "qs (worst)"
    );
    println!("--------------------------------------");

    for i in (10_000..=100_000).step_by(5000) {
        let mut random_numbers: Vec<isize> =
            rand::thread_rng().sample_iter(&range).take(i).collect();
        let mut worst_case: Vec<isize> = descending.clone().into_iter().take(i).collect();
        let mut results: Vec<Duration> = vec![];

        let time = Instant::now();
        quicksort(&mut random_numbers, 0, i as isize - 1);
        results.push(time.elapsed());

        let time = Instant::now();
        quicksort(&mut worst_case, 0, i as isize - 1);
        results.push(time.elapsed());

        println!(
            "{:^7} | {:>12} | {:>12}",
            i,
            format_args!("{:?}", results[0]),
            format_args!("{:?}", results[1]),
        );
    }

    println!("total time: {:?}", total_time.elapsed());
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::{distributions::Uniform, Rng};

    #[test]
    fn quicksort_random_250() {
        let range = Uniform::from(0..=1_000);
        let mut random_250: Vec<isize> = rand::thread_rng().sample_iter(&range).take(250).collect();
        assert!(quicksort(&mut random_250, 0, 249).is_sorted());
    }

    #[test]
    fn quicksort_random_10_000() {
        let range = Uniform::from(0..=1_000_000);
        let mut random_10_000: Vec<isize> = rand::thread_rng()
            .sample_iter(&range)
            .take(10_000)
            .collect();
        assert!(quicksort(&mut random_10_000, 0, 9_999).is_sorted());
    }

    #[test]
    fn quicksort_empty() {
        let mut arr: Vec<isize> = vec![];
        let sorted: Vec<isize> = vec![];
        assert_eq!(quicksort(&mut arr, 0, 0), sorted);
    }

    #[test]
    fn quicksort_all() {
        let mut arr: Vec<isize> = vec![3, 546, 32, 11, 6546, 65, 86, 11, 14, 12];
        let sorted: Vec<isize> = vec![3, 11, 11, 12, 14, 32, 65, 86, 546, 6546];
        assert_eq!(quicksort(&mut arr, 0, 9), sorted);
    }

    #[test]
    fn quicksort_slice_of_5() {
        let mut arr: Vec<isize> = vec![3, 546, 32, 11, 6546, 65, 86, 11, 14, 1];
        let sorted: Vec<isize> = vec![3, 546, 11, 32, 65, 86, 6546, 11, 14, 1];
        assert_eq!(quicksort(&mut arr, 2, 6), sorted);
    }

    #[test]
    fn quicksort_one() {
        let mut arr: Vec<isize> = vec![3, 546, 32, 11, 6546, 65, 86, 11, 14, 1];
        assert_eq!(quicksort(&mut arr, 3, 3), arr);
    }
}
