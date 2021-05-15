#[macro_use] extern crate conrod;
extern crate find_folder;
extern crate image;

mod event_loop;
mod game_window;
mod sources;
mod sprite;
pub mod game;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

pub fn say_hello() {
    println!("Hello from dusk engine!")
}
