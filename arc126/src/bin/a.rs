use proconio::input;

fn main() {
    input! {
        t: usize,
        case: [(usize, usize, usize); t],
    };
    for (n_2, n_3, n_4) in case {
        let (n_1, n_2, n_3) = (n_2, n_4, n_3 / 2);
        // 1 * 5
        // 1 * 3 + 2 * 1
        // 1 * 1 + 2 * 2
        // 1 * 2         + 3
        //         2 * 1 + 3
        let c_1 = n_2.min(n_3);
        let (n_2, n_3) = (n_2 - c_1, n_3 - c_1);

        let c_2 = (n_1 / 2).min(n_3);
        let (n_1, _n_3) = (n_1 - 2 * c_2, n_3 - c_2);

        let c_3 = n_1.min(n_2 / 2);
        let (n_1, n_2) = (n_1 - c_3, n_2 - 2 * c_3);

        let c_4 = (n_1 / 3).min(n_2);
        let (n_1, _n_2) = (n_1 - 3 * c_4, n_2 - c_4);

        let c_5 = n_1 / 5;
        let _n_1 = n_1 - c_5;

        let ans = c_1 + c_2 + c_3 + c_4 + c_5;
        println!("{}", ans);
    }
}
