use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    };
    let mut count = vec![0_usize; 100];
    for a_i in a {
        count[a_i % 100] += 1;
    }

    let mut ans = 0_usize;
    for x in 0..100 {
        for y in x..100 {
            if (x + y) % 100 == 0 {
                ans += if x == y {
                    if count[x] < 2 {
                        0
                    } else {
                        count[x] * (count[x] - 1) / 2
                    }
                } else {
                    count[x] * count[y]
                };
            }
        }
    }
    println!("{}", ans);
}
