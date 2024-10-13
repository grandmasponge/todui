mod config;
mod state;
mod todos;

use chrono::Local;
use clap::{ command, value_parser, Arg, Command};
use colored::Colorize;
use std::{
    io::{self, stdin, stdout, Write},
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
    let state = Arc::new(Mutex::new(state::AppState::read_todo_history(None).await));

    let matches = command!()
        .subcommand(Command::new("add").about("adds a item to the todo list"))
        .subcommand(Command::new("remove").args(&[
            Arg::new("item").short('i').long("item").value_parser(value_parser!(usize))
        ]).about("removes an item from the todo list"))
        .subcommand(Command::new("list").about("lists all items in the todo list"))
        .subcommand(Command::new("complete").arg(Arg::new("delete").short('d').long("delete")).arg(Arg::new("item").short('i').long("item").value_parser(value_parser!(usize))).about("sets a task to be completed"))
        .get_matches();

    match matches.subcommand() {
        Some(("add", _submatches)) => {
            let mut name = String::new();
            let mut description = String::new();
            print!("{}[2J", 27 as char);
            print!("{}", "Input the name of the todo > ".bright_cyan().italic());
            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut name).unwrap();
            print!("{}[2J", 27 as char);
            print!(
                "<{}> {}",
                "Optional".bright_black(),
                "input the description of the todo >".bright_cyan().italic()
            );
            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut description).unwrap();
            print!("{}[2J", 27 as char);
            //check if descripton is empty
            let description = if !description.is_empty() {
                Some(description)
            } else {
                None
            };

            let todo = Todo::new(name, description, false);
            let mut state_ref = state.lock().unwrap();

            state_ref.todo_list.push(todo);
            state_ref.list();
            state_ref.write_to_file().await;
        }
        Some(("list", _submatches)) => {
            state.lock().unwrap().list();
        }
        Some(("complete", submatches)) => {
            let index: usize = match submatches.get_one::<usize>("item") {
                Some(index) => index.clone(),
                None => {
                    print!("{}", "Whats the name of the completed todo? >".bright_cyan().italic());
                    let mut name = String::new();
                    stdout().flush().unwrap();
                    stdin().read_line(&mut name).unwrap();
                    state.lock().unwrap().find_todo(name)
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

            let mut state_ref = state.lock().unwrap();

            if should_delete {
                state_ref.remove(index);
                state_ref.write_to_file().await;
            }
            else {
                let item = &mut state_ref
                .todo_list
                .get_mut(index)
                .unwrap();
                item.completed = true;
                item.date_completed = Some(Local::now());

                state_ref.write_to_file().await;
            }

            state_ref.list();
            
        }
        Some(("remove", submatches)) => {
            let index = match submatches.get_one::<usize>("item") {
                Some(index) => index.clone() as usize,
                None => {
                    let mut name = String::new();
                    print!("{}", "Whats the name of the todo to delete> ".bright_cyan());
                    io::stdout().flush().unwrap();
                    io::stdin().read_line(&mut name).unwrap();

                    state.lock().unwrap().find_todo(name)
                }
            };
            let mut state_ref = state.lock().unwrap();
            state_ref.remove(index);
            state_ref.list();
            state_ref.write_to_file().await;

        }
        _ => {
            println!("{}", TITLE.green())
        },
    }
}
