use std::{{fs::File, io::Read, str::FromStr}};

fn main() {
    let mut input = File::open("input/2022/day1.txt").unwrap();
    let mut input_str = String::new();
    input.read_to_string(&mut input_str).unwrap();
    let mut parsed_elves: Vec<usize> = input_str
        .split("\n\n")
        .map(|elf|{
            elf.lines()
                .map(usize::from_str)
                .map(Result::unwrap)
                .sum::<usize>()

        }).collect();
        parsed_elves.sort();
        //For part - 1
        // let result = parsed_elves.pop().unwrap();
        //For part-2
        let result = parsed_elves.pop().unwrap() + parsed_elves.pop().unwrap() + parsed_elves.pop().unwrap();
        println!("{}", result);

}