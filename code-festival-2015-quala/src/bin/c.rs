use proconio::input;

fn main() {
    input! {
        n: usize,
        t: i64,
        mut ab: [(i64, i64); n],
    };
    let sum = ab.iter().map(|(_, b_i)| b_i).sum::<i64>();
    if sum > t {
        println!("-1");
        return;
    }
    let mut x = sum;
    ab.sort_by_key(|(a_i, b_i)| a_i - b_i);
    let mut count = n;
    for (a_i, b_i) in ab {
        x += a_i - b_i;
        if x <= t {
            count -= 1;
        } else {
            break;
        }
    }
    let ans = count;
    println!("{}", ans);
}
