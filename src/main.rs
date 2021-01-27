use std::collections::HashMap;
use std::iter::Iterator;

use nom::alt;
use nom::tag;
use nom::named;
use nom::take_while;
use nom::separated_pair;
use nom::sequence::terminated;
use nom::combinator::iterator;
use nom::character::is_alphanumeric;
use nom::character::complete::alpha1;
use nom::character::complete::line_ending as eol;

named!(end_of_line<&str, &str>, alt!(tag!(" ") | eol));
named!(alphanumericslash1<&str, &str>, take_while!(|c| is_alphanumeric(c as u8) || c == '/' || c == '-'));
named!(kv_pair<&str, (&str, &str)>, separated_pair!(alpha1, nom::bytes::complete::tag("="), alphanumericslash1) );

fn main() {
    let full = "rule=9 dec=allow perm=execute auid=1003 pid=5555 exe=/usr/bin/bash : path=/usr/bin/vi ftype=application/x-executable\n";
    let clean = full.split(':').last().unwrap().trim_start();
    let mut nom_it = iterator(clean, terminated( kv_pair, end_of_line));

    // map
    let map = nom_it.map(|(k, v)| (k, v)).collect::<HashMap<_, _>>();
    println!("{:?}", map);
    //////
    // {"path": "/usr/bin/vi", "ftype": "application/x-executable"}

    // vec
    // let res: Vec<(&str, &str)> = nom_it.collect();
    // println!("{}", clean);
    // for (x, y) in res.iter() {
    //     println!("{} {}", x, y)
    // }
    //////
    // path /usr/bin/vi
    // ftype application/x-executable
}
