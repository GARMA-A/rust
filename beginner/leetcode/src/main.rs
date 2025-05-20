mod easy;
mod hard;
mod medium;

fn main() {
    let arr = vec![
        vec![2, 3],
        vec![5, 6],
        vec![1, 2],
        vec![6, 7],
        vec![100, -100],
        vec![20, 30],
        vec![10, 5],
    ];
    print!(
        "{:?}",
        medium::KClosestPointstoOrigin::Solution::k_closest(arr, 3)
    );
}
