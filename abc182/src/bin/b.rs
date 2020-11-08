use proconio::input;

macro_rules! chmax {
    ($max_v: expr, $v: expr) => {
        if $v > $max_v {
            $max_v = $v;
            true
        } else {
            false
        }
    };
}

fn main() {
    input! {
        n: usize,
        a: [i64; n],
    };
    let mut max = 0;
    let mut max_value = a[0];
    for k in 2..=1000 {
        let mut count = 0;
        for &a_i in a.iter() {
            if a_i % k == 0 {
                count += 1;
            }
        }
        if chmax!(max, count) {
            max_value = k;
        }
    }
    let ans = max_value;
    println!("{}", ans);
}
