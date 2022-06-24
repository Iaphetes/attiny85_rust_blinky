
use avr_device::attiny85::Peripherals;
// todo put bit combinations here
pub(crate) enum WDTSleepDur {
    Sleep16Ms,
    Sleep32Ms,
    Sleep64Ms,
    Sleep125Ms,
    Sleep250Ms,
    Sleep500Ms,
    Sleep1S,
    Sleep2S,
    Sleep4S,
    Sleep8S
}

#[allow(non_snake_case)]
#[avr_device::interrupt(attiny85)]
unsafe fn WDT() {
    let peripherals = Peripherals::steal();
    peripherals.CPU.mcusr.write(| w |
        w.wdrf().clear_bit()
    );
}
pub(crate) fn wdt_sleep(_dur : WDTSleepDur){
    let peripherals = unsafe { Peripherals::steal() };
    peripherals.CPU.mcusr.write(|w|w.wdrf().clear_bit());

    peripherals.WDT.wdtcr.write(|w|
                                    w
                                        .wdce().set_bit() // Watchdog Change Enable
    );

    peripherals.WDT.wdtcr.write(|w|
                                    w
                                        .wdpl().cycles_256k() //
                                        .wdie().set_bit() //
    );
    peripherals.CPU.mcucr.write(|w|
        w
            .sm().pdown()
            .se().set_bit()
    );
    unsafe {
        llvm_asm!("sleep");
    }
}