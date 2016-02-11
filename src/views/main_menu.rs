use ::phi::{Phi, View, ViewAction};
use ::phi::data::Rectangle;
use ::phi::gfx::{CopySprite, Sprite};
use ::sdl2::pixels::Color;
use ::views::shared::BgSet;


pub struct MainMenuView {
    actions: Vec<Action>,
    selected: i8,

    backgrounds: BgSet,
}
// TODO: make background sync position with when view changes

impl MainMenuView {
    pub fn new(phi: &mut Phi) -> MainMenuView {
        let backgrounds = BgSet::new(&mut phi.renderer);
        MainMenuView::with_backgrounds(phi, backgrounds)
    }
         
    pub fn with_backgrounds(phi: &mut Phi, backgrounds: BgSet) -> MainMenuView {
        MainMenuView {
            actions: vec![
                Action::new(phi, "New Game", Box::new(move |phi, backgrounds| {
                    ViewAction::ChangeView(Box::new(::views::game::ShipView::with_backgrounds(phi, backgrounds)))
                })),
                Action::new(phi, "Quit", Box::new(|_,_| {
                    ViewAction::Quit
                })),
            ],
            selected: 0,
            backgrounds: backgrounds,
        }
    }
}

impl View for MainMenuView {
    fn render(&mut self, phi: &mut Phi, elapsed: f64) -> ViewAction {
        if phi.events.now.quit || phi.events.now.key_escape == Some(true) {
            return ViewAction::Quit;
        }

        // Spacebar or Return key executes selected option
        if phi.events.now.key_space == Some(true) ||
            phi.events.now.key_return == Some(true) {
            // "(phi)" at end prevents attempted invocation of a `func` method
            return (self.actions[self.selected as usize].func)(phi, self.backgrounds.clone());
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
        self.backgrounds.back.render(&mut phi.renderer, elapsed);
        self.backgrounds.middle.render(&mut phi.renderer, elapsed);
        self.backgrounds.front.render(&mut phi.renderer, elapsed);

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
            h: box_h + (border_width * 2.0), // + (margin_h * 2)
            x: ((win_w - box_w) / 2.0) - border_width,
            y: ((win_h - box_h) / 2.0) - border_width, // - margin_h
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

        for (i, action) in self.actions.iter_mut().enumerate() {
            if self.selected as usize == i {
                action.focus(elapsed);
            }
            else {
                action.defocus(elapsed);
            }
            action.sprite = phi.ttf_str_sprite(action.label,
                "assets/belligerent.ttf", action.size as i32, action.color).unwrap();

            let (w, h) = action.sprite.size();
            phi.renderer.copy_sprite(&action.sprite, Rectangle {
                x: (win_w - w) / 2.0,
                // Place each action under the previous one.
                y: ((win_h - box_h) + (label_h - h)) / 2.0 + label_h * i as f64,
                w: w,
                h: h,
            });
        }


        ViewAction::None
    }
}

const ACTION_IDLE_SIZE: i32 = 32;
const ACTION_FOCUS_SIZE: i32 = 38;
const ACTION_IDLE_COLOR: Color = Color::RGB(200, 200, 200);
const ACTION_FOCUS_COLOR: Color = Color::RGB(255, 255, 255);

struct Action {
    /// The function which should be executed if the action is chosen
    // Stored in a box because `Fn` is a trait that can only interact
    // with unsized data through a pointer.
    func: Box<Fn(&mut Phi, BgSet) -> ViewAction>,

    label: &'static str,

    sprite: Sprite,

    size: f64,

    color: Color,
}

impl Action {
    fn new(phi: &mut Phi, label: &'static str,
        func: Box<Fn(&mut Phi, BgSet) -> ViewAction>) -> Action {
        Action {
            func: func,
            label: label,
            sprite: phi.ttf_str_sprite(label, "assets/belligerent.ttf",
                                        ACTION_FOCUS_SIZE, ACTION_FOCUS_COLOR)
                    .unwrap(),

            size: ACTION_IDLE_SIZE as f64,

            color: ACTION_IDLE_COLOR,
        }
    }

    fn focus(&mut self, elapsed: f64) {
        let speed = 40.0 * elapsed;
        self.color = ACTION_FOCUS_COLOR;
        self.size = linear_transition(self.size, ACTION_FOCUS_SIZE as f64, speed)
    }
    fn defocus(&mut self, elapsed: f64) {
        let speed = 40.0 * elapsed;
        self.color = ACTION_IDLE_COLOR;
        self.size = linear_transition(self.size, ACTION_IDLE_SIZE as f64, speed)
    }
}

fn linear_transition(current_state: f64, final_state: f64, speed: f64) -> f64 {
    let mut new_state = current_state;
    if new_state < final_state {
        new_state += speed;
        if new_state > final_state {
            new_state = final_state;
        }
    }
    else {
        new_state -= speed;
        if new_state < final_state {
            new_state = final_state;
        }
    }
    new_state
}
