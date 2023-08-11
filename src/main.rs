use std::collections::{HashMap, HashSet};
use std::{fs, str};
use std::io;
use std::fs::File;
use std::io::{BufRead, BufReader, Read, Write};
use std::process::exit;

struct Token {
    token_type:Type,
    value:String,
}

enum Type {
    Number,
    Literal,
    UserVariable,
    Keyword,
    Comparator,
    Operator,
    Symbol
}

fn load_keywords(file_name:&str) -> HashSet<String>{
    let mut file_path = String::from("C:\\Users\\metal\\Desktop\\FakeLanguageInterpreter\\src\\");
    file_path.push_str(file_name);
    let file_text = fs::read_to_string(file_path).expect("you fucked up");
    let mut keywords:HashSet<String> = HashSet::new();
    for keyword in file_text.split(',') {
        keywords.insert(keyword.to_string());
    }
    keywords
}

fn main() {

    let keywords:HashSet<String> = load_keywords("KeywordList.txt");
    let operators:HashSet<&str> = HashSet::from(["*","/","+","-","%"]);
    let comparators:HashSet<&str> = HashSet::from(["<",">","=="]);
    let symbols:HashSet<&str> = HashSet::from(["="]);

    println!("Start: Load & Parse All");

    let file = File::open("C:\\Users\\metal\\Desktop\\FakeLanguageInterpreter\\src\\myLang.ml").expect("you fucked up");
    let mut reader = BufReader::new(file);
    let mut source_code:Vec<Vec<Token>> = Vec::new();
    parse_all(&mut source_code, &mut reader, &keywords,&operators,&comparators,&symbols);
    println!("End: Load & Parse All");

    println!("Start: Execution");
    // for line in source_code {
    //     for token in line {
    //         print!("{}",token.value);
    //     }
    //     print!("\n");
    // }
    let mut user_variables:HashMap<String,i32> = HashMap::new();
    let mut prev_line_num = None;
    for lines in source_code{
        if let Type::Number = lines[0].token_type {
            let curr_line_num = lines[0].value.parse::<i32>().unwrap();
            if prev_line_num.is_none() {
                prev_line_num = Some(curr_line_num);
            } else if curr_line_num >= prev_line_num.unwrap() {
                    prev_line_num = Some(curr_line_num);
            } else {
                println!("Line numbers must be in ascending order: {} -> {}", prev_line_num.unwrap(), curr_line_num);
                exit(-1);
            }
        } else {
            println!("Lines must start with a number: {} -> {}", prev_line_num.unwrap(), lines[0].value);
            exit(-1);
        }

        execute(&lines[1..lines.len()],&mut user_variables, &keywords,&operators,&comparators,&symbols)
    }




}

fn execute(tokens:&[Token], user_variables: &mut HashMap<String, i32>, keywords:&HashSet<String>, operators:&HashSet<&str>, comparators:&HashSet<&str>, symbols: &HashSet<&str>){
    match tokens[0].value.as_str(){
        "println" => {
            my_println(tokens.get(1));
        }
        "print" => {
            my_print(tokens.get(1));
        }
        "integer" => {
            integer(&tokens[1..tokens.len()] ,user_variables);
        }
        "input" => {
            input(tokens.get(1), user_variables);
        }
        "let" => {
            my_let(&tokens[1..tokens.len()], user_variables);
        }
        "if" => {
            conditional(&tokens[1..tokens.len()], user_variables,keywords,operators,comparators, symbols);
        }
        "then" => {
            execute(&tokens[1..tokens.len()], user_variables, keywords, operators, comparators, symbols);
        }
        "end" => {
            println!("End: Execution");
            exit(0);
        }
        _ => {
            println!("Missing keyword. Found: {}", tokens[0].value);
            exit(-1);
        }
    }

}

fn my_let(tokens:&[Token], user_variables: &mut HashMap<String, i32>){
    if tokens.len() != 3{
        println!("Not enough tokens!");
        exit(-1);
    }
    if let Type::UserVariable = tokens[0].token_type {
        if let Type::Symbol = tokens[1].token_type {
            match tokens[1].value.as_str() {
                "=" => {}
                _ => {
                    println!("Missing Assignment operator. Found: {}", tokens[1].value);
                    exit(-1);
                }
            }
        }
        if let Type::Number = tokens[2].token_type {
            user_variables.insert(tokens[0].value.to_string(), tokens[2].value.parse::<i32>().unwrap());
        } else {
            println!("Expected integer, found: {}", tokens[0].value);
            exit(-1);
        }
    } else {
        println!("{} is not a valid variable", tokens[0].value);
        exit(-1);
    }
}

fn my_println(token:Option<&Token>){
    match token {
        Some(value) => {
            println!("{}", value.value);
            io::stdout().flush().expect("Unexpected error in stdout");
        },
        None => println!()
    };
}

fn my_print(token:Option<&Token>){
    match token {
        Some(value) => {
            print!("{}", value.value);
            io::stdout().flush().expect("Unexpected error in stdout");
        },
        None => {}
    };
}

fn integer(tokens:&[Token], user_variables:&mut HashMap<String,i32>){
    for token in tokens{
        if let Type::UserVariable = token.token_type{
            user_variables.insert(token.value.to_string(),0);
        } else {
            println!("IDK how the hell we got here");
            exit(-1);
        }
    }
}

fn input(token:Option<&Token>, user_variables:&mut HashMap<String,i32>){
    match token {
        Some(value) => {
            if let Type::UserVariable = value.token_type {
                if user_variables.contains_key(&value.value) {
                    let mut user_input: String = String::new();
                    io::stdin().read_line(&mut user_input).expect("Unexpected error in reading input");
                    user_variables.insert(value.value.to_string(), match user_input.trim().parse::<i32>() {
                        Ok(input) => input,
                        Err(e) => {
                            println!("Input must be a valid i32; you entered {}", user_input);
                            exit(-1)
                        }
                    });
                } else {
                    println!("Undeclared variable {}", value.value);
                    exit(-1);
                }
            } else {
                println!("This is not a valid variable name");
                exit(-1);
            }
        }
        None =>{
            println!("You did not provide a variable to bind a value to.");
            exit(-1);
        }
    }
}

fn conditional(tokens:&[Token], user_variables: &mut HashMap<String, i32>, keywords:&HashSet<String>, operators:&HashSet<&str>, comparators:&HashSet<&str>, symbols:&HashSet<&str>){
    if tokens.len() >= 3 {
        if let Type::Comparator = tokens[1].token_type {
            match tokens[1].value.as_str() {
                ">" => {
                    if let Type::UserVariable = tokens[0].token_type{
                        if let Type::UserVariable = tokens[2].token_type{
                            if user_variables.get(tokens[0].value.as_str()).unwrap() > user_variables.get(tokens[2].value.as_str()).unwrap(){
                                execute(&tokens[3..tokens.len()], user_variables, keywords, operators, comparators, symbols)
                            }
                        } else {
                            println!("Tried to use undeclared user variable: {}", tokens[0].value);
                            exit(-1);
                        }
                    } else {
                        println!("Tried to use undeclared user variable: {}", tokens[0].value);
                        exit(-1);
                    }
                }
                "<" => {
                    if let Type::UserVariable = tokens[0].token_type{
                        if let Type::UserVariable = tokens[2].token_type{
                            if user_variables.get(tokens[0].value.as_str()).unwrap() < user_variables.get(tokens[2].value.as_str()).unwrap(){
                                execute(&tokens[3..tokens.len()], user_variables, keywords, operators, comparators, symbols)
                            }
                        } else {
                            println!("Tried to use undeclared user variable: {}", tokens[0].value);
                            exit(-1);
                        }
                    } else {
                        println!("Tried to use undeclared user variable: {}", tokens[0].value);
                        exit(-1);
                    }
                }
                "==" => {
                    if let Type::UserVariable = tokens[0].token_type{
                        if let Type::UserVariable = tokens[2].token_type{
                            if user_variables.get(tokens[0].value.as_str()).unwrap() == user_variables.get(tokens[2].value.as_str()).unwrap(){
                                execute(&tokens[3..tokens.len()], user_variables, keywords, operators, comparators, symbols)
                            }
                        } else {
                            println!("Tried to use undeclared user variable: {}", tokens[0].value);
                            exit(-1);
                        }
                    } else {
                        println!("Tried to use undeclared user variable: {}", tokens[0].value);
                        exit(-1);
                    }
                }
                _ => println!("IDK how in the hell we got here")
            }
        }
    }
}

fn parse_all(source_code:&mut Vec<Vec<Token>>, reader:&mut BufReader<File>,keywords:&HashSet<String>, operators:&HashSet<&str>, comparators:&HashSet<&str>, symbols:&HashSet<&str>){
    for line in reader.lines() {
        match line {
            Ok(str) => {
                match parse_line(&str, keywords,operators,comparators,symbols){
                        Some(token) => source_code.push(token),
                        None => {}
                    }
            },
            Err(e) => exit(-1)
        };
    }
}

fn parse_line(raw_line:&String,keywords:&HashSet<String>, operators:&HashSet<&str>, comparators:&HashSet<&str>, symbols:&HashSet<&str>) -> Option<Vec<Token>>{
    let mut tokens:Vec<Token> = Vec::new();
    let mut quote:bool = false;
    let mut slash:bool = false;
    let mut start =0;
    let mut end =0;
    for i in raw_line.as_bytes() {
        end+=1;
        match *i as char {
            '"' => {
                if quote {
                    tokens.push(get_new_token(Type::Literal, raw_line[start..end-1].to_string()))
                }
                quote=!quote;
                start=end;
            }
            '/' => {
                if !quote {
                    if slash {
                        break;
                    }
                    slash = !slash;
                }
            }
            ' ' | '\r' | '\t' | '\n' | ',' => {
                if !quote {
                    if start != end {
                        let raw_token = raw_line[start..end-1].to_string();
                        tokens.push(get_new_token(determine_type(&raw_token, keywords,operators,comparators,symbols), raw_token));
                        start = end;
                    } else {
                        start = end;
                    }
                }
            }
            _ => {

            }
        }
    }
    if start!=end && !slash {
        let raw_token = raw_line[start..end].to_string();
        tokens.push(get_new_token(determine_type(&raw_token, keywords,operators,comparators,symbols), raw_token));
    }
    if !tokens.is_empty(){
        Some(tokens)
    } else {
        None
    }
}

fn get_new_token(token_type:Type, value:String) -> Token{
    Token {
        token_type,
        value
    }
}

fn determine_type(raw_token:&String,keywords:&HashSet<String>, operators:&HashSet<&str>, comparators:&HashSet<&str>, symbols:&HashSet<&str>) -> Type{
    let raw_token_str = raw_token.as_str();
    if keywords.contains(raw_token_str){
        Type::Keyword
    } else if operators.contains(raw_token_str){
        Type::Operator
    } else if comparators.contains(raw_token_str){
        Type::Comparator
    } else if symbols.contains(raw_token_str) {
        Type::Symbol
    } else {
        if is_digit(raw_token) {
            Type::Number
        } else {
            Type::UserVariable
        }
    }
}

fn is_digit(raw_token:&String) -> bool{
    match raw_token.parse::<i32>() {
        Ok(t) => true,
        Err(e)=> false
    }
}