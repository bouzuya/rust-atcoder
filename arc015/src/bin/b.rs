use proconio::input;

fn main() {
    input! {
        n: usize,
        t: [(f64, f64); n]
    };
    let mut counts = vec![0; 6];
    for (max_t_i, min_t_i) in t {
        if 35_f64 <= max_t_i {
            counts[0] += 1;
        }
        if 30_f64 <= max_t_i && max_t_i < 35_f64 {
            counts[1] += 1;
        }
        if 25_f64 <= max_t_i && max_t_i < 30_f64 {
            counts[2] += 1;
        }
        if 25_f64 <= min_t_i {
            counts[3] += 1;
        }
        if min_t_i < 0_f64 && 0_f64 <= max_t_i {
            counts[4] += 1;
        }
        if max_t_i < 0_f64 {
            counts[5] += 1;
        }
    }
    println!(
        "{} {} {} {} {} {}",
        counts[0], counts[1], counts[2], counts[3], counts[4], counts[5]
    );
}
