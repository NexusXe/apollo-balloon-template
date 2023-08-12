#![feature(core_intrinsics)]

extern crate libc_print;
use libc_print::std_name::println;
//use std::io::{stdout, Write};
use core::arch::{asm, global_asm};
use apollo::{generate_packet, parameters::TOTAL_MESSAGE_LENGTH_BYTES};
mod sensors;
use core::arch::x86_64::*;
use core::intrinsics::*;

#[macro_export]
macro_rules! transmit_packet  {
    ($packet: expr, $delay: expr) => {
        assert!($packet.len() == TOTAL_MESSAGE_LENGTH_BYTES);
        for _byte in $packet.iter() {
            for _bit in 0..8 {
                match _byte & (1 << _bit) {
                    assert!(bit == 0 || bit == 1);
                    0 => {
                        no_operation(usize::pow(2, 20));
                    }
                    _ => {
                        no_operation(usize::pow(2, 20));
                    }
                }
            }
        }
    };
}


// unsafe fn crc32(data: &[u8]) -> u32 {
//     assert!(data.len() == TOTAL_MESSAGE_LENGTH_BYTES);
//     let mut crc = 0u32;
//     for byte in data {
//         crc = _mm_crc32_u8(crc, *byte);
//     }
//     return crc;
// }


// unsafe fn crc32x8(data: &[u8]) -> u64 {
//     assert!(data.len() == TOTAL_MESSAGE_LENGTH_BYTES);
//     assert_eq!(data.len() % 8, 0);
//     let mut crc = 0;
//     for chunk in data.chunks(8) {
//         let mut u64_chunk: u64 = 0;
//         for byte in chunk {
//             u64_chunk = u64_chunk | (*byte as u64) << (8 * (7 - chunk.len()));
//         }
//         crc = _mm_crc32_u64(crc, u64_chunk);
//     }
//     crc
// }


fn no_operation(count: usize) {
    for _ in 0..count {
        // core::arch::x86_64 doesn't have NOP for some reason
        // using the 64-bit noop makes the binary smaller anyway lmao
        unsafe { asm!("NOP DWORD ptr [EAX + EAX*1 + 00000000H]"); }
    }
}

pub fn get_random_packet() -> [u8; TOTAL_MESSAGE_LENGTH_BYTES] {
    unsafe { crate::sensors::init() };
    generate_packet(sensors::get_location, sensors::get_altitude, sensors::get_voltage, sensors::get_temperature)
}

fn main() {
    

    for byte in get_random_packet() {
        print!("{:02x?}", byte);
    }
    
    // let mut seed: u64 = 0;
    // let mut i: u128 = 0;
    // let mut crc: u64 = 0;

    // loop {
    //     let _packet = generate_packet(sensors::get_location, sensors::get_altitude, sensors::get_voltage, sensors::get_temperature);
    //     //transmit_packet!(_packet, delay);

    //     //unsafe { asm!("rdrand {0}", out(reg) seed) } // me when
    //     unsafe{crc = crc32x8(&_packet);}
    //     //println!("{:?}", _packet);
    //     //println!("{}", seed);
    //     //println!("{}", crc);

    //     unsafe{seed = crate::sensors::SEED;}
    //     seed;
    //     _packet;
    //     crc;
    //     i += 1;

    //     if unlikely(i % 1_000_000 == 0) {
    //         //unsafe{println!("{}, {:02x?}, {}, {}", i, _packet, crc, _mm_crc32_u64(0, seed))}
    //         println!("{}, {:02x?}, {}, {}", i, _packet, crc, seed);
    //     }
    // }
}