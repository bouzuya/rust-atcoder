use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    };
    let colors = 8;
    let mut counts = vec![0_usize; colors + 1];
    for a_i in a.into_iter().map(|a_i| a_i.min(3200)) {
        counts[a_i / 400] += 1;
    }
    let mut count = 0_usize;
    for c in counts.iter().copied().take(colors) {
        if c > 0 {
            count += 1;
        }
    }
    let min = count.max(if counts[colors] > 0 { 1 } else { 0 });
    let max = count + counts[colors];
    println!("{} {}", min, max);
}
