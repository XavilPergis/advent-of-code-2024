use crate::{bitset::Bitset, prelude::*};

pub fn add_variants(repo: &mut RunnerRepository) {
    repo.add_variant("part1", part1);
    repo.add_variant("part2", part2);
}

fn parse_number(text: &[u8], cur: &mut usize) -> i32 {
    let mut res = 0;
    let mut mul = 1;
    if *cur < text.len() && text[*cur] == b'-' {
        *cur += 1;
        mul = -1;
    }
    while *cur < text.len() && text[*cur].is_ascii_digit() {
        res *= 10;
        res += (text[*cur] - b'0') as i32;
        *cur += 1;
    }
    mul * res
}

const MAP_WIDTH: i32 = 101;
const MAP_HEIGHT: i32 = 103;

fn part1(ctx: &mut RunContext) -> eyre::Result<impl Display> {
    let mut q1 = 0;
    let mut q2 = 0;
    let mut q3 = 0;
    let mut q4 = 0;
    for line in ctx.input.lines() {
        let line = line.as_bytes();
        let mut cur = 0;
        cur += 2; // "p="
        let x = parse_number(line, &mut cur);
        cur += 1; // ","
        let y = parse_number(line, &mut cur);
        cur += 3; // " v="
        let vx = parse_number(line, &mut cur);
        cur += 1; // ","
        let vy = parse_number(line, &mut cur);

        let final_x = (x + 100 * vx).rem_euclid(MAP_WIDTH);
        let final_y = (y + 100 * vy).rem_euclid(MAP_HEIGHT);

        if final_x < (MAP_WIDTH - 1) / 2 && final_y < (MAP_HEIGHT - 1) / 2 {
            q1 += 1;
        }
        if final_x > (MAP_WIDTH - 1) / 2 && final_y < (MAP_HEIGHT - 1) / 2 {
            q2 += 1;
        }
        if final_x < (MAP_WIDTH - 1) / 2 && final_y > (MAP_HEIGHT - 1) / 2 {
            q3 += 1;
        }
        if final_x > (MAP_WIDTH - 1) / 2 && final_y > (MAP_HEIGHT - 1) / 2 {
            q4 += 1;
        }
    }

    Ok((q1 * q2 * q3 * q4) as u64)
}

// ALL HAIL THE CUBE

// f f f f f f f f 0 0 0 0 0 0 0 6
// 0 0 0 0 0 0 0 c 0 0 0 0 0 0 1 8
// 0 0 0 0 0 0 3 0 0 0 2 0 0 0 6 0
// 0 0 e 0 0 0 c 0 0 3 e 0 0 1 8 0
// 0 f e 0 0 3 0 0 3 f e 0 0 6 0 0
// 1 f 0 0 0 c 0 0 7 f 0 0 1 8 0 1
// f f 0 0 3 0 0 7 f f 0 0 6 0 1 f
// f f 0 0 c 0 0 f f 8 0 1 8 0 3 f
// f 8 0 3 0 0 f f f 8 0 6 0 3 f f
// f 8 0 c 0 f f f f 8 1 8 0 7 f f
// c 0 3 0 1 f f f c 0 6 0 7 f f f
// c 0 c 1 f f f f c 1 8 7 f f f f
// c 3 0 0 0 7 0 0 0 6 0 0 0 e 0 0
// 0 c 0 0 1 c 0 0 1 8 0 0 0 0 0 0
// 3 0 0 0 0 0 0 0 6 0 0 0 0 0 0 0
// c 0 0 0 0 0 0 1 f f f f f f f e

// ###############################
// #                             #
// #                             #
// #                             #
// #                             #
// #              #              #
// #             ###             #
// #            #####            #
// #           #######           #
// #          #########          #
// #            #####            #
// #           #######           #
// #          #########          #
// #         ###########         #
// #        #############        #
// #          #########          #
// #         ###########         #
// #        #############        #
// #       ###############       #
// #      #################      #
// #        #############        #
// #       ###############       #
// #      #################      #
// #     ###################     #
// #    #####################    #
// #             ###             #
// #             ###             #
// #             ###             #
// #                             #
// #                             #
// #                             #
// #                             #
// ###############################

// const TREE_WIDTH: usize = 31;
// const TREE_HEIGHT: usize = 33;
// const TREE_BITS: [u64; 16] = [
//     0xffffffff00000006,
//     0x0000000c00000018,
//     0x0000003000200060,
//     0x00e000c003e00180,
//     0x0fe003003fe00600,
//     0x1f000c007f001801,
//     0xff003007ff00601f,
//     0xff00c00ff801803f,
//     0xf80300fff80603ff,
//     0xf80c0ffff81807ff,
//     0xc0301fffc0607fff,
//     0xc0c1ffffc187ffff,
//     0xc300070006000e00,
//     0x0c001c0018000000,
//     0x3000000060000000,
//     0xc0000001fffffffe,
// ];

fn part2(ctx: &mut RunContext) -> eyre::Result<impl Display> {
    #[derive(Copy, Clone, Debug)]
    struct Robot {
        x: i32,
        y: i32,
        vx: i32,
        vy: i32,
    }

    let mut map = Bitset::new((MAP_WIDTH * MAP_HEIGHT) as usize);
    let mut robots = vec![];

    for line in ctx.input.lines() {
        let line = line.as_bytes();
        let mut cur = 0;
        cur += 2; // "p="
        let x = parse_number(line, &mut cur);
        cur += 1; // ","
        let y = parse_number(line, &mut cur);
        cur += 3; // " v="
        let vx = parse_number(line, &mut cur);
        cur += 1; // ","
        let vy = parse_number(line, &mut cur);

        robots.push(Robot { x, y, vx, vy });
    }

    for step in 0..10000 {
        map.clear_all();
        for robot in &mut robots {
            map.set((MAP_WIDTH * robot.y + robot.x) as usize);
            robot.x += robot.vx;
            robot.y += robot.vy;

            if robot.x >= MAP_WIDTH {
                robot.x -= MAP_WIDTH;
            }
            if robot.x < 0 {
                robot.x += MAP_WIDTH;
            }
            if robot.y >= MAP_HEIGHT {
                robot.y -= MAP_HEIGHT;
            }
            if robot.y < 0 {
                robot.y += MAP_HEIGHT;
            }
        }

        let mut contig = 0;
        let mut prev = false;
        for i in 0..(MAP_WIDTH * MAP_HEIGHT) as usize {
            let cur = map.get(i);
            if prev && cur {
                contig += 1;
            } else {
                contig = 0;
            }

            if contig >= 30 {
                println!(
                    "{step}:\n{:?}",
                    crate::bitset::DebugBitset(&map, MAP_WIDTH as usize, MAP_HEIGHT as usize)
                );
            }

            prev = cur;
        }
    }

    todo!() as eyre::Result<i32>
}
