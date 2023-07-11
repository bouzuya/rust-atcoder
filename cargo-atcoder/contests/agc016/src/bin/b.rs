use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    };

    let max_a_i = a.iter().copied().max().unwrap();
    let min_a_i = a.iter().copied().min().unwrap();
    let ans = match max_a_i - min_a_i {
        // ある色の帽子は 2 匹以上の猫に被られている
        0 => {
            let count_color = max_a_i;
            a.iter().copied().all(|a_i| a_i == n - 1) || 2 * count_color <= n
        }
        // ある色の帽子は 1 匹の猫にしか被られていないことがある
        1 => {
            let count_color = max_a_i;
            let count_max = a.iter().copied().filter(|a_i| a_i == &max_a_i).count();
            let count_min = a.iter().copied().filter(|a_i| a_i == &min_a_i).count();
            count_min < count_color && 2 * count_color <= count_max + 2 * count_min
        }
        _ => false,
    };
    println!("{}", if ans { "Yes" } else { "No" });
}
