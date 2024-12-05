use crate::processor::Processor;


pub fn parse(input: &String) -> i64 {
    let mut processor = Processor::new();

    let mut ret: Vec<i64> = vec![];
    for command in input.split(", ") {
        let command_part: Vec<&str> = command.split("(").collect();
        let keyword = command_part[0];
        let command_part_stripped: Option<&str> = command_part[1].strip_suffix(")");
        let args: Vec<&str>;
        match command_part_stripped {
            None => { dbg!("none found"); args = vec![]; },
            Some(x) => { args = x.split(",").collect(); },
        }

        if keyword == "mul" {
            if args.len() == 2 {
                let arg1 = args[0].parse::<i64>().unwrap();
                let arg2 = args[1].parse::<i64>().unwrap();
                ret.push(processor.command_mul(arg1, arg2));
            }
        }
        else if keyword == "do" {
            processor.command_do();
        }
        else if keyword == "don't" {
            processor.command_dont();
        }
    }

    let mut i: i64 = 0;
    for x in &ret {
        i = i + x;
    }

    return i;
}
