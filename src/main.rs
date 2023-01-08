use std::{
    fs::File,
    io::{stdin, BufRead, Write},
    path::Path,
    process::Command,
};

const PATH_CODE: &str = "C:/rust_fly";

fn main() {
    // Check folder exist and if not, create it
    let path_exist: bool = Path::new(PATH_CODE).exists();
    if !path_exist {
        Command::new("cargo")
            .current_dir("C:/")
            .arg("new")
            .arg("rust_fly")
            .spawn()
            .expect("Creating the folder failed");
    }

    println!("Test your code");
    // initialize data
    let mut lines = stdin().lock().lines();
    let mut user_input: Vec<String> = vec![];
    // accept input
    while let Some(line) = lines.next() {
        let current_input = line.unwrap();

        if current_input == "exit()" {
            break;
        }

        if current_input.len() == 0 {
            create_file(&user_input);
            execute_code();
            user_input.clear();
            continue;
        }

        let new_input = format!("\n{}", current_input);
        user_input.push(new_input);
    }
}

fn create_file(user_input: &Vec<String>) {
    let file_name = format!("{}/src/main.rs", PATH_CODE);
    let mut file = File::create(&file_name).unwrap();
    // write file
    let result = format!(
        "fn main() {{\n {} \n}}",
        String::from_iter(user_input.clone())
    );
    file.write(result.as_bytes()).unwrap();
}

fn execute_code() {
    let executing = Command::new("cargo")
        .current_dir(PATH_CODE)
        .arg("run")
        .arg("--release")
        .spawn()
        .expect("The program execution failed");
    executing.wait_with_output().unwrap();
}
