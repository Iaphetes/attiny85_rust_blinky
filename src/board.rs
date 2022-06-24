pub(crate) fn board_setup(){
    unsafe {
        llvm_asm!("sei");
    }
}