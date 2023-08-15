#![no_std]
#![allow(non_upper_case_globals)]
//Include the Arduboy Library
//Initialize the arduboy object
use arduboy_rust::prelude::*;
const arduboy: Arduboy2 = Arduboy2::new();
//Initialize our counter variable

static mut counter: c_int = 0;
//The setup() function runs once when you turn your Arduboy on
#[no_mangle]
pub unsafe extern "C" fn setup() {
    //Start the Arduboy properly and display the Arduboy logo
    arduboy.begin();
    //Get rid of the Arduboy logo and clear the screen
    arduboy.clear();
    //Assign our counter variable to be equal to 0
    counter = 0;
    //Set framerate to 10
    arduboy.set_frame_rate(10);
}
//The loop() function repeats forever after setup() is done
#[no_mangle]
#[export_name = "loop"]
pub unsafe extern "C" fn loop_() {
    //Skip cycles not in the framerate
    if !arduboy.next_frame() {
        return;
    }
    //Clear whatever is printed on the screen
    arduboy.clear();
    //Check if the UP_BUTTON is being pressed
    if UP.pressed() {
        //Increase counter by 1
        counter = counter + 1;
    }
    //Check if the DOWN_BUTTON is being pressed
    if DOWN.pressed() {
        //Decrease counter
        counter = counter - 1;
    }
    //Check if counter is equal to 36
    if counter == 36 {
        //Move the cursor to the position 30, 30 of the screen
        arduboy.set_cursor(30, 30);
        //Printing the yay (important always put the \0 at the end for &str)
        arduboy.print(f!(b"Yay!\0"));
    }
    //Move the cursor back to the top-left of the screen
    arduboy.set_cursor(0, 0);
    //Print out the value of counter
    arduboy.print(counter);
    //Refresh the screen to show whatever's printed to it
    arduboy.display();
}
