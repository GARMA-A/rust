mod easy;
mod hard;
mod medium;

fn main() {
    let mut arr: Vec<i32> = vec![4, 1, 2, 6, 5, 3];
    let mut arr2 = vec![4, 1, 2, 6, 5, 3];
    let sort = easy::sorting::Sorting::new();
    println!(
        "{:?} \n {:?} => {:?}",
        sort.MergeSort(&mut arr),
        sort.quick_sort(&mut arr2, 0, 5),
        arr2
    );
}
