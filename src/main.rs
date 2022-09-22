use std::{
    io::{stdin, stdout, Write},
    process::Command,
};

fn main() {
    let current_directory = Command::new("pwd").arg("").output().expect("msg");
    loop {
        // use the `>` character as the prompt
        // need to explicitly flush this to ensure it prints before read_line
        print!(
            "{} -> ",
            String::from_utf8_lossy(&current_directory.stdout).trim()
        );
        stdout().flush();

        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        let command = input.trim();

        let child = Command::new(command).spawn();

        let error_message = match child {
            Ok(mut child) => {
                child.wait(); "".to_string()
            }
            Err(ve) => ve.to_string(),
        };
        println!("{}",error_message)
    }
}
