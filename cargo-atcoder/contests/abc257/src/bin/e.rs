use proconio::input;

fn main() {
    input! {
        n: usize,
        c: [usize; 9]
    };
    let min_c_i = c.iter().copied().min().unwrap();
    let count = n / min_c_i;

    let mut used = 0;
    let mut ans = vec![];
    for i in (0..count).rev() {
        let m = n - used - min_c_i * i;

        let mut max_c_j = 0;
        let mut max_j = 0;
        for (j, c_j) in c.iter().copied().enumerate() {
            let j = j + 1;
            if c_j > m {
                continue;
            }
            if j > max_j {
                max_j = j;
                max_c_j = c_j;
            }
        }

        used += max_c_j;

        ans.push((b'0' + max_j as u8) as char);
    }
    println!("{}", ans.into_iter().collect::<String>());
}
