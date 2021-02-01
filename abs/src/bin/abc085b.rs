use proconio::{fastout, input};
use std::collections::BTreeSet;

#[fastout]
fn main() {
    input! {
        n: i8,
        d: [i8; n]
    }
    let result: BTreeSet<i8> = d.into_iter().map(|num| num as i8).collect();
    print!("{}", result.len());
}
