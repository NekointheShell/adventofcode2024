use std::env::args;
use std::path::Path;
use std::fs::read_to_string;


fn readfile(filepath: &String) -> String {
    let path = Path::new(filepath);
    if !path.exists() { panic!("File doesn't exist!"); }

    let content = read_to_string(path).unwrap();
    return content;
}


fn range_test(elements: &Vec<i64>, use_strike1: bool) -> (bool, bool) {
    let mut safe = true;
    let mut strike1: bool;

    if use_strike1 == true { strike1 = false; }
    else { strike1 = true; }

    let mut i = 0;
    while i < elements.len() - 1 {
        if elements[i] == elements[i + 1] {
            if strike1 == true { safe = false; }
            else { strike1 = true; }
        }
        if elements[i] < elements[i + 1] - 3 {
            if strike1 == true { safe = false; }
            else { strike1 = true; }
        }
        if elements[i] > elements[i + 1] + 3 {
            if strike1 == true { safe = false; }
            else { strike1 = true; }
        }
        i = i + 1;
    }

    return (safe, strike1);
}


fn inc_dec_test(elements: &Vec<i64>, use_strike1: bool) -> (bool, bool) {
    let mut safe = true;
    let mut inc_flag = false;
    let mut dec_flag = false;
    let mut strike1: bool;

    if use_strike1 == true { strike1 = false; }
    else { strike1 = true; }

    let mut i = 0;
    while i < elements.len() - 1 {
        if strike1 == false && inc_flag == true && elements[i] > elements[i + 1] { strike1 = true; }
        else if strike1 == false && dec_flag == true && elements[i] < elements[i + 1] { strike1 = true; }
        else if elements[i] < elements[i + 1] { inc_flag = true; }
        else if elements[i] > elements[i + 1] { dec_flag = true; }
        i = i + 1;
    }

    if inc_flag == true && dec_flag == true { safe = false; }
    if inc_flag == false && dec_flag == false { safe = false; }

    return (safe, strike1);
}


#[allow(warnings)]
fn main() {
    let arguments: Vec<String> = args().collect();
    if arguments.len() < 2 { panic!("File not specified!"); }
    else if arguments.len() > 2 { panic!("Too many files!"); }

    let content = readfile(&arguments[1]);
    let mut safe_count = 0;
    let mut safe_count2 = 0;

    for line in content.lines() {
        let split = line.split(" ");
        let element_count = split.clone().count();

        let mut elements = vec![0; element_count];

        let mut i = 0;
        for element in split {
            elements[i] = element.parse::<i64>().unwrap();
            i = i + 1;
        }

        let mut safe: bool;
        let mut strike1_used: bool;
        (safe, strike1_used) = range_test(&elements, false);
        if safe == true && strike1_used == true {
            (safe, strike1_used) = inc_dec_test(&elements, false);
        }

        if safe == true && strike1_used == true {
            safe_count = safe_count + 1;
        }

        let mut safe2: bool;
        (safe2, strike1_used) = range_test(&elements, true);
        if safe2 == true && strike1_used == false {
            (safe2, strike1_used) = inc_dec_test(&elements, true);
        }
        else if safe2 == true && strike1_used == true {
            (safe2, strike1_used) = inc_dec_test(&elements, false);
        }

        if safe2 == true {
            safe_count2 = safe_count2 + 1;
        }
    }

    println!("Part 1 Result: {}", safe_count);
    println!("Part 2 Result: {}", safe_count2);

    
}
