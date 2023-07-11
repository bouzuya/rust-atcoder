use proconio::input;

fn main() {
    input! {
        (sx, sy): (i64, i64),
        (tx, ty): (i64, i64),
    };
    let dx = tx - sx;
    let dy = ty - sy;
    let mut ans = String::new();
    ans.push_str(&"R".repeat(dx as usize));
    ans.push_str(&"U".repeat(dy as usize));
    ans.push_str(&"L".repeat(dx as usize));
    ans.push_str(&"D".repeat(dy as usize));
    ans.push_str("D");
    ans.push_str(&"R".repeat((dx + 1) as usize));
    ans.push_str(&"U".repeat((dy + 1) as usize));
    ans.push_str("L");
    ans.push_str("U");
    ans.push_str(&"L".repeat((dx + 1) as usize));
    ans.push_str(&"D".repeat((dy + 1) as usize));
    ans.push_str("R");
    println!("{}", ans);
}
