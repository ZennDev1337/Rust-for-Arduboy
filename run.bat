@echo off

set option=%1
set optionpath=%2
set folderpath=
if [%optionpath%]==[] (
    set optionpath=""
)


if [%option%]==[] (
    powershell -Command "cargo build -p game --release; cp ./target/arduboy/release/libgame.a ./arduboy-rust/Wrapper-Project/lib/libgame.a; cd arduboy-rust/Wrapper-Project/; pio run -v -t upload; cp ./.pio/build/arduboy/firmware.hex ./build/game.hex; pio run -t clean; rm lib/libgame.a; cd ../../"
    goto :eof
)
if %option%==fxbuild (
    for /d /r "./Examples/" %%a in (*) do if /i %%~nxa==%optionpath% (
        set folderpath=%%a
        goto :fxbuild
    )
    for /d /r "./Project/" %%a in (*) do if /i %%~nxa==%optionpath% (
        set folderpath=%%a
        goto :fxbuild
    )
    goto :help
) else if %option%==fxupload (
      set optionpath=%2
      for /d /r "./Examples/" %%a in (*) do if /i %%~nxa==%optionpath% (
          set folderpath=%%a
          goto :fxupload
      )
      for /d /r "./Project/" %%a in (*) do if /i %%~nxa==%optionpath% (
          set folderpath=%%a
          goto :fxupload
      )
      goto :help
) else if %option%==fxall (
      set optionpath=%2
      for /d /r "./Examples/" %%a in (*) do if /i %%~nxa==%optionpath% (
          set folderpath=%%a
          goto :fxall
      )
      for /d /r "./Project/" %%a in (*) do if /i %%~nxa==%optionpath% (
          set folderpath=%%a
          goto :fxall
      )
      goto :help
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

:fxbuild
powershell -Command "$ErrorActionPreference='Stop'; python ./Tools/Arduboy-Python-Utilities/fxdata-build.py %folderpath%/fxdata/fxdata.txt"
if ERRORLEVEL 1 (
   goto :help
)
goto :eof

:fxupload
powershell -Command "$ErrorActionPreference='Stop'; python ./Tools/Arduboy-Python-Utilities/fxdata-upload.py %folderpath%/fxdata/fxdata.bin"
if ERRORLEVEL 1 (
   goto :help
)
goto :eof

:fxall
powershell -Command "$ErrorActionPreference='Stop'; python ./Tools/Arduboy-Python-Utilities/fxdata-build.py %folderpath%/fxdata/fxdata.txt"
if ERRORLEVEL 1 (
   goto :help
)
powershell -Command "$ErrorActionPreference='Stop'; python ./Tools/Arduboy-Python-Utilities/fxdata-upload.py %folderpath%/fxdata/fxdata.bin"
if ERRORLEVEL 1 (
   goto :help
)
set option=%optionpath%
goto :run


:help
@echo Usage build and upload Project:
@echo   .\run.bat                          For uploading /Project/game
@echo   .\run.bat ^<Project-Name^>           For uploading a game
@echo:
@echo Usage FX-Data build and upload:
@echo   .\run.bat fxbuild ^<Project-Name^>   Build your fxdata
@echo   .\run.bat fxupload ^<Project-Name^>  Upload your fxdata
@echo   .\run.bat fxall ^<Project-Name^>     Build and Upload your fxdata
@echo                                      and the game in one step