@echo off

set option=%1

if [%option%]==[] (
    powershell -Command "cargo build -p game --release; cp ./target/arduboy/release/libgame.a ./arduboy-rust/Wrapper-Project/lib/libgame.a; cd arduboy-rust/Wrapper-Project/; pio run -v -t upload; cp ./.pio/build/arduboy/firmware.hex ./build/game.hex; pio run -t clean; rm lib/libgame.a; cd ../../"
    goto :eof
) else if %option%==doc (
   powershell -Command "cargo doc -p arduboy-rust; rm -r ./docs/doc/; cp -r ./target/arduboy/doc ./docs/"
   goto :eof
) else if %option%==eeprom-byte (
   powershell -Command "cargo build -p %option% --release; cp ./target/arduboy/release/libeeprom_byte.a ./arduboy-rust/Wrapper-Project/lib/libgame.a; cd arduboy-rust/Wrapper-Project/; pio run -v -t upload; cp ./.pio/build/arduboy/firmware.hex ./build/%option%.hex; pio run -t clean; rm lib/libgame.a; cd ../../"
   goto :eof
) else (
    goto :run
)

:run
powershell -Command "$ErrorActionPreference='Stop'; cargo build -p %option% --release; cp ./target/arduboy/release/lib%option%.a ./arduboy-rust/Wrapper-Project/lib/libgame.a; cd arduboy-rust/Wrapper-Project/; pio run -v -t upload; cp ./.pio/build/arduboy/firmware.hex ./build/%option%.hex; pio run -t clean; rm lib/libgame.a; cd ../../"
if ERRORLEVEL 1 (
   goto :help
)
goto :eof

:help
@echo Usage: .\run.bat                // for uploading your game
@echo Usage: .\run.bat ^<Example Game^> // for uploading an example game