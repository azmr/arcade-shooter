extern crate rand;
extern crate sdl2;
extern crate sdl2_image;
extern crate sdl2_ttf;

mod phi;
mod views;

// use ::phi::{Events, Phi, View, ViewAction};

fn main() {
    ::phi::spawn("Arcade Shooter", |phi| {
        Box::new(::views::main_menu::MainMenuView::new(phi))
    });
}
