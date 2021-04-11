use std::fs;
use std::collections::HashMap;

fn main() {
    let mut arguments = std::env::args().skip(1);
    let key = arguments.next().unwrap();
    let value = arguments.next().unwrap();
    println!("Key '{}', '{}'", key, value);

    // let contents = format!("{}\t{}\n", key, value);
    // let write_result = fs::write("kv.db", contents).unwrap();

    let mut database = Database::new().expect("Database::new() crashed");
    database.insert(value.clone(), key.to_uppercase());
    database.insert(key, value);
    database.flush().unwrap();
}

struct Database {
    map: HashMap<String, String>,
    flush: bool
}

impl Database {
    fn new() -> Result<Database, std::io::Error> {
        let mut map = HashMap::new();
        // let contents = match fs::read_to_string("kv.db") {
        //     Ok(c) => c,
        //     Err(error) => {
        //         Err(error);
        //     }
        // };
        
        let contents = fs::read_to_string("kv.db")?;

        for line in contents.lines() {
            let mut chunks = line.splitn(2, '\t');
            let key = chunks.next().expect("No key");
            let value = chunks.next().expect("No value");
            map.insert(key.to_owned(), value.to_owned());
        }

        Ok(Database { map, flush: false })
    }

    fn insert(&mut self, key: String, value: String) {
        self.map.insert(key, value);
    }

    fn flush(self) -> std::io::Result<()> {
        do_flush(&self)
    }
}

impl Drop for Database {
    fn drop(&mut self) {
        if !self.flush {
            let _ = do_flush(self);
        }
    }
}

fn do_flush (database: &Database) -> std::io::Result<()> {
    let mut contents = String::new();
    for (key, value) in &database.map {
        // let kvpair = format!("{}\t{}\n", key, value);
        contents.push_str(&key);
        contents.push('\t');
        contents.push_str(&&&&value);
        contents.push('\n');
    }
    fs::write("kv.db", contents)
}
