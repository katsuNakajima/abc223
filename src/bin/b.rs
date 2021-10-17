#[allow(unused_macros)]
macro_rules! parse_line {
    ( $t:ty ) => (
        {
            let mut line = String::new();
            ::std::io::stdin().read_line(&mut line).unwrap();
            let mut iter = line.split_whitespace();
            iter.next().unwrap().parse::<$t>().unwrap()
        }
    );

    ( $( $t:ty), +) => (
        {
            let mut line = String::new();
            ::std::io::stdin().read_line(&mut line).unwrap();
            let mut iter = line.split_whitespace();
            ( $(iter.next().unwrap().parse::<$t>().unwrap()),* )
        }
    );
}

#[allow(unused_macros)]
macro_rules! read_line {
    () => {{
        let mut line = String::new();
        ::std::io::stdin().read_line(&mut line).unwrap();
        line.pop();
        line
    }};
}

#[allow(unused_macros)]
macro_rules! parse_vec {
    ( $t:ty ) => {{
        let mut line = String::new();
        ::std::io::stdin().read_line(&mut line).unwrap();
        let iter = line.split_whitespace();
        iter.map(|v| v.parse::<$t>().unwrap()).collect::<Vec<_>>()
    }};
}

fn main() {
    let s = parse_line!(String);
    let len = s.len();
    let mut s_chars: Vec<_> = s.as_str().chars().collect();
    let mut s_vec = Vec::new();
    for _ in 0..len {
        let x = s_chars.remove(0);
        s_chars.push(x);
        let ss: String = s_chars.iter().collect();
        s_vec.push(ss);
    }
    let ans_min = s_vec.iter().min().unwrap();
    let ans_max = s_vec.iter().max().unwrap();
    println!("{}", ans_min);
    println!("{}", ans_max);
}
