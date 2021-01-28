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

fn is_valid_value_char(chr: char) -> bool {
    is_alphanumeric(chr as u8) || chr == '/' || chr == '-'
}

named!(end_of_line<&str, &str>, alt!(tag!(" ") | eol));
named!(alphanumericslash1<&str, &str>, take_while!(is_valid_value_char));
named!(kv_pair<&str, (&str, &str)>, separated_pair!(alpha1, nom::bytes::complete::tag("="), alphanumericslash1) );

static FULL: &str = "rule=9 dec=allow perm=execute auid=1003 pid=5555 exe=/usr/bin/bash : path=/usr/bin/vi ftype=application/x-executable\n";

#[test]
fn single_entry_to_map() {
    let clean = FULL.split(':').last().unwrap().trim_start();
    let mut nom_it = iterator(clean, terminated( kv_pair, end_of_line));

    let res = nom_it.map(|(k, v)| (k, v)).collect::<HashMap<_, _>>();

    // {"path": "/usr/bin/vi", "ftype": "application/x-executable"}
    println!("{:?}", res);
}

#[test]
fn single_entry_to_vec() {
    let clean = FULL.split(':').last().unwrap().trim_start();
    let mut nom_it = iterator(clean, terminated( kv_pair, end_of_line));

    let res: Vec<(&str, &str)> = nom_it.collect();

    // path /usr/bin/vi
    // ftype application/x-executable
    for (x, y) in res.iter() {
        println!("{} {}", x, y)
    }
}
