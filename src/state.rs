use std::{fmt::Display, io::{stdout, SeekFrom, Write}};
use crate::{read_line, todos::{Todo, TodoList}};
use prettytable::{format, row, Table};
use tokio::fs::{read_dir, File, OpenOptions};

use std::path::PathBuf;
use chrono::{DateTime, Datelike, Local};
use tokio::io::{AsyncReadExt, AsyncSeekExt, AsyncWriteExt};

#[derive(Debug)]
pub enum AppErrorKind {
    Error,
    WrongExtension
}

impl Display for AppErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
          AppErrorKind::Error => {write!(f, "AppError")}
          _ => unreachable!()
        }
    }
}

#[derive(Debug)]
pub struct AppError {
    kind: AppErrorKind,
    description: String,
}

impl AppError {
    pub fn new<T>(kind: AppErrorKind, description: T) -> Self
    where T: ToString
    {
        Self { kind, description: description.to_string() }
    }
}

impl std::error::Error for AppError {}

impl Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error: {} description: {}", self.kind, self.description)
    }
}


type AppStateResult<T> = Result<T, AppError>;

pub struct AppState{
    pub current_list: TodoList,
    pub global_lists: Vec<TodoList>
}

impl AppState {

    pub async fn init<T>(dir_path: T, _default: Option<String>) -> AppStateResult<AppState>
    where T: ToString
     {
        let path = PathBuf::from(dir_path.to_string());
        let mut dir = read_dir(path)
        .await
        .unwrap();

        let mut lists = Vec::new();

        while let Some(entry) = dir.next_entry().await.unwrap() {

                let path = entry.path();
                let extension =  match &path.extension() {
                    Some(ext) => ext.to_str().unwrap(),
                    None => {
                        return Err(AppError::new(AppErrorKind::Error, "recourse folder is only for json lists", ));
                    }
                };

                if extension.trim() != "json" {
                    return Err(AppError::new(AppErrorKind::WrongExtension, format!("Expected Json found {}", extension)));
                }
        
                let list = match TodoList::read_from_file("./recources/default.json").await {
                    Ok(eh) => {
                        eh
                    }
                    Err(e) => {
                        println!("{e}");
                        panic!()
                    }
                };

                lists.push(list);
        
        }
        let current  = if lists.len() == 0 {
            let awns = read_line("No list found want to create a default list".to_string()).unwrap();
            if awns.trim() == "yes" {
                println!("Creating list ...");
                let path = PathBuf::from("./recources/default.json");
                TodoList::create_new("default".to_string(), Some(path)).await
            }else {
                return Err(AppError::new(AppErrorKind::Error, "No list found"));
            }
        }else {
            lists[0].clone()
        };
        Ok(AppState {
            current_list: current.clone(),
            global_lists: lists
        })
    }

    pub fn single_path<T>(todo_path: T) -> AppStateResult<()> {
        todo!()
    }
    
    pub fn update_current(&self, name: String, id: Option<usize>) -> AppStateResult<()> {
        Ok(())
    }

    pub fn find_list(&self, name: String) -> AppStateResult<usize> {
        for (id, list) in self.global_lists.iter().enumerate() {
            if list.name.trim() == name.trim() {
                return Ok(id);
            }
        }
        Err(AppError::new(AppErrorKind::Error, "List does NOT exist"))
    }

    pub fn add_to_list<T>(id: Option<usize>) -> AppStateResult<()> {
        todo!()
    }

    pub fn remove_from_list<T>(id: Option<usize>) -> AppStateResult<()> {
        todo!()
    }

    pub fn show_list(&self, index: Option<usize>) -> AppStateResult<()> {
        let current = &self.current_list;
        current.list();
        Ok(())
    }

    pub fn complete_list_task(list_id: usize, todo_id: usize) {

    }


}

