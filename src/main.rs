
use std::fs;
use std::env;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::Read;
use std::io::Write;



fn main() {

    check_for_log();

    let env: Vec<String> = env::args().collect();

    match env {
        v if v.len() < 2 =>  println!("Not enough arguments!"),
        v if v.get(1) == Some(&String::from("list")) => command_list(),
        v if v.get(1) == Some(&String::from("add")) => command_add(&v[2..]),
        v if v.get(1) == Some(&String::from("delete")) => command_delete(&v[2..]),
        v if v.get(1) == Some(&String::from("done")) => command_done(&v[2..]),

        Vec{..} => return
    }







    
    

}

fn command_done(to_done : &[String]) {
    let mut file = parse_file();


    for item in to_done {
        match item.parse::<usize>() {
            Ok(item) => {
                let index = item - 1;
                let mut buff =  file.get(index).unwrap().to_owned();
                buff.push_str(" \u{2713}");
                file[index] = buff.to_string();
            }
            Err(e) => println!("{e}"),
        }

    }
    
    write_file(file)
}

fn command_delete(to_delete : &[String]) {
    let mut file: Vec<String> = parse_file();

    for item in to_delete {
        match item.parse::<usize>() {
            Ok(item) => {
                let index = item - 1;
                if index > file.len(){
                    println!("Out of bounds");
                    break;
                }
                file.remove(index);
            }
            Err(e) => println!("{e}"),
        }

    }


    write_file(file)


}
fn write_file(contents: Vec<String>) {
    let mut buff = String::new();


    for str in contents {
        let cleaned_str = str.trim();  // Remove leading and trailing whitespace
        buff.push_str(cleaned_str);
        buff.push('\n'); 
    }


    let mut file = OpenOptions::new().write(true).truncate(true).open("log.txt").expect("No file found");

    _ = file.write(buff.as_bytes())

}

fn parse_file() -> Vec<String> {
    let mut file = OpenOptions::new().read(true).write(true).create(true).open("log.txt").unwrap();

    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let mut split: Vec<String> = contents.split("\n").map(str::to_string).collect();
    split.retain(|s| s != "");

    return split;
}


fn command_list() {

    let split = parse_file();

    for (i,el) in split.iter().enumerate() {
        if el == ""{
            continue;
        }
        println!("{} : {el}", i + 1)
    }

}

fn command_add(to_add : &[String]) {

    let mut buff = String::new();

    for str in to_add {
        buff = buff + str + "\n";
    }

    let mut file = OpenOptions::new().append(true).create(true).open("log.txt").expect("No file found");

    _ = file.write(buff.as_bytes())


}

fn check_for_log() {
    match fs::metadata("log.txt") {
        Ok(metadata) => {
            if metadata.is_file() {
                return;
            }
            else{
                let _file = File::create("log.txt");
            }
        },
        Err(e) => println!("Error encountered with file checking:  {e}"),
        
    }
}


// struct for the CLI
    // Get input, parse command
    // parse arguments
    // pass struct with command and arguments
// struct for the log output
// struct for the file writing


