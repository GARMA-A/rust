// piles[i] the number of bananas
// h -> the number of hours to eat bananas
// your bananas-per-hour eating rate of k

// search from 1 to mx number on the array using binary search
// in each time you eterate over the array and eat and calc hours spend
// if the hours spend is > h given l = mid+1 else hours spend <= h r=mid-1

pub fn min_eating_speed(piles: Vec<i32>, h: i32) -> i32 {
    let mx = match piles.iter().max() {
        Some(&mx) => mx,
        None => return 0,
    };
    let (mut l, mut r) = (1, mx);
    let mut ans: i32 = 0;
    while l <= r {
        let mid = l + (r - l) / 2;
        if calc_hours(mid, &piles) > h as i64{
            l = mid + 1;
        } else {
            ans = mid;
            r = mid - 1;
        }
    }

    ans
}

fn calc_hours(eating_rate: i32, arr: &Vec<i32>) -> i64 {
    let mut hours_needed: i64 = 0;

    for &num in arr {
        hours_needed += ((num + eating_rate - 1) / eating_rate)as i64;
    }

    hours_needed
}
