use std::{
    fs::File,
    io::{BufRead, BufReader, Error, Write},
};

use rand::{distributions::Uniform, Rng};

fn heapify(a: &mut Vec<u32>, i: usize, heapsize: usize) {
    let l = 2 * i + 1;
    let r = 2 * i + 2;
    let mut largest = if l <= heapsize && a[l] > a[i] { l } else { i };
    if r <= heapsize && a[r] > a[largest] {
        largest = r
    };
    if largest != i {
        a.swap(i, largest);
        heapify(a, largest, heapsize);
    }
}

// fn heapify_iter(a: &mut Vec<u32>, mut i: usize, heapsize: usize) {
//     while i <= heapsize {
//         let l = 2 * i + 1;
//         let r = 2 * i + 2;
//         let mut largest = if l <= heapsize && a[l] > a[i] { l } else { i };
//         if r <= heapsize && a[r] > a[largest] {
//             largest = r
//         };
//         if largest != i {
//             a.swap(i, largest);
//             i = largest;
//         } else {
//             break;
//         }
//     }
// }

fn build_heap(mut a: &mut Vec<u32>) {
    let heapsize = a.len() - 1;
    for i in (0..a.len() / 2).rev() {
        heapify(&mut a, i, heapsize)
    }
}

fn heapsort(mut a: Vec<u32>) -> Vec<u32> {
    build_heap(&mut a);
    let mut heapsize = a.len() - 1;

    for _ in (1..a.len()).rev() {
        a.swap(0, heapsize);
        heapsize -= 1;
        heapify(&mut a, 0, heapsize);
    }

    a
}
fn write_numbers(n: Vec<u32>, filename: &str) -> Result<(), Error> {
    let mut output = File::create(filename)?;
    for i in n {
        writeln!(output, "{}", i)?;
    }

    Ok(())
}

pub fn run() -> Result<(), Error> {
    let range = Uniform::from(0..=u32::MAX);
    let random_numbers: Vec<u32> = rand::thread_rng().sample_iter(&range).take(250).collect();
    write_numbers(random_numbers, "numbers.txt")?;

    let input = File::open("numbers.txt")?;
    let buffer = BufReader::new(input);
    let numbers: Vec<u32> = buffer
        .lines()
        .map(|line| line.unwrap().parse::<u32>().unwrap())
        .collect();

    write_numbers(heapsort(numbers), "sorted_numbers.txt")?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use rand::{distributions::Uniform, Rng};

    use crate::algorithms::heapsort::heapsort;

    #[test]
    fn heapsort_random_250() {
        let range = Uniform::from(0..=1_000);
        let random_250: Vec<u32> = rand::thread_rng().sample_iter(&range).take(250).collect();
        assert!(heapsort(random_250).is_sorted());
    }

    #[test]
    fn heapsort_random_10_000() {
        let range = Uniform::from(0..=1_000_000);
        let random_10_000: Vec<u32> = rand::thread_rng()
            .sample_iter(&range)
            .take(10_000)
            .collect();
        assert!(heapsort(random_10_000).is_sorted());
    }
}
