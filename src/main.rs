use rand::prelude::*;
use std::time::Instant;

/// Generate a random-sized list
///
/// # Args
/// - size:`usize` - length of list
///
/// # Returns
/// `Vec<u32>` representing the resulting random list
fn random_list(size:usize) -> Vec<u32> {
    let mut rng = rand::thread_rng();
    let vals: Vec<u32> = (0..size).map(|_| rng.gen()).collect();

    return vals;
}

/// Insertion sort
///
/// # Args
/// - list:`&[u32]` - slice of list to be sorted
///
/// # Returns
/// `Vec<u32>` representing the sorted list
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
/// # Args
/// - list:`&[u32]` - slice of list to be sorted
/// - max:`usize` - the maximum value possible to be passed in
///
/// # Returns
/// `Vec<u32>` representing the sorted list
fn counting_sort(list:&[u32], max:usize) -> Vec<u32> {
    // We first create the buckets from which we sort
    // let mut buckets = [0usize; max];

    // We then create that many buckets. 


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

// Worse and best case insertion: do nothing + do everything
fn best_case_insertion(size:usize) -> Vec<u32> {
    let vals: Vec<u32> = (0..size).map(|i| i as u32).collect();
    return vals;
}

fn worse_case_insertion(size:usize) -> Vec<u32> {
    let vals: Vec<u32> = (0..size).rev().map(|i| i as u32).collect();
    return vals;
}

// Define the mechanism to use in mainloop
const MECHANISM:(&str, &dyn Fn(&[u32]) -> Vec<u32>) = ("insertion", &insertion_sort);
// Define the max number of steps to have
const STEPS:usize = 2;

fn main() {
    for i in 0..STEPS  {
        let n = i*1000;

        let l0 = random_list(n);
        let t0 = Instant::now();
        MECHANISM.1(&l0);
        println!("\nSorted {} random digits via {} sort. Elasped {:?}.", MECHANISM.0, n, t0.elapsed());

        let l1 = best_case_insertion(n);
        let t1 = Instant::now();
        MECHANISM.1(&l1);
        println!("Sorted {} best-case digits via {} sort. Elasped {:?}.", MECHANISM.0, n, t1.elapsed());

        let l2 = worse_case_insertion(n);
        let t2 = Instant::now();
        MECHANISM.1(&l2);
        println!("Sorted {} worse-case digits via {} sort. Elasped {:?}.\n", MECHANISM.0, n, t2.elapsed());
    }
}
