use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
    };
    if h == 1 || w == 1 {
        println!("{}", 1);
    } else {
        println!("{}", h * (w / 2) + if w % 2 == 0 { 0 } else { (h + 1) / 2 });
    }
}
