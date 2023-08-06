#![no_std]
//Include the Arduboy Library
//Initialize the arduboy object
use arduboy_rust::prelude::*;
//Initialize our counter variable
#[allow(non_upper_case_globals)]
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
    //Set framerate to 30
    arduboy.set_frame_rate(30);
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
    //Move the cursor to the position 30, 30 of the screen
    arduboy.set_cursor(0, 0);
    //Increase counter's value by 1
    counter += 1;
    //Print out the value of counter
    arduboy.print(counter);
    //Refresh the screen to show whatever's printed to it
    arduboy.display();
}
