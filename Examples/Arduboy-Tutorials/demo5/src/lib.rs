#![no_std]
#![allow(non_upper_case_globals)]

//Include the Arduboy Library
//Initialize the arduboy object
use arduboy_rust::prelude::*;

//Initialize variables used in this game
static mut playerwin: c_int = 0;
static mut attempts: c_int = 0;
static mut guessednumber: c_int = 0;
static mut randomnumber: c_int = 0;
static mut lastguess: c_int = 0;

//The setup() function runs once when you turn your Arduboy on
#[no_mangle]
pub unsafe extern "C" fn setup() {
    // put your setup code here, to run once:
    arduboy.begin();
    arduboy.clear();
    arduboy.init_random_seed();
    randomnumber = random_between(1, 101) as i16;
}
//The loop() function repeats forever after setup() is done
#[no_mangle]
#[export_name = "loop"]
pub unsafe extern "C" fn loop_() {
    // put your main code here, to run repeatedly:
    arduboy.clear();
    arduboy.poll_buttons();
    if playerwin == 0 {
        //Ask the player for a number and play the game
        if attempts == 7 {
            //Game Over screen
            arduboy.set_cursor(0, 0);
            arduboy.print("You lost!\0");
            arduboy.print("\n\0");
            arduboy.print("Correct Number: \0");
            arduboy.print(randomnumber);
            if A.just_pressed() {
                randomnumber = random_between(1, 101) as i16;
                attempts = 0;
                playerwin = 0;
            }
        } else {
            //Player has more attempts
            if UP.just_pressed() {
                guessednumber += 1;
            }
            if DOWN.just_pressed() {
                guessednumber -= 1;
            }
            if A.just_pressed() {
                if guessednumber == randomnumber {
                    playerwin += 1;
                } else {
                    attempts += 1;
                    lastguess = guessednumber
                }
            }
            arduboy.set_cursor(0, 0);
            arduboy.print("Attempt: \0");
            arduboy.print(attempts);
            arduboy.print("\n\0");
            arduboy.print("Number to guess: \0");
            arduboy.print(guessednumber);
            arduboy.print("\n\0");
            if attempts == 0 {
                arduboy.print("Good luck!\0");
            } else {
                arduboy.print(lastguess);
                if lastguess > randomnumber {
                    arduboy.print(" is too high!\0");
                }
                if lastguess < randomnumber {
                    arduboy.print(" is too low!\0");
                }
            }
        }
    } else {
        //Tell the player that they won!
        arduboy.set_cursor(0, 0);
        arduboy.print("You won!\0");
        arduboy.print("\n\0");
        arduboy.print("Correct Number: \0");
        arduboy.print(randomnumber);

        if A.just_pressed() {
            randomnumber = random_between(1, 101) as c_int;
            attempts = 0;
            playerwin = 0;
        }
    }
    arduboy.display();
}
