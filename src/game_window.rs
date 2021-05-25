use crate::{event_loop::EventLoop};
use crate::game::GameWindowBuilder;
use crate::game_path::GamePath;
use conrod::backend::glium::glium::{self, Surface};
use conrod::{widget, Colorable, Positionable, Sizeable, Widget};

widget_ids!(struct Ids{
    text,
    test_image,
});

pub struct GameWindow {
    events_loop: glium::glutin::EventsLoop,
    display: glium::Display,
    ui: conrod::Ui,
    assets: GamePath,
    ids: Ids,
    imagine_map: conrod::image::Map<glium::texture::Texture2d>,
    renderer: conrod::backend::glium::Renderer,
    event_loop: EventLoop,
}

impl GameWindow {
    pub fn new(builder: GameWindowBuilder) -> Self {
        let width = builder.width;
        let height = builder.height;
        // stored
        // let mut events_loop = glium::glutin::EventsLoop::new();
        let events_loop = glium::glutin::EventsLoop::new();
        // temp
        let window = glium::glutin::WindowBuilder::new()
            .with_title(builder.title_s)
            .with_dimensions(builder.width, builder.height);
        // temp
        let context = glium::glutin::ContextBuilder::new()
            .with_vsync(true)
            .with_multisampling(4);
        // stored
        let display = glium::Display::new(window, context, &events_loop).unwrap();
        // stored
        let mut ui = conrod::UiBuilder::new([width as f64, height as f64]).build();
        // stored
        // let assets = find_folder::Search::KidsThenParents(3, 5)
        //      .for_folder("assets")
        //      .unwrap();
        let assets = GamePath::folder("assets");
        // let font_path = assets.join("fonts/segoepr.ttf");
        let font_path = assets.clone()
            .sub("fonts").file("segoepr.ttf");
        ui.fonts.insert_from_file(font_path).unwrap();

        // stored
        let ids = Ids::new(ui.widget_id_generator());
        // stored
        let imagine_map = conrod::image::Map::<glium::texture::Texture2d>::new();
        // stored
        // let mut renderer =
        let renderer = conrod::backend::glium::Renderer::new(&display).unwrap();
        // stored
        // let mut event_loop = EventLoop::new();
        let event_loop = EventLoop::new();

        GameWindow {
            events_loop,
            display,
            ui,
            assets,
            ids,
            imagine_map,
            renderer,
            event_loop,
        }
    }
    pub fn detect_exit_event(
        &mut self,
        is_game_running: &mut bool,
        w: u32,
        h: u32,
        img: conrod::image::Id,
    ) {
        for event in self.event_loop.next(&mut self.events_loop) {
            if let Some(event) = conrod::backend::winit::convert_event(event.clone(), &self.display)
            {
                self.ui.handle_event(event);
            }

            match event {
                glium::glutin::Event::WindowEvent { event, .. } => match event {
                    glium::glutin::WindowEvent::Closed
                    | glium::glutin::WindowEvent::KeyboardInput {
                        input:
                            glium::glutin::KeyboardInput {
                                virtual_keycode: Some(glium::glutin::VirtualKeyCode::Escape),
                                ..
                            },
                        ..
                    } => *is_game_running = false,
                    _ => (),
                },
                _ => (),
            }
        }

        // render "UI", display on screen
        if let Some(primitives) = self.ui.draw_if_changed() {
            self.renderer
                .fill(&self.display, primitives, &self.imagine_map);
            let mut target = self.display.draw();
            target.clear_color(0.0, 1.0, 0.0, 1.0);

            let ui = &mut self.ui.set_widgets();
            // Hello World in the middle of the window.
            widget::Text::new("Hello World!")
                .middle_of(ui.window)
                .color(conrod::color::WHITE)
                .font_size(32)
                .set(self.ids.text, ui);

            widget::Image::new(img)
                .w_h(w as f64, h as f64)
                .middle()
                .set(self.ids.test_image, ui);

            self.renderer
                .draw(&(self.display), &mut target, &(self.imagine_map))
                .unwrap();
            target.finish().unwrap();
        }
    }
    pub fn test_image_init(&mut self) -> (u32, u32, conrod::image::Id) {
        let test_image = self.load_image();
        let (w, h) = (test_image.get_width(), test_image.get_height().unwrap());
        let test_image = self.imagine_map.insert(test_image);
        (w, h, test_image)
    }
    fn load_image(&self) -> glium::texture::Texture2d {
        let img_path = self.assets.clone()
            .sub("images").file("image.jpg");
        let rgba_image = image::open(&std::path::Path::new(&img_path))
            .unwrap()
            .to_rgba();
        let image_dimensions = rgba_image.dimensions();
        let raw_image = glium::texture::RawImage2d::from_raw_rgba_reversed(
            &rgba_image.into_raw(),
            image_dimensions,
        );
        let texture = glium::texture::Texture2d::new(&self.display, raw_image).unwrap();
        texture
    }
}
