@echo off

set option=%1

if [%option%]==[] (
    powershell -Command "cargo build -p game --release; cp ./target/arduboy/release/libgame.a ./arduboy-rust/Wrapper-Project/lib/libgame.a; cd arduboy-rust/Wrapper-Project/; pio run -v -t upload; cp ./.pio/build/arduboy/firmware.hex ./build/game.hex; pio run -t clean; rm lib/libgame.a; cd ../../"
    goto :eof
)

if %option%==snake (
   goto :run
) else if %option%==pong (
   goto :run
) else if %option%==rustacean (
   goto :run
) else if %option%==tone (
   goto :run
) else if %option%==eeprom (
   goto :run
) else if %option%==eeprom-byte (
   powershell -Command "cargo build -p %option% --release; cp ./target/arduboy/release/libeeprom_byte.a ./arduboy-rust/Wrapper-Project/lib/libgame.a; cd arduboy-rust/Wrapper-Project/; pio run -v -t upload; cp ./.pio/build/arduboy/firmware.hex ./build/%option%.hex; pio run -t clean; rm lib/libgame.a; cd ../../"
goto :eof
) else if %option%==progmem (
   goto :run
) else if %option%==demo2 (
   goto :run
) else if %option%==demo3 (
   goto :run
) else if %option%==demo4 (
   goto :run
) else if %option%==demo5 (
   goto :run
) else if %option%==demo6 (
   goto :run
) else (
    goto :help
)

:run
powershell -Command "cargo build -p %option% --release; cp ./target/arduboy/release/lib%option%.a ./arduboy-rust/Wrapper-Project/lib/libgame.a; cd arduboy-rust/Wrapper-Project/; pio run -v -t upload; cp ./.pio/build/arduboy/firmware.hex ./build/%option%.hex; pio run -t clean; rm lib/libgame.a; cd ../../"
goto :eof

:help
@echo Usage: .\run.bat                // for uploading your game
@echo Usage: .\run.bat ^<Example Game^> // for uploading an example game