mod config;
mod state;
mod todos;

use chrono::Local;
use clap::{ command, value_parser, Arg, Command};
use colored::Colorize;
use state::AppState;
use std::{
    io::{self, stdin, stdout, Read, Write},
    sync::{Arc, Mutex},
};
use todos::Todo;

const TITLE: &str = "\
████████  ██████  ██████  ██    ██ ██
   ██    ██    ██ ██   ██ ██    ██ ██
   ██    ██    ██ ██   ██ ██    ██ ██
   ██    ██    ██ ██   ██ ██    ██ ██
   ██     ██████  ██████   ██████  ██
";

#[tokio::main]
async fn main() {
    let state = Arc::new(Mutex::new(AppState::init("./recources", None).await.unwrap()));

    let matches = command!()
        .subcommand(Command::new("add").args(
            &[
                Arg::new("name").short('n').long("name"),
                Arg::new("description").short('d').long("desc")
            ]
        ).about("adds a item to the todo list"))
        .subcommand(Command::new("remove").args(&[
            Arg::new("item").short('i').long("item").value_parser(value_parser!(usize))
        ]).about("removes an item from the todo list"))
        .subcommand(Command::new("list").about("lists all items in the todo list"))
        .subcommand(Command::new("complete").arg(Arg::new("delete").short('d').long("delete")).arg(Arg::new("item").short('i').long("item").value_parser(value_parser!(usize))).about("sets a task to be completed"))
        .get_matches();

    match matches.subcommand() {
        Some(("add", submatches)) => {
            let name = match submatches.get_one::<String>("name"){
                Some(name) => name.clone(),
                None => {
                   let name = read_line("whats the name of your task? ".to_string()).unwrap();
                    name
                }
            };

            let description = submatches.get_one::<String>("description").cloned();
            let todo = Todo::new(name, description, false);
            let mut state_ref = state.lock().unwrap();
            state_ref.add_to_list(None, todo).await.unwrap();
            state_ref.current_list.list();


        }
        Some(("list", _submatches)) => {
            let state_ref = state.lock().unwrap().current_list.list();

        }
        Some(("complete", submatches)) => {
            
            let index: usize = match submatches.get_one::<usize>("item") {
                Some(index) => index.clone(),
                None => {
                    print!("{}", "Whats the name of the completed todo? >".bright_cyan().italic());
                    let mut name = String::new();
                    stdout().flush().unwrap();
                    stdin().read_line(&mut name).unwrap();
                    unreachable!()
                   
                }
            };

            let should_delete: bool = match submatches.get_one::<String>("delete") {
                Some(awns) => {
                    awns.trim() == "yes".trim()
                }
                None => {
                    print!("{}", "should we delete the todo? >".bright_cyan().italic());
                    stdout().flush().unwrap();
                    let mut awns = String::new();
                    io::stdin().read_line(&mut awns).unwrap();
                    awns.trim() == "yes".trim()

                }
            };

            
        }
        Some(("remove", submatches)) => {
            let index = match submatches.get_one::<usize>("item") {
                Some(index) => index.clone() as usize,
                None => {
                    let name = read_line("what is the name of the task to remove? ").unwrap();
                    let id  = state.lock().unwrap().current_list.find_todo_id(name).unwrap();
                    id
                }
            };

            state.lock().unwrap().remove_from_list(index).await.unwrap()


        }
        _ => {
            println!("{}", TITLE.green())
        },
    }
}


fn read_line<T>(prompt: T) -> Result<String, std::io::Error>
where T: ToString
{
    let mut contents = String::new();

    print!("{} > ", prompt.to_string().bright_cyan().italic());
    io::stdout().flush()?;
    io::stdin().read_line(&mut contents)?;

    Ok(contents)
}


//i love you becca <3