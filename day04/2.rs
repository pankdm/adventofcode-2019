const START:i64 = 172851;
const END:i64 = 675869;

fn is_ok(x: i64) -> bool {
    let mut ds = Vec::new();
    let mut n = x;
    while (n > 0) {
        ds.push(n % 10);
        n /= 10;
    }
    assert!(ds.len() == 6);
    ds.reverse();

    let mut same = false;
    for i in 1..ds.len() {
        if ds[i] == ds[i - 1] {
            let mut bigger = false;
            if i + 1 < ds.len() && ds[i + 1] == ds[i] {
                bigger = true;
            }
            if i >= 2 && ds[i - 2] == ds[i - 1] {
                bigger = true;
            }
            if !bigger {
                same = true;
            }
        }
        if ds[i] < ds[i - 1] {
            return false;
        }
    }
    if !same {
        return false;
    }
    return true;
}

fn main() {
    let mut count = 0;
    for x in  START..END + 1 {
        if is_ok(x) {
            count += 1;
        }
    }
    println!("ans = {}", count);
}