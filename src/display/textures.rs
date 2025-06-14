use std::{cell::RefCell, collections::HashMap};
use sdl2::{pixels::Color, render::{Texture}};
use std::rc::Rc;

// const TEXT_SIZE: (u8,u8) = (64,64);

pub enum TextureType<'a> {
    Color(Color),
    Texture(Rc<RefCell<Texture<'a>>>),
}


impl<'a> Clone for TextureType<'a> {
    fn clone(&self) -> Self {
        match self {
            TextureType::Color(c) => TextureType::Color(*c),
            TextureType::Texture(t) => TextureType::Texture(t.clone()),
        }
    }
}



#[derive(Clone)]
pub struct TextureMap<'a> {
    map : HashMap<u8,TextureType<'a>>,
}

impl<'a> TextureMap<'a> {
    pub fn new() -> Self {
        Self { map: HashMap::new() }
    }

    pub fn add_texture(&mut self, code: u8, texture: Rc<RefCell<Texture<'a>>>) -> Result<(),String> {
        match self.map.get(&code)  {
        Some(v) => {
            match v {
                TextureType::Color(c) => return Err(format!("the code {} is already used for the color {:?}",code,c)),
                TextureType::Texture(_) => return Err(format!("the code {} is already used for a texute",code)),
            };
        },
        None => {},
        };
        self.map.insert(code, TextureType::Texture(texture));
        Ok(())
    }

    pub fn add_color(&mut self,code: u8, color: Color) -> Result<(),String>{
        match self.map.get(&code)  {
        Some(v) => {
            match v {
                TextureType::Color(c) => return Err(format!("the code {} is already used for the color {:?}",code,c)),
                TextureType::Texture(_) => return Err(format!("the code {} is already used for a texture",code)),
            };
        },
        None => {},
        };
        self.map.insert(code, TextureType::Color(color));
        Ok(())
    }

    pub fn get(&self,code: u8) -> Option<&TextureType<'a>> {
        self.map.get(&code)
    }
}