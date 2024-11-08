use crate::db::init_db;
use crate::views::{login_dialog, open_subwindow, MenuOption};
use cursive::view::{Nameable, Resizable};
use cursive::views::{NamedView, ResizedView, SelectView};
use cursive::{Cursive, CursiveExt};

mod db;
mod model;
mod utils;
mod views;

fn main() {
    init_db().expect("Failed to initialize database");
    let mut siv = Cursive::default();
    login_dialog(&mut siv);
    siv.run();
}

fn main_menu_select() -> ResizedView<NamedView<SelectView<MenuOption>>> {
    let mut select = SelectView::new();
    select.add_item("Aufgabe hinzufügen", MenuOption::AddTask);
    select.add_item("Aufgaben anzeigen", MenuOption::ListTasks);
    select.add_item("Abmelden", MenuOption::Logout);
    select.add_item("Beenden", MenuOption::Quit);
    select.set_on_submit(open_subwindow);
    select.with_name("operator").fixed_width(30)
}
