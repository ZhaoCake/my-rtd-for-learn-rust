use crate::model::Item;
use std::fs::{self, OpenOptions};
use std::io::{self, BufRead, BufReader, Write};

pub struct Csv {
    filename: String,
}

impl Csv {
    pub fn new(filename: String) -> Self {
        Csv { filename }
    }

    pub fn add_item(&self, item: &Item) -> io::Result<()> {
        let mut file = OpenOptions::new().append(true).open(&self.filename)?;
        writeln!(file, "{}", item.to_string())
    }

    pub fn update_item(&self, updated_item: &Item) -> io::Result<()> {
        let items = self.get_all()?;
        let mut file = OpenOptions::new().write(true).open(&self.filename)?;
        file.set_len(0)?; // 清空文件

        for item in items {
            let item_to_write = if item.id == updated_item.id {
                updated_item
            } else {
                &item
            };
            writeln!(file, "{}", item_to_write.to_string())?;
        }
        Ok(())
    }

    pub fn delete_item(&self, id: u32) -> io::Result<()> {
        let items = self.get_all()?;
        let mut file = OpenOptions::new().write(true).open(&self.filename)?;
        file.set_len(0)?; // 清空文件

        for item in items {
            if item.id != id {
                writeln!(file, "{}", item.to_string())?;
            }
        }
        Ok(())
    }

    pub fn get_all(&self) -> io::Result<Vec<Item>> {
        println!("Reading from file: {}", &self.filename);
        let file = fs::File::open(&self.filename)?;
        let reader = BufReader::new(file);
        let mut items = Vec::new();

        for line in reader.lines() {
            println!("Reading line: {:?}", line);
            if let Some(item) = Item::from_string(&line?) {
                // 从文件中读取的每一行都是一个Item
                items.push(item);
            }
        }

        println!("Number of items: {}", items.len());
        Ok(items)
    }

    pub fn get_item_by_id(&self, id: u32) -> io::Result<Option<Item>> {
        let items = self.get_all()?;
        for item in items {
            if item.id == id {
                return Ok(Some(item));
            }
        }
        Ok(None)
    }

    pub fn get_max_id(&self) -> u32 {
        match self.get_all() {
            Ok(items) => items.iter().map(|item| item.id).max().unwrap_or(0),
            Err(_) => 0,
        }
    }
}
