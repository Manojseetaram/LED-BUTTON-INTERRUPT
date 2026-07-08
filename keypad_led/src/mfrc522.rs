use crate::spi;
use crate::gpio;

const COMMAND_REG: u8   = 0x01;
const COM_IRQ_REG: u8   = 0x04;
const ERROR_REG: u8     = 0x06;
const FIFO_DATA_REG: u8 = 0x09;
const FIFO_LEVEL_REG: u8= 0x0A;
const BIT_FRAMING_REG: u8 = 0x0D;
const MODE_REG: u8      = 0x11;
const TX_CONTROL_REG: u8= 0x14;
const TX_ASK_REG: u8    = 0x15;
const T_MODE_REG: u8    = 0x2A;
const T_PRESCALER_REG: u8 = 0x2B;
const T_RELOAD_H_REG: u8  = 0x2C;
const T_RELOAD_L_REG: u8  = 0x2D;
const VERSION_REG: u8   = 0x37;

const CMD_IDLE: u8      = 0x00;
const CMD_TRANSCEIVE: u8= 0x0C;
const CMD_SOFT_RESET: u8= 0x0F;

const PICC_REQA: u8     = 0x26;
const PICC_SEL_CL1: u8  = 0x93;

pub static mut UID: [u8; 4] = [0; 4];

fn write_reg(reg: u8, val: u8) {
    spi::cs_select();
    spi::spi1_transfer((reg << 1) & 0x7E);
    spi::spi1_transfer(val);
    spi::cs_deselect();
}

fn read_reg(reg: u8) -> u8 {
    spi::cs_select();
    spi::spi1_transfer(((reg << 1) & 0x7E) | 0x80);
    let val = spi::spi1_transfer(0x00);
    spi::cs_deselect();
    val
}

fn set_bit_mask(reg: u8, mask: u8) {
    let cur = read_reg(reg);
    write_reg(reg, cur | mask);
}
fn clear_bit_mask(reg: u8, mask: u8) {
    let cur = read_reg(reg);
    write_reg(reg, cur & !mask);
}

pub fn mfrc522_init() {
    spi::spi1_init();

    gpio::enable_gpio_clock(crate::mcu::GPIOB_BASE);
    gpio::set_gpio_mode_output(crate::mcu::GPIOB_BASE, 0);
    gpio::set_gpio_output_type_push_pull(crate::mcu::GPIOB_BASE, 0);
    gpio::set_gpio_pin_state(crate::mcu::GPIOB_BASE, 0, gpio::PinState::Hight);

    write_reg(COMMAND_REG, CMD_SOFT_RESET);
    for _ in 0..100000 { unsafe { core::arch::asm!("nop"); } }

    write_reg(T_MODE_REG, 0x80);
    write_reg(T_PRESCALER_REG, 0xA9);
    write_reg(T_RELOAD_H_REG, 0x03);
    write_reg(T_RELOAD_L_REG, 0xE8);
    write_reg(TX_ASK_REG, 0x40);
    write_reg(MODE_REG, 0x3D);

    set_bit_mask(TX_CONTROL_REG, 0x03);
}

pub fn read_version() -> u8 {
    read_reg(VERSION_REG)
}

fn transceive(send_data: &[u8], back_data: &mut [u8], back_len: &mut usize) -> bool {
    write_reg(COMMAND_REG, CMD_IDLE);
    write_reg(FIFO_LEVEL_REG, 0x80);
    write_reg(COM_IRQ_REG, 0x7F);

    for &b in send_data {
        write_reg(FIFO_DATA_REG, b);
    }
    write_reg(COMMAND_REG, CMD_TRANSCEIVE);
    set_bit_mask(BIT_FRAMING_REG, 0x80);

    let mut count: u32 = 0;
    loop {
        let irq = read_reg(COM_IRQ_REG);
        if irq & 0x30 != 0 { break; }
        count += 1;
        if count > 2000 { return false; }
    }
    clear_bit_mask(BIT_FRAMING_REG, 0x80);

    let err = read_reg(ERROR_REG);
    if err & 0x1B != 0 { return false; }

    let n = read_reg(FIFO_LEVEL_REG) as usize;
    let n = n.min(back_data.len());
    for i in 0..n {
        back_data[i] = read_reg(FIFO_DATA_REG);
    }
    *back_len = n;
    true
}

fn halt_card() {
    let mut halt_resp = [0u8; 2];
    let mut halt_len = 0;
    let _ = transceive(&[0x50, 0x00], &mut halt_resp, &mut halt_len);
}

pub fn read_card_uid() -> Option<[u8; 4]> {
    write_reg(BIT_FRAMING_REG, 0x07);
    let mut atqa = [0u8; 2];
    let mut len = 0;
    if !transceive(&[PICC_REQA], &mut atqa, &mut len) {
        return None;
    }

    write_reg(BIT_FRAMING_REG, 0x00);
    let mut uid_resp = [0u8; 5];
    let mut resp_len = 0;
    if !transceive(&[PICC_SEL_CL1, 0x20], &mut uid_resp, &mut resp_len) {
        return None;
    }
    if resp_len < 5 {
        return None;
    }

    let bcc = uid_resp[0] ^ uid_resp[1] ^ uid_resp[2] ^ uid_resp[3];
    if bcc != uid_resp[4] {
        rtt_target::rprintln!("BCC mismatch! Corrupted read.");
        return None;
    }

    let uid = [uid_resp[0], uid_resp[1], uid_resp[2], uid_resp[3]];
    unsafe { UID = uid; }
    halt_card();
    Some(uid)
}