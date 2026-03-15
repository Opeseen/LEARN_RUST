fn main() {
    let mut folder = Folder::new(String::from("Documents"));
    folder.create_file(String::from("main.rs"));
    folder.create_file(String::from("lib.rs"));
    println!("{:#?}", folder);

    folder.delete_file(1);
    println!("{:#?}", folder);

    match folder.get_file(1) {
        Some(file) => println!("Retrieved file: {file:?}"),
        None => println!("There was no file"),
    }
}

#[derive(Debug)]
struct File {
    name: String,
}

#[derive(Debug)]
struct Folder {
    name: String,
    content: Vec<File>,
}

impl Folder {
    fn new(name: String) -> Folder {
        Folder {
            name,
            content: vec![],
        }
    }

    fn create_file(&mut self, name: String) {
        let file = File { name };
        self.content.push(file);
    }

    fn delete_file(&mut self, index: usize) -> File {
        self.content.remove(index)
    }

    fn get_file(&self, index: usize) -> Option<&File> {
        self.content.get(index)
    }
}
