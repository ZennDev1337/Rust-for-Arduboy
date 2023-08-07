@echo off

set option=%1

if [%option%]==[] (
    powershell -Command "cargo build -p game --release; cp ./target/arduboy/release/libgame.a ./Wrapper-Project/lib/libgame.a; cd Wrapper-Project/; pio run -v -t upload; cp ./.pio/build/arduboy/firmware.hex ./.pio/build/game.hex; pio run -t clean; rm lib/libgame.a; cd .."
    goto :eof
)

if %option%==snake (
    powershell -Command "cargo build -p snake --release; cp ./target/arduboy/release/libsnake.a ./Wrapper-Project/lib/libgame.a; cd Wrapper-Project/; pio run -v -t upload; cp ./.pio/build/arduboy/firmware.hex ./.pio/build/snake.hex; pio run -t clean; rm lib/libgame.a; cd .."
    goto :eof
) else if %option%==pong (
    powershell -Command "cargo build -p pong --release; cp ./target/arduboy/release/libpong.a ./Wrapper-Project/lib/libgame.a; cd Wrapper-Project/; pio run -v -t upload; cp ./.pio/build/arduboy/firmware.hex ./.pio/build/pong.hex; pio run -t clean; rm lib/libgame.a; cd .."
    goto :eof
) else if %option%==rustacean (
    powershell -Command "cargo build -p rustacean --release; cp ./target/arduboy/release/librustacean.a ./Wrapper-Project/lib/libgame.a; cd Wrapper-Project/; cp ./.pio/build/arduboy/firmware.hex ./.pio/build/rustacean.hex; pio run -v -t upload; pio run -t clean; rm lib/libgame.a; cd .."
    goto :eof
) else if %option%==tone (
    powershell -Command "cargo build -p tone --release; cp ./target/arduboy/release/libtone.a ./Wrapper-Project/lib/libgame.a; cd Wrapper-Project/; pio run -v -t upload; cp ./.pio/build/arduboy/firmware.hex ./.pio/build/tone.hex; pio run -t clean; rm lib/libgame.a; cd .."
    goto :eof
) else if %option%==eeprom (
    powershell -Command "cargo build -p eeprom-demo --release; cp ./target/arduboy/release/libeeprom_demo.a ./Wrapper-Project/lib/libgame.a; cd Wrapper-Project/; pio run -v -t upload; cp ./.pio/build/arduboy/firmware.hex ./.pio/build/eeprom.hex; pio run -t clean; rm lib/libgame.a; cd .."
    goto :eof
) else if %option%==progmem (
    powershell -Command "cargo build -p progmem --release; cp ./target/arduboy/release/libprogmem.a ./Wrapper-Project/lib/libgame.a; cd Wrapper-Project/; pio run -v -t upload; cp ./.pio/build/arduboy/firmware.hex ./.pio/build/progmem.hex; pio run -t clean; rm lib/libgame.a; cd .."
    goto :eof
) else if %option%==demo2 (
    powershell -Command "cargo build -p demo2 --release; cp ./target/arduboy/release/libdemo2.a ./Wrapper-Project/lib/libgame.a; cd Wrapper-Project/; pio run -v -t upload; cp ./.pio/build/arduboy/firmware.hex ./.pio/build/demo2.hex; pio run -t clean; rm lib/libgame.a; cd .."
    goto :eof
) else if %option%==demo3 (
    powershell -Command "cargo build -p demo3 --release; cp ./target/arduboy/release/libdemo3.a ./Wrapper-Project/lib/libgame.a; cd Wrapper-Project/; pio run -v -t upload; cp ./.pio/build/arduboy/firmware.hex ./.pio/build/demo3.hex; pio run -t clean; rm lib/libgame.a; cd .."
    goto :eof
) else if %option%==demo4 (
    powershell -Command "cargo build -p demo4 --release; cp ./target/arduboy/release/libdemo4.a ./Wrapper-Project/lib/libgame.a; cd Wrapper-Project/; pio run -v -t upload; cp ./.pio/build/arduboy/firmware.hex ./.pio/build/demo4.hex; pio run -t clean; rm lib/libgame.a; cd .."
    goto :eof
) else if %option%==demo5 (
    powershell -Command "cargo build -p demo5 --release; cp ./target/arduboy/release/libdemo5.a ./Wrapper-Project/lib/libgame.a; cd Wrapper-Project/; pio run -v -t upload; cp ./.pio/build/arduboy/firmware.hex ./.pio/build/demo5.hex; pio run -t clean; rm lib/libgame.a; cd .."
    goto :eof
) else if %option%==demo6 (
    powershell -Command "cargo build -p demo6 --release; cp ./target/arduboy/release/libdemo6.a ./Wrapper-Project/lib/libgame.a; cd Wrapper-Project/; pio run -v -t upload; cp ./.pio/build/arduboy/firmware.hex ./.pio/build/demo6.hex; pio run -t clean; rm lib/libgame.a; cd .."
    goto :eof
)

@echo Usage: .\run.bat                // for uploading your game
@echo Usage: .\run.bat ^<Example Game^> // for uploading an example game