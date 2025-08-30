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

    // bubble sort examples
    let mut array_3: Vec<i32> = vec![3, 4, 10, 2, 7];
    print!("array_3:{:?} -> ", array_3);
    let sorted_3 = bubble_sort(&mut array_3);
    println!("{:?}", sorted_3);

    let mut array_4: Vec<i32> = vec![3, -20, 4, 108, 22, 7, 0, 50, 1_024];
    print!("array_4:{:?} -> ", array_4);
    let sorted_4 = bubble_sort(&mut array_4);
    println!("{:?}", sorted_4);

    // Quick sort example

}
