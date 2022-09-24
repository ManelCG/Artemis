extern crate gtk;
extern crate gdk_pixbuf;
use gtk::prelude::*;

// use crate::artemis::file_io;
use crate::artemis::folderstate::Folderstate;
use crate::artemis::*;

pub fn draw_main_window(window: &gtk::Window, fs: &mut Folderstate){
    #[cfg(debug_assertions)]
    {   println!("Running draw_main_window...");
        println!("FOLDERSTATE: '{}' WITH FILTER '{}'", fs.get_path(), fs.get_filter());}


    //Boxes
    let main_vbox: gtk::Box = gtk::Box::new(gtk::Orientation::Vertical, 0);
    let main_hbox: gtk::Box = gtk::Box::new(gtk::Orientation::Horizontal, 0);

    //Widgets
    let menu_bar: gtk::MenuBar = get_main_menubar();
    let mut scrollbox_files: gtk::ScrolledWindow = gtk::ScrolledWindow::new(gtk::Adjustment::NONE, gtk::Adjustment::NONE);
    let mut scrollbox_folders: gtk::ScrolledWindow = gtk::ScrolledWindow::new(gtk::Adjustment::NONE, gtk::Adjustment::NONE);
    scrollbox_folders.set_border_width(10);
    scrollbox_files.set_border_width(15);
    fill_folders_container(&mut scrollbox_folders, fs);
    fill_files_container(&mut scrollbox_files, fs);
    scrollbox_files.set_policy(gtk::PolicyType::Never, gtk::PolicyType::Automatic);

    main_hbox.pack_start(&scrollbox_folders, false, false, 0);
    let separator = gtk::Separator::new(gtk::Orientation::Vertical);
    main_hbox.pack_start(&separator, false, false, 0);
    main_hbox.pack_start(&scrollbox_files, true, true, 0);

    main_vbox.pack_start(&menu_bar, false, false, 0);
    main_vbox.pack_start(&main_hbox, true, true, 0);

    window.add(&main_vbox);
    window.show_all();
}

fn get_button_with_image<'a>(image_name: &'a String, artemis_path_length: usize) -> Result<gtk::Box, &'a str>{
    let pb = gdk_pixbuf::Pixbuf::from_file_at_scale(&image_name, 250, 250, true);
    if pb.is_ok() {
        let button = gtk::Button::new();
        let vbox = gtk::Box::new(gtk::Orientation::Vertical, 0);
        let mut label_name = &image_name[artemis_path_length..];
        if label_name.chars().nth(0) == Some('/'){
            label_name = &label_name[1..];
        }
        let label = gtk::Label::new(Some(&label_name));

        let pb = pb.unwrap();
        // pb.scale_simple(100, 100, gtk::gdk::);
        let image = gtk::Image::from_pixbuf(Some(&pb));
        button.set_image(Some(&image));

        vbox.pack_start(&button, true, true, 0);
        vbox.pack_end(&label, false, false, 0);

        return Ok(vbox);
    }
    return Err("Error");
}

fn fill_files_container(scrollbox: &mut gtk::ScrolledWindow, fs: &mut Folderstate) {
    let files: Vec<String> = fs.get_files();

    let vbox = gtk::Box::new(gtk::Orientation::Vertical, 5);

    let npics = files.len();
    let cols = 5;

    let rows = npics / cols;
    let extra = npics % cols;

    let path = file_io::get_artemis_path();

    for j in 0..rows {
        let hbox = gtk::Box::new(gtk::Orientation::Horizontal, 5);
        for i in (j*cols)..((j+1)*cols) {
            let button = get_button_with_image(&files[i], path.len());
            if button.is_ok(){
                let button = button.unwrap();
                hbox.pack_start(&button, true, false, 0);
            }
        }
        vbox.pack_start(&hbox, false, false, 0);
    }

    let hbox = gtk::Box::new(gtk::Orientation::Horizontal, 5);
    for i in 0..extra {
        let button = get_button_with_image(&files[i], path.len());
        if button.is_ok(){
            let button = button.unwrap();
            hbox.pack_start(&button, true, false, 0);
        }
    }
    vbox.pack_start(&hbox, false, false, 0);
    scrollbox.add(&vbox);
}

fn fill_folders_container<'a>(scrollbox: &mut gtk::ScrolledWindow, fs: &'a mut Folderstate) {
    scrollbox.set_policy(gtk::PolicyType::Never, gtk::PolicyType::Automatic);

    let scrollbox_vbox: gtk::Box = gtk::Box::new(gtk::Orientation::Vertical, 5);

    let folders: Vec<String> = fs.get_folders();

    {
        let folder_button: gtk::Button = gtk::Button::with_label("..");
        folder_button.set_size_request(200, 30);

        folder_button.connect_clicked(
        |button| {
            let newfolder: String = String::from(button.label().unwrap());
            std::env::set_current_dir(&newfolder).expect("Directory does not exist");
        });

        scrollbox_vbox.pack_start(&folder_button, false, false, 0);
    }

    for i in 0..folders.len() {
        let folder_button: gtk::Button = gtk::Button::with_label(&folders[i]);
        folder_button.set_size_request(200, 30);

        folder_button.connect_clicked(
        |button| {
            let newfolder: String = String::from(button.label().unwrap());
            std::env::set_current_dir(&newfolder).expect("Directory does not exist");

            let window = gtk::Window::new(gtk::WindowType::Toplevel);
            // draw_main_window(&window, &mut fs);
            // clean_window(&mut window);
        });

        scrollbox_vbox.pack_start(&folder_button, false, false, 0);
    }

    scrollbox.add(&scrollbox_vbox);
}

// fn empty_scrollbox<CT: IsA<&mut gtk::Container>> (container: &mut CT){
fn empty_scrollbox(container: &mut gtk::ScrolledWindow){
    unsafe {
        container.forall(|widget| {gtk::Widget::destroy(widget)});
    }
}

fn clean_window(window: &mut gtk::Window){
    unsafe {
        window.forall(|widget| {gtk::Widget::destroy(widget)});
    }
}

fn get_main_menubar() -> gtk::MenuBar {
    let menu_bar: gtk::MenuBar = gtk::MenuBar::new();

    //Menus
    let menu_filemenu: gtk::Menu = gtk::Menu::new();
    let menu_editmenu: gtk::Menu = gtk::Menu::new();
    let menu_helpmenu: gtk::Menu = gtk::Menu::new();

    //Submenus
    //File menu
    let menu_file_mi: gtk::MenuItem = gtk::MenuItem::with_label("File");
    let menu_file_quit: gtk::MenuItem = gtk::MenuItem::with_label("Quit");

    menu_file_mi.set_submenu(Some(&menu_filemenu));
    menu_filemenu.append(&menu_file_quit);

    //Edit menu
    let menu_edit_mi: gtk::MenuItem = gtk::MenuItem::with_label("Edit");

    menu_edit_mi.set_submenu(Some(&menu_editmenu));

    //Help menu
    let menu_help_mi: gtk::MenuItem = gtk::MenuItem::with_label("Help");

    menu_help_mi.set_submenu(Some(&menu_helpmenu));


    //Append to menu shell
    menu_bar.append(&menu_file_mi);
    menu_bar.append(&menu_edit_mi);
    menu_bar.append(&menu_help_mi);

    menu_bar
}
