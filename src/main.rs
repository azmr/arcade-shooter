extern crate sdl2;
extern crate sdl2_image;
extern crate sdl2_ttf;

use ::views::shared::BgSet;

mod phi;
mod views;

// use ::phi::{Events, Phi, View, ViewAction};

fn main() {
    ::phi::spawn("Arcade Shooter", |phi| {
        let backgrounds = BgSet::new(&phi.renderer);
        Box::new(::views::main_menu::MainMenuView::new(phi,
            backgrounds))
    });
}
