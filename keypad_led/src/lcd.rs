use crate::i2c;

const LCD_ADDR: u8 = 0x27; // try 0x3F if 0x27 doesn't work

const RS_BIT: u8 = 0x01;
const EN_BIT: u8 = 0x04;
const BACKLIGHT: u8 = 0x08;

fn delay(cycles: u32) {
    for _ in 0..cycles {
        unsafe { core::arch::asm!("nop"); }
    }
}

fn write4(nibble: u8, rs: bool) {
    let mut data = (nibble << 4) & 0xF0;
    data |= BACKLIGHT;
    if rs { data |= RS_BIT; }

    i2c::i2c1_write(LCD_ADDR, &[data | EN_BIT]);
    delay(500);
    i2c::i2c1_write(LCD_ADDR, &[data & !EN_BIT]);
    delay(500);
}

fn send_byte(byte: u8, rs: bool) {
    write4(byte >> 4, rs);
    write4(byte & 0x0F, rs);
}

pub fn lcd_command(cmd: u8) {
    send_byte(cmd, false);
}

pub fn lcd_data(data: u8) {
    send_byte(data, true);
}

pub fn lcd_init() {
    i2c::i2c1_init();
    delay(5_000_000); // power-on wait

    write4(0x03, false);
    delay(200_000);
    write4(0x03, false);
    delay(50_000);
    write4(0x03, false);
    delay(50_000);
    write4(0x02, false); // switch to 4-bit mode

    lcd_command(0x28); // function set: 4-bit, 2 lines, 5x8 font
    lcd_command(0x0C); // display on, cursor off
    lcd_command(0x06); // entry mode: increment cursor
    lcd_command(0x01); // clear display
    delay(200_000);
}

pub fn lcd_set_cursor(row: u8, col: u8) {
    let addr = if row == 0 { 0x80 + col } else { 0xC0 + col };
    lcd_command(addr);
}

pub fn lcd_print(text: &str) {
    for c in text.bytes() {
        lcd_data(c);
    }
}