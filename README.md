# A Rust driver for the Microchip LAN9252 EtherCAT SubDevice Controller (ESC)

## AliExpress dev board EEPROM dump

This is in `lan9252-aliexpress.bin`, dumped using "STlink GUI", installable with
`apt install stlink-gui`. Address starts at `0x08000000`.

## Connecting to Nucleo F429

I2C1 SCL/SDA (PB8, PB9) connect to D14 and D15, top left, marked SCL/SDA.

The firmware must be written to the F429 on the nucleo, NOT the F40whatever on the devboard! I have
made this mistake too many times!

The dev board can be fully assembled and powered to read/write the EEPROM.

The white soldered wire is SCL.
