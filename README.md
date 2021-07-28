# Working with STM32F3's LEDs using Registers
This template pertains to turning LEDS ON/OFF using register's address.

## Steps to run this program
* Firstly we need to launch openocd suing this:
  * `cd /tmp
    openocd -f interface/stlink-v2-1.cfg -f target/stm32f3x.cfg
    `
* The second step is to run the program using:
  * `cargo run`
* This command will take us to the **gdb**, now we need to the details of the target using:
  * `target remote :333`
* Now we are ready to load our binary to STM32F3 using:
  * `load`
* After loading the program, we just to need to run the program inside STM32F3 using:
  * continue

After all these steps the LEDs(D=LD3 & LD4) will start blinking on the board.

To more on this [click here](https://blog.knoldus.com/embedded-rust-turning-leds-on-off-using-registers/). 



