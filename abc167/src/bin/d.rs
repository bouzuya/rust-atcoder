use proconio::input;
use proconio::marker::Usize1;

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [Usize1; n],
    };

    // k <= n (k <= 2*10^5 と十分小さいので愚直に探索する)
    if k <= n {
        println!("{}", (0..k).fold(0, |c, _| a[c]) + 1);
        return;
    }

    // k > n (k 回移動した先は cycle の中にある)
    let (start, cycle) = {
        let mut i = vec![None; n];
        let mut ci = 0;
        let mut ca = 0;
        while let None = i[ca] {
            i[ca] = Some(ci);
            ci += 1;
            ca = a[ca];
        }
        match i[ca] {
            Some(i_i) => (i_i, ci - i_i),
            None => unreachable!(),
        }
    };
    let ans = (0..(k - start) % cycle).fold((0..start).fold(0, |c, _| a[c]), |c, _| a[c]) + 1;
    println!("{}", ans);
}
