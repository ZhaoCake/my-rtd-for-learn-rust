use crate::model::Item;
use crate::storage::Csv;
use std::time::SystemTime;

pub struct Service {
    storage: Csv,
}

impl Service {
    pub fn new(filename: String) -> Self {
        Service {
            storage: Csv::new(filename),
        }
    }

    pub fn add_item(&self, name: String) {
        let id = self.storage.get_max_id() + 1;
        let new_item = Item::new(id, name.clone(), self.current_time());
        if let Err(e) = self.storage.add_item(&new_item) {
            eprintln!("Add {} fail: {}", name, e);
        }
    }

    pub fn complete_item(&self, id: u32) {
        if let Ok(Some(mut item)) = self.storage.get_item_by_id(id) {
            item.completed = true;
            item.completed_at = Some(self.current_time());
            if let Err(e) = self.storage.update_item(&item) {
                eprintln!("Completed todo {} fail: {}", id, e);
            }
        }
    }

    pub fn uncomplete_item(&self, id: u32) {
        if let Ok(Some(mut item)) = self.storage.get_item_by_id(id) {
            item.completed = false;
            item.completed_at = None;
            if let Err(e) = self.storage.update_item(&item) {
                eprintln!("Uncompleted todo {} fail: {}", id, e);
            }
        }
    }

    pub fn delete_item(&self, id: u32) {
        if let Ok(Some(mut item)) = self.storage.get_item_by_id(id) {
            item.deleted = true;
            item.deleted_at = Some(self.current_time());
            if let Err(e) = self.storage.update_item(&item) {
                eprintln!("Delete todo {} fail: {}", id, e);
            }
        }
    }

    pub fn restore_item(&self, id: u32) {
        if let Ok(Some(mut item)) = self.storage.get_item_by_id(id) {
            item.deleted = false;
            item.deleted_at = None;
            if let Err(e) = self.storage.update_item(&item) {
                eprintln!("Restore todo {} fail: {}", id, e);
            }
        }
    }

    pub fn destroy_item(&self, id: u32) {
        if let Err(e) = self.storage.delete_item(id) {
            eprintln!("Destroy todo {} fail: {}", id, e);
        }
    }

    pub fn destroy_deleted(&self) {
        let items = self.storage.get_all().unwrap_or_default();
        for item in items {
            if item.deleted {
                self.destroy_item(item.id);
            }
        }
    }

    pub fn clear(&self) {
        let items = self.storage.get_all().unwrap_or_default();
        for item in items {
            self.destroy_item(item.id);
        }
    }

    pub fn list_uncompleted(&self) {
        let items = self.storage.get_all().unwrap_or_default();
        for item in items {
            if !item.completed && !item.deleted {
                println!("{}", item.to_prettier_string());
            }
        }
    }

    pub fn list_completed(&self) {
        let items = self.storage.get_all().unwrap_or_default();
        for item in items {
            if item.completed && !item.deleted {
                println!("{}", item.to_prettier_string());
            }
        }
    }

    pub fn list_deleted(&self) {
        let items = self.storage.get_all().unwrap_or_default();
        for item in items {
            if item.deleted {
                println!("{}", item.to_prettier_string());
            }
        }
    }

    pub fn list_all(&self) {
        let items = self.storage.get_all().unwrap_or_default();
        for item in items {
            println!("{}", item.to_prettier_string());
        }
    }

    fn current_time(&self) -> String {
        SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs()
            .to_string()
    }
}

