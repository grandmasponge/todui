use std::{fmt::Display, io::Write, path::PathBuf};

use chrono::{DateTime, Local};
use colored::Colorize;
use serde::{Deserialize, Serialize};
use tokio::{fs::{File, OpenOptions}, io::{AsyncReadExt, AsyncSeekExt, AsyncWriteExt}};

#[derive(Debug)]
pub enum TodoErrors {
    TodoNotFound,
    TodoClosing,
    OutOfBounds
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


    pub async fn add(&mut self, todo: Todo) -> TodoResult<()> {
        self.list.push(todo);
        self.write_to_file().await?;
        Ok(())
    }

    pub async fn remove(&mut self, id: usize) -> TodoResult<()> {
        if 0 <= id && id <= self.list.len() {
            self.list.remove(id);
            self.write_to_file().await?;
            Ok(())
        }
        else {
            Err(TodoErrors::OutOfBounds)
        }
    }

    pub fn complete(&mut self, id: usize) -> TodoResult<()> {
        if 0 <= id && id <= self.list.len() {
            let todo = match self.list.get_mut(id) {
                Some(todo) => todo,
                None => return Err(TodoErrors::TodoNotFound)
            };
            todo.completed = true;

            Ok(())

        }
        else {
            Err(TodoErrors::OutOfBounds)
        }
    }

    pub async fn write_to_file(&self) -> TodoResult<()> {
        Ok(())
    }

    pub fn find_todo_id(&self, name: String) -> TodoResult<usize> {
        for (index, todo )in self.list.iter().enumerate() {
            if todo.name.trim() == name.trim() {
                return Ok(index);
            }
        }
        Err(TodoErrors::TodoNotFound)
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


// pub async fn write_to_file(&mut self) {
//     let data = serde_json::to_string_pretty(&self.todo_list)
//         .unwrap();
//     let mut file = OpenOptions::new()
//         .write(true)
//         .read(true)
//         .truncate(true)
//         .create(true)
//         .open("./recources/todo.json")
//         .await.unwrap();

//     file.seek(SeekFrom::Start(0)).await.unwrap();

//     file.write_all(data.as_bytes())
//         .await
//         .unwrap();

// }


// pub fn list(&self) {
//     println!("Todos");

//     let fmt_date = |x: DateTime<Local>| {
//         format!("{}/{}/{}", x.day(), x.month(), x.year())
//     };

//     let mut table = Table::new();
//     table.set_format(*format::consts::FORMAT_NO_BORDER_LINE_SEPARATOR);
//     table.set_titles(row!["Item", "Todo", "Description", "Completed", "Date", "Date Completed"]);
//     for (index, data)  in self.todo_list.iter().enumerate() {
//         let description = match &data.description {
//             Some(s) => s.clone(),
//             None => "None".to_string(),
//         };
//         let completed = if data.completed {
//             "Yes".to_string()
//         } else {
//             "No".to_string()
//         };

//         let date_completed =  match data.date_completed {
//             Some(date) => fmt_date(date),
//             None => "N/A".to_string()
//         };

//         let added = fmt_date(data.date_added); 
//         table.add_row(row![index, data.name, description, completed, added, date_completed]);
//     }

//     table.printstd();
// }