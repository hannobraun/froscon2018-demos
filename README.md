# FrOSCon 2018 Embedded Rust Demos

These demos were running at the Rust booth at [FrOSCon](https://www.froscon.de/) 2018. I've uploaded them for anyone interested in looking at the code. Please be aware that all of the demos were quickly hacked together and their code isn't of the highest quality.

To run them, you need:
- A nightly version of Rust (last tested with `1.30.0-nightly (f7202e40f 2018-08-27)`)
- arm-none-eabi-gdb (last tested with `8.1.1`)
- openocd (last tested with `0.10.0`)

To run the [LPCXpresse824-MAX](https://www.nxp.com/support/developer-resources/evaluation-and-development-boards/lpcxpresso-boards/lpcxpresso824-max-board-for-lpc82x-family-mcus:OM13071) or [STM32F0 Discovery](https://www.st.com/resource/en/data_brief/32f072bdiscovery.pdf) demos, connect the board via USB, go to the respective directory, and type `cargo run --release`. That should take care of everything!

Sorry, but I don't have instructions for the LPC82x Breadboard demo.
