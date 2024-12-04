use std::{fs::File, process::exit};
use std::io::Read;
use std::iter::zip;
use std::str::SplitWhitespace;

use regex::{self, Regex};
use lazy_static::lazy_static;

fn aoc1() -> std::io::Result<()> {
    
    println!("Starting Task 1, Part 1:");
    
    let mut file = File::open("data/aoc1")?;
    
    let mut buf = String::new();

    file.read_to_string(&mut buf).unwrap();

    let lines: Vec<String> = buf.lines().map(String::from).collect();

    let mut tuple: Vec<(i32, i32)> = Vec::new();

    for line in lines {
        let s = line.split_whitespace();
        
        let mut iter = s.peekable();

        let x = iter.next().unwrap();
        let y = iter.peek().unwrap().clone();

        let item: (i32, i32) = (x.parse::<i32>().unwrap(), y.parse::<i32>().unwrap());
        
        tuple.push(item.clone());
    }

    let (mut left, mut right): (Vec<i32>, Vec<i32>) = tuple.into_iter().unzip();
    
    left.sort();
    right.sort();
    
    let fin: i32 = zip(&left, &right).map(|tup| (tup.0 - tup.1).abs()).sum();

    println!("{}", fin);
 
    println!("Starting Task 1, Part 2:");

    let mut accumulator = 0;

    for item in left {
        let x = item;
        let counter = right.iter()
                           .filter(|p| **p == x)
                           .count();
        
        accumulator += x as usize * counter;
    }

    println!("{}", accumulator);

    Ok(())
}

fn aoc2() -> std::io::Result<()> {

    println!("Starting Task 2, Part 1:");

    let mut file = File::open("data/aoc2")?;
    
    let mut buf = String::new();

    file.read_to_string(&mut buf).unwrap();

    let lines: Vec<String> = buf.lines().map(String::from).collect();

    let mut matrix: Vec<Vec<i32>> = Vec::with_capacity(lines.len());

    for line in lines {
        let nums: SplitWhitespace = line.split_whitespace();
        
        let mut split: Vec<i32> = Vec::with_capacity(15);

        for n in nums {
            split.push(n.parse::<i32>().unwrap());
        } 
        
        matrix.push(split);
    }

    let mut acc: u32 = 0;

    let backup: Vec<Vec<i32>> = matrix.clone();

    for mut report in matrix {

        // If the report is in descending order we flip it
        if report[0] > report[1] {
            report.reverse();    
        }

        // Filter for pairs that are in increasing order while being no more than 3 units apart but at least 1 apart
        let filtered = report.windows(2)
              .all(|win| (win[0] < win[1]) && 
                                     ((win[0] - win[1]).abs() <= 3) &&
                                     ((win[0] - win[1]).abs() > 0));        

        if filtered {
            acc += 1;
        }
    }

    println!("Safe: {}", acc);

    println!("Starting Task 2, Part 2:");

    let mut accumulator: u32 = 0;

    for report in backup {

        let options = report.len();
        let mut pass = false;

        for i in 0..options{
            let mut mod_report: Vec<i32> = report.clone();

            mod_report.remove(i);

            if mod_report[0] > mod_report[1] {
                mod_report.reverse();    
            }
    
            // Filter for pairs that are in increasing order while being no more than 3 units apart but at least 1 apart
            let filtered = mod_report.windows(2)
                  .all(|win| (win[0] < win[1]) && 
                                         ((win[0] - win[1]).abs() <= 3) &&
                                         ((win[0] - win[1]).abs() > 0));        
            if filtered {
                pass = true;
                break;
            }
        }

        if pass {
            accumulator += 1;
        }
    }

    println!("Safe: {}", accumulator);


    Ok(())
}


#[derive(Debug, Clone)]
enum Node {
    Mul(i32),
    Enabled(bool),
}

lazy_static! {
    static ref reg: Regex = Regex::new(r"mul\(([1-9][0-9]{0,2}),([1-9][0-9]{0,2})\)").expect("It's a fucking regex");
    static ref reg2: Regex = Regex::new(r"mul\(([1-9][0-9]{0,2}),([1-9][0-9]{0,2})\)|do\(\)|don't\(\)").expect("It's a fucking regex");
}



fn aoc3() -> std::io::Result<()> {

    println!("Starting Task 3, Part 1:");
    

    let mut file: File = File::open("data/aoc3")?;

    let mut buf: String = String::new();

    file.read_to_string(&mut buf).unwrap();

    let instructions: i32 = reg.captures_iter(&buf)
                               .map(|c| c.get(1).unwrap().as_str().parse::<i32>().unwrap() * c.get(2).unwrap().as_str().parse::<i32>().unwrap())
                               .sum();

    println!("Solution: {}", instructions);

    println!("Starting Task 3, Part 2:");
                                
    let mut in_vec: Vec<Node> = Vec::with_capacity(100);

    let mut start = 0;

    while let Some(mat) = reg2.find_at(&buf, start) {

        let s = mat.as_str();

        if s.find(|c: char| c.is_ascii_digit()).is_some() {
            in_vec.push(Node::Mul(reg.captures(mat.as_str()).unwrap().get(1).unwrap().as_str().parse::<i32>().unwrap() * reg.captures(mat.as_str()).unwrap().get(2).unwrap().as_str().parse::<i32>().unwrap()));
        } else if s.contains(r"'") {
            in_vec.push(Node::Enabled(false));
        } else {
            in_vec.push(Node::Enabled(true));
        }

        start = mat.end();
    }

    let mut accumulator: i32 = 0;
    let mut include: bool = true;

    for ins in in_vec {

        match ins {
            Node::Mul(num) => {
                if include {
                    accumulator += num;
                }
            },
            Node::Enabled(enable) => include = enable,
        }

    }

    println!("Solution: {}", accumulator);
    
    Ok(())
}




// Function stub
// fn aoc3() -> std::io::Result<()> {

//     println!("Starting Task n, Part 1:");
    
//     let mut file = File::open("data/aoc3")?;

//     Ok(())
// }


fn main() {
    aoc1().unwrap();

    aoc2().unwrap();

    aoc3().unwrap();

}
