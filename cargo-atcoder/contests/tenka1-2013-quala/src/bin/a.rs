fn main() {
    let mut c = 42_usize;
    while c <= 130_000_000 {
        c += c;
    }
    let ans = c;
    println!("{}", ans);
}
