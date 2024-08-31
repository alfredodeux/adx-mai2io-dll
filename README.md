# Yuancon ADX Controller mai2io.dll
This is a test to see whether we can improve button delay by bypassing segatools io4/hid implementation. This uses the segatools mai2io API which can be found in `segatools/mai2io/mai2io.h`

The `bin` directory provides prebuilt executables and dll. 

The `pytests` directory provides simple examples of reading from the buttons and touch sensor with python.

Notes:
- Controller uses 2 usb ports. The front is a composite USB device featuring 3 device classes. The rear is power and can be plugged into wall. 
- Touch is USB Serial COM (usually COM3) and outputs every 1ms (average)
- LED is USB Serial COM (usually COM21). Commands can be found in segatools
- Button is HID and the VID/PID changes depending on the firmware flashed. IO4 firmware outputs every 2ms (average) while HID firmware polls every 100ms but sends data faster when button pressed

Conclusions:
- does not help with delay, makes it worse actually because the buttons seem to be polled by frame
- computer runs much cooler and I experience less stutter
- io4 firmware is slightly faster and more consistent than hid
- 2p ADX can be easily setup using the principles in this repo

Open Questions:
- can we add a callback to the mai2io button function to poll faster?
- can we add a delay to the touch sensor within segatools to synchronize touch/button timing?
- are we dealing with a different issue? Some users report fast buttons depending on setup such as screen refresh rate. 

This is created by Rust but there are many `unsafe` is used. Memory leaks may appear and the stability still need to be tested.

## Setup Instruction
1. Find out whether you are using hid or io4 firmware
2. Place `adx-{io4/hid}-server.exe` and `adx_mai2io.dll` into folder with start.bat.
3. Add this to `segatools.ini`
    ```ini
    [mai2io]
    path=adx_mai2io.dll
    ```
4. Make sure that the enable line is NOT INCLUDED under [io4] in `segatools.ini`. Both enable=0 and enable=1 will not work. 
5. Modify your `start.bat` to launch the button server before injection:
    ```cmd
    @echo off
    cd /d %~dp0

    start adx-{io4/hid}-server.exe

    timeout 3

    start "AM Daemon" /min inject -d -k mai2hook.dll amdaemon.exe -f -c config_common.json config_server.json config_client.json
    inject -d -k mai2hook.dll sinmai -screen-fullscreen 0 -popupwindow -screen-width 2160 -screen-height 1920  -silent-crashes
    taskkill /f /im amdaemon.exe > nul 2>&1

    echo.
    echo Game processes have terminated
    pause
    ```

## Usage Instruction
1. Connect ADX with either IO4 or HID firmware
2. Start `adx-{io4/hid}-server.exe` (Don't need to start it seperately if you already modified the `start.bat`)
3. Start the game. You should see a message that the custom dll loaded. 
4. Test that it works by pressing buttons. If buttons stop working after shutting down button server then it is working. 
---

## Compile Instruction
For `adx_mai2io.dll` and `adx-{io4/hid}-server.exe`:

`cargo build --target=x86_64-pc-windows-msvc --release`

---

## Acknowledgements
This is based on work from https://github.com/caxerx/chuniio_tasoller_new
