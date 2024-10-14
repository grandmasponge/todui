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
    



}

