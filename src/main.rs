
fn bubble_sort(mut arr: Vec<i32>) -> Vec<i32> {
    let len = arr.len();

    for i in 0..len {
        for j in 0..len - 1 - i {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
            }
        }
    }

    arr
}

fn main() {
    println!("Hello, world!");
    let unsorted_array = vec![50, 32, -8, 104, 288, 1, -32, 44];
    println!("original array: {:?}", unsorted_array);
    let sorted_array = bubble_sort(unsorted_array);
    println!("sorted array: {:?}", sorted_array);
}
