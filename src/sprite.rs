extern crate sdl2;

use sdl2::image::{InitFlag, LoadTexture};
use sdl2::render::Texture;
use sdl2::render::TextureCreator;
use sdl2::video::WindowContext;

pub struct TextureHolder<'a> {
    pub texture: Texture<'a>,
    pub cell_width: u32,
    pub cell_height: u32,
}

impl<'a> TextureHolder<'a> {
    fn load(
        filename: &str,
        cell_width: u32,
        cell_height: u32,
        creator: &'a TextureCreator<WindowContext>,
    ) -> TextureHolder<'a> {
        let texture = creator.load_texture(filename).unwrap_or_else(|filename| {
            panic!("this should not fail with loading a texture, yikes!")
        });
        TextureHolder {
            texture,
            cell_width,
            cell_height,
        }
    }
}

pub struct Sprite<'a> {
    holder: &'a TextureHolder<'a>,
    nr: u32,
}

impl<'a> Sprite<'a> {
    pub fn new(holder: &'a TextureHolder, nr: u32) -> Self {
        Sprite { holder, nr }
    }
    pub fn render(&self, canvas: &mut WindowContext) {}
}
