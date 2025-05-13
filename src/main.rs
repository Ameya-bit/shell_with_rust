use std::process::{Command, Stdio, Child};
use std::io::{stdin, stdout, Write};
use std::path::Path;
use std::env;


fn main(){
    loop {
        print!("> ");
        stdout().flush();

        // get user command
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        // split by pipes
        let mut commands = input.trim().split(" | ").peekable();
        let mut previous_command = None;

        // iterate over each command
        while let Some(command) = commands.next() {
            // split by command and arguments
            let mut input_parts = command.trim().split_whitespace();
            let command = input_parts.next().unwrap();
            let args: Vec<&str> = input_parts.collect();
        
            // match command to action
            match command {
                "exit" => return,
                "cd" => {
                    let new_dir = args.first().iter().peekable().peek().map_or("/" , |x| *x);
                    let root = Path::new(new_dir);
                    if let Err(e) = env::set_current_dir(&root) {
                        eprintln!("{}", e);
                    }
                }
                command => {
                    let stdin = previous_command
                        .map_or(
                            Stdio::inherit(),
                            |output: Child| Stdio::from(output.stdout.unwrap())
                        );
                    let stdout = if commands.peek().is_some() {
                        Stdio::piped()
                    } else {
                        Stdio::inherit()
                    };


                    let mut com = Command::new(command)
                        .args(args)
                        .stdin(stdin)
                        .stdout(stdout)
                        .spawn();
                        

                    match com{
                        Ok(mut com) => {previous_command = Some(com);},
                        Err(e) => {
                            previous_command = None;
                            eprintln!("Error: {}", e);
                        }
                    }
                }
            }
        }
        // run the full command
        if let Some(mut final_command) = previous_command {
            // block until the final command has finished
            final_command.wait();
        }
        
    }
}
