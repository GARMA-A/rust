mod easy;
mod hard;
mod medium;

fn main() {

       let nums = vec![1,2,2,3,3,3];
       let k = 2;
       println!("{:?}", medium::TopKFrequentElements::top_k_frequent(nums , k));

 

}