fn part(input: &str, p1: bool) -> u32 {
    let b = input.as_bytes();
    let xsize = memchr::memchr(b'\n', b).unwrap();
    let mut nodes = vec![0u8; 128 * 16 * 2]; // max 16 (y,x) pairs for each char
    let mut counts = vec![0u8; 128];
    let mut y = 0;
    for chunk in b.chunks(xsize + 1) {
        for (x, &ch) in chunk[0..chunk.len() - 1].iter().enumerate() {
            if ch != b'.' {
                let iidx = (ch - b'0') as usize;
                let curr = counts[iidx];
                counts[iidx] = curr + 1;
                let idx = iidx * 32 + (curr as usize) * 2;
                nodes[idx] = y as u8;
                nodes[idx + 1] = x as u8;
            }
        }
        y += 1;
    }
    let mut bits = vec![0u64; 1 + y * xsize / 8];
    let ysize = y as i32;
    let xsize = xsize as i32;
    for (k, &ck) in counts.iter().enumerate() {
        if ck == 0 {
            continue;
        }
        let kk = k as usize * 32;
        for i in 0..ck - 1 {
            for j in i + 1..ck {
                let ii = kk + (i as usize) * 2;
                let jj = kk + (j as usize) * 2;
                let (ay, ax) = (nodes[jj] as i32, nodes[jj + 1] as i32);
                let (by, bx) = (nodes[ii] as i32, nodes[ii + 1] as i32);
                let (dy, dx) = (ay - by, ax - bx);
                if p1 {
                    for m in [-2, 1] {
                        let ny = ay + m * dy;
                        if ny < 0 || ny >= ysize {
                            continue;
                        }
                        let nx = ax + m * dx;
                        if nx >= 0 && nx < xsize {
                            let pos = (ny * xsize + nx) as usize;
                            unsafe {
                                *bits.get_unchecked_mut(pos / 8) |= 1u64 << (pos % 8);
                            };
                        }
                    }
                } else {
                    for d in [-1, 1] {
                        for i in 0..ysize - 1 {
                            let ny = ay + d * i * dy;
                            if ny < 0 || ny >= ysize {
                                break;
                            }
                            let nx = ax + d * i * dx;
                            if nx < 0 || nx >= xsize {
                                break;
                            }
                            let pos = (ny * xsize + nx) as usize;
                            unsafe {
                                *bits.get_unchecked_mut(pos / 8) |= 1u64 << (pos % 8);
                            };
                        }
                    }
                    let pos = (by * xsize + bx) as usize;
                    unsafe {
                        *bits.get_unchecked_mut(pos / 8) |= 1u64 << (pos % 8);
                    };
                }
            }
        }
    }
    bits.iter().map(|b| b.count_ones()).sum()
}

pub fn part1(input: &str) -> u32 {
    part(input, true)
}

pub fn part2(input: &str) -> u32 {
    part(input, false)
}

pub const INPUT: &str = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";

//pub const INPUT: &str = include_str!("../input/d8.txt");
