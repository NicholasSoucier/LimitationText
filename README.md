# Limitation Text
# Nicholas Soucier - 2022

LimitationText is a simplistic text editor and compiler for a novelty programming language, Limitation. 
This text editor can open, save, compile, and execute code for a novelty language created in 2021.

All code is open source and ready to read. This project was made in Rust using the Piston Graphics API, OpenGL, the find-folder API, and the Rust Standard Library.

Included in the Git Repository: /release (finished executable release) | /dev (development files)
 
How to use the program:
When first launching the program, a new folder /saves/ will be created in the same directory as the executable, this is the folder for saved programs or text files.
Typing will put valid characters into the input buffer, which can be used like most text editors with standard controls.

Top Ribbon Controls (in order from left to right):
* Save: Allows the user to change the file name and save the file into the /saves/ directory. CTRL+S for quick-save if the file name is not Untitled.txt
* Open: Lists all files in the /saves/ directory. Choose a file to open with the arrow keys and Enter.
* Build: Will compile the code and check for syntax errors. All progress and errors will be reported to the console at the bottom.
* Execute: Will attempt to execute all the code in the input buffer. Will only work after the code is built.
* Execute Step: Will execute code one token at a time. Will only work after the code is built.
* Reset Execution: Will reset the execution environment and rebuild the code.
* Quick Guide: Will display or hide the quick guide on the right side of the screen for quick token reference.

How the Limitation Language works:
* Similar to BrainFuck, this language is a bare-bones operatable language constructed of a limited amount of operations that the programmer is allowed to perform, and is executed in a simulated environment.
* The execution environment is made of an array list of 32-bit unsigned integers and a small 32-bit unsigned integer stack.
* The array initally starts pointed at index 0, with all values being 0. The stack starts empty
There are 15 operations available in the language:
* + - increment the pointed value by 1
* - - decrement the pointed value by 1
* < - shift the pointer left
* < - shift the pointer right
* ^ - shift the pointer to the value of the current pointed value
* _ - set the pointed value to 0
* # - copy the pointed value and push to the stack
* $ - pop the top of the stack to the pointed value
* ?0 - user input as a number
* ?a - user input as a string of ASCII characters
* &0 - output the pointed value as an integer
* &a - output the pointed value as an ASCII character
* { - if the pointed value = 0, jump to the next matching '}' moving forwards.
* } - conditional jump marker
* : - always jump backwards to the matching '='
* = - non-conditional jump marker
