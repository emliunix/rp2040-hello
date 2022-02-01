# make gd25q16 boot2 works

Comparing gd25q64 and w25q080:

* the read command is 0xe7 (was 0xeb)
* WAIT_CYCLES is 2 (was 4)
** It's true it's 4 for 0xeb and 2 for 0xe7 according to gd's datasheet
* extra `CMD_WRITE_STATUS2` 0x31 (was 0x01)
* wrote 1B with `WR_ST2`, (was 2B with `WR_ST`), plus the extra command byte.
** the skipped 2nd byte is a dummy 0x00

Now looking datasheets for cmd references:

## Read Command `0xe7`

Mostly the same, `0xe7` requires `WAIT_CYCLES = 2` and address bit `A0 = 0`. So the address is 2B aligned. Not sure if 2B is a word.

## Write Status `0x31` vs `0x01`

There's no `0x31` in gd25q16, so first, the `0x01` in gd25q16, then `0x31` in gd25q64.

### `0x01` in gd25q16

It accepts 2 bytes, the low and high bytes of the status regisger. And `QE` is S9, so it's correct to write 0x00 then 0x02.

### Write Status in gd25q64

It's quite different in writing status register in gd25q64.

It has 3 commands (0x01/0x031/0x011) to write to each of the three bytes of the status register. Each command accepts single data byte.

So it's not compatible with gd25q16 nor w25q080.

## Make gd25q16 work

1. revert write status commands in the gd25q64 boot2 code.
2. (optional) issue HPM command to enable HPM (High Performance Mode).

And it's not working.

In the debug session, it successfully reached the end of boot2, and is about to jump to the main code. Then it crashes with DFSR = 0x001 - an alignment issue.

The datasheets say that both chips requires a0 = 0 for 0xa7 read command. But I guess maybe it's actually not required on gd25q64.

## Summary

I'll continue with w25q080 boot2.