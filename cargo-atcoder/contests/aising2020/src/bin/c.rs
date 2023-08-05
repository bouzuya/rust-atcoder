use proconio::input;

fn main() {
    input! {
        n: usize,
    };
    let mut count = vec![0_usize; n + 1];
    for x in 1_usize..=100 {
        for y in 1_usize..=100 {
            for z in 1_usize..=100 {
                let v = x.pow(2) + y.pow(2) + z.pow(2) + (x * y) + (y * z) + (z * x);
                if v > n {
                    break;
                }
                count[v] += 1;
            }
        }
    }
    for c in count.iter().skip(1) {
        println!("{}", c);
    }
}
