use std::env::args;
use std::path::Path;
use std::fs::read_to_string;


fn readfile(filepath: &String) -> String {
    let path = Path::new(filepath);
    if !path.exists() { panic!("File doesn't exist!"); }

    let content = read_to_string(path).unwrap();
    return content;
}


fn main() {
    let arguments: Vec<String> = args().collect();
    if arguments.len() < 2 { panic!("File not specified!"); }
    else if arguments.len() > 2 { panic!("Too many files!"); }

    let content = readfile(&arguments[1]);
    let line_count = content.lines().count();

    let mut llist = vec![0; line_count];
    let mut rlist = vec![0; line_count];
    let mut results = vec![0; line_count];

    let mut i = 0;
    for line in content.lines() {
        let mut split = line.split("   ");
        let values = [""; 2].map(|_| split.next().unwrap());
        let lvalue = values[0].parse::<i64>().unwrap();
        let rvalue = values[1].parse::<i64>().unwrap();
        llist[i] = lvalue;
        rlist[i] = rvalue;
        i = i + 1;
    }

    llist.sort();
    rlist.sort();

    i = 0;
    while i < line_count {
        let mut result = 0;

        if llist[i] > rlist[i] {
            result = llist[i] - rlist[i];
        }
        else if llist[i] < rlist[i] {
            result = rlist[i] - llist[i];
        }

        results[i] = result;
        i = i + 1;
    }

    let mut result: i64 = results.iter().sum();
    println!("Part 1 Result: {}", result);

    let mut similarity = vec![0; line_count];
    i = 0;
    for lelement in &llist {
        let mut c = 0;
        for relement in &rlist {
            if lelement == relement {
                c = c + 1;
            }
        }
        similarity[i] = lelement * c;
        i = i + 1;
    }

    result = similarity.iter().sum();
    println!("Part 2 Result: {}", result);
}
