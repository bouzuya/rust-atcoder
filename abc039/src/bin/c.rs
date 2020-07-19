use proconio::input;

fn main() {
    input! {
        s: String,
    };
    let p = vec![
        ("WBWBWWBWBWBW", "Do"),
        ("WBWWBWBWBWWB", "Re"),
        ("WWBWBWBWWBWB", "Mi"),
        ("WBWBWBWWBWBW", "Fa"),
        ("WBWBWWBWBWWB", "So"),
        ("WBWWBWBWWBWB", "La"),
        ("WWBWBWWBWBWB", "Si"),
    ];
    for &(p_i, ans) in p.iter() {
        if s.starts_with(p_i) {
            println!("{}", ans);
            return;
        }
    }
}
