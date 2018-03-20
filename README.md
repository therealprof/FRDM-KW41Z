FRDM-KW41Z
==========

_FRDM-KW41Z_ contains everything required to get started with the use of Rust
to create firmwares for the NXP [FRDM-KW41Z][] microcontroller board.

This little board contains has a lot of peripherals built in, most prominently
a 802.15.4 and BLE radio and a few more peripherals connected to GPIOs, ADC,
I2C and SPI. It also includes a capable debugging interface which can be used
either with the default J-Link firmware or with a DAPLink firmware, the latter
is easier to interface with and thus assumed by the supplied OpenOCD (later
than the latest released 0.10.0 version only!) based programming scripts.

The [FRDM-KW41Z mbed OS][] page contains a useful schematic and description of the available
peripherals.

[FRDM-KW41Z]:(https://www.nxp.com/products/processors-and-microcontrollers/arm-based-processors-and-mcus/kinetis-cortex-m-mcus/w-serieswireless-conn.m0-plus-m4/freedom-development-kit-for-kinetis-kw41z-31z-21z-mcus:FRDM-KW41Z)
[FRDM-KW41Z mbedOS]:(https://os.mbed.com/platforms/FRDM-KW41Z/)
[cortex-m]:(https://github.com/japaric/cortex-m)
[cortex-m-rt]:(https://github.com/japaric/cortex-m-rt)

License
-------

[0-clause BSD license](LICENSE-0BSD.txt).
