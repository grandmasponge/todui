use std::{fmt::Display, io::Write, path::PathBuf};

use chrono::{DateTime, Local};
use colored::Colorize;
use serde::{Deserialize, Serialize};
use tokio::{fs::{File, OpenOptions}, io::{AsyncReadExt, AsyncSeekExt, AsyncWriteExt}};

#[derive(Debug)]
pub enum TodoErrors {
    TodoNotFound,
    TodoClosing,
}

impl Display for TodoErrors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error")
    }
}

impl std::error::Error for TodoErrors {}

type TodoResult<T> = Result<T, TodoErrors>;


#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct TodoList {
    pub path: PathBuf,
    pub name: String,
    pub list: Vec<Todo>
}

impl TodoList {
    pub async fn create_new(name: String, path: Option<PathBuf>) -> TodoList {
        let path = path.unwrap_or_else(|| {
            let fmt_path = format!("./recources/{}.json", name);
            PathBuf::from(fmt_path)
        });

        let mut file = OpenOptions::new()
            .create(true)
            .truncate(true)
            .read(true)
            .write(true)
            .open(&path).await.unwrap();

        file.write(b"[]").await.unwrap();

        TodoList {
            path,
            name,
            list: Vec::new(),
        }
    
    }

    pub async fn read_from_file<T>(path: T) -> TodoResult<TodoList>
    where T: ToString 
    {
        let path = PathBuf::from(path.to_string());

        let file = File::open(&path)
        .await;

        let mut file = match file {
            Ok(file) => file,
            Err(e) => {
                let mut awns = String::new();
            print!("{}", "Todo list not found or courrupted would you like to create a new Todo list at that location? > ".bright_cyan().italic());
            std::io::stdout().flush().unwrap();
            std::io::stdin().read_line(&mut awns).unwrap();
            if awns.trim() == "yes" {
                let name = &path
                .file_name()
                .unwrap()
                .to_str()
                .unwrap()
                .to_string();
            
                return Ok(Self::create_new(name.to_owned(), Some(path)).await);
            }
            else{
                println!("Closing...");
                return Err(TodoErrors::TodoClosing)
            }
            }
        };

        file.seek(std::io::SeekFrom::Start(0)).await.unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents)
        .await
        .unwrap();

        let list: TodoList = serde_json::from_str(&contents)
        .unwrap();

        Ok(list)

    }


    pub fn add(&mut self, todo: Todo) -> TodoResult<()> {
        self.list.push(todo);
        Ok(())
    }

    pub fn remove(&self, id: usize) -> TodoResult<()> {

        Ok(())
    }

    pub fn complete(&self) -> TodoResult<()> {
        Ok(())
    }

    pub async fn write_to_file(&self) -> TodoResult<()> {
        Ok(())
    }

    pub fn find_todo_id(&self, name: String) -> TodoResult<usize> {
        for todo in self.list {
            
        }

    }

}


#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Todo {
    pub name: String,
    pub description: Option<String>,
    pub date_added: DateTime<Local>,
    pub completed: bool,
    pub date_completed: Option<DateTime<Local>>
}

impl Todo {
    pub fn new(name: String, description: Option<String>, completed: bool) -> Self {
        let current = Local::now();
        Self {
            name,
            description,
            date_added: current,
            completed,
            date_completed: None
        }
    }
}

