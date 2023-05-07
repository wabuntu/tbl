use std::io;
use clap::Parser;

struct Args {
    name: String,
    count: u8,
}

fn main() {
    /*
    let args = Args::parse();
    for _ in 0..args.count {
        println!("Hello {}!", args.name)
    }
    */

    let (head, tail, mid, virt, corner) = ('=', '=', '-', '|', '+'); 
    const del: &str = "\t";
    let mut csv = String::from("");
    
    loop {
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(len) => if len == 0 {
                break;
            } else {
                csv += &input;
            } 
            Err(error) => {
                eprintln!("error: {}", error);
                break;
            }
        }
    }

/*
    let input = io::stdin().lock().lines().fold("".to_string(), |acc, line| {
        acc + &line.unwrap() + "\n"
    });
    dbg!(input);
*/

    let mut col_num = 0;
    let mut col_len: [usize; 100] = [0; 100];
    let mut col = 0;

    // Count the longest for each column
    for line in csv.lines() {
        col = 0;
        col_num = 0;
        for item in line.split(del) {
            //println!("[{}]", item);
            col_len[col] = if item.len() > col_len[col] { item.len() } else { col_len[col] };
            col += 1;
            col_num += 1;            
        }
    }

    for line in csv.lines() {
        // Header
        print!("{}", corner);
        for with in col_len {
            for _ in 0..with {
                print!("{}", head);                
            }
        }
        for _ in 0..col_num -1 {
            print!("{}", head);                
        }    
        print!("{}", corner);
        println!();
        break;
    }

    for line in csv.lines() {
        // Header
        print!("{}", virt);
        col = 0;
        for item in line.split(del) {
            print!("{:>len$}", item, len = col_len[col]);
            print!("{}", virt);
            col += 1;
        }
        println!();
    }

    for line in csv.lines() {
        // Bottom
        print!("{}", corner);
        for with in col_len {
            for _ in 0..with {
                print!("{}", head);                
            }
        }
        for _ in 0..col_num -1 {
            print!("{}", head);                
        }    
        print!("{}", corner);
        println!();
        break;
    }    

}