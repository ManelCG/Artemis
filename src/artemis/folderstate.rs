use crate::artemis::file_io;

trait Fs {
    fn new(path: String) -> Self;
}

pub struct Folderstate {
    path: String,
    filter: String,

    folders: Vec<String>,
    files: Vec<String>,
}

impl Folderstate {
    pub fn new(path: String) -> Folderstate {
        let files_tup: (Vec<String>, Vec<String>) = file_io::get_file_list(path.as_str());
        return Folderstate{path,
                           filter: String::from(""),
                           files: files_tup.1,
                           folders: files_tup.0,};
    }

    pub fn set_filter(&mut self, filter: String) {
        self.filter = filter;
    }
    pub fn get_filter(&self) -> String {
        return String::from(self.filter.clone());
    }

    pub fn set_path(&mut self, path: String){
        self.path = path;
    }
    pub fn get_path(&self) -> String {
        return String::from(self.path.clone());
    }

    pub fn chdir(&mut self, path: String){

    }

    pub fn get_files(&self) -> Vec<String> {
        return self.files.clone();
    }
    pub fn get_folders(&self) -> Vec<String> {
        return self.folders.clone();
    }
}
