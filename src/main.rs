use rand::prelude::*;
use std::time::Instant;

/// Generate a random-sized list
///
/// # Generic Args
/// - MAX: `usize` - the maximum size of the elements in the array 
///
/// # Args
/// - size:`usize` - length of list
///
/// # Returns
/// `Vec<u32>` representing the resulting random list
fn random_list<const MAX:usize>(size:usize) -> Vec<u32> {
    let mut rng = rand::thread_rng();
    let vals: Vec<u32> = (0..size).map(|_| (rng.gen::<u32>() % MAX as u32)).collect();

    return vals;
}

/// Insertion sort
///
/// # Args
/// - list:`&[u32]` - slice of list to be sorted
///
/// # Returns
/// `Vec<u32>` representing the sorted list
#[allow(dead_code)]
fn insertion_sort(list:&[u32]) -> Vec<u32> {
    // We first clone the slice into a vector which we will
    // modify in place
    let mut vec = Vec::from(list);

    // We will first declare a pointer for the element we are
    // attempting to move around
    for i in 1usize..vec.len() {
        // We will get the current element we are comparing against
        let current = vec[i];
        // We will also get the possible element to swap with, which is
        // possibly the element before. We will swap our way back
        let mut j:usize = i-1;

        // as long as we have not hit the end & the prev number is bigger
        // than the current one

        // a quick note on the neq 0 business. [`usize`] in rust does not
        // allow negative values, as, of course, it is unsigned.
        // Therefore, if we need to swap into the beginning of the list,
        // we have to have a special case for 0 comparisons.
        while j > 0 && vec[j] > current {
            // We swap the current element back, and move the next element
            // into correct order

            // Move the larger element up
            vec[j+1] = vec[j];

            // Move the smaller element down
            vec[j] = current;

            // Move the comparator pointer
            j -= 1
        }

        // Special case for j=0, same swap
        if j == 0 && vec[j] > current {
            vec[j+1] = vec[j];
            vec[j] = current;
        }

    }

    // We finally return the now-sorted vector
    return vec;
}

/// Counting sort
///
/// # Generic Args
/// - MAX: `usize` - the maximum size of the elements in the array 
///
/// # Args
/// - list:`&[u32]` - slice of list to be sorted
///
/// # Returns
/// `Vec<u32>` representing the sorted list
#[allow(dead_code)]
fn counting_sort<const MAX: usize>(list:&[u32]) -> Vec<u32> {
    // We first create the buckets from which we sort
    let mut buckets = [0usize; MAX];

    // We iterate over the list and add to the required bucket
    for i in list {
        buckets[*i as usize] += 1;
    }

    // We then construct our new list
    let mut vec = vec![];
    for i in 0..MAX {
        for _ in 0..buckets[i] {
            vec.push(i as u32);
        }
    }

    // We finally return the now-sorted vector
    return vec;
}

// Define the mechanism to use in mainloop
const MECHANISM:(&str, &dyn Fn(&[u32]) -> Vec<u32>) = ("counting", &(|list:&[u32]| { counting_sort::<50>(list) }));
// const MECHANISM:(&str, &dyn Fn(&[u32]) -> Vec<u32>) = ("insertion", &insertion_sort);

// Define the max number of steps to have
const STEPS:usize = 10;

fn main() {
    for i in 0..STEPS  {
        let n = i*1000;

        let l0 = random_list::<20>(n);
        let t0 = Instant::now();
        MECHANISM.1(&l0);
        println!("\nSorted {} random digits via {} sort. Elasped {:?}.", n,MECHANISM.0, t0.elapsed());
    }
}
