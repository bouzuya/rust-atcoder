use proconio::input;

fn main() {
    input! {
        s: String,
        t: String,
    };
    let tbl = vec![
        "B9", "B8", "B7", "B6", "B5", "B4", "B3", "B2", "B1", "1F", "2F", "3F", "4F", "5F", "6F",
        "7F", "8F", "9F",
    ];
    let i_s = tbl.iter().position(|&s_i| s_i == s).unwrap();
    let i_t = tbl.iter().position(|&s_i| s_i == t).unwrap();
    let ans = if i_s > i_t { i_s - i_t } else { i_t - i_s };
    println!("{}", ans);
}
