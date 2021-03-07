use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i64; n],
    };

    let offset = 200;
    let mut count: Vec<i64> = vec![0; 200 + 1 + 200 + 1];
    for &a_i in a.iter() {
        count[(a_i + offset) as usize] += 1;
    }
    let mut sum = 0_i64;
    for a_i in -200..=200 {
        let i = (a_i + offset) as usize;
        for a_j in -200..=200 {
            let j = (a_j + offset) as usize;
            if a_i > a_j {
                continue;
            }
            sum += count[i] * count[j] * (a_j - a_i).pow(2);
        }
    }
    let ans = sum;
    println!("{}", ans);

    // let mut sum = 0_i64;
    // for &a_i in a.iter() {
    //     sum += a_i.pow(2) * (n as i64 - 1);
    // }
    // let mut c = 0;
    // for i in (1..n).rev() {
    //     c += a[i];
    //     sum -= 2 * a[i - 1] * c;
    // }
    // let ans = sum;
    // println!("{}", ans);
}
