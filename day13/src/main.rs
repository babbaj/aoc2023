use std::cmp::min;

fn find_vertical_reflection<T: PartialEq>(pattern: &[T]) -> Option<usize> {
    for i in 1..pattern.len() {
        let a = &pattern[0..i];
        let b = &pattern[i..];
        let len = min(a.len(), b.len());
        if a.iter().rev().take(len).eq(b.iter().take(len)) {
            return Some(i);
        }
    }
    return None;
}

fn single_diff<I, J>(iter1: I, iter2: J) -> (bool, bool)
where
    I: Iterator,
    I::Item: PartialEq<J::Item>,
    J: Iterator,
    J::Item: PartialEq
{
    let mut zip = iter1.zip(iter2).filter(|(a, b)| !a.eq(&b));
    return (zip.next().is_some(), zip.next().is_some());
}

fn find_vertical_single_diff<T: AsRef<[u8]>>(pattern: &[T]) -> Option<usize>
{
    'outer: for i in 1..pattern.len() {
        let a = &pattern[0..i];
        let b = &pattern[i..];
        let len = min(a.len(), b.len());
        let zip = a.iter().rev().take(len).zip(b.iter().take(len));
        let mut diffs = zip
            .map(|(a, b)| single_diff(a.as_ref().iter(), b.as_ref().iter()));
        // 1 element
        let mut found = false;
        for (first, more) in diffs {
            if more || (first && found) {
                continue 'outer;
            }
            found |= first;
        }
        if found {
            return Some(i);
        }
    }
    return None;
}

fn transpose(pattern: &[&str]) -> Vec<String> {
    let mut out = vec![String::new(); pattern[0].len()];
    for line in pattern {
        for i in 0..line.len() {
            out[i].push(line.as_bytes()[i] as char);
        }
    }
    return out;
}

fn part1() {
    let input = include_str!("../input3").trim_end();
    let patterns: Vec<Vec<&str>> = input.split("\n\n").map(|p| p.split('\n').collect()).collect();
    let mut columns_left = 0;
    let mut rows_above = 0;
    for pattern in patterns {
        if let Some(i) = find_vertical_reflection(&pattern) {
            rows_above += i;
            println!("{i} rows above");
        } else {
            let i = find_vertical_reflection(&transpose(&pattern)).unwrap();
            columns_left += i;
            println!("{i} columns left");
        }
    }
    println!("{}", columns_left + (100 * rows_above));
}

fn main() {
    part1();

    let input = include_str!("../input").trim_end();
    let patterns: Vec<Vec<&str>> = input.split("\n\n").map(|p| p.split('\n').collect()).collect();
    let mut columns_left = 0;
    let mut rows_above = 0;
    for pattern in patterns {
        if let Some(i) = find_vertical_single_diff(&pattern) {
            rows_above += i;
            println!("{i} rows above");
        } else {
            let i = find_vertical_single_diff(&transpose(&pattern)).unwrap();
            columns_left += i;
            println!("{i} columns left");
        }
    }
    println!("{}", columns_left + (100 * rows_above));
}
