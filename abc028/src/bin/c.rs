use proconio::input;

fn main() {
    input! {
        abcde: [i64; 5],
    };
    let mut n: Vec<i64> = vec![];
    for bits in 0..1 << 5 {
        let mut is = vec![];
        for i in 0..5 {
            if (bits >> i) & 1 == 1 {
                is.push(i);
            }
        }
        if is.len() != 3 {
            continue;
        }
        n.push(is.iter().map(|&i| abcde[i]).sum());
    }
    n.sort_by_key(|n_i| -n_i);
    let ans = n[2];
    println!("{}", ans);
}
