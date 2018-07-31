#![allow(unused)]

use std::{cell, cmp, collections, convert, fmt, io, iter, ops, str};
use io::{Read, Write};

fn read_line() -> String {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    buf.trim().to_owned()
}

fn parse_vec<T: str::FromStr>() -> Result<Vec<T>, T::Err> {
    read_line()
        .split_whitespace()
        .map(|e| e.parse())
        .collect()
}

fn main() {
    let n = {
        let ab = parse_vec::<i32>();
        (ab.unwrap()[0])
    };

    let result = (0..n+1).fold(n, |memo, ind| {
        let cc = calc(n, ind);

        if memo > cc {
            return cc;
        }
        memo
    });

    println!("{}", result);
}

fn calc(n: i32, ind: i32) -> i32 {
    let mut cc = 0;
    let mut tmp = ind;
    while tmp > 0 {
        cc += tmp % 6;
        tmp /= 6;
    }
    let mut tmp = n - ind;
    while tmp > 0 {
        cc += tmp % 9;
        tmp /= 9;
    }
    cc
}
