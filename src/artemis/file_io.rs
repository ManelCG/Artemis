pub fn get_file_list(path: &str) -> (Vec<String>, Vec<String>) {
    #[cfg(debug_assertions)]
    println!("Getting files in '{}':", path);

    let mut files_vec: Vec<String> = Vec::new();
    let mut dirs_vec: Vec<String> = Vec::new();
    let paths = std::fs::read_dir(path).unwrap();

    for name in paths {
        let path: String = String::from(name.unwrap().path().into_os_string().into_string().unwrap());
        let md = std::fs::metadata(&path).unwrap();

        if md.is_file(){
            files_vec.push(path);
        } else if md.is_dir() {
            dirs_vec.push(path);
        }
    }

    #[cfg(debug_assertions)]
    {
        println!("##DIRS:");
        for i in 0..dirs_vec.len(){
            println!("  {}", dirs_vec[i]);
        }
        println!("##FILES:");
        for i in 0..files_vec.len(){
            println!("  {}", files_vec[i]);
        }
    }


    #[cfg(debug_assertions)]
    { println!("{} files.", files_vec.len());
      println!("{} dirs.", dirs_vec.len()); }

    dirs_vec.sort();
    files_vec.sort();

    (dirs_vec, files_vec)
}

pub fn get_artemis_path() -> String{
    let wallpaper_path = std::env::var("HORIZON_ARTEMIS_DIRECTORY");
    if wallpaper_path.is_ok(){
        wallpaper_path.unwrap()
    } else {
        String::from(".")
    }
}
