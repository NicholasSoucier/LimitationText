/*
    ----------------------------------------------------------------------------------------------
    LimitationTEXT text editor and compiler
    main.rs
    Nicholas Soucier

    Main file, to control the event loop, the windows, and basic funtions for the whole of the program
    ----------------------------------------------------------------------------------------------
*/

//#![windows_subsystem = "windows"]

extern crate piston;
extern crate piston_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate find_folder;

use piston::window::WindowSettings;
use piston_window::*;
use opengl_graphics::{OpenGL};
use std::io::*;
use std::path::*;
use std::fs::*;
use std::env;

//Colors as f32 arrays, color references for drawing.
pub struct Palette {
    background: [f32; 4],
    output_background: [f32; 4],
    border: [f32; 4],
    highlight: [f32; 4],
    disable_highlight: [f32; 4],
    text: [f32; 4],
}

//External Rust files and their objects
mod save;
use save::SaveWindow;
mod open;
use open::OpenWindow;
mod lang;
use lang::InterpreterObject;

//Struct for icons to draw to screen
pub struct Icons {
    saveicon: G2dTexture,
    openicon: G2dTexture,
    buildicon: G2dTexture,
    executeicon: G2dTexture,
    stepicon: G2dTexture,
    resetexecutionicon: G2dTexture,
    helpicon: G2dTexture,
}

fn main(){
    //Set up window and graphics API
    let opengl = OpenGL::V3_2;
    let window_settings = WindowSettings::new("LimitationTEXT - ", [720; 2])
        .graphics_api(opengl).exit_on_esc(false);
    let mut window: PistonWindow = window_settings.build().expect("Failed to build Glutin Window");

    //---------------------------------
    //   IDE Arguments and Variables
    //---------------------------------

    //Basic Data for IDE
    let palette = Palette {
        background: [0.263, 0.282, 0.325, 1.0],
        output_background: [0.233, 0.252, 0.295, 1.0],
        border: [0.333, 0.361, 0.412, 1.0],
        highlight: [0.4, 0.435, 0.502, 0.75],
        disable_highlight: [0.233, 0.252, 0.295, 0.5],
        text: [0.914, 0.918, 0.929, 1.0],
    };

    //Get the file path of the executable and create a directory if needed
    println!("{}", env::current_dir().unwrap().display());
    let directory = env::current_dir().unwrap().read_dir().unwrap();
    let mut saves: PathBuf = PathBuf::from("");
    for path in directory {
        let truepath = path.unwrap();
        if truepath.file_name() == "saves" {
            saves = truepath.path();
        }
    }
    if saves.to_str().unwrap() == "" {
        let mut newfolder = env::current_dir().unwrap();
        newfolder.push("saves");
        create_dir(Path::new(&newfolder)).expect("Could not create saves folder.");
        saves = PathBuf::from(newfolder);
    }

     //Add assets file to execution environment
     let assets = find_folder::Search::ParentsThenKids(3, 3).for_folder("assets").unwrap();
     //Find each icon as an asset file path
     let saveicon_file = assets.join("saveicon.png");
     let openicon_file = assets.join("openicon.png");
     let buildicon_file = assets.join("buildicon.png");
     let executeicon_file = assets.join("executeicon.png");
     let stepicon_file = assets.join("stepicon.png");
     let resetexecutionicon_file = assets.join("resetexecutionicon.png");
     let helpicon_file = assets.join("helpicon.png");
     let icons = Icons {
        saveicon: Texture::from_path(&mut window.create_texture_context(), &saveicon_file, Flip::None, &TextureSettings::new()).unwrap(),
        openicon: Texture::from_path(&mut window.create_texture_context(), &openicon_file, Flip::None, &TextureSettings::new()).unwrap(),
        buildicon: Texture::from_path(&mut window.create_texture_context(), &buildicon_file, Flip::None, &TextureSettings::new()).unwrap(),
        executeicon: Texture::from_path(&mut window.create_texture_context(), &executeicon_file, Flip::None, &TextureSettings::new()).unwrap(),
        stepicon: Texture::from_path(&mut window.create_texture_context(), &stepicon_file, Flip::None, &TextureSettings::new()).unwrap(),
        resetexecutionicon: Texture::from_path(&mut window.create_texture_context(), &resetexecutionicon_file, Flip::None, &TextureSettings::new()).unwrap(),
        helpicon: Texture::from_path(&mut window.create_texture_context(), &helpicon_file, Flip::None, &TextureSettings::new()).unwrap(),
    };
    let mut glyphs = window.load_font(assets.join("SourceCodePro-Regular.ttf")).unwrap();
    let mut filename: String = "Untitled.txt".to_string();

    //Input booleans
    let mut shift = false;
    let mut control = false;

    //Input buffer as lines
    let mut input_lines: Vec<String> = Vec::new();
    input_lines.push(String::new());

    //Cursor
    let mut cursorpos = [0; 2];

    //Display windows
    let mut save_window = SaveWindow::new(String::from(&filename));
    let mut open_window = OpenWindow::new(saves);
    let mut display_save_window = false;
    let mut display_open_window = false;
    let mut display_help_window = false;

    //Event variables
    let mut mousecursor = [0.0; 2];
    let mut holdbutton: Vec<[f64; 2]> = Vec::new();
    let mut language_interpreter = InterpreterObject::new();

    //Event loop
    while let Some(e) = window.next() {
        let mut windowtitle: String = String::from("LimitationTEXT - ");
        windowtitle.push_str(&filename);
        window.set_title(windowtitle);
        let windowsize = window.size();
        window.draw_2d(&e, |c, g, device|{
            //Draw background
            clear(palette.background, g);
            //Draw ribbon
            Rectangle::new(palette.border).draw([0.0, 0.0, windowsize.width, 64.0], &c.draw_state, c.transform, g);
            image(&icons.saveicon, c.transform.trans(0.0, 0.0), g);
            image(&icons.openicon, c.transform.trans(64.0, 0.0), g);
            image(&icons.buildicon, c.transform.trans(128.0, 0.0), g);
            image(&icons.executeicon, c.transform.trans(192.0, 0.0), g);
            image(&icons.stepicon, c.transform.trans(256.0, 0.0), g);
            image(&icons.resetexecutionicon, c.transform.trans(320.0, 0.0), g);
            image(&icons.helpicon, c.transform.trans(384.0, 0.0), g);
            //Draw ribbon mouse-over highlight
            draw_ribbon_highlight(&c, g, &palette, &mousecursor, &language_interpreter);   
            //Draw text
            draw_input_buffer_updated(&c, g, &input_lines, &palette, &mut glyphs, &cursorpos, &windowsize); 
            glyphs.factory.encoder.flush(device); 

            if display_help_window {
                draw_help_window(&c, g, &palette, &windowsize);
                draw_help_window_text(&c, g, &palette, &mut glyphs, &windowsize);
            }

            draw_output_console(&c, g, &palette, &windowsize);
            draw_output_buffer(&c, g, &mut language_interpreter, &palette, &mut glyphs, &windowsize);

            //Draw Extra Windows
            if display_save_window {
                save_window.draw_window(&c, g);
                save_window.draw_glyphs(&c, g, &mut glyphs);
            }
            if display_open_window {
                open_window.draw_window(&c, g);
                open_window.draw_glyphs(&c, g, &mut glyphs);
            }
        });
       
        //Update mouse position on screen
        use piston::input::*;
        if let Some(args) = e.mouse_cursor_args() {
            mousecursor = args;
        }
        //Get mouse button click
        if let Some(Button::Mouse(MouseButton::Left)) = e.press_args() {
            //TODO: Input left mouse button
            //Test Ribbon Buttons
            if mousecursor[1] > 0.0 && mousecursor[1] < 64.0 {
                //Save Button
                if mousecursor[0] > 0.0 && mousecursor[0] < 64.0 {
                    display_save_window = true;
                    display_open_window = false;
                }
                //Open Button
                else if mousecursor[0] > 64.0 && mousecursor[0] < 128.0 {
                    display_open_window = true;
                    display_save_window = false;
                }
                //Build Code Button
                else if mousecursor[0] > 128.0 && mousecursor[0] < 192.0 {
                    language_interpreter.populate_input(&input_lines);
                    language_interpreter.build();
                }
                //Execute Code Button
                else if mousecursor[0] > 192.0 && mousecursor[0] < 256.0 {
                    if language_interpreter.is_executable() == true {
                        language_interpreter.start_execution();
                    }
                }
                //Execute By Step BUtton
                else if mousecursor[0] > 256.0 && mousecursor[0] < 320.0 {
                    if language_interpreter.is_executable() == true {
                        language_interpreter.execute_step();
                    }
                }
                //Reset Execution Button
                else if mousecursor[0] > 320.0 && mousecursor[0] < 384.0 {
                    language_interpreter.reset_execution();
                }
                //Quick Reference Button
                else if mousecursor[0] > 384.0 && mousecursor[0] < 448.0 {
                    if display_help_window{
                        display_help_window = false;
                    }else{
                        display_help_window = true;
                    }
                }
                else{
                    display_open_window = false;
                    display_save_window = false;
                }
            }else{
                display_save_window = false;
                display_open_window = false;
            }
        }
        //Keyboard key press event
        if let Some(Button::Keyboard(key)) = e.press_args() {
            //Shift
            if key == Key::LShift || key == Key::RShift {
                shift = true;
            }
            //Control
            else if key == Key::LCtrl || key == Key::RCtrl {
                control = true;
            }
            //Escape
            else if key == Key::Escape {
                display_save_window = false;
                display_open_window = false;
                display_help_window = false;
            }
            //Keyboard shortcut: CTRL+N : Creates new file as if program was just opened.
            else if control == true && key == Key::N {
                filename = String::from("Untitled.txt");
                input_lines.clear();
                cursorpos[0] = 0;
                cursorpos[1] = 0;
                language_interpreter.reset_execution();
            }
            //Keyboard shortcut: CTRL+S : Quick-save, or open save window
            else if control == true && key == Key::S {
                if filename.eq("Untitled.txt") == false {
                    savefile(&filename, &input_lines);
                } else {
                    display_save_window = true;
                }
            }
            else{
                //Handle input for all other cases
                holdbutton.push([key.code() as f64, 0.0]);
                //Input for save window
                if display_save_window == true {
                    if key == Key::Return {
                        if save_window.filename.len() > 0 {
                            filename = String::from(&save_window.filename);
                            savefile(&filename, &input_lines);
                            display_save_window = false;
                        }
                    } else {
                        save_window.handle_input(&key, &shift);
                    }
                //Input for open file window
                } else if display_open_window == true {
                    if key == Key::Return {
                        let input_buffer = open_window.get_file_buffer();
                        input_lines.clear();
                        input_lines.push(String::new());
                        for letter in input_buffer.chars() {
                            if letter == '\n' {
                                input_lines.push(String::new());
                            } else {
                                input_lines.last_mut().unwrap().push(letter);
                            }
                        }
                        filename = open_window.get_filename();
                        display_open_window = false;
                        cursorpos[0] = 0;
                        cursorpos[1] = 0;
                        save_window.filename = open_window.get_filename();
                    } else {
                        open_window.handle_input(&key);
                    }
                //Input for output window
                } else if language_interpreter.is_waiting() {
                    language_interpreter.handle_input(&key, &shift);
                } 
                //Input for text editor
                else {
                    handle_input(&mut input_lines, &key, &mut cursorpos, &shift, &control);
                }
            }
        }

        //Update event, when no other event present is active
        if let Some(frame_delta) = e.update_args() {
            for item in 0..holdbutton.len(){
                holdbutton[item][1] += frame_delta.dt;
                if holdbutton[item][1] >= 0.4 {
                    if display_save_window == true {
                        save_window.handle_input(&Key::from(holdbutton[item][0] as u32), &shift);
                    }
                    else if display_save_window == true {
                        open_window.handle_input(&Key::from(holdbutton[item][0] as u32));
                    } else {
                        handle_input(&mut input_lines, &Key::from(holdbutton[item][0] as u32), 
                            &mut cursorpos, &shift, &control);
                    }
                    holdbutton[item][1] = 0.39;
                }
            }
            if language_interpreter.can_step() && language_interpreter.is_waiting() == false {
                language_interpreter.execute_step();
                while language_interpreter.can_recur_step(){
                    language_interpreter.execute_step();
                }
            }
        }

        //Button release event
        if let Some(Button::Keyboard(key)) = e.release_args() {
            if key == Key::LShift || key == Key::RShift {
                shift = false;
            }
            else if key == Key::LCtrl || key == Key::RCtrl {
                control = false;
            }
            holdbutton.clear();
        }

        //Special handler for when '"', '~' buttons are pushed
        if let Some(args) = e.button_args() {
            let scan = args.scancode;
            if scan != None {
                let scancode = scan.unwrap();
                if scancode == 40 && args.state == ButtonState::Press {
                    if shift == true {
                        input_lines.get_mut(cursorpos[1] ).unwrap().insert(cursorpos[0] , '\"');
                        cursorpos[0] += 1;
                    }else {
                        input_lines.get_mut(cursorpos[1] ).unwrap().insert(cursorpos[0] , '\'');
                        cursorpos[0] += 1;
                    }
                }
                if scancode == 41 && args.state == ButtonState::Press{
                    if shift == true {
                        input_lines.get_mut(cursorpos[1] ).unwrap().insert(cursorpos[0] , '~');
                        cursorpos[0] += 1;
                    }else {
                        input_lines.get_mut(cursorpos[1] ).unwrap().insert(cursorpos[0] , '`');
                        cursorpos[0] += 1;
                    }
                }
            }
        }
    }
}

//Draw the top ribbon highlights, when the mouse intersects with the ribbon
fn draw_ribbon_highlight<G: Graphics>(c: &Context, g: &mut G, colors: &Palette, mousecursor: &[f64; 2], language_interpreter: &InterpreterObject){
    if mousecursor[1] < 64.0 {
        if mousecursor[0] > 0.0 && mousecursor[0] < 64.0 {
            Rectangle::new(colors.highlight).draw([0.0, 0.0, 64.0, 64.0], &c.draw_state, c.transform, g);
        }else if mousecursor[0] > 64.0 && mousecursor[0] < 128.0 {
            Rectangle::new(colors.highlight).draw([64.0, 0.0, 64.0, 64.0], &c.draw_state, c.transform, g);
        }else if mousecursor[0] > 128.0 && mousecursor[0] < 192.0 {
            Rectangle::new(colors.highlight).draw([128.0, 0.0, 64.0, 64.0], &c.draw_state, c.transform, g);
        }else if mousecursor[0] > 192.0 && mousecursor[0] < 256.0 {
            if language_interpreter.is_executable() {
                Rectangle::new(colors.highlight).draw([192.0, 0.0, 64.0, 64.0], &c.draw_state, c.transform, g);
            } else {
                Rectangle::new(colors.disable_highlight).draw([192.0, 0.0, 64.0, 64.0], &c.draw_state, c.transform, g);
            }
        }else if mousecursor[0] > 256.0 && mousecursor[0] < 320.0 {
            if language_interpreter.is_executable() {
                Rectangle::new(colors.highlight).draw([256.0, 0.0, 64.0, 64.0], &c.draw_state, c.transform, g);
            } else {
                Rectangle::new(colors.disable_highlight).draw([256.0, 0.0, 64.0, 64.0], &c.draw_state, c.transform, g);
            }       
        }else if mousecursor[0] > 320.0 && mousecursor[0] < 384.0 {
            if language_interpreter.is_executable() {
                Rectangle::new(colors.highlight).draw([320.0, 0.0, 64.0, 64.0], &c.draw_state, c.transform, g);
            } else {
                Rectangle::new(colors.disable_highlight).draw([320.0, 0.0, 64.0, 64.0], &c.draw_state, c.transform, g);
            }        
        }else if mousecursor[0] > 384.0 && mousecursor[0] < 448.0 {
            Rectangle::new(colors.highlight).draw([384.0, 0.0, 64.0, 64.0], &c.draw_state, c.transform, g);
        }
    }
}

//Draw output background
fn draw_output_console<G: Graphics>(c: &Context, g: &mut G, colors: &Palette, windowsize: &Size){
    Rectangle::new(colors.output_background).draw([0.0, windowsize.height - 200.0, windowsize.width, 200.0], &c.draw_state, c.transform, g);
}

//Draw the output buffer as text glyphs
fn draw_output_buffer<C: CharacterCache, G: Graphics<Texture = <C as CharacterCache>::Texture>>
(c: &Context, g: &mut G, language: &mut InterpreterObject, colors: &Palette, glyphs: &mut C, windowsize: &Size) {
    let output_buffer = String::from(language.get_output());
    let output_entry = String::from(language.get_console_entry());
    let mut output_lines: Vec<String> = Vec::new();
    output_lines.push(String::new());
    for character in output_buffer.chars() {
        if character != '\n'{
            output_lines.last_mut().unwrap().push(character);
        } else {
            if output_lines.len() >= 11 {
                output_lines.remove(0);
            }
            output_lines.push(String::new());
        }
    }
    for lines in 0..output_lines.len() {
        text::Text::new_color(colors.text, 18).draw(&output_lines.get(lines).unwrap(), glyphs, &c.draw_state, 
        c.transform.trans(10.0, (windowsize.height-180.0) + lines as f64 * 18.0), g).unwrap_or_default();
    }
    if language.is_waiting() == true {
        Rectangle::new(colors.border).draw([10.0, windowsize.height - 225.0, windowsize.width - 20.0, 25.0], &c.draw_state, c.transform, g);
        text::Text::new_color(colors.text, 18).draw(&output_entry, glyphs, &c.draw_state, 
        c.transform.trans(10.0, windowsize.height-205.0), g).unwrap_or_default();
    }
}

//Draw the help window background
fn draw_help_window<G: Graphics>(c: &Context, g: &mut G, colors: &Palette, windowsize: &Size){
    Rectangle::new(colors.border).draw([windowsize.width - 215.0, 64.0, 215.0, windowsize.height-64.0], &c.draw_state, c.transform, g);
}

//Draw the help window text as glyphs
fn draw_help_window_text<C: CharacterCache, G: Graphics<Texture = <C as CharacterCache>::Texture>>
(c: &Context, g: &mut G, colors: &Palette, glyphs: &mut C, windowsize: &Size){
    Text::new_color(colors.text, 10).draw("'+' - Increment", glyphs, &c.draw_state, c.transform.trans(windowsize.width-210.0, 80.0), g).unwrap_or_default();
    Text::new_color(colors.text, 10).draw("'-' - Decrement", glyphs, &c.draw_state, c.transform.trans(windowsize.width-210.0, 90.0), g).unwrap_or_default();
    Text::new_color(colors.text, 10).draw("'<' - Shift index left", glyphs, &c.draw_state, c.transform.trans(windowsize.width-210.0, 100.0), g).unwrap_or_default();
    Text::new_color(colors.text, 10).draw("'>' - Shift index right", glyphs, &c.draw_state, c.transform.trans(windowsize.width-210.0, 110.0), g).unwrap_or_default();
    Text::new_color(colors.text, 10).draw("'^' - Shift index to value", glyphs, &c.draw_state, c.transform.trans(windowsize.width-210.0, 120.0), g).unwrap_or_default();
    Text::new_color(colors.text, 10).draw("'_' - Reset value to 0", glyphs, &c.draw_state, c.transform.trans(windowsize.width-210.0, 130.0), g).unwrap_or_default();
    Text::new_color(colors.text, 10).draw("'#' - Push to stack", glyphs, &c.draw_state, c.transform.trans(windowsize.width-210.0, 140.0), g).unwrap_or_default();
    Text::new_color(colors.text, 10).draw("'$' - Pop from stack", glyphs, &c.draw_state, c.transform.trans(windowsize.width-210.0, 150.0), g).unwrap_or_default();
    Text::new_color(colors.text, 10).draw("'?0' - Request numeric input", glyphs, &c.draw_state, c.transform.trans(windowsize.width-210.0, 160.0), g).unwrap_or_default();
    Text::new_color(colors.text, 10).draw("'?a' - Request ASCII input", glyphs, &c.draw_state, c.transform.trans(windowsize.width-210.0, 170.0), g).unwrap_or_default();
    Text::new_color(colors.text, 10).draw("'&0' - Output as number", glyphs, &c.draw_state, c.transform.trans(windowsize.width-210.0, 180.0), g).unwrap_or_default();
    Text::new_color(colors.text, 10).draw("'&a' - Output as ASCII", glyphs, &c.draw_state, c.transform.trans(windowsize.width-210.0, 190.0), g).unwrap_or_default();
    Text::new_color(colors.text, 10).draw("'{' - Conditional jump on 0", glyphs, &c.draw_state, c.transform.trans(windowsize.width-210.0, 200.0), g).unwrap_or_default();
    Text::new_color(colors.text, 10).draw("'}' - Conditional jump marker", glyphs, &c.draw_state, c.transform.trans(windowsize.width-210.0, 210.0), g).unwrap_or_default();
    Text::new_color(colors.text, 10).draw("':' - Non-conditional jump", glyphs, &c.draw_state, c.transform.trans(windowsize.width-210.0, 220.0), g).unwrap_or_default();
    Text::new_color(colors.text, 10).draw("'=' - Non-conditional jump marker", glyphs, &c.draw_state, c.transform.trans(windowsize.width-210.0, 230.0), g).unwrap_or_default();
}

//Draw the file chooser window and save the file from input
fn savefile(filename: &String, input: &Vec<String>) {
    let filedir = "saves/";
    let filepath = [filedir, filename].join(""); 
    let mut file = std::fs::File::create(filepath).expect("Unable to create file.");
    let mut output_buffer = String::new();
    for line in input {
        output_buffer.push_str(line);
        output_buffer.push('\n');
    }
    file.write_all(output_buffer.as_bytes()).expect("Unable to write");
}

//Handle keyboard input for the input buffer
fn handle_input(input_buffer: &mut Vec<String>, key: &Key, cursorpos: &mut [usize; 2], shift: &bool, control: &bool) {
    if *key == Key::Return {
        //Pressed the enter key
        //Get the string from cursorpos to the end of the line and turn that into the next line.
        let mut newline = String::new();
        for _ in cursorpos[0]..input_buffer[cursorpos[1]].len() {
            let push_char = input_buffer[cursorpos[1]].chars().nth(cursorpos[0]).unwrap();
            newline.push(push_char);
            input_buffer[cursorpos[1]].remove(cursorpos[0]);
        }
        input_buffer.insert(cursorpos[1]+1, newline);
        cursorpos[0] = 0;
        cursorpos[1] += 1;
    }
    else if *key == Key::Backspace && input_buffer.len() > 0 {
        //Pressed backspace
        if cursorpos[0] > 0 || cursorpos[1] >= 1 {
            if input_buffer.get(0).unwrap().len() == 1 && input_buffer.len() == 1 {
                cursorpos[0] = 0;
                cursorpos[1] = 0;
                input_buffer.clear();
            } else if cursorpos[0] == 0 {
                let push_str = String::from(input_buffer[cursorpos[1]].to_string());
                input_buffer[cursorpos[1]-1].push_str(&push_str);
                input_buffer.remove(cursorpos[1]);
                cursorpos[1] -= 1;
                cursorpos[0] = input_buffer.get(cursorpos[1]).unwrap().len();                
            } else {
                input_buffer.get_mut(cursorpos[1]).unwrap().remove(cursorpos[0]-1);
                cursorpos[0] -= 1;
            }
        }
    }
    else if *control == true && *key == Key::D {
        //Pressed CTRL+D to clear buffer
        input_buffer.clear();
        cursorpos[0] = 0;
        cursorpos[1] = 0;
    }
    //Tab key 
    else if *key == Key::Tab {
        input_buffer.get_mut(cursorpos[1]).unwrap().insert(cursorpos[0], ' ');
        input_buffer.get_mut(cursorpos[1]).unwrap().insert(cursorpos[0], ' ');
        input_buffer.get_mut(cursorpos[1]).unwrap().insert(cursorpos[0], ' ');
        cursorpos[0] += 3;
    }
    //Right key
    else if *key == Key::Right{
        if cursorpos[0] < input_buffer.get(cursorpos[1] ).unwrap().len() {
            cursorpos[0] += 1;
        }else if cursorpos[1] < input_buffer.len()-1  {
            cursorpos[0] = 0;
            cursorpos[1] += 1;
        }
    }
    //Left key
    else if *key == Key::Left {
        if cursorpos[0] > 0 {
            cursorpos[0] -= 1;
        }else if cursorpos[1] > 0 {
            cursorpos[1] -= 1;
            cursorpos[0] = input_buffer.get(cursorpos[1] ).unwrap().len() ;
        }
    }
    else if *key == Key::Up{
        //Up key pressed.
        if cursorpos[1] > 0 {
            cursorpos[1] -= 1;
            if cursorpos[0] > input_buffer.get(cursorpos[1] ).unwrap().len() {
                cursorpos[0] = input_buffer.get(cursorpos[1] ).unwrap().len() ;
            } 
        }
    }
    else if *key == Key::Down{
        //down key pressed.
        if cursorpos[1] < input_buffer.len()-1 {
            cursorpos[1] += 1;
            if cursorpos[0] > input_buffer.get(cursorpos[1] ).unwrap().len() {
                cursorpos[0] = input_buffer.get(cursorpos[1] ).unwrap().len() ;
            } 
        }
    }
    //Handle the rest of the input, including ' and "
    else if *shift == true {
        let upperchar = getupperchar(&key.code());
        if upperchar != 0 as char {
            if input_buffer.is_empty() {
                input_buffer.push(String::from(upperchar));
            }else{
                input_buffer.get_mut(cursorpos[1]).unwrap().insert(cursorpos[0] , upperchar);
            }
            cursorpos[0] += 1;
        }
        if upperchar == '{' {
            input_buffer[cursorpos[1]].insert(cursorpos[0], '}');
        }
    }else if *shift == false {
        let lowerchar = getlowerchar(&key.code());
        if lowerchar != 0 as char {
            if input_buffer.is_empty() {
                input_buffer.push(String::from(lowerchar));
            }else{
                input_buffer.get_mut(cursorpos[1]).unwrap().insert(cursorpos[0] , lowerchar);
            }            cursorpos[0] += 1;
        }
    }
}

//Return character from keycode, lowercase
pub fn getlowerchar(key: &i32) -> char{
    match key {
        97 => return 'a', 98 => return 'b', 99 => return 'c', 100 => return 'd', 101 => return 'e', 102 => return 'f', 103 => return 'g',
        104 => return 'h', 105 => return 'i', 106 => return 'j', 107 => return 'k', 108 => return 'l', 109 => return 'm',
        110 => return 'n', 111 => return 'o', 112 => return 'p', 113 => return 'q', 114 => return 'r', 115 => return 's',
        116 => return 't', 117 => return 'u', 118 => return 'v', 119 => return 'w', 120 => return 'x', 121 => return 'y',
        122 => return 'z', 48 => return '0', 49 => return '1', 50 => return '2', 51 => return '3', 52 => return '4', 53 => return '5',
        54 => return '6', 55 => return '7', 56 => return '8', 57 => return '9', 45 => return '-', 61 => return '=', 91 => return '[',
        93 => return ']', 92 => return '\\', 59 => return ';', 44 => return ',', 46 => return '.', 47 => return '/', 9 => return '\t',
        32 => return ' ', 
        _ => return 0 as char,
    }
}

//Return character from keycode, uppercase
pub fn getupperchar(key: &i32) -> char{
    match key {
        97 => return 'A', 98 => return 'B', 99 => return 'C', 100 => return 'D', 101 => return 'E', 102 => return 'F', 103 => return 'G',
        104 => return 'H', 105 => return 'I', 106 => return 'J', 107 => return 'K', 108 => return 'L', 109 => return 'M',
        110 => return 'N', 111 => return 'O', 112 => return 'P', 113 => return 'Q', 114 => return 'R', 115 => return 'S',
        116 => return 'T', 117 => return 'U', 118 => return 'V', 119 => return 'W', 120 => return 'X', 121 => return 'Y',
        122 => return 'Z', 48 => return ')', 49 => return '!', 50 => return '@', 51 => return '#', 52 => return '$', 53 => return '%',
        54 => return '^', 55 => return '&', 56 => return '*', 57 => return '(', 45 => return '_', 61 => return '+', 91 => return '{',
        93 => return '}', 92 => return '|', 59 => return ':', 44 => return '<', 46 => return '>', 47 => return '?', 9 => return '\t',
        32 => return ' ', 
        _ => return 0 as char,
    }
}

//Draw the input buffer as text glyphs (Updated for performace)
fn draw_input_buffer_updated<C: CharacterCache, G: Graphics<Texture = <C as CharacterCache>::Texture>>
(c: &Context, g: &mut G, input: &Vec<String>, colors: &Palette, glyphs: &mut C, cursorpos: &[usize; 2], windowsize: &Size) {
    let mut draw_x_offset = 0;
    let mut draw_y_offset = 0;
    let font_size: u32 = 20;
    
    //Get the draw offset from the cursor position
    if input.is_empty() == false {
        if cursorpos[0] > (windowsize.width / font_size as f64) as usize {
            draw_x_offset = cursorpos[0] - (windowsize.width / font_size as f64) as usize;
        }
        if cursorpos[1] > ((windowsize.height / font_size as f64) - 20.0) as usize {
            draw_y_offset = (cursorpos[1]) - ((windowsize.height / font_size as f64) - 19.0) as usize;
        }


        //Draw lines as text
        let mut is_cursor_drawn = false;
        for line in draw_y_offset..input.len() {
            if input.get(line).unwrap().len() > draw_x_offset {
                let draw_text = String::from(&input.get(line).unwrap()[draw_x_offset..input.get(line).unwrap().len()]);
                let glyph_transform = c.transform.trans(20.0, 90.0 + ((line-draw_y_offset) as f64*25.0));
                text::Text::new_color(colors.text, font_size).draw(&draw_text, glyphs, &c.draw_state, glyph_transform, g).unwrap_or_default();
                if line == cursorpos[1] {
                    let cursorpostext = String::from(&input.get(line).unwrap()[draw_x_offset..cursorpos[0]]);
                    let cursor_transform = c.transform.trans(15.0+glyphs.width(font_size, &cursorpostext).unwrap_or_default(), 90.0 + ((line-draw_y_offset) as f64*25.0));
                    is_cursor_drawn = true;
                    text::Text::new_color(colors.highlight, font_size).draw("|", glyphs, &c.draw_state, cursor_transform, g).unwrap_or_default();
                }
            }
        }
        if is_cursor_drawn == false {
            text::Text::new_color(colors.highlight, font_size).draw("|", glyphs, &c.draw_state, c.transform.trans(15.0, 90.0 + ((cursorpos[1]-draw_y_offset) as f64 * 25.0)), g).unwrap_or_default();
        }
    }
    

}
