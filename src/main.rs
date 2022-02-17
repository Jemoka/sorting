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

/// Counting sort (carry) - counting sort with backpacking values
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
fn counting_sort_carry<T:Clone, const MAX: usize>(list:&[(u32, T)]) -> Vec<T> {
    // We first create the buckets from which we sort
    let mut buckets = vec![vec![]; MAX];

    // We iterate over the list and add to the required bucket
    for i in list {
        buckets[i.0 as usize].push(i.1.clone());
    }

    // We then construct our new list
    let mut vec = vec![];
    for i in 0..MAX {
        for j in &buckets[i] {
            vec.push(j.clone());
        }
    }

    // We finally return the now-sorted vector
    return vec;
}

/// Counting sort (numbers) - counting sort but no backpacking
/// Implemented using counting sort (carry)
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
    // We essentially just carry the sorting value itself
    let sortable = list.iter().map(|i| (*i,*i)).collect::<Vec<_>>();

    // And we sort and return!
    counting_sort_carry::<u32, MAX>(sortable.as_slice())
}

/// Radix Sort
///
/// # Args
/// - list: `&[u32]` - slice of list to be sorted
///
/// # Returns
/// `Vec<u32>` representing the sorted list
#[allow(dead_code)]
fn radix_sort(list:&[u32]) -> Vec<u32> {
    // digits seperator, returns tuples
    let digits = | i:u32 | {
        i.to_string()
            .chars()
            .map(|d| d.to_digit(10).unwrap())
            .collect::<Vec<_>>()
    };

    // do the digits seperation on a list 
    // and get the maximum number of digits
    let mut list_digits = vec![];
    let mut max = 0;

    // for each element in the list
    for i in list {
        // we seperate it into digits
        let d = digits(*i);
        // we update the maximum value
        max = std::cmp::max(d.len(), max);
        // push the resulting digits up
        list_digits.push((i, d));
    }

    // sort list digits up until max digit is sorted
    for i in 0..max {
        // build the sorting list by getting the nth digit
        let mut sorting_list = vec![];

        for j in &list_digits {
            // get the index of subtraction: we want the index-th
            // number. the if statement checks if we are going to
            // have a subtraction overflow (i.e. there's not enough
            // digits to reach)
            let index = if j.1.len() >= i {j.1.len()-i} else {0};

            // we actuall perform the indexing on digits---returning
            // 0 when we get out of bounds values
            let current_digit = if index > 0 {j.1[index-1]} else {0};

            // we push the indexed digit, backpacking the whole of
            // the digits set, to be sorted
            sorting_list.push(
                (current_digit, j.clone())
            );
        }

        // sort and set value
        list_digits = counting_sort_carry::<_, 10>(sorting_list.as_slice());
    }

    // Return just the list
    return list_digits.iter().map(|i|*(i.0)).collect();
}


// Define the mechanism to use in mainloop
// const MECHANISM:(&str, &dyn Fn(&[u32]) -> Vec<u32>) = ("counting", &(|list:&[u32]| { counting_sort::<50>(list) }));
// const MECHANISM:(&str, &dyn Fn(&[u32]) -> Vec<u32>) = ("insertion", &insertion_sort);
const MECHANISM:(&str, &dyn Fn(&[u32]) -> Vec<u32>) = ("radix", &radix_sort);

// Define the max number of steps to have
const STEPS:usize = 100;

fn main() {
    for i in 1..STEPS+1  {
        let n = i*1000;

        let l0 = random_list::<1500>(n);
        let t0 = Instant::now();
        MECHANISM.1(&l0);
        println!("\nSorted {} random digits via {} sort. Elasped {:?}.", n,MECHANISM.0, t0.elapsed());
    }
}
