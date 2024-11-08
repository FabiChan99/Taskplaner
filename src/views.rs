use crate::main_menu_select;
use crate::model::User;
use cursive::view::{Nameable, Resizable};
use cursive::views::{Dialog, EditView, LinearLayout, TextView};
use cursive::Cursive;
use std::process::exit;

pub enum MenuOption {
    AddTask,
    ListTasks,
    Logout,
    Quit,
}

pub fn login_dialog(siv: &mut Cursive) {
    siv.add_layer(
        Dialog::new()
            .title("Login")
            .content(
                LinearLayout::vertical()
                    .child(TextView::new("Username:"))
                    .child(EditView::new().with_name("username").fixed_width(20))
                    .child(TextView::new("Passwort:"))
                    .child(
                        EditView::new()
                            .secret()
                            .with_name("password")
                            .fixed_width(20),
                    ),
            )
            .button("Login", |s| {
                let username = s
                    .call_on_name("username", |view: &mut EditView| view.get_content())
                    .unwrap();
                let password = s
                    .call_on_name("password", |view: &mut EditView| view.get_content())
                    .unwrap();

                match User::login(&username, &password) {
                    Ok(true) => {
                        s.pop_layer();
                        let sel = main_menu_select();
                        s.add_layer(Dialog::new().content(sel).title("Hauptmenü"));
                    }
                    Ok(false) => {
                        s.add_layer(
                            Dialog::new()
                                .content(TextView::new("Falscher Benutzername oder Passwort"))
                                .button("OK", |s| {
                                    s.pop_layer();
                                }),
                        );
                    }
                    Err(err) => {
                        s.add_layer(Dialog::info(format!("Fehler beim Login: {}", err)));
                    }
                }
            })
            .button("Abbrechen", |s| {
                s.quit();
            }),
    );
}

pub fn open_subwindow(s: &mut Cursive, operator: &MenuOption) {
    match operator {
        MenuOption::AddTask => {
            s.pop_layer();
            // add_task_dialog(s);
        }
        MenuOption::ListTasks => {
            s.pop_layer();
            // list_tasks(s);
        }
        MenuOption::Quit => {
            s.quit();
        }
        MenuOption::Logout => {
            s.pop_layer();
            login_dialog(s); // Rückkehr zum Login-Dialog beim Abmelden
        }
    }
}
