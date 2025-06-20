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


fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 0 {
        let mut flag: &str = "none";
        let mut file: &str = "";
        let mut start: isize = 0;
        let mut end: isize = 50;
        let mut help: bool = false; 
        for arg in &args{
            
            if flag == "file" {
            file = arg;
            } else if flag == "start" { 
            start = arg.parse().unwrap();
            } if flag == "end" {
            end = arg.parse().unwrap(); 
            } 
            
            flag = match &arg[..] {
                "--help" => "help", 
                "--all"  => "all",
                    "-f" => "file",
                    "-s" => "start",
                    "-e" => "end",
                       _ => "none",
            };
            
            if flag == "all" {
            end = -1;
            } else if flag == "help" {
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

    } else {
        println!("Empty Try appending --help for help with this program");
    }
}

