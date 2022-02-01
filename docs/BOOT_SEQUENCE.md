## Boot sequence

Starting from `0x0` defines the initial SP and exception handlers, among which is the reset handler that's called on power up. That's the standard boot sequqnce of ARM Cortex-M chips. All these are hard-burnt to the chip as bootrom.

Reset handler is at `0xea`, that's why it's the `PC` I see in gdb on `reset halt`, hence it's the main entrance.

Bootrom loads 256 bytes (0x100) data from flash to 0x20041f00 (which is also the initial SP, but stack goes downwards, boot2 goes onwards) and jumps here to execute boot2. Though boot2 is stored at `0x1000_0000`.

According to disassembling, on my bootrom version (3), it's `bx r5` at `0x2578` that bootrom jumps to boot2. I have to know this becuase somehow I can't set breakpoints on boot2.

## Boot2 

I'm using the `rp2040_boot2::BOOT_LOADER_GD25Q64CS` boot2 code, which is in [crates.io|https://crates.io]. After disassembling it, found it jumps to main code at `0x20041f90`.

After some breakpoints, stepping, continuing, now I'm certain it crashes at boot2. It runs to `0x20041f00`, but failed to reach `0x200f1f90`.

I recall that the board actually works with the default pico-sdk settings. So I tried to re-program it with boot2 `w25q080`, and it works like a charm.

So the `gd25q64cs` boot2 actually is not working for `gd25q16cs`. Here're some more findings:

* Both flash chips have a `0xeb` **Quad I/O Fast Read** command and a `0xe7` **Quad I/O Word Fast Read** command
* The pi-pico defaut `w25q080` boot2 uses `0xeb` while the `gd25q64cs` boot2 uses `0xe7`.
* The status register of `gd25q64` is one byte larger than `gd25q16` indicating a *high performance mode* only available on the 64 variant.

So far I think choosing `w25q080` boot2 should be enough, maybe there're some performance enahcnements if we dedicate to gd's chip.

## breakpoints not working

I found the SRAM boot2 address at a very early stage, but I just can't break here. Instead, I found where the bootrom jumps to boot2 after copying it from flash to SRAM only because I can't break at boot2.

I'm not sure where's going wrong. It's very hard debugging. I must figure out later.
