use std::fs;
use std::io::{Read, Seek, SeekFrom};

fn hex_from_file(filepath : &str, mut start: isize, mut end: isize){
    
    if !fs::exists(filepath).expect("uh oh"){
        println!("{filepath} does not exist");
        return;
    }

    let mut file = fs::File::open(filepath).expect("error creating file");
    let metadata = fs::metadata(filepath);

    if end == -1{
    end = metadata.expect("error accessing metadata").len() as isize;
    start = 0;
    } else if end - start < 1{
    println!("Start, End are setup wrong. Try --help");
    return;
    }

    //type conversion
    let length = (end - start) as usize;
    let start: u64 = start as u64;

    println!("-f {filepath}, -s {start}, -e {end}"); 

    // allocatye memory on the heap
    // load the values into the heap memory and print to screen

    let mut buf : Vec<u8> = vec![0; length];
    
    file.seek(SeekFrom::Start(start)).expect("error seeking");
    file.read_exact(&mut buf).expect("error reading file");
    
    for index in 0..length {
    if (index+1)%10 == 0{
        println!("{:X?}", buf[index]);
    } else {
        print!("{:X?} ", buf[index]);
    
    }}
    
}

#[derive(PartialEq, Eq)]
enum Flag {
    None,
    Help,
    File,
    Start,
    End,
    All,
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut flag = Flag::None;
    let mut file: &str = "";
    let mut start: isize = 0;
    let mut end: isize = 50;
    let mut help: bool = false; 
    for arg in &args{
            
        if flag == Flag::File {
        file = arg;
        } else if flag == Flag::Start { 
        start = arg.parse().unwrap();
        } if flag == Flag::End {
        end = arg.parse().unwrap(); 
        } 
            
        flag = match &arg[..] {
            "--help" => Flag::Help, 
            "--all"  => Flag::All,
                "-f" => Flag::File,
                "-s" => Flag::Start,
                "-e" => Flag::End,
                   _ => Flag::None,
        };
            
        if flag == Flag::All {
        end = -1;
        } else if flag == Flag::Help {
        help = true;
        } 
    } 

    if help {
    println!("Append the pattern -f with a valid filepath e.g. -f file.txt");
    println!("Other options:");
    println!("--all : Specifies show the whole file");
    println!("-s : Which byte to start at. Must be a number!");
    println!("-e : Which byte to end at. Must be a number!");
    println!("by default uses -s 0, -e 50");
    } else if file != "" {
        hex_from_file(file, start, end);
    }else {
    println!("Try appending --help for help with this program");
    }

     
}

