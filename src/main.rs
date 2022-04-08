use std::env;
use std::io::Write;
use std::fs::File;

fn main() {
    let mut args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        println!("Wrong fkfile parameter. Please only insert the file name and size.\nexample bellow create a file output.txt with 10KB:\n\tfkfile output.txt 10000");
        return
    }
    let filename = &args[1];

    let filesize_str = &args[2];
    let filesize = filesize_str.parse::<i32>().expect("Filesize isnt integer. Please pass an integer number.");
    
    let mut f = File::create(filename).expect("Unable to create file");
    for  _ in 0..filesize {                                                                                                                                                                  
        write!(&mut f, "\0");
    }
    println!("File writen");                                                                                                                                                        
}