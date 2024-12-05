use regex::Regex;


pub fn filter(input: &str, dosanddonts: bool) -> String {
    let re: Regex;

    if !dosanddonts {
        re = Regex::new(r"mul\([0-9]{1,3},[0-9]{1,3}\)").unwrap();
    }
    else {
        re = Regex::new(r"mul\([0-9]{1,3},[0-9]{1,3}\)|do\(\)|don\'t\(\)").unwrap();
    }

    let mut results: Vec<String> = vec![];

    for matched in re.find_iter(&input) {
        results.push(matched.as_str().to_string());
    }

    return results.join(", ");
}
