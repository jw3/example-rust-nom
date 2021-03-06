use nom::character::complete::{alphanumeric1, line_ending};
use nom::character::complete::digit1;
use nom::character::complete::space1;
use nom::combinator::iterator;
use nom::InputIter;
use nom::sequence::terminated;

#[derive(Debug)]
struct File {
    path: String,
    size: i64,
    digest: Option<String>,
}

fn filepath(i: &str) -> nom::IResult<&str, &str> {
    nom::bytes::complete::is_not(" \t\n")(i)
}

fn modestring(i: &str) -> nom::IResult<&str, &str> {
    nom::bytes::complete::is_a("01234567")(i)
}

fn digest_or_not(i: &str) -> Option<&str> {
    if i.iter_elements().all(|c| c == '0') { None } else { Some(i) }
}

/// path size mtime digest mode owner group isconfig isdoc rdev symlink
fn parse_line(i: &str) -> nom::IResult<&str, File> {
    match nom::combinator::complete(nom::sequence::tuple((
        filepath, space1,
        digit1, space1,
        digit1, space1,
        alphanumeric1, space1,
        modestring, space1,
        alphanumeric1, space1,
        alphanumeric1, space1,
        digit1, space1,
        digit1, space1,
        digit1, space1,
        filepath
    )))(i) {
        Ok((remaining_input, (
            path, _,
            size, _,
            _, _, // mtime
            digest, _,
            _, _, // mode
            _, _, // owner
            _, _, // group
            _, _, // isconfig
            _, _, // isdoc
            _, _, // rdev
            _,    // symlink
        ))) => {
            Ok((remaining_input, File {
                path: path.to_string(),
                size: size.parse().unwrap(),
                digest: digest_or_not(digest).map(|s| s.to_string()),
            }))
        }
        Err(e) => Err(e)
    }
}

//////////

static A: &str = "/usr/bin/hostname 21664 1557584275 26532eeae676157e70231d911474e48d31085b5f2e511ce908349dbb02f0f69c 0100755 root root 0 0 0 X";
static A2: &str = "/usr/bin/hostname 21664 1557584275 26532eeae676157e70231d911474e48d31085b5f2e511ce908349dbb02f0f69c 0100755 root root 0 0 0 X\n";
static B: &str = "/usr/share/man/man1/dnsdomainname.1.gz 13 1557584275 0000000000000000000000000000000000000000000000000000000000000000 0120777 root root 0 1 0 hostname.1.gz";
static C: &str = "/usr/lib/.build-id/a8/a7ee9d5002492edfc62e3e2e44149e981f9866 28 1557584275 0000000000000000000000000000000000000000000000000000000000000000 0120777 root root 0 0 0 ../../../../usr/bin/hostname";

#[test]
fn parse_a() {
    let expected = File {
        path: "/usr/bin/hostname".to_string(),
        size: 21664,
        digest: Some("26532eeae676157e70231d911474e48d31085b5f2e511ce908349dbb02f0f69c".to_string()),
    };
    {
        let (_, actual) = parse_line(A).unwrap();
        println!("{:?}", actual);

        assert_eq!(actual.path, expected.path);
        assert_eq!(actual.size, expected.size);
        assert_eq!(actual.digest, expected.digest);
    }

    {
        let (_, actual) = parse_line(A2).unwrap();
        println!("{:?}", actual);

        assert_eq!(actual.path, expected.path);
        assert_eq!(actual.size, expected.size);
        assert_eq!(actual.digest, expected.digest);
    }
}

#[test]
fn parse_b() {
    let expected = File {
        path: "/usr/share/man/man1/dnsdomainname.1.gz".to_string(),
        size: 13,
        digest: None,
    };
    let (_, actual) = parse_line(B).unwrap();
    println!("{:?}", actual);

    assert_eq!(actual.path, expected.path);
    assert_eq!(actual.size, expected.size);
    assert_eq!(actual.digest, expected.digest);
}

#[test]
fn parse_c() {
    let expected = File {
        path: "/usr/lib/.build-id/a8/a7ee9d5002492edfc62e3e2e44149e981f9866".to_string(),
        size: 28,
        digest: None,
    };
    let (_, actual) = parse_line(C).unwrap();
    println!("{:?}", actual);

    assert_eq!(actual.path, expected.path);
    assert_eq!(actual.size, expected.size);
    assert_eq!(actual.digest, expected.digest);
}

#[test]
fn parse_abc() {
    let mut abc = String::new();
    abc.push_str(A);
    abc.push('\n');
    abc.push_str(B);
    abc.push('\n');
    abc.push_str(C);
    abc.push('\n');

    let mut it = iterator(abc.as_str(), terminated(parse_line, line_ending));

    let files: Vec<File> = it.collect();
    let (unused, _) = it.finish().unwrap();

    assert!(unused.is_empty());
    assert_eq!(files.len(), 3);
    for x in files {
        println!("{:?}", x);
    }
}
