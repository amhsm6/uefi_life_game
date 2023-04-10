#![no_std]
#![no_main]

extern crate alloc;

use alloc::vec;
use alloc::collections::BTreeSet;
use uefi::prelude::*;
use uefi::proto::{
    console::gop::{GraphicsOutput, BltOp, BltPixel, BltRegion},
    console::text::Color
};
use uefi_services::{print, println};

fn init_creatures() -> BTreeSet<(i32, i32)> {
    let mut alive = BTreeSet::new();

    alive.insert((50, 50));
    alive.insert((51, 51));
    alive.insert((51, 52));
    alive.insert((50, 52));
    alive.insert((49, 52));

    alive.insert((54, 54));
    alive.insert((55, 54));
    alive.insert((56, 54));
    alive.insert((56, 55));
    alive.insert((56, 56));
    alive.insert((55, 56));
    alive.insert((54, 56));
    alive.insert((53, 53));

    alive.insert((56, 56));
    alive.insert((57, 57));
    alive.insert((58, 57));
    alive.insert((58, 57));

    alive.insert((81, 81));
    alive.insert((80, 80));
    alive.insert((79, 80));
    alive.insert((79, 81));
    alive.insert((79, 82));

    alive.insert((88, 81));
    alive.insert((87, 80));
    alive.insert((86, 80));
    alive.insert((86, 81));
    alive.insert((86, 82));

    alive.insert((88, 75));
    alive.insert((87, 74));
    alive.insert((86, 74));
    alive.insert((86, 75));
    alive.insert((86, 76));

    alive.insert((98, 85));
    alive.insert((99, 84));
    alive.insert((97, 84));
    alive.insert((97, 85));
    alive.insert((97, 86));

    alive
}

#[entry]
fn main(image: Handle, mut system_table: SystemTable<Boot>) -> Status {
    uefi_services::init(&mut system_table).unwrap();

    {
        let stdout = system_table.stdout();
        stdout.clear().unwrap();
        stdout.set_color(Color::Yellow, Color::Black).unwrap();
    }

    let boot_services = system_table.boot_services();

    print!("Initializing Graphics...");
    let gop_handle = boot_services.get_handle_for_protocol::<GraphicsOutput>().unwrap();
    let mut gop = boot_services.open_protocol_exclusive::<GraphicsOutput>(gop_handle).unwrap();

    let (width, height) = gop.current_mode_info().resolution();
    println!("Success\nResolution: {}x{}", width, height);

    let mut alive = init_creatures();

    loop {
        gop.blt(BltOp::VideoFill {
            color: BltPixel::new(255, 255, 255),
            dest: (0, 0),
            dims: (width, height)
        }).unwrap();
        
        for &(x, y) in &alive {
            gop.blt(BltOp::BufferToVideo {
                buffer: &vec![BltPixel::new(0, 0, 0); 25],
                src: BltRegion::Full,
                dest: (x as usize * 5, y as usize * 5),
                dims: (5, 5)
            }).unwrap();
        }
        
        let mut to_make_alive = vec![];
        let mut to_make_dead = vec![];

        for a in &alive {
            let mut to_make_dead_count = 0;

            for (dx, dy) in [
                (1, 1), (1, 0), (1, -1),
                (0, 1), (0, -1),
                (-1, 1), (-1, 0), (-1, -1)
            ] {
                let x_new = a.0 + dx;
                let y_new = a.1 + dy;
                
                if alive.contains(&(x_new, y_new)) {
                    to_make_dead_count += 1;
                }

                let mut to_make_alive_count = 0;

                if !alive.contains(&(x_new, y_new)) {
                    for (dx2, dy2) in [
                        (1, 1), (1, 0), (1, -1),
                        (0, 1), (0, -1),
                        (-1, 1), (-1, 0), (-1, -1)
                    ] {
                        if alive.contains(&(x_new + dx2, y_new + dy2)) {
                            to_make_alive_count += 1;
                        }
                    }
                }

                if to_make_alive_count == 3 {
                    to_make_alive.push((x_new, y_new));
                }
            }

            if to_make_dead_count < 2 || to_make_dead_count > 3 {
                to_make_dead.push((a.0, a.1));
            }
        }

        for (x, y) in to_make_alive {
            alive.insert((x, y));
        }

        for (x, y) in to_make_dead {
            alive.remove(&(x, y));
        }

        boot_services.stall(20_000);
    }

    Status::SUCCESS
}
