use rand::prelude::*;

fn main() {
    let mut nums: Vec<i32> = (1..=100).collect();
    nums.shuffle(&mut rand::thread_rng());
    quick_sort(&mut nums);
    println!("{:?}", nums)
}

// time complexity: O(n^2)
fn bubble_sort(arr: &mut Vec<i32>) {
    for i in 0..arr.len() {
        for j in 0..(arr.len() - i - 1) {
           if arr[j] > arr[j + 1] {
               let temp = arr[j];
               arr[j] = arr[j + 1];
               arr[j + 1] = temp;
           }
        }
    }
}

// time complexity: O(n^2)
// good to use if the data is almost sorted
fn insertion_sort(arr: &mut Vec<i32>) {
    for i in 0..arr.len() {
        let current = arr[i];
        
        let mut j = i - 1;
        
        while j as i32 > -1 && current < arr[j] {
            arr[j - 1] = arr[j];
            j -= 1;
        }
        arr[j + 1] = current;
    }
}

// time complexity: O(n^2)
fn selection_sort(arr: &mut Vec<i32>) {
    for i in 0..arr.len() {
        let mut min = i;
        for j in i..arr.len() {
            if arr[j] < arr[min] {
                min = j;
            }
        }

        if min != i {
            [arr[i], arr[min]] = [arr[min], arr[i]];
        }
    }
}

// time complexity: )(n log n)
// use more memory to do it
fn merge_sort(mut arr: Vec<i32>) -> Vec<i32> {
    let mid = arr.clone().len() / 2;

    if arr.len() < 2 {
        return arr
    }

    let left: Vec<i32> = arr.splice(0..mid, vec![]).collect();
    return merge(merge_sort(left), merge_sort(arr));
}

fn merge(mut left: Vec<i32>, mut right: Vec<i32>) -> Vec<i32> {
    let mut arr = Vec::new();
    left.reverse();
    right.reverse();
    while left.len() != 0  && right.len() != 0 {
        if left[left.len() - 1] < right[right.len() - 1] {
            arr.push(left.pop().unwrap());
        } else {
            arr.push(right.pop().unwrap());
        }
    }
    left.reverse();
    right.reverse();

    arr.append(left.as_mut());
    arr.append(right.as_mut());

    return arr;
}

//time complexity: O(n log n)
fn quick_sort(arr: &mut Vec<i32>) {
    _quick_sort(arr, 0, arr.len()  as i32 - 1)
}

fn _quick_sort(arr: &mut Vec<i32>, left: i32, right: i32) {
    if left >= right {
        return;
    }

    let pivot_index = partition(arr, left, right);
    _quick_sort(arr, left, pivot_index - 1);
    _quick_sort(arr, pivot_index + 1, right);
    
}

fn partition(arr: &mut Vec<i32>, left: i32, right: i32) -> i32 {
    let pivot_value = arr[right as usize];
    let mut partition_index = left;

    for i in left..right {
        if arr[i as usize] < pivot_value {
            arr.swap(i as usize, partition_index as usize);
            partition_index += 1;
        }
    }

    arr.swap(right as usize, partition_index as usize);
    partition_index
}

//time complexity: 
// used for integers or binary trees
// use for tabulation most of
fn radix_sort(arr: &mut Vec<i32>) {
    
}
