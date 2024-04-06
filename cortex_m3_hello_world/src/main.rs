#![no_std]
#![no_main]

use cortex_m_rt::entry;
use stm32f1xx_hal::{
    pac,
    prelude::*,
    serial::{Config, Serial},
};
use core::fmt::Write; // `write` 메서드를 사용하기 위해 필요합니다.
use core::panic::PanicInfo;

#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();
    let _cp = cortex_m::Peripherals::take().unwrap();

    let mut flash = dp.FLASH.constrain();
    let rcc = dp.RCC.constrain();

    let clocks = rcc.cfgr.freeze(&mut flash.acr);

    let mut gpioa = dp.GPIOA.split();
    let tx_pin = gpioa.pa9.into_alternate_push_pull(&mut gpioa.crh);
    let rx_pin = gpioa.pa10;

    // afio의 MAPR를 사용하기 위한 초기화
    let mut afio = dp.AFIO.constrain();
    
    
    let serial = Serial::usart1(
        dp.USART1,
        (tx_pin, rx_pin),
        &mut afio.mapr,
        Config::default().baudrate(115200.bps()),
        clocks,
    );
    let (mut tx, _rx) = serial.split();

    // writeln 대신 write를 사용
    tx.write_str("Hello, world!\r\n").unwrap();

    loop {
        // 여기에 필요한 코드 추가
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}