extern crate gtk;
use gtk::prelude::*;

//Artemis files
pub mod artemis;

fn main() {
    if gtk::init().is_err(){
        println!("Failed to initialize GTK");
        return;
    }

    let window: gtk::Window = gtk::Window::new(gtk::WindowType::Toplevel);
    window.set_title("Artemis");
    window.set_default_size(1400, 800);

    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    let wallpaper_path: String = artemis::file_io::get_artemis_path();
    let mut fs = artemis::folderstate::Folderstate::new(wallpaper_path);

    artemis::gui_templates::draw_main_window(&window, &mut fs);

    gtk::main();
}
