use ::phi::{Phi, View, ViewAction};
use ::phi::data::Rectangle;
use ::phi::gfx::{CopySprite, Sprite};
use ::sdl2::pixels::Color;
use ::views::shared::Background;


pub struct MainMenuView {
    actions: Vec<Action>,
    selected: i8,

    bg_back: Background,
    bg_middle: Background,
    bg_front: Background,
}
// TODO: make background sync position with when view changes

impl MainMenuView {
    pub fn new(phi: &mut Phi) -> MainMenuView {
        MainMenuView {
            actions: vec![
                Action::new(phi, "New Game", Box::new(|phi| {
                    ViewAction::ChangeView(Box::new(::views::game::ShipView::new(phi)))
                })),
                Action::new(phi, "Quit", Box::new(|_| {
                    ViewAction::Quit
                })),
            ],
            selected: 0,

            bg_back: Background {
                pos: 0.0,
                vel: 20.0,
                sprite: Sprite::load(&mut phi.renderer, "assets/starBG.png").unwrap(),
            },
            bg_middle: Background {
                pos: 0.0,
                vel: 40.0,
                sprite: Sprite::load(&mut phi.renderer, "assets/starMG.png").unwrap(),
            }, 
            bg_front: Background {
                pos: 0.0,
                vel: 80.0,
                sprite: Sprite::load(&mut phi.renderer, "assets/starFG.png").unwrap(),
            },
        }
    }
}

impl View for MainMenuView {
    fn render(&mut self, phi: &mut Phi, elapsed: f64) -> ViewAction {
        if phi.events.now.quit || phi.events.now.key_escape == Some(true) {
            return ViewAction::Quit;
        }

        // Spacebar executes selected option
        // TODO: make Return also execute option
        if phi.events.now.key_space == Some(true) {
            // "(phi)" at end prevents attempted invocation of a `func` method
            return (self.actions[self.selected as usize].func)(phi);
        }

        // Up and Down keys change selection
        if phi.events.now.key_up == Some(true) {
            self.selected -= 1;
            // if bottom reached, wrap to top.
            if self.selected < 0 {
                self.selected = self.actions.len() as i8 - 1;
            }
        }

        if phi.events.now.key_down == Some(true) {
            self.selected += 1;
            // if bottom reached, wrap to top.
            if self.selected >= self.actions.len() as i8 {
                self.selected = 0;
            }
        }

        // Clear the screen
        phi.renderer.set_draw_color(Color::RGB(0, 0, 0));
        phi.renderer.clear();

        // Render backgrounds
        self.bg_back.render(&mut phi.renderer, elapsed);
        self.bg_middle.render(&mut phi.renderer, elapsed);
        self.bg_front.render(&mut phi.renderer, elapsed);

        let (win_w, win_h) = phi.output_size();
        let label_h = 50.0;
        let border_width = 3.0;
        let box_w = 360.0;
        let box_h = self.actions.len() as f64 * label_h;
        let margin_h = 10.0;

        // Render the border of menu box
        phi.renderer.set_draw_color(Color::RGB(180, 180, 255));
        phi.renderer.fill_rect(Rectangle {
            w: box_w + (border_width * 2.0),
            h: box_h + (border_width * 2.0),
            x: ((win_w - box_w) / 2.0) - border_width,
            y: ((win_h - box_h) / 2.0) - border_width,
            }.to_sdl().unwrap());

        // Render menu box
        phi.renderer.set_draw_color(Color::RGB(80, 80, 200));
        phi.renderer.fill_rect(Rectangle {
            w: box_w,
            h: box_h,
            x: ((win_w - box_w) / 2.0),
            y: ((win_h - box_h) / 2.0),
            }.to_sdl().unwrap());

        // Render labels in the menu
        let (win_w, win_h) = phi.output_size();

        for (i, action) in self.actions.iter().enumerate() {
            if self.selected as usize == i {
                let (w, h) = action.focus_sprite.size();
                phi.renderer.copy_sprite(&action.focus_sprite, Rectangle {
                    x: (win_w - w) / 2.0,
                    // Place each action under the previous one.
                    y: ((win_h - box_h) + (label_h - h)) / 2.0 + label_h * i as f64,
                    w: w,
                    h: h,
                });
            }
            else {
                let (w, h) = action.idle_sprite.size();
                phi.renderer.copy_sprite(&action.idle_sprite, Rectangle {
                    x: (win_w - w) / 2.0,
                    // Place each action under the previous one.
                    y: ((win_h - box_h) + (label_h - h)) / 2.0 + label_h * i as f64,
                    w: w,
                    h: h,
                });
            }
        }
        // TODO: make label size animate when (de)selected


        ViewAction::None
    }
}


struct Action {
    /// The function which should be executed if the action is chosen
    // Stored in a box because `Fn` is a trait that can only interact
    // with unsized data through a pointer.
    func: Box<Fn(&mut Phi) -> ViewAction>,

    /// The sprite that is rendered when the label is not focussed on
    idle_sprite: Sprite,

    /// The sprite that is rendered when the label is focussed on
    focus_sprite: Sprite,
}

impl Action {
    fn new(phi: &mut Phi, label: &'static str,
           func: Box<Fn(&mut Phi) -> ViewAction>) -> Action {
        Action {
            func: func,
            idle_sprite: phi.ttf_str_sprite(label, "assets/belligerent.ttf",
                                            32, Color::RGB(220, 220, 220))
                .unwrap(),
            focus_sprite: phi.ttf_str_sprite(label, "assets/belligerent.ttf",
                                            38, Color::RGB(255, 255, 255))
                .unwrap(),
        }
    }
}
