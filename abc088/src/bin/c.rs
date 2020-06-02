use proconio::input;

fn main() {
    input! {
        c: [[i64; 3]; 3],
    };

    let mut ans = false;
    for a_1 in 0..=100 {
        let b_1 = c[0][0] - a_1;
        let b_2 = c[0][1] - a_1;
        let b_3 = c[0][2] - a_1;
        let a_2 = c[1][0] - b_1;
        let a_3 = c[2][0] - b_1;
        if vec![
            vec![a_1 + b_1, a_1 + b_2, a_1 + b_3],
            vec![a_2 + b_1, a_2 + b_2, a_2 + b_3],
            vec![a_3 + b_1, a_3 + b_2, a_3 + b_3],
        ] != c
        {
            continue;
        }
        ans = true;
    }
    println!("{}", if ans { "Yes" } else { "No" });
}
