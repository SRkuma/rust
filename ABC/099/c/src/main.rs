#![allow(unused)]

use std::{cell, cmp, collections, convert, fmt, io, iter, ops, str};
use io::{Read, Write};

fn read_line() -> String {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    buf.trim().to_owned()
}

fn parse_vec<T: str::FromStr>() -> Vec<T> {
    read_line()
        .split_whitespace()
        .map(|e| e.parse().ok().unwrap())
        .collect()
}

fn main() {

    let n = {
        let ab = parse_vec::<i32>();
        (ab[0])
    };

    let mut res = n;

    for ind in 0..n {
        let mut cc = 0;
        calc(n, ind, &mut cc);

        if res > cc {
            res = cc;
        }
    }

    println!("{}", res);
}

fn calc(n: i32, ind: i32, cc: &mut i32) {
    let mut tmp = ind;
    while tmp > 0 {
        *cc += tmp % 6;
        tmp /= 6;
    }
    tmp = n - ind;
    while tmp > 0 {
        *cc += tmp % 9;
        tmp /= 9;
    }
}
