use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    };
    let mut count = 0_usize;
    for i1 in 0..n {
        for i2 in i1 + 1..n {
            for i3 in i2 + 1..n {
                for i4 in i3 + 1..n {
                    for i5 in i4 + 1..n {
                        let x = [i1, i2, i3, i4, i5].iter().map(|&i| a[i]).sum::<usize>();
                        if x == 1000 {
                            count += 1;
                        }
                    }
                }
            }
        }
    }
    let ans = count;
    println!("{}", ans);
}
