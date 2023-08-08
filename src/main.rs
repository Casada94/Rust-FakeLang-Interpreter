use std::collections::HashMap;
use std::str;
use std::io;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::process::exit;

struct LineItem {
    line_num:i32,
    code:String,
}

fn main() {
    println!("Begin the interpretation");

    let file = File::open("C:\\Users\\metal\\Desktop\\FakeLanguageInterpreter\\src\\myLang.ml").expect("you fucked up");
    let mut reader = BufReader::new(file);
    let mut source_code:Vec<LineItem> = Vec::new();
    linize_me(&mut reader, &mut source_code);

    let mut variables:HashMap<String,i32> = HashMap::new();

    let mut previous_line_number:i32 = -1;
    'lines: for line in source_code{
        if previous_line_number == -1 {
            previous_line_number = line.line_num;
        } else if previous_line_number >= line.line_num {
            println!("Line number must be non repeating and in ascending order");
            exit(-1);
        }
        let tokens  = tokenize_me(&line.code);
        let mut index = 0;
        'tokens: while index < tokens.len() {
            match u8_arr_to_str(tokens[index]) {
                "integer" => {
                    index+=1;
                    integer(index,&tokens,&mut variables);
                    break 'tokens;
                }
                "println" => {
                    if index+1 < tokens.len() {
                        println!("{}",u8_arr_to_str(tokens[index+1]));
                    }
                    break 'tokens;
                }
                "print" => {
                    if index+1 <= tokens.len() {
                        print!("{} ", u8_arr_to_str(tokens[index + 1]));
                        io::stdout().flush().expect("That sucks for you bro");  //stdout is buffered need to flush if want to see before stdin
                    }
                    break 'tokens;
                }
                "input" => {
                    if index+1 < tokens.len() {
                        get_input(&tokens[index + 1], &mut variables);
                        break 'tokens;
                    } else {
                        println!("No variable defined for storing user input");
                        exit(-1);
                    }
                }
                "if" => {
                    conditional(index+1,&tokens,&variables);
                    break 'tokens;
                }
                "then" => {
                    println!("\"then\" has no matching \"if\" statement!");
                    exit(-1);
                }
                "end" => {
                    println!("End interpretation");
                    exit(0);
                }
                _ => {
                    println!("Unknown token: {}", u8_arr_to_str(tokens[index]));
                    exit(-1);
                }
            }
        }
    }
}

fn conditional(index:usize,tokens:&Vec<&[u8]>, variables:&HashMap<String,i32>){
    if tokens.len() - index >= 3 {
        if variables.contains_key(&u8_arr_to_string(tokens[index]))
            && variables.contains_key(&u8_arr_to_string(tokens[index+2])){
            match u8_arr_to_str(tokens[index+1]) {
                ">" => {
                    if variables.get(&u8_arr_to_string(tokens[index])) > variables.get(&u8_arr_to_string(tokens[index+2])){
                        //do the then stuff
                        then(index+4,tokens);
                    }
                }
                "<" => {
                    if variables.get(&u8_arr_to_string(tokens[index])) < variables.get(&u8_arr_to_string(tokens[index+2])){
                        //do the then stuff
                        then(index+4,tokens);
                    }
                }
                "=" => {
                    if variables.get(&u8_arr_to_string(tokens[index])) == variables.get(&u8_arr_to_string(tokens[index+2])){
                        //do the then stuff
                        then(index+4,tokens);
                    }
                }
                _ => {
                    println!("Invalid comparison operator: {}",u8_arr_to_str(&tokens[index+1]) );
                    exit(-1);
                }
            }
        }
    } else {
        println!("Missing variables in conditional");
        exit(-1);
    }
}

fn then(start:usize,tokens:&Vec<&[u8]>){
    match u8_arr_to_str(tokens[start]) {
        "println" => {
            println!("{}",&u8_arr_to_str(tokens[start+1]));
        }
        "print" => {
            print!("{}",&u8_arr_to_str(tokens[start+1]));
            io::stdout().flush().expect("that fucked up");
        }
        _ => {
            println!("Unknown token");
            exit(-1);
        }
    }
}

fn get_input(var_name:&[u8], variables:&mut HashMap<String,i32>){
    if variables.contains_key(u8_arr_to_str(var_name)) {
        let mut user_input: String = String::new();
        io::stdin().read_line(&mut user_input).expect("you fucked up");
        let value = match user_input.trim().parse::<i32>() {
            Ok(input) => input,
            Err(e) => {
                println!("Input must be a valid i32; you entered {}", user_input);
                exit(-1)
            }
        };
        variables.insert(u8_arr_to_string(var_name), value);
    } else {
        println!("Attempted to initialized an undeclared variable");
        exit(-1);
    }
}

fn integer(start:usize, tokens:&Vec<&[u8]>, variable:&mut HashMap<String,i32>) {
    for i in start..tokens.len() {
        variable.insert(u8_arr_to_string(tokens[i]), 0);
    }
}

fn u8_arr_to_str(input:&[u8]) -> &str {
    str::from_utf8(input).unwrap()
}

fn u8_arr_to_string(input:&[u8]) -> String{
    String::from_utf8_lossy(input).to_string()
}

fn u8_arr_to_i32(input:&[u8]) -> i32 {
    i32::from_be_bytes(input.try_into().unwrap())
}

fn get_line_item(line_num:i32, code:String) -> LineItem{
    LineItem {
        line_num,
        code,
    }
}

fn linize_me(reader:&mut BufReader<File>, line_map:&mut Vec<LineItem>){
    for lines in reader.lines() {
        let line = match lines {
            Ok(str) => str,
            Err(e) => exit(-1)
        };
        let num_code: (&str, &str) = line.split_once(' ').unwrap().clone();
        line_map.push(get_line_item(num_code.0.clone().parse::<i32>().expect("Line must start with valid number"), num_code.1.clone().parse().unwrap()));
    }

}

fn tokenize_me(code:&String) -> Vec<&[u8]> {
    let mut index = 0;
    let mut begin =0;
    let mut tokens:Vec<&[u8]> = Vec::new();
    let line_bytes = code.as_bytes();
    while index < line_bytes.len() {
        match line_bytes[index] as char {
            '"' => {
                begin+=1;
                index+=1;
                while line_bytes[index] != '"' as u8{
                    if index == line_bytes.len() - 1 {
                        println!("non matching quotation!");
                        exit(-1);
                    }
                    index+=1;
                }
                tokens.push(&line_bytes[begin..index]);
                index+=1;
                begin = index;
            }
            ' ' | ',' | '\t' => {
                if !line_bytes[begin..index].is_empty(){
                    tokens.push(&line_bytes[begin..index]);
                }
                index+=1;
                begin = index;
            }
            _ =>{
                index+=1;
            }
        }
    }
    if begin != index{
        tokens.push(&line_bytes[begin..index]);
    }
    tokens
}