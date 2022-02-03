I'm using ssd1306 crate as the driver lib.

I'd say it's working perfectly well. The model of the oled screen on XIAO expansion board is not ssd1306 but should be compatible.

Firstly it's not working, but I found out I should lower the I2C frequency. It's not working at 400kHz (fast I2C mode), but works at 100kHz (maybe called normal mode). I tried with 350kHz, it's not reporting error but the screen is not working, maybe it has got stuck because I conifgured 400kHz just before.

Just mentioning, the type-level gpio of rp2040-hal is pretty interesting.

1. It precisely corrected me that I'm using I2C1 pins but with PAC I2C0. (It just don't type-check)
2. But it's pretty hard to abstract your code now, for example, when I try to configure 3 pins for my 3 color led, the 3 pins are of 3 distinct types. So it becomes extremely hard to factor out that piece of code into a function. How should you give the pin arguments a type or types. How specific should you type the pins.

I'd like to bound it with OutputPin trait in embedded_hal (which is kind of a standard across chips), but I have to type it like following:

```rust
fn configure_led<P1,P2,P3,E>(p1: &mut P1, p2: &mut P2, p3: &mut P3) -> Result<(), E>
where P1: OutputPin<Error = E>, P2: OutputPin<Error = E>, P3: OutputPin<Error = E>
{
    // do something
    Ok(())
}
```
`Pn` instead of a single `P` because each pin is of a distinct type.