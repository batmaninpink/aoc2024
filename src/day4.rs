pub fn run(input: &str) -> u32 {
    part1(input)
}

fn get(m: &Vec<&[u8]>, y: i32, x: i32) -> u8 {
    if y < 0 || x < 0 {
        return b' ';
    }
    let y = y as usize;
    let x = x as usize;
    if y >= m.len() || x >= m[y].len() {
        return b' ';
    }
    unsafe { *m[y].get_unchecked(x) }
}

fn check1x(m: &Vec<&[u8]>, y: i32, x: i32, dx: i32) -> bool {
    get(m, y - 1, x + dx) == b'M'
        && get(m, y - 2, x + 2 * dx) == b'A'
        && get(m, y - 3, x + 3 * dx) == b'S'
}

fn check1s(m: &Vec<&[u8]>, y: i32, x: i32, dx: i32) -> bool {
    get(m, y - 1, x + dx) == b'A'
        && get(m, y - 2, x + 2 * dx) == b'M'
        && get(m, y - 3, x + 3 * dx) == b'X'
}

pub fn part1(input: &str) -> u32 {
    let mut count = 0;
    let m = input
        .lines()
        .map(|line| line.as_bytes())
        .collect::<Vec<_>>();
    let ylen = m.len() as i32;
    let xlen = m[0].len() as i32;
    for y in 0..ylen {
        for x in 0..xlen {
            let c = unsafe { *m[y as usize].get_unchecked(x as usize) };
            if c == b'X' {
                for dx in [-1, 0, 1] {
                    if check1x(&m, y, x, dx) {
                        count += 1;
                    }
                }
                if x < xlen - 3 {
                    let x = x as usize;
                    if &m[y as usize][x + 1..x + 4] == b"MAS" {
                        count += 1;
                    }
                }
            } else if c == b'S' {
                for dx in [-1, 0, 1] {
                    if check1s(&m, y, x, dx) {
                        count += 1;
                    }
                }
                if x < xlen - 3 {
                    let x = x as usize;
                    if &m[y as usize][x + 1..x + 4] == b"AMX" {
                        count += 1;
                    }
                }
            }
        }
    }
    count
}

fn get2(m: &[u8], ylen: i32, xlen: i32, y: i32, x: i32) -> u8 {
    if y < 0 || x < 0 || y >= ylen || x >= xlen {
        // This check is not needed, but ends up being faster on my machine
        return b' ';
    }
    let idx = (xlen + 1) * y + x;
    unsafe { *m.get_unchecked(idx as usize) }
}

fn check2(m: &[u8], ylen: i32, xlen: i32, y: i32, x: i32) -> bool {
    if get2(m, ylen, xlen, y, x) == b'A' {
        let d11 = get2(m, ylen, xlen, y - 1, x - 1);
        let d22 = get2(m, ylen, xlen, y - 1, x + 1);
        let d12 = get2(m, ylen, xlen, y + 1, x + 1);
        let d21 = get2(m, ylen, xlen, y + 1, x - 1);
        let d1 = d11 + d12;
        let d2 = d21 + d22;
        return d1 == 160 && d2 == 160;
    }
    false
}

pub fn part2(input: &str) -> u32 {
    let mut count = 0;
    let m = input.trim().as_bytes();
    let xlen = memchr::memchr(b'\n', m).unwrap() as i32;
    let ylen = m.len() as i32 / xlen;
    for y in 1..ylen - 1 {
        for x in 1..xlen - 1 {
            if check2(&m, ylen, xlen, y, x) {
                count += 1;
            }
        }
    }
    count
}

pub const INPUT: &str = "MSSMMSXSMSMSXMXXXMSMSSMSAMXAXASXSASMSMXMXMMAMSMMMSMSXSAMXSSXMASMMMXXMASXMSXMSSXXXAMMXMXSMSAMAAASAMXMMSSSXXSMMXXXSMXSXMXMMXXMASMMMMSXXMAXXMAS
XAAAAAXMASMSASXMAMAAMXAAMSSXSAMASASAAAXSAMXAMAAASAASAMMSMMMAXAXAMSMMAAAAAXASXAASXSSMAMAMXMASXMMAASASXMAMAMMAMSMMMMAAAMMSMXSAMXSAMASMSMSSMSSM
MMSMMMSMAMAMAMSAMXMSMMXMAAXAMAMAMMMXMSMSAMSASMSMSMMMAMAAMAMSMMSXMAAMMSSMMSXMMMMAAAXMAMAMMSXMAXXXXSASXMASAASAMXMAAMSSXMXAAAMASAMXMXSAAAAXAXXS
AXXXSAMMAMAMMMSXMASAMXMXMMMXXAMXSXAXXAAMAMSMMAAXSXXXXMSSSSSXAASMSMSMAAXAASAMXSSMMMXMXXASAMAMAMSSMMXMASAXMXXXSASMSXAAMSSMSMSAMASXSMSXMSMMSMMM
SSMAMASXMSASXASASMSAXXAXXSXXSAXAXMSXMMSMAASASXMMSAMXAXMMAMXMMASAAAAMMSSMMSAXAXMXXAMSSSXMXXAMAMXAAXMSAMMSMMSMMXSAXMMSMAAAAXMASAMMMASAMXXSMAAM
XAMMMAMXAXAXMASAMMSXMMMSAMXMAMSMSXMASAXMSXSMMASXMAMSAMXMSMXAMSMMMSMSMAXMASAMSSMMMMSAAXMAMSASXSSSMMAMXSAAASAMSAMXMAMAMAMXMMSXMASAMSMMMMMXMMMM
SSMMMASMXMMMXAMXMXMASAAMMXXSAMAAMASAMAMXXAMASAMMMSMAAAXSAMXAMAAAMXMAMXMMASMAXAMXAXMMMMMAMMAMXMAAXMMSAMMMSMAXSASXAMSASXSSMXXXSAMASAMASASMSMSA
AXAAMMXMXMAAMMSMMASMMMXSXXMAXSMAMAMAMASMMMMAMXSAXAMAMSMSMMXXXMSMSXSXSMSSMMMMSAMMSMSAMAMMXSSMMMSMMMAMASAXXMSMSAMMAMMMMMMAMSMMMMSMMASASMSAAAAS
ASXMSMASMSMSMSAAXAMAXSAMXMAMASXSMSSXMAAXAAMASXSMSXSXMAMMXMMSMXMMXMMASAAAMASASAMXAAMXSAXSAMXAXMASAMASXMXXSAMXMAMMXXSMSAMMMAAAXMAXSXMASXMMMXMA
MXMAMMMSAXMAXSSSMSSSMXMAXSMXMMAXAXAMMSSSSMSMSXMAXMXAMXXMASAAMAMAAAMAMMSMMAMXSAMMMSMASAMMMMMMMSASMSASAMXXMAMASMXMMXSASMMXMMMMMSSMXAAASAMXSSMM
ASMMSSMMMMMSXMAMAXAMMMMSMMASXMAMXMXXXAAMAMMAXAMSMASMSSXSAMSMXAMMSSMSSXXAMXXASMMMAAMXMMMMMXMAMMMMAMXXAMAXMAMMMMAAMSMAMSMSMSMSMAXMASMASAMXMAMX
MXSAAMXSMAAMAMXMXMAMXAAAASAMAMAMMAMXMMSMMAXXAMXAXXMAAAAMXMAXMSSXAAAMAAMSSSMASXAMSSSMAASAMXSASAMXMXAXSMSMSASXMSSSXMMSMMMAAAAAMXMXAXMMMMMMSMMM
XAMMSMASMMSSMMMMXMMMSMSSXMMSXSXMXXAXSAMXXMSAMXMMSSMMMMMSXMASAAAMSMMMMSXAAAMXMXSXMXAMSMSASAMMSMSXXMSXMAXAMASAMXAMASAAMASMMMSMSMSAXAXSMSSMMXXS
MSMMXMXSAMAMXAAMAMMAMMMMAMASASAMSSSMMMSAMXMMXAAXAASAMAXMXAAMMSSMXMASXMMMSMMAMAMXSMMMMXSMMMSMXMMMXAMAMMMSMMMAXMAXAMMXSASXXXXMAAAMMSAMXAAAXSAM
XMASXMASAMMXSSXSASMASAMXAMMMAMAMAAXAMASAMXAMXXMMSMMMASAMXSXSXXAMXSMMMMAXMASXSSMMSAXAXASMSXAXXAAAMSMAMAAXAXMSMMSMMXAMMASMMSAMMSMXAMAAMSSMMMAX
MSAMAMASAMXMMAMXMAMASASXXSAMXXXMMSMSMXXAMSSMSSMXMAASMXAMXXAMXSAMAMXAASXMSAMMAAXASXMASMSASXSSXSAMSAXSSSMSAMSMSAAMAMSXMAMMAXXMXAXMAMMXAMXMXSMM
XMAXXMASAMMAMAMAXAAMMAMAASXSMSSMAMMXMASMMXXAASMAMXMMAMMMMMXMAXAMMMSSMMAMMMMMSMMMSAMAMXMAMAMAMXAXXAMXAMAXMXAAMSSMAXAAMMSMSSMXXMSSSMMASXMXAMAS
SSSMSMASAMSASMSMMMSXMXMMXMXAMAAMSMSAMXXMASMMMSMASAMMXMAMAXXSMSMMAAMMMSAMXMAMAMSXSXMMMAMAMXMXMMMMSMSMAMMMMSMSXXMXXSSXMAAAMXMASXAAAAMAMAXMXSAM
MAAAAMXSMMAASXAXMAAMSMSMSMAMMMSMMAXXXSAMXMMAAXXASASAXSASXMMMAAMMMMSAASAMXSXSAXSASXSAXAMASASASXAAAMAXSMXXMXXMMSMMMMXAMSSSXAMASMMSMMMSMXMMAMAS
MSMMMXAMMSMMMMMSMAMMAAAXXXSMSAAAMMMSMSXXSXSMSSMXSAMAMMASAAAMMMSXAMMMMSAMAMMSXMMAMASMSXSMSASXMXMSSSMAXAMXXMAMAXSAAMSMMAMXMXMASAMAMAAXSMAMXSAM
XXAXXMASAXAAAASAMMSSMSMSMXMAXSMMMSASAMXASAMAMXMXMXMXMMXMMSSSSMXMXSAAXSXMMSAMMMMAMXMAAASXMAMAAXXXAXXXSAMMXMXSSMSXXMAAMXSASXMASAMMSMXXAMXMASAS
MSMSMMMMASXMSMSASAMXAAAXAMMMMXMMAMASAXMASMMAMAMMMXAMXXMMXMAAXMASMSXSMSAMAMMXAAMXXAMMMMXAMXMXMXMSMMSMAAMXXXMAAAXASMMXMMMXXAMAXAMXSAMSSMSSMSAA
MAMAAAXXMXMAXAMMMXSMSMSMMMASMSAMXXMMMXMASXSXMASASASASXAMAMMMMSXXAMAMASAMASMSSSSMXXSMSSSSMSMAMAMAXMAAXMMSMMAMMMMXSAMSSMMSSSMSSMMAMAMAMMXAAMXM
SASMSMSMMAMASASMSMMAMXMXMMAMASXMMSSMSAMXSAMASASMSAXAMAXSASXSMMSMSMAMXMAMASXAAAAASMMMAAXXAMMAMAXASMMSMXXXXAXXMXSAMXMAAAXAAXAMAAMXSAMMSMSMMMAM
SAXXAAAXMXMASAMXAAMAMASAMMSMMMAMAAAMAXAMMAMAMMSXMXMAMSMSAMMXAASMAXAXSSSMXSMMSMMMXAAMMSMMMMSMMMMMXAMMASXMSXSAMXMASMMSSMMMSMSSXMAMAMSXAAXAAMAS
MMMSMSMSXAXXMAMMXMMASMMASAAAASAMMSSMMMSXSMMMSAMXMMSMMMAMAMMXMMSSMSXSMAAAXMAXMXMXSXMMAAAAMMAMXMASXSMMMAAAAAXAMXSMMMMAAAXXAXXAMSASASMXMSMSMSMS
XSAMAMAAXMMXMSMXAXSAXAMAMMXMMXASAMAAXAXAMMAXMAMAXAAAAMAMXSAASAXAXAMSMSMMMSSMMASXSAAMMSMMXSASASMSAXAMSSMMMSXMMXXSAMMSSMMMSSSMAXAMXMXAXMXXXAXX
MMASAMAMXSAMXAASMMMMSSMSXMMSSXMMMMMMMMSAASXSSSXMSSMSMSXSAMMSMASXMXMSAAAMAMMMSAMAMSMMAXAMXSASMSAMAMAMAAAAAXASMMMMXSAAAAXAMXAXSMMMSXSMMXASMSSM
MSMMASXXMSSSMMMMAAXAAAXMAMAMAASMSSXMSXSMMXAAAMXMAMXXAMXMXSSXMXMAMMASXMXMXSAAMAMXMMXMASAMMMMMMMXMXMAMSSMMAMAMMAAAXMMMSMMSSXMAXASXAASXMMXAMAMA
MAMXAMXSAMAXXXMSSMMMSMMSMMAMSMMAXMASXAXASMMMMMSMASAMAMASXMMASMSSMMASMMSMMMMSSXMSXSAMXSMMAXXAAMMAAMAMXMMMMMSMMSMSSSMXAMXMAMXASXSAMXMAXXXXMAMM
SASXMMMMAMMMSMXMASXMAMMAMSSXMAMMMSXMMXMMAAMXXAMSMMMSSMMXMAMXMAAXXMAMAXAAAXMAXAAMXMASXMASXMSSSMASMSMSAMXASAXAMXAMMAMXSAMXXMMXXAXXXAXXMASXMMSX
SAMXXAASAMAAAAMSXMXMAXXAMAXAMAMAAMASASMXSMMMMMXAXMMAMASMSAMXMMMMSMSMSMSXMSMASMMMASAMXSAMXXAAAMMMAAXMMMSASXMASMAMMXMAMAMAAXXASMXSSSMXSXMAASMM
MAMASMMSMSMSXSAMXMXMXSSMMSSMSSSMSSXMASMXASXXAXSXSXMXSAMXSASASXAXAAMMXXMASAMXMXAMXSASXMASMMXSXMASXMXAAXMMMMSMMXSMMXMAXAMSSXMMMXAMAMMXMAMSMMAM
XMMMAMAXMAMMAAAXAASXXMASAAAXMMAMXMXMXMMMMAXMMMSMMXXXMAMASXSASMMXMMMXSAMXSAMASMXSASMMAASMMAXMAXXSASXSMXSAAMMMSMMAMMSSSMXAMAXXAMMMAMXAXMMAMMSM
XXAMAMXMXSAMXMMSXXAMMSAMMSMSXSAMAMMSAMXSXMSMSASAXAAMSSMMXAMXMASASASMSMMAMMMASXXAMXMSXMASXSAMXSAMAMMAXAMMMSAASASAMXAASMMASXMMAMAMSSSSSSMMSAMX
ASXSMSMMMXAXXAMAXSMAAMXSXMMSASXSMSASASASAXMAMASMMMAAAAAMSMMXMMSASASAMMMSMMMMMXAMXSMXMASMXXSXSAMMMXSMMSXSAMMXSASXSMMSMMMMAMXAMMMMMAAXAAXAMASA
MMAMXAAMASMXSAMAXAXSMMAMXAAMAMAAAMMSAMXSASMXMAMAMXAXXSMMAMMMSAXAMAMMSXAMXSASMMXMASAAAMXMASMXSMXAXMMXAXXSAMXAMAMXXXXAXASXMAMMXASAMMMMXMMXSAMM
XXSSSSSMAAAASAMSAMXAXMASMMMSSMMMAMMXAMXMASAMXXSSMSMSMAMMXSAAMXSXMXMAMMMMAMASMXSXMXSMMMAXMAMAMXSMMAMAMAMXMXMMMXMXMASXMMMXXSSMSAXAMSAAAXXXMXSX
XXXAAAXMSMMMSXMXMAXXMSAXMAAAAXAMASXSXMXMXMMAXXMAXSAAMXMAASMSXMAXXSMMSASMAXSMMASAMAMMXSMXXAMASMSAXSASMSMMMAAAAAMASAXSAMXSXMAXXASMMXMSSSXXMASA
MSMMMMXMAXMAXAXMMSSSMMSMSAMSMMASAXXSAMXXAAXSMSAMXMSMSXMMXSXMASMMMSAMSAMMMMMAMMSAMSXSXMAMXASXXAMXMMAMAMAASMSMSXSAMAMAXSAMAMMMMMAMXMXXAMXAMAMM
AXAXXXXMMSMMXSMXAAMAAAAXXMXMASXMMSMXAMAXSSMAXMASMMMXSXMMXMASAMXAAMAMMSMASAMAMXSAMAAXMSAMMMMMMSMMMMAMMSSMSXXMAAMMMSMMXMASAMAXSAXAMAXMAMMMMASA
SMXMMMMAAMAAAXAMMSSSMMSSXMAMMMAXAAAXSMSMMAMAXMASMASAMAXAASAMASMMXSXMAXMXMAXAMXSXMMSMAMXSASASMXAXXSXMXMMASMSMMSMMAMAMMSAMXSSSMXMXASMMAMMSAXAA
XAAMXAXMMSMMMSASXAAMXMMMMXAXASMMSMSMMAMXSAMXMMMSMAMASAMMMMXSXMXSASAMAMXAXXMSSMMASAMMAMXSAMASASXMASMSSSMMMMSASMXSASAMASMSXAMXMASXSMMSXSAMXSSM
MSSSSXMAXMXAASAMMSSMMSAASXMMASMAAAAMMSMXMASMXAXMMAMXMMSAAXXAXMXMASASAMXSSXXMAAMXMAXSASXMSMAMMMXSMSAXMAMXAAMXMAASXSASXXAXXXAAXSAMXAAMMMASAAXS
XXAXAXSXMXMXXMAMAMAAXMMMSASMAMMSMMSXAXXAXAMASMSXSSSMXAXMSMMAMXAMAMXMASAAAMMSSMMAMMMMASMAXMAMAAASAMXSAMMSMSMAMMMXASAMXMMMMMSMSMSASMMSMSAMXMSA
XSMMMMSMAXAMXSMMSSSMMXSXMAMMMMAXXAXMMSSMMSXXAMMMXXAAMSSMAMMAMXMSAXXSAMMSMSAMXASASXSMAMMSMSSSMMSMAMAMXAMXAAXXMMSMMMAMMXAAAAAMAAXMMAMAAMSMSXAM
MXXAMASXMMXSAMAAAMAASAXAMAMASMSSMSXAAAAXAXMMAASMSMMMMAAXASMMMAMAMAXMASXMXMMSMMMASAAXSXXAAAAAXXXXAMXAMXMMSMMXSAXAXMASMSMSMSSSMSMMSAMMSMAXMMMX
AMXXSASAMAXMASMMSSMSMASMSASASAMXAAMMMSMMASASXMAXMASAMMSMXSMMSMSASMMSAMXSSSMSASXXMXAMXAAMXMSMMMMSXMMMSMAXMAMXMXSAMSASAAXXMAXAMXAXMAMAMMMSMAXS
SAMXMXSAMASXMMASXMXAMXMAAASMMMMMMMXSXMASMSAMMSMXXAMMSXAXAXAXAMMAMXXMASXXXAASAMMXSXMXXSMSXMXMXAAMASAAAXMMSMMMXAMAXMSMXMSMASXMMAXSSSMAMAAAASAS
XXXXSMSXMASAMXXMAMSMSAMXMMMMAAXAXXASASAMMXMAMMXSMXSAMXSMMSMMMASAMXSSMMMMMMMMAMSXMASMXMAXAMASMSMSAMMSMSXAXXAMMMSAMXAXASXXAXAMMSAMAXASMMSSSMAS
XXSAMXSXMASAMAAMAMSASMSMSMASMSSSXMMMAMMMMMMXMXAMXAMAMSMAAAXAMASASAAXAAAAXXXSAMMAMAMXAXXSMMASAMAMXSXMXMMSMXXMAXXAMXSMMSAMXMAMAMAMXMAMAMXXXMAM
XAMXXMSASASAMXXMAXMAMASAASXMXMXAASXMXMAAMAAAMMSSSXSAMXASMMSSMXSAMXSSSSSSSMASMSMXMMXSSSMMXMASMMMMXMASAXAAAMSSMSMSMAXXSXMASXXMXSAMAAXSSMAXMMAS
MXMMAAMAMAMAXSASXSMMMAMSMSMMAMMSMMAMSMSSSMSXSAXAAMXAAAXMAMAXMAMXSAMXMAMAAMAMMAMMMSAAMAAAAMXSXSAMAMAXXMMMXMAAXAAXMAMSMAMXMASAASMSSSXMAMXMMASM
SAXMASMMMMSAMSAMXAAXMMMMMMASMXAXMSAMSAMAAMAMMXMXMAXXMXSSXMAXMAXMSAMXMAMSMMSMMAXAAMMMMSMMSSMMASMSSSSXSAMSSMSXMMSMMXSXSAMMMAMMMMXAMXXAAXMSSMMX
SMASAMXXAXMAMMXMSSMMSSMAAMMMSMAXMMMMXAMSMMAXXMAMXSAXXXAMMMSMSMSASAMXSAMAXAXMSMMMMSXXMXXXAAAMAMXAMAMAMXMXAAAMSMMXMXMAMXMXMASMMMMMMMMSSSMAAASX
MAMSAXSMMMXXMAMXAMXAXASMMMSAMMMMSSMXSXMXXSMMSASXAMMMSAMXAAAXAAXMSSMMSSSMMMXAAAASXSMSMMAAASMMMXMMMAMSMSMSMMMAXAXASXMSMSMSAASXAAAAAAAAAXMAMMAS
XAMXAMXMMXSAMMMMMSMMXMMSAMMAMSXMAAMAMAMXMSAXASMMMSAAASXSMSXMMXMXMASAMAMXASXSMSMSAXXAAMASXMMXMAXMMSMMAMXMXXXSSXSMMXAAAAAAMMMXMSSXMMMXMAMSSSXS
SMSXSMSXAAXXMAAMXMASASAMXMMAMMAMXXMXSAMAMMAMMAXAASMMXXXMAAXAXAXASMMMSAMMMXAMAMAMAMSSSMMMMSSMSXSAAMAMXMSMMSMAAASXMMAMSMSMXMASXAXASMSSXSXAAMAM
SAAAAAXMMXSSSSSSMSAMXMASAMMSMSAMSMSMMAXAMXAAXMXMMXAMSMXMXMASXXSASXAXMASASMAMAMSMSMAAMXAAAXAAAAXMMMAMXMAAAXMMMMMAMAXXMXAXXMASMSMMMAASAAXMXMAM
SMMSMSMXSXAMAMAMAXASXMAMASAAXSMSAAAASXSSSSMSSMSXMMSMAMSXMMMAAXMAMXMSMXXASAAMASXAMMMMSMSMSSMMMSMSMMMMXSSSMSAXMASAMSAMXSASMMMXAMAXMMMMMMMSASAS
SAXMAXMAMMMMAMXMXMXAAMXSAMMSMXSXMSMXMMAAAMAAAASMMAMSMXMAAXXMMMMAMAMAMAMSMXASXSMAMAAAAXMAMAXMXMXMAASMMMAAAXMMMASMSMMMXXMAMAASMSSMSAAAAAASASMS
XMMMAMMASAXSMSMSSSMSAMAMAXSAMXMMXAXXXMMMMMMMSMMAMAMAXAMSMMMMAASASASAMMXXASXSASMXSMMMSXSXSMMMAXXMSMXAMXSMMMXAMMMMAAASMSMASMMXAAMASXSSSMMSAXAM
MSXMASMXSXXSAAAMAMXAAMXSAMAAMMAASMSMAMXMSMXMMASMMSSSMSMXASMSSXSASMSMXMAMXMAMAXMMSXMXMXSAMXAMXSAMXXSSMAMXXAMMSSXMMSMMAASASXSMAMMXMAMAAMAMAMSM
XSAMASAXMASMSMSMAMXMMMAXMASMMMMAMAAAAMXXMMAAMAMAAAAAAMAXAMAAMAMXSMMASMASAMXMXMSASMXAMAMAMSMSAXXMAXAAMXMXSMMSAMMSMXXMSMMMSASASMXMMXMSMMXXAAAX
MMSMAXMAXSXMASAMASMSSMMMSAMXXMMSMSMSMSXAASMMSASMMSSMSMAMSMMMSXMAXAMAMMXSASASAAMASAMXSASAMAXMXMSSMSSSMASMAASMAMAAMXMMMASAMXSAMXXAMXAXXSSSSSMS
SXMMSSXSMASMXSXSMSASXAMAMAMMSXAXAMXMAXXMMSAXMAMMAMXAXMAMAASAMASMSSMMSSXMMSASMSMSMXAMSXXMXSSMSXMAXMAXMASMMXMSAMSAXXMAMAMASXSAMXASXMXMAMAAAAXA
AAXAMXAXAXMSAMXSAMXMAMMASXMAMMMMMSAMXMASAMMMMAMMMSSSMSXSSSMASAMMAMAAXXASXMXMAMMXMSSXMMSSXAAAMXSSMMMMMMSXMAMXMAMAMSXMMASAMMSAMSMMXASMSMMMMMSS
MSMMMMMMMMSXSXAMMMSMSMSXSMMSSSMAMXAMMMXMAMXASAMXSAXAAXMXMAMAMXXMASMMXSMMAAXMXMASMAMAASAMMMMMXAMAAMXAXMXAMSXSSSMAMMAMSMMASAMAMAAXXMSAAMAXAAXX
XMAMAMMAXSXAAMMSMAAAAASASAXAAXMASXSMAAAXXMSXSMSMMXSMMMAXMAMXSXMXMSXSXAAMAMMMAMSMMASXMMASMSSSMASAMMSMXMSMMAMXAASMMMAMAASAMXSSSSSMMXMXMAMMMMMM
XMXMAMXMSMMMMMXAMMMMMSMAMSMMSMMXSAMXXSASXMMASXAAAMXMSSSMXSSMMXSAXMAMXSSMMXAXXMAASAXAXMAMAAAAMAMASXXAAXAMXMAMMMAXASASMXMMSAXAXMAXSAMMXSSXSASM
MSMXMSMMAASMASMSMSSMMMMXMAMSAXMAMMMXXAAXAAMMMXXSMXMAMAAXAXAAAAMAMMAMAAMAMSSMMSMMMMSMSMSSMSMMMMSXMXMSMXASMXSAXXMSXMASMMMSMMSSSSSMMXSAAMAXSASX
MASAAAXSMSMMASMAMAASASMMSSXSAMMAMXMXMASXSMMMMSMMMAXXMXMMMSSMMMSAXSAMMMSAMAXAAAMSMXAAAAMSAMXXXXAMXAXAMMSMMXMASXXSAMXMAXXAAMSAAAAXXAMMSSMAMAMA
SASMSMMXXXAMAXMAMXSMMMAMAMMMAMSASAXSMMMAMASAAAAAMMSMMSXSXAMAXXAMXSASXASXSAMMMMSAAMMMMSMMMMSMSMSSSMSASMMAASMMMMAXMMMXMMSSXMMMMMMMMXMAXAXSMSMS
MASXXXSSXSAMSSSXSXMMXSXMASXMSMSASXMSAMMXMASMSSSMSAMXAMMMMMSAMAMSXMASMXMAXMXSAXSMSXSAMXAXMASMXAAAMXMXMASXMXSAAMSMSAMAAAAAASXSMXAAXMASXMMXAXAX
SXSASAAXASAMXAAASXMAMAMXXAXAMAMMMAMXAMSAMASAAMMMMMMMMMAMSMMASMXSAMAMMAMXMAXMAMXXMASAMSSMMASXMMMMSAMAMMMXMASXMSAASASXSMMSMSASAXSXSXMMAMMMMMMM
MAMAMSMMMMASMAMMMAMMSAMSSSMXMAMSSXMSSMMXSAMXMAAAASXSMSMSAAXXAXAMAMMMSASAAXMMMAMAMAMAMAAAMASAAXXASASASXSAMMSXMMMXMAMAAAAXXMAMMMMAMXMSAMAAAAXX
MXMAMAXAASMMAXMASXMXMAMAAMAXXAMXXAMAXAXMMMSAXSXSMMAAAAMSXSMSAMXSSMMMSASXSMSXMAAAMASXMXSXMXSMSSMMMXXASXXAXASAXMSSMSMSSMMSAMXMXAMAMMAMASMSXXSA
AAMAXASMXXAXSXSASAMSSXMMMMMMSMSMSSMMSSSXAMXMAXAXXMSMMMMXAXAMMMMMAASAMXMMAMAASMMSXMXMSXMASXXMAXAMMSMMMMSSMASAMXSAAXAXAMXXMXSSMXSAAXMSMMAMMSMX
SSSSXMMMXSMMMAMMSAMMSXAAMAAMAAAXXAAAAXAMAMAXAMMMMXMAMXMMMMSAMXAMMMMAMAMXAMXXXMAMXMMXSAMXSAMMSMSMASAMXAAAMAMXXMXMMMXMMSMMXAMAMXMMXAMAMXAMMMAM
AAAAXSSMMAXAXXXXXXMAXMSMSSMSMSMSXXMMSMMAASXSASAAAAXAMAXAAMAXMSSSXXSSMSSSMSSSMMAMASAMXAMAXMMAXAMMXSSMMMMMMASXMMMSAMXXXXASMMMSAXXAASXMSSMSAMXM
MMMMMXAAXXXASAMXSSSSMMXMXMAXXAMXXASXXXXSASXXASMMMSSSSSSSSSXMASAAXMAXAXAAAAAAAMMMAMXXXSMSMXMASASXMXASXMMXSASMAAAXASXMASMMSAAMSAMMXMAXXMASASMS
XAXXXSSMMMXXMAAASAAAAAXSAMXMSMMXAMXAXMXMASXMAMASAMXXXAAMAXMXXMMMMAAMMMSMMMSSMMXMSSMMSMAXXAMASXMMMSAMXSAXMASAMMMSAMXMASAAXMXSMXXMASXMMSXXASAX
XAXSAXXMXXAASXMMMMMSMMMMASXMXXMXSAMXMSMMSMMAAXXMSSMMMMMMAMMMXXXSSMSMMAXAXXXMMSMMAAASAMAMSSMXXMXAMMAMAMAXMAMMSAMXASAMXXMMMXSXMSASXMSAMXMMMMMX
AXAMAMXMAXXXMMXAAAXAAMASAMAMMASAMAMXXMAXAAMSXSAAXXAXMAXMMSAAXSMAMXXAMSSMMMSMAXAMMSMMAMMMAAMXMAMSSSXMXSXSMAXXMAXSSXMMSXAMSXMAXSASAMXMXAXAXAXM
SMXSAAAMASMASASMSSSMXMAMAMSMSAMXSAMXMMSSSMMAAXMXMXMMSXSAASMSMSMAXXSAMAAAAAXMASMMMMMSSMMMMXSAMSAMAAXSXMXAMAMSMMMXXASASMSMAMSMMMAMMMXMSMSMSXSA
XAXMAXXXAXXAMXMAAMAASMSXXSAMMXSAMMSXAAAMAMMXSSSXSSMAAMMMMSXAAXSSMMXAMSSSMMSMMXMAMMAAAAASXMSASASMMMAMAMXMMSMSAASASXMASAAAMMAAAMAXAXAXAXAXMAMX
SSSSMSMMMSMMMAMMMMMXMAMAXSAMAXMASMSMMMXMAMXAMAXXMAMMMSAMXMMSSXXMASXSMXAMXASASMSMMMMMXSMSAMXMMMXXAMAXAMXXMXAMSMMASMMMMXMMXMAMXSASMXSSMSSSMAMX
XAAXAAAAMAASXSMMAAXAMAMSMSMMMSMMSAMASASXXXMXSAAMSSMSASASAXXMMAXMXMAMXMXMMAMAMXAASXSMAMASAMAXAXMASXSSSSMAMMMMMXMXMXAXMASXMMSSMMMSAAMAMXMAMXMM
MXMMSMSMSXSAAMSSSMSSXMXAAXXAXAXMMASMMASAMSXMMASXAXXMXSAMAXXASMSMXMXXXSXSMSMXMSMSAMAMAXASASXSMSAXMAXAAMMASAXAXAMMMSMSMXAAAAAAAAXMMMMAMXXSMSXA
AAMAXXAXSXMMMMAXAAAMAASMSMSMSMSXSAMXMXMASMXMXAMMMSMMAMXXMMMMAAAMMSSMMMAAAAMXXXMAMAMMSSMMAMXAASMSMMMMSXSASASMSMMAMAXAAMMMMMSMMMAMSSSSMMXAAMSS
MAMASMSSMAXXXMXSMMMMMMSXMAXMAMMMMMSAMAMMAMMMMMXXMSMASXMXAMXSMSMMAMAAAMSMMMSMMXSAMSXAAMAMMMXSASMMAXXXAAMAMASAAMSMSMSMSXXASMMMSXSXAAAXAXSMSMAM
SMSMSAMAMAMMSSMMAXSAMXXAXSMSAMXMAMMMXAMXSXAAAAASMXMAMAMSXMASXMMMSSSMMXXAXXAAMAMASMMMMMAMAMAMAMAXXMXMMXMMMSMMMMAASXSXXXMASAMASAXSMSMMMMMAXMXM
AAAAMXSAMASASAMAAASASXSMMMXMMMMMMSXSSXSAASXSMXAMAAMXSAMAAMASXXMXMAAXXXXSMSSSMASMMXAAMSSSXMXSSSSMXMAAMASAMXAXXMMMMAXAMMAAMAMAMXMAXAXAAASMSMSS
MSMSMASASAAXSAMXMXSASASXSMSMSAAMAMXXAAMMMXMAMMAMSMSXSXMXSMASAMXMMSMMMAMXAXMAXASXSSMMXAMMAMXXAAMAXSAMSASASXMMXXXSAMMMAAXMSMMXSMMSSMSSSXSAAAAX
XMAXMASMMASMSXMXMXMAMXMASAAASMSMASMMMMMSSSMAMMAMAXXMMASAXMMMMXSAAAXAXSAMMMSAMXMAMXMMMMSXMMAMMMMSMMAMMASXMAMXAAXAXMAXXMMXAXXMXAAAAXAAXAMMMMMS
XMXMMMSXMAMAMXMMSAMAMXMXMMMXMAMXMXAAAMAAAXMMSSMSXSMMSAMXSMXXAAMSSXSASXMASAXXMXMMMXAXAAXASXXSAMMAAMSXMMMMMAMMMMMMMXXMASXXMSSXSMMSMMMSMSMXXAMX
SXMXMAMMSASAMXMASASXSASASASAMXXAMSMMMSMMSMXAAAXMAAMAMASXMASMMMXMXMAXMXXXSASMSSSSMSMSMMSAMAXMASMSXMMMMMMSMMXXAAAXAASMMSAASAMMAMXXMAXAAXAXSXSX
XXMAMASASASXXXMXMAMASASASXSXSMSAXAAXMXXXAMMMXMMMXMMXSSMXXMAXMMAXMASMMSMMMAMAAAAAAAXXAAMMMMMSXMAMXAMMSAXMASXSMSMSAXMAXMMMMASXSMASMXMMSMMASAMM
MASASXSAMAMASXMSXXMMMXMAMAXAAASMSMSMAAXSASXSXSASAAMXXMASMSMXSSSSMSXMAAMAMAMMMMMMSMSMXMSXSAMXMMAMSSMASXMSAMAAAAXXMASXMSSXMXMAAMXMXXSXMASAMASX
MMMAMMXAMSMMMAAXAMXXMXMAMXMSMAMXXXMMMMAXAMXSAAASMSMSSMAMAAASMAMMXXXMSSSSSSMXSAMXAAAMMMSAXXMAXSXMAMMMXAMXAMSMMMXSXMMAMXAXXXSXMASXMXMASXMXSAMX
MASXSMSAMXAASMMMSMSASAXXXMAAMMSAMXMXXSMSMSAMXMXMXXMXMXAMSMMMMAMMASAXXAXAAAXASMSAMSMSSMMAMSSSMASAMXMSMSXMAMXMASMMAXMAMMMMMMAAXAMAAMSAMXXXMAMA
MMSAAAAXXSSMMAMXXASAXXXSMXSXSAXMAXAAXXXAAMMSAXAMMSSMSSXMXMXSSMSMASXMMMMMMMMMSAAMMMXXAAMXMXAAAMXSXMASMAXMXSXSASASXMSAMAAAASXSMXSASXMASMXXSAMM
MXMXMMMMAMAXXAMMMAMAMSSMAAMSMMSSSMSSSSMMXMASMSMSAAASAMXAXMAMAMXMXSASASAXXXXAMXMMSXMSSMMXAMSMMXMMXXASXMMXAXMMXSAMXAMAXSSSXSAXAMXXMASXAXSMSASX
SMMSXSAXXSAMSXMAMMMSMXAMSMSAXAAXAAMAAXXAXMAMAMXMMMMMMMXSAMASAMXSMSASXSXSAMMSSMXASAMAAAMXMAAXMASMAMSMAXAMSSMMMMMMSMMSMMAMAMAMAMSASAMSXAMAXXMM
AASMASXMMMAMAMSSMSAMMSSMMXMMMMSSMMMMMMXMMSAXSMSSMSSMXSAMMSAXAXXSAMXMAMAMAXAAAAMXSAMMSMMAMMMSMAAXASASXMMMAAXMAAAAXXAXAMXMXMASAMXAXMASMSMMMAXA
MSAMXMAMXSAMAXAAAMXSAAAMMAXXXAAMAMXSASAMAMMMXAAAAAAMAMAMXMMXSMMMAMSSSMASAMMSSMMXSXMAAASMXMAAMXMSXSASXSXMMSMSSXMMXMMMAMSSXSXSASMSMXAXMMAMSMMM
XXXMAXAMXSXSSMSAMMMMMMSASASXMMSSMSASMSAMSAMAMMMMMMSMSSMMSXXAMASXSMXAASAMXSXAMAMXMMSSMMXAXAXXSAXMAMAMASMSAAAMMASMMSMSAMAXASASXMAAAMASXMXMAAAX
AXAXXSXSXXAXAAMMAXAXAXXAXXAMAXXXAMASASAMXAMXSXMASXXAMAXSAMMMSAMXAAMSMMMSMXMSXSMMSAXAMMMMMMXAMXMXSMMMAMAAMMMMSAMAAAASAMXSAMAMAMSMXAXAXMASXSSS
MSSXMAMSMMXMMMMXSSMSASMSMSMAXSXMMMMMAMMMSXMMMMXASMMAMAMSAMAMMAMSXMMMMAAAAMMMAAAXMMMMMAAAXXASXMSXMAXMMSSMXMAXMXMMMSXMXXXMXSXMMXAAXSMXMSAXAAAA
MAXAAMASASMAMAMXMAMXAMAAAMXSXAAMXAAMSMMAMASMXSMMSASXMAXMAMXMXAMMAMXXSMSSSXAMSMMMMMSSSMSMXAXXAASXSAMSMMAXXMMSASMXMXXAXMXMASAXXSXSMMAMAAXSMMMM
MSSMMSXSMMASASMMSAMMMMSMSMAXAMMMSXMMXMMAXAMMAXAMSAMMXMSSXMAMXAAXAMMXXXAMAMMMAXXAMXAAAMAXAXMMMMMAMXXAAXMMMSMMASXAMMSXMMAMXXAMXAMMAXMAXAXAXAXM
MAAAASASXSSMMXAMSAMAAMAAAMMMMSSXMMSMAXMSXSAMXMAMMMMSSMAAAMASXSMSASXSMMMMAXXMXSSSXMMMMMAASAMXAAMXSSSSSMSAXAAMSMMMSASAMSMSMMSMMSXSAMMMSAMXXMAS
MSSMMMAMAMXMXSAMXSSSMSMSXMSAAAXAMAAMASXAAMXSAMXXXAAXAMXSXMASXAMSMSAMAAAMSMMSAMAXXMSXMMSXMMAMSXSAXAAAMMMSSMSMXAXXAMXAMAXMAAAAXAAXASXAAAXAMXAM
XMAMSMMMSMMSAMAMAXXXXAAMAASMMMSXMSSMAMMMMMAAAXAMSMSSXMMMXXMSXMXXXMAMMMMMAAXSAMAMSAXMMAMAMXAAAAMXMMMMMXMAMXXAMXMMMSSMMSSXMASXMMXSSMMMXSMXXASX
MMAMAAAAMSAMXSSMMSMSSMMMMMXXXMXMMXXMAXAXAMXMMMSXMAAXAAAMMMMMMSMMMSAMSMSSMSMSMMXMXXMSXAMAMSMSMXMMSASAXMMXSASAAASMMAAAAAMMXMMAAXAMXAXAXMAXAAXS
MMMSMMMXXAMXAAXAXAMMXXAXXSMSMSASXMXXASMSSSMXAAXASMSSMMMXAASAXAXAMMASXAXXXAAMXSXSMMASMMMAXXAAMAMXSAXASXAAMMSXSMMAMSSMMMSAASMSMMSSSMMXMMAMSXMX
MAMAXSSMMSSMAMXAMMMSAASMXAAAASAMASAXXXMAXAAMMMSAMAMMAAXMSMSAMMSXXAXMMXMASMSMAMAAAMAMAMSMMMXMSSSMMSMAMMMMSXSAMXSAMAAAAXMXSAAMAAMXAXSAMMMMXAMX
SXSAXMAMXAAMMMASXSAMXMMAXMSMMMMMXMMMMAMXSSSSSXMMMSMSMSSXMAXAMXAMSSMXXXAMXAAMAMSMXMASMMASXASMXAAMAMXSMXMXSAMAMASASXSMMMAMMMMMMSMSMMSXMAXSMSMS
XMMMMSMMMSSMXMXXAMASMSASXXMMSMMMAMAAXAMXAAAAXAMXAXXXAAMMMMMSMXAXAMAXSMXMXSMXAMAXMSASMSAXXMAXAXMMXSAMXXXAMAMXMXSMMMMMMSSMASXMXMAMXAMASMMSAAAX
XAAAXAXXAXAMMMAMAMXMASAMSXSAAAASAMMMXAMXMMMMMMMMMMMMMMSAMAAXMXMMMMSAAXAMXXXMSSMSAMASAMXXSASMSSSXAMMMMMMSSMMSMXSXSXAXMAAMASAMAMAMMAMMMXAMSMMM
MSSSSSMMMSXMAMASAMAMMMAMSMMXMSMXASASXMMAXXAXSMASMASAMASASMSSMXSAXSMMMSASMSSMAAAAMMAMMMSAMAXAXAXMXSAAAXAAXAASMMSAMSMSMSSMXSAMASAMSMXSAMMMXXXA
AAAMAXXAXAAMXSMMASXSAMAMXASXMMXSAMASXSMSSMXSAMAXXAMAMAXMMXMAAASAMXAAXSXAXAAXMMMMMMSSMAMASAMSMMMSXSMSSMMSSMMSAAMXMAXMXAAXASASASAMSAASAMSAMXXA
SMSMAMSSMMMSMAMSMMASXSSSSSMAAAXMXMMMMAXMMXMAMMASXMSAMMSMSXSMMMSASXSMASXMMSMMXAXSAXXAMXSAMXSXMSAAMXAXMAAAAXASMMSMSMSSMMMMMSAMASMMMMMSAMMMSMSX
AAMMSXXXAXAAXAMAASMSXAAAMMSXMMSMAXAAXMMXAAXAXXAMXAXASXAASXSXSXSAMAAMAMAXAXXAXXMSMXSAMXMXSMMAAMXXAMSMSMMSAMXSXXMAXMAMAXXAXMAMMMXSSMAMAMXMAMMM
MMMAMMSSMMSSSMSMXMAXMMMXMASMSAAMASXXMXAMMMXXSMASMMSMMMMMMASXMAMXMSXMASMMMXSSSMASAMMXMXXXXAXAMMMMSAAASXMMMSAXMMSXXMASMSSMMSSMXMXAAMXSAMXMAXAM
XMMAMXAAAAXAAAAXSMSMXXXXMASAMXSMMXMAMXMAXXAMXMAXAXAXSXSAMMMAMSMSMMXMAXMASAAXMAXSAMXAMSMMMSSMMSAAMSMXMAXXAMMMSAAMSXMXAXXAASAMXAMSSMXSASXSMSAS
SXMMSMSSMMMSMMMSXAXMAMASMAMAMMXASXXSAMXSASMMMAXSSSSXSAXMMASAMAMAAXAMMMSAMMSMSSMMXMSXXAAAAMAAAMSMMMXMSSMMMXSAMSSSMASMSMMMXXAMMXMAXMXSMMMAMSAM
XAMAXAMXAMXMMXXXMMMMAMXAMASAMASXMAAXAMAMAXMASMMMAAXAMSMXSASASXSSSMSXSAMXSAXMAMXXSMSMSXSMSMMMMXAXMXAMAAAXAAMMMAMAMAMAAAAXASMXMAMMASAMXAMAMMMM
SSMSSSMSSSSMSSMAMMAMAMXMSMSASMMAMMMMMMXSMSAMXXAMMMMAMMMXMASAMMMAXAAAMXMSMMSMMSSXXAXAAXXXMXMAMXXMASMMSSMMMSSXMXSSMSSSSSMSMXAAMMSSMSASMXSMSAMX
XXAAAXXXXAMSAXMAMMMSASMXAAXAMXSAMAAAAMXXAMXAMMSMXXAMSASXSXMXMAMAMSMMMSXMAXAXMAMAMMMMMXMAMAMAMSSMMSAMXMASAAMMMMXXAXAAAMAMXSSSMXAAMSAMMASASASM
AMMMSMMMXMMMSSMMSAASAMXSMSMMMMMASMSSXSASXMSASMAMASMMSASAMAMAMXMMXMAXAXASMMSMMXSMSAAAAAMAMSSMMAAMXSXMAMSMMXSAAASMSMMMMMAMXAAMXMSSMMMMMAMXSAMM
SXMAXAXMASMXAXAAXMASAMASAMXAAMSMMMAMXMASAAMAXXAMAAAAMXMASASASAXMASMMSSMMAAXAMAMXMASMSMMAMMAMMSMMASMSMSAAMXSXSMSAAMASASXSAMMSXXAAXAAAMXSXMAMX
MAMAXXMXAXAMMSMMSXMSAMMXXXXSXXAMAMAMXMASXMMSMSMSMSMMMXXASMMASMMSASAAAXXXMMSXMASASXMMXMSMSMAMXMAMASMXSSXSMAMAMAMMMSASAMSAXXAXAMMSMSSMSMSASAMX
SAMMSMSMSSXMXSAXMAMXXAXSMSMASMXSASXSXMXMASAXAXAXAXASMSSMMAMXMMAMSSMMMSSSXMAMSASXXAMXAXAAXMSMMXSMAMASXMAMMAMAMAMAXMAMAXAMMMMSMAMAAXAMAMXAMMSS
SASXAAXAAXAMXSXMSMMMSMAAAAMXMXMSMSAMXASMMMSSXMXMAXXAAAXAMMMSAMXSXSASMXMASMXMMASAMSMSSSMXMMMASAXMXSMMAMAMSASAMXSMMSSMMMMXAAMMMMAMXXMMSSMSMSAX
MMMXMSMMMSMMAMAAXXAAAMSMSMSMMSMSAMAMMMXAXAXMXSAMXSSMMMSMMSXMAMXMASMMMAMAMMMMMAMAMXAXMAMASASAMXMMAAXSSMAMSXSASAAXAAAAXAXSSXSAAMXSSMSAASAMXMMS
SAAXSXAXAAAAXMMMAMSSMMAAAMAAMXAMXSAMXXSAMXSAAXXXMAXXAXAXAXAAXMAMAMAAMAMMMSAMMXSSMMXMMAMAAAMSMAMXAMMXXXXMMAMAAXXMMSSMMSMMMAMXMSAXAAMMMMMMXAAM
SMASXSMMSSSMSXSMSMXAXSMSMSSSMMXMASASMASAMAXMASMMSMMSSSMMSSXMASAMASXMSSSMAMSASMMAAMMSSSSXMAMAMMMSAMAMMMMAMAMMMASAMAXAAAAAMMMSXMASMSMMASAMSMSS
SAMXXXXAAMAMMAXAXMASMXXAAMXMASXMXSAMMAMAMMSAMSAASAAAXAXAMSASASASMSAAAAAMXSMMMASMMSAAMXMXXSSMSAASAMSXMASASASAXMASMMSMMSSMSXXSAMXXXAXSASAMMAAM
SSXMASMMSSMMMAMMMASAMXSMSMASAMAXAMAMMSSXMXXXMSXMSMMXSAMSSSXMMSAMXSMMMSMMSMMXAXXXSMMXSAXSAMXXMMXSSMASMXSMSMSASMXXMXXXAXMASAXSXMSASAMMXSXMMMMS";
