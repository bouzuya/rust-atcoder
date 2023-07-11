use proconio::input;

fn main() {
    input! {
        n: usize,
    };
    let mut count = vec![0; n + 1];
    for x in 1..=100 {
        for y in 1..=100 {
            for z in 1..=100 {
                let v = x * x + y * y + z * z + x * y + y * z + z * x;
                if v < n + 1 {
                    count[v] += 1;
                }
            }
        }
    }
    for count_i in count.into_iter().skip(1) {
        println!("{}", count_i);
    }
}
