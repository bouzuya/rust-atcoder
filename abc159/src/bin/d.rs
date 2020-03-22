use proconio::input;

fn main() {
    input! {
        n: usize,
        av: [usize; n],
    };
    let mut cv = vec![0usize; n + 1];
    for &a in av.iter() {
        cv[a] += 1;
    }

    let mut sum = 0;
    let mut xv = vec![0usize; n + 1];
    for a in 0..cv.len() {
        if cv[a] < 2 {
            continue;
        }
        // nCr
        let x = cv[a] * (cv[a] - 1) / 2;
        xv[a] = x;
        sum += x;
    }

    for &a in av.iter() {
        let ans = sum - xv[a]
            + if cv[a] < 2 {
                0
            } else {
                (cv[a] - 1) * (cv[a] - 2) / 2
            };
        println!("{}", ans);
    }
}
