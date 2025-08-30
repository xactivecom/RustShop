///////////////////////////////
// Course Section 21
///////////////////////////////

// Selection sort: sequentially swap smallest searched value with current
fn selection_sort(array: &mut Vec<i32>) -> Vec<i32> {
    // Outer scan
    for i in 0..array.len() - 1 {
        let mut smallest = i;

        // Search for smallest index, then swap with outer index
        for j in (i + 1)..array.len() {
            if array[j] < array[smallest] {
                smallest = j
            }
        }
        array.swap(smallest, i);
    }
    array.to_vec()
}

// Bubble sort: swap smallest adjacent value
fn bubble_sort(array: &mut Vec<i32>) -> Vec<i32> {
    // Optimize size
    let n = array.len();

    // Outer scan
    for _ in 1..(n - 1) {
		let mut swapped = false;

        // Swap smallest adjacent value
        for j in 0..=(n - 2) {
			if array[j] > array[j + 1] {
				array.swap(j, j + 1);
				swapped = true;
			}
		}

        // No swap indicates sorted
		if !swapped {
			break;
		}
	}
	array.to_vec()
}

// Merge sort: subdivide at midpoint, sort each part and then merge the sorted parts
fn merge_sort(array: &mut [i32]) -> Vec<i32> {
    // Optimize size
    let n = array.len();

    if n > 1 {
        // Compute midpoint
        let mid = n / 2;

        // Independently sort left and right parts
        merge_sort(&mut array[..mid]);
        merge_sort(&mut array[mid..]);

        // Merge parts
        merge_array(array, mid);
    }
	array.to_vec()
}

// Merge array parts
fn merge_array(array: &mut [i32], mid: usize) {
    // Get left and right parts
    let left = array[..mid].to_vec();
    let right = array[mid..].to_vec();

    // Merge in sorted order
    let mut l = 0;
    let mut r = 0;
    for value in array {
        // If at end of right or left is smaller than right then take left, otherwise take right
        if r == right.len() || (l < left.len() && left[l] < right[r]) {
            *value = left[l];
            l += 1;
        } else {
            *value = right[r];
            r += 1;
        }
    }
}

// Quicksort: iterate from start and end of array
fn quick_sort(array: &mut [i32]) -> Vec<i32> {
    quick_inner(array, 0, array.len() - 1)
}

// Quicksort inner
fn quick_inner(array: &mut [i32], start: usize, end: usize) -> Vec<i32> {
    if start < end {
        // Sort left and right parts
        let part = quick_partition(array, start, end);
        quick_inner(array, start, part - 1);
        quick_inner(array, part + 1, end);
    }
    array.to_vec()
}

// Quicksort partitioner
fn quick_partition(array: &mut [i32], start: usize, end: usize) -> usize {
    // Iterate i from start and set pivot to the end
    let mut i = start;
    let pivot = end;

    // Iterate j from start to just before pivot
    for j in start..=(end - 1) {
        // Move smallest to where iterator i points, then increment it
        if array[j] < array[pivot] {
            array.swap(i, j);
            i += 1
        }
    }

    // Update pivot to middle
    array.swap(i, pivot);
    i
}

pub fn run_lesson() {
    println!("\nSection 21:");

    // Selection sort examples
    let mut array_1: Vec<i32> = vec![3, 4, 1, 2];
    print!("array_1:{:?} -> ", array_1);
    let sorted_1 = selection_sort(&mut array_1);
    println!("{:?}", sorted_1);

    let mut array_2: Vec<i32> = vec![3, 4, 1, 2, 7, 10];
    print!("array_2:{:?} -> ", array_2);
    let sorted_2 = selection_sort(&mut array_2);
    println!("{:?}", sorted_2);

    // // Bubble sort examples
    let mut array_3: Vec<i32> = vec![3, 4, 10, 2, 7];
    print!("array_3:{:?} -> ", array_3);
    let sorted_3 = bubble_sort(&mut array_3);
    println!("{:?}", sorted_3);

    let mut array_4: Vec<i32> = vec![3, -20, 4, 108, 22, 7, 0, 50, 1_024];
    print!("array_4:{:?} -> ", array_4);
    let sorted_4 = bubble_sort(&mut array_4);
    println!("{:?}", sorted_4);

    // Merge sort examples
    let mut array_5: Vec<i32> = vec![3, 4, 10, 2, 7];
    print!("array_5:{:?} -> ", array_5);
    let sorted_5 = merge_sort(&mut array_5);
    println!("{:?}", sorted_5);

    let mut array_6: Vec<i32> = vec![3, -20, 4, 108, 22, 7, 0, 13, 50, -9, 1_024];
    print!("array_6:{:?} -> ", array_6);
    let sorted_6 = merge_sort(&mut array_6);
    println!("{:?}", sorted_6);

    // Quick sort example
    let mut array_7: Vec<i32> = vec![8, 5, 1, 2, 7, 3, 4];
    print!("array_7:{:?} -> ", array_7);
    let sorted_7 = quick_sort(&mut array_7);
    println!("{:?}", sorted_7);

    let mut array_8: Vec<i32> = vec![3, -20, 24, 98, 22, 7, 0, 13, 50, -9, 1_024];
    print!("array_8:{:?} -> ", array_8);
    let sorted_8 = quick_sort(&mut array_8);
    println!("{:?}", sorted_8);

}
