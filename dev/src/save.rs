
/*
    ----------------------------------------------------------------------------------------------
    NovelText text editor and compiler
    save.rs
    Nicholas Soucier

    code used to control the save function of the editor, including the save window
    ----------------------------------------------------------------------------------------------
*/

use piston::input::*;

pub struct SaveWindow {
    pub filename: String,
    border: [f32; 4],
    highlight: [f32; 4],
    text: [f32; 4],
    windowwidth: f64,
}

use graphics::*;
impl SaveWindow {
    pub fn new(file: String) -> SaveWindow{
        SaveWindow {
            filename: String::from(file),
            border: [0.333, 0.361, 0.412, 1.0],
            highlight: [0.4, 0.435, 0.502, 0.5],
            text: [0.914, 0.918, 0.929, 1.0],
            windowwidth: 0.0,
        }
        
    }

    pub fn draw_window<G: Graphics>(&self, c: &Context, g: &mut G){
        //Draw rectangle for save name input
        Rectangle::new(self.border).draw([10.0, 64.0, self.windowwidth, 40.0], &c.draw_state, c.transform, g);
        Rectangle::new(self.highlight).draw([10.0, 64.0, self.windowwidth, 40.0], &c.draw_state, c.transform, g);
        Rectangle::new(self.border).draw([12.0, 66.0, self.windowwidth-4.0, 36.0], &c.draw_state, c.transform, g);
    }

    //Draw the text as glyphs for save window
    pub fn draw_glyphs<C: CharacterCache, G: Graphics<Texture = <C as CharacterCache>::Texture>>
    (&mut self, c: &Context, g: &mut G, glyphs: &mut C){
        text::Text::new_color(self.text, 25).draw(&self.filename, glyphs, &c.draw_state, c.transform.trans(15.0, 90.0), g).unwrap_or_default();
        self.windowwidth = glyphs.width(25, &self.filename).unwrap_or_default() + 15.0;
    }

    //Handle keyboard input
    pub fn handle_input(&mut self, key: &Key, shift: &bool){
        if *key == Key::Backspace {
            if self.filename.len() > 0 {
                self.filename = self.filename[0..self.filename.len()-1].to_string();
            }
        } else {
            if *shift {
                let keychar = getupperchar(&key.code());
                if keychar != 0 as char {
                    self.filename.push(keychar);
                }
            } else {
                let keychar = getlowerchar(&key.code());
                if keychar != 0 as char {
                    self.filename.push(keychar);
                }
            }
        }
    }
}

fn getlowerchar(key: &i32) -> char{
    match key {
        97 => return 'a', 98 => return 'b', 99 => return 'c', 100 => return 'd', 101 => return 'e', 102 => return 'f', 103 => return 'g',
        104 => return 'h', 105 => return 'i', 106 => return 'j', 107 => return 'k', 108 => return 'l', 109 => return 'm',
        110 => return 'n', 111 => return 'o', 112 => return 'p', 113 => return 'q', 114 => return 'r', 115 => return 's',
        116 => return 't', 117 => return 'u', 118 => return 'v', 119 => return 'w', 120 => return 'x', 121 => return 'y',
        122 => return 'z', 48 => return '0', 49 => return '1', 50 => return '2', 51 => return '3', 52 => return '4', 53 => return '5',
        54 => return '6', 55 => return '7', 56 => return '8', 57 => return '9', 45 => return '-',  46 => return '.',
        32 => return ' ', 
        _ => return 0 as char,
    }
}

fn getupperchar(key: &i32) -> char{
    match key {
        97 => return 'A', 98 => return 'B', 99 => return 'C', 100 => return 'D', 101 => return 'E', 102 => return 'F', 103 => return 'G',
        104 => return 'H', 105 => return 'I', 106 => return 'J', 107 => return 'K', 108 => return 'L', 109 => return 'M',
        110 => return 'N', 111 => return 'O', 112 => return 'P', 113 => return 'Q', 114 => return 'R', 115 => return 'S',
        116 => return 'T', 117 => return 'U', 118 => return 'V', 119 => return 'W', 120 => return 'X', 121 => return 'Y',
        122 => return 'Z', 45 => return '_', 32 => return ' ', 
        _ => return 0 as char,
    }
}