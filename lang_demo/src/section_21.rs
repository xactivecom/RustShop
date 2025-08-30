///////////////////////////////
// Course Section 21
///////////////////////////////

// Selection sort: sequentially swap smallest value
fn selection_sort(array: &mut Vec<i8>) -> Vec<i8> {
    // Outer index
    for i in 0..array.len() - 1 {
        let mut smallest = i;

        // Search for smallest index, then swap with outer index
        for j in (i+1)..array.len() {
            if array[j] < array[smallest] {
                smallest = j
            }
        }
        array.swap(smallest, i);
    }
    array.to_vec()
}

pub fn run_lesson() {
    println!("\nSection 21:");

    // Selection sort example
    let mut array_1: Vec<i8> = vec![3, 4, 1, 2];
    let sorted_1 = selection_sort(&mut array_1);
    println!("soted_1:{:?}", sorted_1);

    let mut array_2: Vec<i8> = vec![3, 4, 1, 2, 7, 10];
    let sorted_2 = selection_sort(&mut array_2);
    println!("sorted_2:{:?}", sorted_2);

    // bubble sort example

    // Quick sort example

}
