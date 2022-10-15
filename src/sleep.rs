
use avr_device::attiny85::Peripherals;
// todo put bit combinations here
const WDT_OSC : u32 = 128_000;
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

struct WDTPrescalerVals {
    prescaler_l : u8,
    prescaler_h : bool
}
// The prescalers are numbered 0 through 9 and conveniently correspond to the resulting timing of
// WDT
impl WDTPrescalerVals{
    fn from_wdt_sleep_dur (_dur : WDTSleepDur) -> WDTPrescalerVals{
        let presc_idx = _dur as u8; 
        return WDTPrescalerVals{prescaler_l : (presc_idx & 0b111) as u8, prescaler_h : (presc_idx & 0b1000) != 0}
    }
}
#[allow(non_snake_case)]
#[avr_device::interrupt(attiny85)]
unsafe fn WDT() {
    let peripherals = Peripherals::steal();
    peripherals.CPU.mcusr.write(| w |
        w.wdrf().clear_bit()
    );
}


/// Uses the Attiny85s internal oscilator which runs at 128 kHz
fn _wdt_sleep(_dur : WDTSleepDur){
    let peripherals = unsafe { Peripherals::steal() };
    peripherals.CPU.mcusr.write(|w|w.wdrf().clear_bit());
    let prescaler_vals : WDTPrescalerVals = WDTPrescalerVals::from_wdt_sleep_dur(_dur);
    peripherals.WDT.wdtcr.write(|w|
                                    w
                                        .wdce().set_bit() // Watchdog Change Enable
    );
    
    peripherals.WDT.wdtcr.write(|w|
                                    w
                                        .wdpl().bits(prescaler_vals.prescaler_l) //
                                        .wdph().bit(prescaler_vals.prescaler_h) //
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

pub(crate) fn wdt_sleep_s(mut dur : u32){
    while dur >= 8{
        _wdt_sleep(WDTSleepDur::Sleep8S);
        dur -= 8;
    }
    if dur >= 4{
        _wdt_sleep(WDTSleepDur::Sleep4S);
        dur -= 4;
    }
    if dur >= 2{
        _wdt_sleep(WDTSleepDur::Sleep2S);
        dur -= 2;
    }
    if dur == 1{
        _wdt_sleep(WDTSleepDur::Sleep1S);
        dur -= 1;
    }
}
