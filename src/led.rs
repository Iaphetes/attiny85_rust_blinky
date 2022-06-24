use avr_device::attiny85::Peripherals;
//todo better solution for steal?
pub fn led_01_setup(){
    let peripherals = unsafe { Peripherals::steal() };
    let portb = peripherals.PORTB;
    portb.ddrb.write(|w| w.pb1().set_bit());
}

pub fn led_01_set(enable : bool){
    let peripherals = unsafe { Peripherals::steal() };
    let portb = peripherals.PORTB;
    if enable {
        portb.portb.write(|w| w.pb1().set_bit());
    }else{
        portb.portb.write(|w| w.pb1().clear_bit());
    }
}