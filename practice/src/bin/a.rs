use proconio::{input, fastout};

#[fastout]
fn main() {
    input! {
        a: i32,
        b: i32,
        c: i32,
        s: String
    }

    print!("{} {}", a + b + c, s);
}
