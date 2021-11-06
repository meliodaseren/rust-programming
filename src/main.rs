// Bubble sort
fn sort(array: &mut Vec<i32>) {
    for i in 0..array.len() {
        for j in 0..array.len() - i - 1 {
            if array[j + 1] < array[j] {
            // let tmp = array[j];
            // array[j] = array[j + 1];
            // array[j + 1] = tmp;
            array.swap(j, j + 1);
            }
        }
    }
}

fn main() {
    // let arr: Vec<i32> = vec![];
    let mut arr = vec![5, 3, 4, 1, 2];
    sort(&mut arr);
    println!("{}", arr[0]);
}
