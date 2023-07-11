// <https://kmjp.hatenablog.jp/entry/2014/10/08/1000>

fn main() {
    let mut pos = vec![(0, 0); 100];
    let n = 5;
    for i in 0..100 / 4 {
        let k = i + 1;
        let ox = i / n;
        let oy = i % n;
        let ol = (1500 / n) * ox;
        let or = (1500 / n) * (ox + 1);
        let ot = (1500 / n) * oy;
        let ob = (1500 / n) * (oy + 1);
        pos[k - 1] = (ol + k, ob - k);
        pos[51 - k - 1] = (or - (51 - k), ot + (51 - k));
        pos[50 + k - 1] = (or - (50 + k), ob - (50 + k));
        pos[101 - k - 1] = (ol + (101 - k), ot + (101 - k));
    }
    for (x_k, y_k) in pos {
        println!("{} {}", x_k, y_k);
    }
}
