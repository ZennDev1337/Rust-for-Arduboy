import sys
import os
import tomllib
import subprocess
import platform

# Global Params
param1 = ""
param2 = ""
fx_mode = False
project_list = []
parameter_validation_check = False
project = []


# Functions
def get_args():
    global param1
    global param2
    global fx_mode
    if len(sys.argv) == 1:
        help()
    if len(sys.argv) > 1:
        param1 = sys.argv[1]
    if len(sys.argv) > 2:
        param2 = sys.argv[2]
        fx_mode = True


def help():
    print("Usage build and upload Project:")
    print("  run.py list                     Get a list of all Projects")
    print("  run.py new <Project-Name>       Create a new game in the Project folder")
    print("  run.py <Project-Name>           For uploading a game")
    print("")
    print("Usage FX-Data build and upload:")
    print("  run.py fxbuild <Project-Name>   Build your fxdata")
    print("  run.py fxupload <Project-Name>  Upload your fxdata")
    print("  run.py fxall <Project-Name>     Build and Upload your fxdata")
    print("                                  and the game in one step")
    sys.exit()


def get_projects():
    global project_list
    with open("Cargo.toml", "rb") as f:
        data = tomllib.load(f)
        path = data["workspace"]["members"]
        for p in path:
            name = p.split("/")[-1]
            if name == "arduboy-rust":
                continue
            project = [name, p]
            project_list.append(project)


def validate_args():
    global parameter_validation_check
    global project
    if fx_mode:
        for p in project_list:
            if param2 in p:
                parameter_validation_check = True
                project = p

    else:
        for p in project_list:
            if param1 in p:
                parameter_validation_check = True
                project = p
    if not parameter_validation_check:
        help()


def execute_options():
    if param1 == "list":
        group = ""
        for p in project_list:
            if not group == p[1].split("/")[-2]:
                group = p[1].split("/")[-2]
                print(f"\n{group}:")
            print(f"   {p[0]}")
        print("")
        sys.exit(0)
    if param1 == "new":
        create_new_project()
        sys.exit(0)
    if param1 == "doc":
        if platform.system() == "Linux":
            cmd = "cargo doc -p arduboy-rust; rm -r ./docs/doc/; cp -r ./target/arduboy/doc ./docs/"
            process = subprocess.Popen(cmd, stdout=subprocess.PIPE)

        elif platform.system() == "Windows":
            cmd = 'cargo doc -p arduboy-rust; rm -r ./docs/doc/; cp -r ./target/arduboy/doc ./docs/'
            process = subprocess.Popen(["powershell", "-Command", cmd], stdout=subprocess.PIPE)
        else:
            sys.exit(1)
        for c in iter(lambda: process.stdout.read(1), b""):
            sys.stdout.buffer.write(c)
        if process.returncode != 0:
            sys.exit(1)


def _dumps_value(value):
    if isinstance(value, bool):
        return "true" if value else "false"
    elif isinstance(value, (int, float)):
        return str(value)
    elif isinstance(value, str):
        return f'"{value}"'
    elif isinstance(value, list):
        return f"[{', '.join(_dumps_value(v) for v in value)}]"
    else:
        raise TypeError(f"{type(value).__name__} {value!r} is not supported")


def dumps(toml_dict, table=""):
    toml = []
    for key, value in toml_dict.items():
        if isinstance(value, dict):
            table_key = f"{table}.{key}" if table else key
            toml.append(f"\n[{table_key}]\n{dumps(value, table_key)}")
        else:
            toml.append(f"{key} = {_dumps_value(value)}")
    return "\n".join(toml)


def create_new_project():
    project_name = param2.replace("-", "_")
    for p in project_list:
        if p[0] == project_name:
            sys.exit(1)
    error_code = os.system(f'cargo new --vcs=none --lib ./Project/{project_name}')
    if error_code > 0:
        sys.exit(1)
    # Edit main Cargo.toml
    with open("Cargo.toml", "rb") as f:
        data = tomllib.load(f)
    data["workspace"]["members"].append(f"Project/{project_name}")
    with open("Cargo.toml", "w") as f:
        f.write(dumps(data))
    # Edit new project Cargo.toml
    with open(f"Project/{project_name}/Cargo.toml", "rb") as f:
        data = tomllib.load(f)
    newdata = {}
    newdata["package"] = data["package"]
    newdata["lib"] = {'crate-type': ["staticlib"]}
    newdata["dependencies"] = {"arduboy-rust": {"path": "../../arduboy-rust"}}
    with open(f"Project/{project_name}/Cargo.toml", "w") as f:
        f.write(dumps(newdata))
    # Edit lib.rs
    with open(f"Project/{project_name}/src/lib.rs", "w") as f:
        f.write('''#![no_std]
#![allow(non_upper_case_globals)]

//Include the Arduboy Library
#[allow(unused_imports)]
use arduboy_rust::prelude::*;

#[allow(dead_code)]
const arduboy: Arduboy2 = Arduboy2::new();

// Progmem data

// dynamic ram variables

// The setup() function runs once when you turn your Arduboy on
#[no_mangle]
pub unsafe extern "C" fn setup() {
    // put your setup code here, to run once:
}

// The loop() function repeats forever after setup() is done
#[no_mangle]
#[export_name = "loop"]
pub unsafe extern "C" fn loop_() {
    // put your main code here, to run repeatedly:
}''')
    with open(f"Project/{project_name}/config.toml", "w") as f:
        data = '''Libraries = [
    "Arduboy2",
    #"ArduboyTones",
    #"ArduboyFX",
    #"ArdVoice",
    #"Serial",
    "EEPROM",
    "Arduino",
]'''
        f.writelines(data)


def generate_import_h():
    with open(f"{project[1]}/config.toml", "rb") as f:
        data = tomllib.load(f)
        strings = ["#pragma once\n"]
        for lib in data["Libraries"]:
            strings.append(f"#define {lib}_Library ;\n")
    f = open("arduboy-rust/Wrapper-Project/src/imports.h", "w")
    f.writelines(strings)
    f.close()


def upload_to_arduboy():
    if not fx_mode:
        game_name = param1
    else:
        game_name = param2
    if platform.system() == "Linux":
        cmd = f"cargo build -p {game_name} --release; cp ./target/arduboy/release/lib{game_name}.a ./arduboy-rust/Wrapper-Project/lib/libgame.a; cd arduboy-rust/Wrapper-Project/; pio run -v -t upload; cp ./.pio/build/arduboy/firmware.hex ./build/{game_name}.hex; pio run -t clean; rm ./lib/libgame.a; cd ../../"
        process = subprocess.Popen(cmd, shell=True, stdout=subprocess.PIPE)

    elif platform.system() == "Windows":
        cmd = f'cargo build -p {game_name} --release; cp ./target/arduboy/release/lib{game_name}.a ./arduboy-rust/Wrapper-Project/lib/libgame.a; cd arduboy-rust/Wrapper-Project/; pio run -v -t upload; cp ./.pio/build/arduboy/firmware.hex ./build/{game_name}.hex; pio run -t clean; rm lib/libgame.a; cd ../../'
        process = subprocess.Popen(["powershell", "-Command", cmd], stdout=subprocess.PIPE)
    else:
        sys.exit(1)
    for c in iter(lambda: process.stdout.read(1), b""):
        sys.stdout.buffer.write(c)
    if process.returncode != 0:
        sys.exit(1)


def fx_build():
    error_code = os.system(f'python ./Tools/Arduboy-Python-Utilities/fxdata-build.py ./{project[1]}/fxdata/fxdata.txt')
    if error_code > 0:
        sys.exit(1)


def fx_upload():
    error_code = os.system(f'python ./Tools/Arduboy-Python-Utilities/fxdata-upload.py ./{project[1]}/fxdata/fxdata.bin')
    if error_code > 0:
        sys.exit(1)


def fx_commands():
    print("")
    if param1 == "fxbuild":
        fx_build()
    elif param1 == "fxupload":
        fx_upload()
    elif param1 == "fxall":
        fx_build()
        fx_upload()
        upload_to_arduboy()


# Execute Script
get_args()
get_projects()
execute_options()
validate_args()

generate_import_h()

if fx_mode:
    fx_commands()
else:
    upload_to_arduboy()
