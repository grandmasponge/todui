use std::{fmt::Display, io::{stdin, Write}, path::PathBuf};
use std::io::SeekFrom;
use chrono::{DateTime, Datelike, Local};
use colored::Colorize;
use prettytable::{format, row, Table};
use serde::{Deserialize, Serialize};
use tokio::{fs::{File, OpenOptions}, io::{stdout, AsyncReadExt, AsyncSeekExt, AsyncWriteExt}};

#[derive(Debug)]
pub enum TodoErrors {
    TodoNotFound,
    TodoClosing,
    TodoSeralizationError,
    OutOfBounds
}

impl Display for TodoErrors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error")
    }
}

impl std::error::Error for TodoErrors {}

type TodoResult<T> = Result<T, TodoErrors>;


#[derive(Serialize, Deserialize, Debug, Clone)]
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

        // Try opening the file asynchronously
        let file = File::open(&path).await;

        let mut file = match file {
            Ok(file) => file,
            Err(_) => {
                // Handle file not found or corrupted error
                let mut answer = String::new();
               
                
                stdout().write_all(b"Todo list not found or corrupted. Would you like to create a new Todo list at that location? > ").await.unwrap();
                stdout().flush().await.unwrap();
    
                stdin().read_line(&mut answer).unwrap();
                if answer.trim() == "yes" {
                    let name = path.file_name()
                        .and_then(|n| n.to_str())
                        .unwrap_or("default_todo_list")
                        .to_string();
    
                    return Ok(Self::create_new(name.to_owned(), Some(path)).await);
                } else {
                    println!("Closing...");
                    return Err(TodoErrors::TodoClosing);
                }
            }
        };

        file.seek(std::io::SeekFrom::Start(0)).await.unwrap();
    

        let mut contents = String::new();
        file.read_to_string(&mut contents).await.unwrap();

        let list: TodoList = serde_json::from_str(&contents).unwrap_or_else(|err| {
            println!("{err}\n Failed to parse list initalizing and empty one instead");
            TodoList {
                path: PathBuf::from("./recources/default.json"),
                name: String::from("default"),
                list: Vec::new(),
            }
        });
    
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
            let time = Local::now();
            todo.completed = true;
            todo.date_completed = Some(time);

            Ok(())

        }
        else {
            Err(TodoErrors::OutOfBounds)
        }
    }

    pub async fn write_to_file(&self) -> TodoResult<()> {
        let mut file = OpenOptions::new()
            .create(false)
            .read(true)
            .write(true)
            .truncate(true)
            .open(&self.path)
            .await
            .unwrap();
        file.seek(SeekFrom::Start(0)).await.unwrap();

        let data = serde_json::to_string_pretty(self).unwrap();

        file.write_all(data.as_bytes())
            .await
            .unwrap();
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

    pub fn list(&self) {

        let fmt_date = |x: DateTime<Local>| {
             format!("{}/{}/{}", x.day(), x.month(), x.year())
        };
        let mut table = Table::new();
            table.set_titles(row!["id", "Name", "Description", "Date added", "Completed", "Date Completed"]);
            table.set_format(*format::consts::FORMAT_NO_BORDER_LINE_SEPARATOR);
        for (index, todo) in self.list.iter().enumerate() {


            let description = match &todo.description {
                Some(desc) => desc,
                None => "N/A"
            };
            let completed = if todo.completed {
                "Yes"
            } else {
                "No"
            };
            let date_added = fmt_date(todo.date_added);
            let date_completed = match &todo.date_completed {
                Some(date) => fmt_date(date.clone()),
                None => "N/A".to_string()
            };

            table.add_row(row![index, todo.name, description, date_added, completed, date_completed]);
            
        }
        table.printstd();
    }

}


#[derive(Serialize, Deserialize, Debug, Clone)]
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

