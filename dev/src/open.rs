
/*
    ----------------------------------------------------------------------------------------------
    NovelText text editor and compiler
    open.rs
    Nicholas Soucier

    open file, used to control the open file functions and the window
    ----------------------------------------------------------------------------------------------
*/

use graphics::*;
use piston::input::*;
use std::fs;

pub struct OpenWindow {
    border: [f32; 4],
    highlight: [f32; 4],
    text: [f32; 4],
    windowwidth: f64,
    windowheight: f64,
    saves: std::path::PathBuf,
    iteritem: u32,
    amount_items: u32,
}

impl OpenWindow {
    pub fn new(saves_path: std::path::PathBuf) -> OpenWindow {
        OpenWindow {
            border: [0.333, 0.361, 0.412, 1.0],
            highlight: [0.4, 0.435, 0.502, 0.5],
            text: [0.914, 0.918, 0.929, 1.0],
            windowwidth: 0.0,
            windowheight: 0.0,
            saves: saves_path,
            iteritem: 0,
            amount_items: 0,
        }
    }

    //Draw the open file window
    pub fn draw_window<G: Graphics>(&self, c: &Context, g: &mut G){
        //Draw rectangle for open file window
        Rectangle::new(self.border).draw([74.0, 64.0, self.windowwidth, self.windowheight], &c.draw_state, c.transform, g);
        Rectangle::new(self.highlight).draw([74.0, 64.0, self.windowwidth, self.windowheight], &c.draw_state, c.transform, g);
        Rectangle::new(self.border).draw([76.0, 66.0, self.windowwidth-4.0, self.windowheight-4.0], &c.draw_state, c.transform, g);
        //Draw the highlight above the selected item
        Rectangle::new(self.highlight).draw([76.0, (66.0+(self.iteritem as f64 * 25.0)), self.windowwidth-4.0, 30.0],
            &c.draw_state, c.transform, g);
    }

    //Draw the text as glyphs
    pub fn draw_glyphs<C: CharacterCache, G: Graphics<Texture = <C as CharacterCache>::Texture>>
    (&mut self, c: &Context, g: &mut G, glyphs: &mut C){
        let mut items = 0;
        let g_transform_x = 81.0;
        let mut g_transform_y = 90.0;
        let readdir_count = self.saves.read_dir().unwrap();
        let readdir = self.saves.read_dir().unwrap();
        if readdir_count.count() > 0 {
            for file in readdir.into_iter() {
                let filestring: String = String::from(file.unwrap().path().file_name().unwrap().to_str().unwrap());
                text::Text::new_color(self.text, 25).draw(&filestring, glyphs, &c.draw_state, c.transform.trans(g_transform_x, g_transform_y), g).unwrap_or_default();
                g_transform_y += 25.0;
                items += 1;
                let strwidth = glyphs.width(25, &filestring).unwrap_or_default();
                if strwidth + 15.0 > self.windowwidth + 15.0 {
                    self.windowwidth = strwidth + 15.0;
                }
            }
            self.amount_items = items;
            self.windowheight = 10.0 + (items as f64 * 25.0);
        } else {
            self.windowwidth = glyphs.width(25, "/saves/ directory empty").unwrap_or_default() + 15.0;
            self.windowheight = 40.0;
            text::Text::new_color(self.text, 25).draw("/saves/ directory empty", glyphs, &c.draw_state, c.transform.trans(g_transform_x, g_transform_y), g).unwrap_or_default();
        }
    }

    //Handle input for open window
    pub fn handle_input(&mut self, key: &Key){
        if *key == Key::Up && self.iteritem > 0 {
            self.iteritem -= 1;
        }
        if *key == Key::Down && self.iteritem < self.amount_items-1 {
            self.iteritem += 1;
        }
        if *key == Key::Delete {
            self.delete_file();
        }
    }

    //Delete a file from the directory if the delete key is pressed
    pub fn delete_file(&mut self) {
        let mut readdir = self.saves.read_dir().unwrap();
        let file: String = String::from(readdir.nth(self.iteritem as usize).unwrap().unwrap().path().to_str().unwrap());
        std::fs::remove_file(file).expect("Unable to delete file!");
        if self.iteritem > readdir.count() as u32 {
            self.iteritem -= 1;
        }
    }

    //Get the text from within a file and return it as a buffer
    pub fn get_file_buffer(&mut self) -> String {
        let mut readdir = self.saves.read_dir().unwrap();
        let readdir_count = self.saves.read_dir().unwrap();
        let mut content: String = String::new();
        if readdir_count.count() > 0 {
            let file: String = String::from(readdir.nth(self.iteritem as usize).unwrap().unwrap().path().to_str().unwrap());
            println!("{}", file);
            content = fs::read_to_string(file).expect("Unable to read from file");
        }
        return content;
    }

    //Get the name of a file
    pub fn get_filename(&mut self) -> String {
        let mut filename = String::new();
        let mut readdir = self.saves.read_dir().unwrap();
        let readdir_count = self.saves.read_dir().unwrap();
        if readdir_count.count() > 0 {
            filename = readdir.nth(self.iteritem as usize).unwrap().unwrap().file_name().into_string().unwrap();
        }
        return filename;
    }
}