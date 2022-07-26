
/*
    ----------------------------------------------------------------------------------------------
    NovelText text editor and compiler
    lang.rs
    Nicholas Soucier

    lang file, used to control the compiler and executor of the novelty language. 
    ----------------------------------------------------------------------------------------------
*/

use piston::input::*;

//Tokens
const INCREMENT: u8 = 0;               //+
const DECREMENT: u8 = 1;               //-
const SHIFTLEFT: u8 = 2;               //<
const SHIFTRIGHT: u8 = 3;              //>
const SHIFTNUM: u8 = 4;                //^
const RESET: u8 = 5;                   //_
const STACKPUSH: u8 = 6;               //#
const STACKPOP: u8 = 7;                //$
const INPUTNUM: u8 = 8;                //?0
const INPUTALPHA: u8 = 9;              //?a
const OUTPUTNUM: u8 = 10;              //&0
const OUTPUTALPHA: u8 = 11;            //&a
const CONDITIONALJUMP: u8 = 12;        //{
const CONDITIONALMARKER: u8 = 13;      //}
const NONCONDITIONALJUMP: u8 = 14;     //:
const NONCONDITIONALMARKER: u8 = 15;   //=
const ENDOFINPUT: u8 = 16;             //EOI

pub struct InterpreterObject {
    input: String,
    output: String,
    token_list: Vec<u8>,
    index: usize,
    execute_array: Vec<u32>,
    execute_stack: Vec<u32>,
    execute_index: usize,
    can_execute: bool,
    wait_for_input: bool,
    is_executing: bool,
    input_type: u8,
    console_entry: String,
}

impl InterpreterObject {
    pub fn new() -> InterpreterObject {
        InterpreterObject {
            input: String::new(),
            output: String::new(),
            token_list: Vec::new(),
            execute_array: vec![0; 512],
            execute_stack: Vec::new(),
            execute_index: 0,
            index: 0,
            input_type: 0,
            can_execute: false,
            wait_for_input: false,
            is_executing: false,
            console_entry: String::new(),
        }
    }
    //Take in a string vector and create a String buffer
    pub fn populate_input(&mut self, input: &Vec<String>){
        self.input.clear();
        self.output.clear();
        for line in 0..input.len() {
            self.input += input.get(line).unwrap();
            if line < input.len()-1 {
                self.input += "\n";
            }
        }
    }
    //Basic code compilation by looking for tokens and placing the tokens in a list.
    //Will fail if a valid token cannot be found
    pub fn build(&mut self,) {
        self.index = 0;
        for item in 0..self.execute_array.len(){
            self.execute_array[item] = 0;
        }
        self.execute_stack.clear();
        self.token_list.clear();
        self.console_entry.clear();
        loop{
            if !self.ignore_whitespace() {
                break;
            }
            if self.input.is_empty(){
                self.token_list.push(ENDOFINPUT);
                self.output.push_str("[INFO]: Build Successful\n");
                self.can_execute = true;
                break;
            }
            let next_char = self.input.chars().next().unwrap();
            match next_char {
                '+'=> {
                    self.token_list.push(INCREMENT);
                    self.input.remove(0);
                }
                '-'=> {
                    self.token_list.push(DECREMENT);
                    self.input.remove(0);
                }
                '<'=> {
                    self.token_list.push(SHIFTLEFT);
                    self.input.remove(0);
                }
                '>'=> {
                    self.token_list.push(SHIFTRIGHT);
                    self.input.remove(0);
                }
                '^'=> {
                    self.token_list.push(SHIFTNUM);
                    self.input.remove(0);
                }
                '_'=> {
                    self.token_list.push(RESET);
                    self.input.remove(0);
                }
                '#'=> {
                    self.token_list.push(STACKPUSH);
                    self.input.remove(0);
                }
                '$'=> {
                    self.token_list.push(STACKPOP);
                    self.input.remove(0);
                }
                '?'=> {
                    let second_token = self.input.chars().nth(1).unwrap();
                    if second_token == '0'{
                        self.token_list.push(INPUTNUM);
                    }else if second_token == 'a'{
                        self.token_list.push(INPUTALPHA);
                    }else {
                        self.output.push_str("[ERROR]: Expected '0' or 'a' after ? token for expected input type\n");
                        break;
                    }
                    self.input.remove(0);
                    self.input.remove(0);
                }
                '&'=> {
                    let second_token = self.input.chars().nth(1).unwrap();
                    if second_token == '0'{
                        self.token_list.push(OUTPUTNUM);
                    }else if second_token == 'a'{
                        self.token_list.push(OUTPUTALPHA);
                    }else {
                        self.output.push_str("[ERROR]: Expected '0' or 'a' after & token for expected output type\n");
                        break;
                    }
                    self.input.remove(0);
                    self.input.remove(0);
                }
                '{'=> {
                    self.token_list.push(CONDITIONALJUMP);
                    self.input.remove(0);
                }
                '}'=> {
                    self.token_list.push(CONDITIONALMARKER);
                    self.input.remove(0);
                }
                ':'=> {
                    self.token_list.push(NONCONDITIONALJUMP);
                    self.input.remove(0);
                }
                '='=> {
                    self.token_list.push(NONCONDITIONALMARKER);
                    self.input.remove(0);
                }
                
                _ => {
                    self.output.push_str("[ERROR]: Build Failure, unable to recognize character as a token: ");
                    self.output.push(next_char);
                    self.output.push('\n');
                    break;
                }
            }
        }
    }
    //While building, look for any incoming whitespace and remove it, so it cannot be read. 
    //This also includes comments
    pub fn ignore_whitespace(&mut self,) -> bool{
        loop {
            if self.input.is_empty() {
                return true;
            }
            if self.input.chars().next().unwrap() == ' '{
                self.input.remove(0);
            }
            else if self.input.chars().next().unwrap().is_whitespace() {
                self.input.remove(0);
            }
            else if self.input.chars().next().unwrap() == '/' {
                if self.input.chars().nth(1).unwrap() == '/' {
                    loop{
                        if self.input.is_empty() {
                            break;
                        }
                        if self.input.chars().next().unwrap() != '\n' {
                            self.input.remove(0);
                        }else{
                            self.input.remove(0);
                            break;
                        }
                    }
                }else{
                    self.output.push_str("[ERROR]: Build Failure, unable to recognize character as a token: /");
                    return false;
                }
            } else {
                return true;
            }
        }
    }
    
    pub fn is_executable(&self) -> bool {
        return self.can_execute;
    }
    pub fn is_waiting(&self) -> bool {
        return self.wait_for_input;
    }
    pub fn start_execution(&mut self,){
        if self.can_execute {
            self.is_executing = true;
        }
    }
    pub fn can_step(&self) -> bool{
        return self.is_executing;
    }

    //Execute a single token of the compiled code
    pub fn execute_step(&mut self,){
        if self.token_list[self.index] == ENDOFINPUT {
            self.is_executing = false;
            self.output.push_str("\n[INFO]: Finished Execution\n");
            self.can_execute = false;
            return;
        }
        let current_token = self.token_list[self.index];
        match current_token {
           INCREMENT => {
            self.increment();
           }
           DECREMENT => {
            self.decrement();
           }
           SHIFTLEFT => {
            self.shiftleft();
           }
           SHIFTRIGHT => {
            self.shiftright();
           }
           SHIFTNUM => {
            self.shiftnum();
           }
           RESET => {
            self.reset();
           }
           STACKPUSH => {
            self.stackpush();
           }
           STACKPOP => {
            self.stackpop();
           }
           INPUTNUM => {
            self.inputnum();
           }
           INPUTALPHA => {
            self.inputalpha();
           }
           OUTPUTNUM => {
            self.outputnum();
           }
           OUTPUTALPHA => {
            self.outputalpha();
           }
           CONDITIONALJUMP => {
            self.conditionaljump();
           }
           CONDITIONALMARKER => {
            self.index += 1;
           }
           NONCONDITIONALJUMP => {
            self.nonconditionaljump();
           }
           NONCONDITIONALMARKER => {
            self.index += 1;
           }
           _ => {
            //This should never happen
            self.output.push_str("No idea how, but the token list is corrupted. Very sorry\n");
            self.is_executing = false;
           }
        }
    }

    //If the next token is not an input token, we can perform the next step within the same loop
    pub fn can_recur_step(&self) -> bool{
        let next_token = self.token_list[self.index];
        if next_token != INPUTALPHA && next_token != INPUTNUM && self.is_executing {
            return true;
        }
        return false;
    }

    //Increment the pointed value by 1
    fn increment(&mut self,){
        if self.execute_array[self.execute_index] >= 16777216 {
            self.output.push_str("[ERROR]: Attempted to increment value at index ");
            self.output.push_str(&self.execute_index.to_string());
            self.output.push_str(" above integer max.\n");
            self.is_executing = false;
            self.can_execute = false;
        }else{
            self.execute_array[self.execute_index] += 1;
            self.index += 1;
        }
    }
    //Decrement the pointed value by 1
    fn decrement(&mut self,){
        if self.execute_array[self.execute_index] <= 0 {
            self.output.push_str("[ERROR]: Attempted to decrement value at index ");
            self.output.push_str(&self.execute_index.to_string());
            self.output.push_str(" below 0.\n");
            self.is_executing = false;
            self.can_execute = false;
        }else{
            self.execute_array[self.execute_index] -= 1;
            self.index += 1;
        }
    }
    //Shift pointer left (-1)
    fn shiftleft(&mut self,){
        if self.execute_index <= 0 {
            self.output.push_str("[ERROR]: Attempted to shift array index below 0.\n");
            self.is_executing = false;
            self.can_execute = false;
        }else{
            self.execute_index -= 1;
            self.index += 1;
        }
    }
    //Shift pointer right (+1)
    fn shiftright(&mut self,){
        if self.execute_index >= 512 {
            self.output.push_str("[ERROR]: Attempted to shift array index above 512.\n");
            self.is_executing = false;
            self.can_execute = false;
        }else{
            self.execute_index += 1;
            self.index += 1;
        }
    }
    //Shift pointer to the value at the index
    fn shiftnum(&mut self,){
        if self.execute_array[self.execute_index] >= 512 {
            self.output.push_str("[ERROR]: Attempted to shift array index above 512.\n");
            self.is_executing = false;
            self.can_execute = false;
        }else{
            self.execute_index = self.execute_array[self.execute_index] as usize;
            self.index += 1;
        }
    }
    //Set the pointed value to 0
    fn reset(&mut self,){
        self.execute_index = 0;
        self.index += 1;
    }
    //Push the pointed value to the stack
    fn stackpush(&mut self,){
        self.execute_stack.push(self.execute_array[self.execute_index]);
        self.index += 1;
    }
    //Set the pointed value to the popped value of the stack
    fn stackpop(&mut self,){
        if self.execute_stack.is_empty() {
            self.output.push_str("[ERROR]: Attempted to pop from empty stack.\n");
            self.is_executing = false;
            self.can_execute = false;
        } else {
            self.execute_array[self.execute_index] = self.execute_stack.pop().unwrap();
            self.index += 1;
        }
    }
    //Accept input as a number
    fn inputnum(&mut self,){
        self.wait_for_input = true;
        self.input_type = 0;
    }
    //Accept input as alphanumeric characters
    fn inputalpha(&mut self,){
        self.wait_for_input = true;
        self.input_type = 1;
    }
    //Output the pointed value as a number
    fn outputnum(&mut self,){
        self.output.push_str(&self.execute_array[self.execute_index].to_string());
        self.index += 1;
    }
    //Output the pointed value as a character
    fn outputalpha(&mut self,){
        self.output.push(char::from_u32(self.execute_array[self.execute_index]).unwrap());
        self.index += 1;
    }
    //Jump to matching '}' if the pointed value is 0
    fn conditionaljump(&mut self,){
        let mut matching = 0;
        if self.execute_array[self.execute_index] == 0 {
            self.index += 1;
            //If the value at index is 0, perform a matching jump!
            loop {
                if self.index >= self.token_list.len() {
                    self.output.push_str("[ERROR]: Unable to find matching '}' for conditional jump.");
                    self.is_executing = false;
                    break;
                }else if self.token_list[self.index] == CONDITIONALJUMP {
                    matching += 1;
                    self.index += 1;
                }else if self.token_list[self.index] == CONDITIONALMARKER {
                    if matching > 0 {
                        matching -= 1;
                        self.index += 1;
                    }else{
                        self.index += 1;
                        break;
                    }
                }else{
                    self.index += 1;
                }
            }
        }else{
            self.index += 1;
        }
    }
    //Jump backwards to matching =
    fn nonconditionaljump(&mut self,){
        let mut matching = 0;
        self.index -= 1;
        loop {
            if self.index == 0 && self.token_list[0] != NONCONDITIONALMARKER {
                self.output.push_str("[ERROR]: Unable to find matching '=' for non-conditional jump.");
                self.is_executing = false;
                break;
            }else if self.token_list[self.index] == NONCONDITIONALJUMP {
                matching += 1;
                self.index -= 1;
            }else if self.token_list[self.index] == NONCONDITIONALMARKER {
                if matching > 0 {
                    matching -= 1;
                    self.index -= 1;
                } else {
                    self.index += 1;
                    break;
                }
            } else {
                self.index -= 1;
            }
        }
    }

    //Clear everything and rebuild, effectively restarting the execution
    pub fn reset_execution(&mut self,) {
        self.wait_for_input = false;
        self.can_execute = false;
        self.console_entry.clear();
        self.output.clear();
        self.build();
    }

    //return the output string
    pub fn get_output(&mut self,) -> &String{
        return &self.output;
    }

    //Get the user input buffer, used for inputnum and inputalpha
    pub fn get_console_entry(&mut self,) -> &String {
        return &self.console_entry;
    }

    //handle all input for output, execution, and awating input
    pub fn handle_input(&mut self, key: &Key, shift: &bool){
        if *key == Key::Backspace {
            if self.console_entry.len() > 0 {
                self.console_entry = self.console_entry[0..self.console_entry.len()-1].to_string();
            }
        } else if *key == Key::Return {
            self.output.push_str(&self.console_entry);
            self.output.push('\n');
            self.wait_for_input = false;
            self.index += 1;
            if self.input_type == 0 {
                self.execute_array[self.execute_index] = u32::from_str_radix(&self.console_entry, 10).unwrap();
            }else if self.input_type == 1 {
                for entry in self.console_entry.chars() {
                    self.execute_array[self.execute_index] = entry as u32;
                    if self.execute_index < 512 {
                        self.execute_index += 1;
                    }
                }
            }
            self.console_entry.clear();
        } else {
            if *shift {
                let keychar = getupperchar(&key.code());
                if keychar != 0 as char {
                    self.console_entry.push(keychar);
                }
            } else {
                let keychar = getlowerchar(&key.code());
                if keychar != 0 as char {
                    self.console_entry.push(keychar);
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