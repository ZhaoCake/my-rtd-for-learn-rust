pub struct Item {
    pub(crate) id: u32,
    name: String,
    pub(crate) completed: bool,
    pub(crate) deleted: bool,
    created_at: String,
    pub(crate) completed_at: Option<String>,
    pub(crate) deleted_at: Option<String>,
}

impl Item {
    pub fn new(id: u32, name: String, created_at: String) -> Self {
        Item {
            id,
            name,
            completed: false,
            deleted: false,
            created_at,
            completed_at: None,
            deleted_at: None,
        }
    }

    pub fn to_prettier_string(&self) -> String {
        format!(
            "ID: {}, Name: {}, Completed: {}, Deleted: {}",
            self.id, self.name, self.completed, self.deleted
        )
    }

    pub fn to_string(&self) -> String {
        format!(
            "{},{},{},{},{:?}, {:?},{:?}",
            self.id,
            self.name,
            self.completed,
            self.deleted,
            self.created_at,
            self.completed_at,
            self.deleted_at
        )
    }

    pub fn from_string(data: &str) -> Option<Self> {
        println!("[from_string] data: {:?}", data);
        let fields: Vec<&str> = data.split(',').collect();
        if fields.len() != 7 {
            return None;
        }
        Some(Item {
            id: fields[0].parse().ok()?,
            name: fields[1].to_string(),
            completed: fields[2] == "true",
            deleted: fields[3] == "true",
            created_at: fields[4].to_string(),
            completed_at: if fields[5] == "None" {
                None
            } else {
                Some(fields[5].to_string())
            },
            deleted_at: if fields[6] == "None" {
                None
            } else {
                Some(fields[6].to_string())
            },
        })
    }
}
