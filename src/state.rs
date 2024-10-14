use std::io::SeekFrom;
use crate::todos::{Todo, TodoList};
use prettytable::{format, row, Table};
use tokio::fs::{File, OpenOptions};

use std::path::PathBuf;
use chrono::{DateTime, Datelike, Local};
use tokio::io::{AsyncReadExt, AsyncSeekExt, AsyncWriteExt};

pub struct AppState {
    pub current_list: TodoList,
    pub global_lists: Vec<TodoList>
}

impl AppState {
    


    pub fn list(&self) {
        println!("Todos");

        let fmt_date = |x: DateTime<Local>| {
            format!("{}/{}/{}", x.day(), x.month(), x.year())
        };

        let mut table = Table::new();
        table.set_format(*format::consts::FORMAT_NO_BORDER_LINE_SEPARATOR);
        table.set_titles(row!["Item", "Todo", "Description", "Completed", "Date", "Date Completed"]);
        for (index, data)  in self.todo_list.iter().enumerate() {
            let description = match &data.description {
                Some(s) => s.clone(),
                None => "None".to_string(),
            };
            let completed = if data.completed {
                "Yes".to_string()
            } else {
                "No".to_string()
            };

            let date_completed =  match data.date_completed {
                Some(date) => fmt_date(date),
                None => "N/A".to_string()
            };

            let added = fmt_date(data.date_added); 
            table.add_row(row![index, data.name, description, completed, added, date_completed]);
        }

        table.printstd();
    }

    pub fn find_todo(&self, name: String) -> usize {
        for (index, todo) in self.todo_list.iter().enumerate() {
            if todo.name.trim() == name.trim() {
                return index;
            }
        }
        unreachable!()
    }

    pub async fn write_to_file(&mut self) {
        let data = serde_json::to_string_pretty(&self.todo_list)
            .unwrap();
        let mut file = OpenOptions::new()
            .write(true)
            .read(true)
            .truncate(true)
            .create(true)
            .open("./recources/todo.json")
            .await.unwrap();

        file.seek(SeekFrom::Start(0)).await.unwrap();

        file.write_all(data.as_bytes())
            .await
            .unwrap();

    }
}

