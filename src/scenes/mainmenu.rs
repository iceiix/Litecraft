use client::Client;
use scenes::scene::Scene;
use scenes::gui::{SceneManager, Image, Component, Button, ButtonSize, ContainerPosition};

use allegro::Color;

pub struct MainMenu<'a> {
    scenemanager: SceneManager<'a>,
}

impl<'a> Component for MainMenu<'a> {}

impl<'a> Scene for MainMenu<'a> {
    fn draw(&self, client: &mut Client) -> Option<Box<Scene>> {
        client.get_core().clear_to_color(
            Color::from_rgb_f(1.0, 1.0, 1.0),
        );

        let w = client.get_display().get_width() as f32;
        let h = client.get_display().get_height() as f32;

        match client.get_timer().get_count() / 1000 % 9 {
            0 => self.draw_2d(client, 0.0, 0.0, w, h, "menu_1"),
            1 => self.draw_2d(client, 0.0, 0.0, w, h, "menu_2"),
            2 => self.draw_2d(client, 0.0, 0.0, w, h, "menu_3"),
            4 => self.draw_2d(client, 0.0, 0.0, w, h, "menu_4"),
            5 => self.draw_2d(client, 0.0, 0.0, w, h, "menu_5"),
            6 => self.draw_2d(client, 0.0, 0.0, w, h, "menu_6"),
            7 => self.draw_2d(client, 0.0, 0.0, w, h, "menu_7"),
            8 => self.draw_2d(client, 0.0, 0.0, w, h, "menu_8"),
            _ => self.draw_2d(client, 0.0, 0.0, w, h, "menu_9"),
        }

        self.scenemanager.render(client);
        None
    }
}

impl<'a> MainMenu<'a> {
    pub fn new() -> Self {
        let mut sm = SceneManager::new();

        sm.add_image(Image::new("title", 0.0, 50.0, 400.0, 71.342,
                ContainerPosition::UpCenter));

        sm.add_button(Button::new(0.0, -90.0, 250.0,
                ContainerPosition::MiddleCenter, "Singleplayer", ButtonSize::Normal));
        sm.add_button(Button::new(0.0, 0.0, 250.0,
                ContainerPosition::MiddleCenter, "Multiplayer", ButtonSize::Normal));
        sm.add_button(Button::new(0.0, 90.0, 250.0,
                ContainerPosition::MiddleCenter, "Minecraft Realms", ButtonSize::Normal));

        sm.add_button(Button::new(0.0, 180.0, 250.0,
                ContainerPosition::MiddleCenter, "Options", ButtonSize::Small));

        sm.add_button(Button::new(400.0, 180.0, 250.0,
                ContainerPosition::MiddleCenter, "Quit Game", ButtonSize::Small));

        MainMenu { scenemanager: sm }
    }
}