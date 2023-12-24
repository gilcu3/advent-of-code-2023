advent_of_code::solution!(24);
use itertools::Itertools;

// general solutions in 3d using ternary search was not necessary
// fn dist(p1: (f64, f64, f64), p2: (f64, f64, f64)) -> f64{
//     (p1.0 - p2.0).powi(2) + (p1.1 - p2.1).powi(2) + (p1.2 - p2.2).powi(2)
// }

// fn cpoint(p1: (f64, f64, f64), v1: (f64, f64, f64), t:f64) -> (f64, f64, f64){
//     (p1.0 + v1.0 * t, p1.1 + v1.1 * t, p1.2 + v1.2 * t)
// }

// fn mindist(p1: (f64, f64, f64), p2: (f64, f64, f64), v2:(f64, f64, f64)) -> (f64, (f64, f64, f64)){
//     let (mut bt, mut et) = (0.0, 1e4);
//     let err = 1e-9;
//     while et - bt > err{
//         let t1 = (2.0 * bt + et) / 3.0;
//         let t2 = (bt + 2.0 * et) / 3.0;
//         let p21 = cpoint(p2, v2, t1);
//         let p22 = cpoint(p2, v2, t2);
//         let (d1, d2) = (dist(p1, p21), dist(p1, p22));
//         if d1 + err < d2{
//             et = t2;
//         }
//         else{
//             bt = t1;
//         }
//     }
//     (dist(p1, cpoint(p2, v2, bt)).sqrt(), cpoint(p2, v2, bt))
// }

// fn intersect(p1: (f64, f64, f64), v1: (f64, f64, f64), p2: (f64, f64, f64), v2:(f64, f64, f64), zone:(f64, f64)) -> bool{

//     let (mut b1, mut e1) = (0.0, 1e4);
//     let err = 1e-9;
//     while e1 - b1 > err{
//         //println!("{} {}", bv, ev);
//         let vv1 = (2.0 * b1 + e1) / 3.0;
//         let vv2 = (b1 + 2.0 * e1) / 3.0;
//         let d1 = mindist(cpoint(p1, v1, vv1), p2, v2).0;
//         let d2 = mindist(cpoint(p1, v1, vv2), p2, v2).0;
//         if d1 + err < d2{
//             e1 = vv2;
//         }
//         else{
//             b1 = vv1;
//         }
//     }
//     let (d, pp) = mindist(cpoint(p1, v1, b1), p2, v2);
//     //println!("{} {:?}", d, pp);
//     d < 1e-4 && pp.0 >= zone.0 && pp.0 <= zone.1 && pp.1 >= zone.0 && pp.1 <= zone.1
// }

fn intersect_ax(p: (f64, f64), d: (f64, f64), ax: f64) -> f64 {
    p.1 + (ax - p.0) * d.1 / d.0
}

fn ccw(p1: (f64, f64), p2: (f64, f64), p3: (f64, f64)) -> f64 {
    (p2.0 - p1.0) * (p3.1 - p1.1) - (p2.1 - p1.1) * (p3.0 - p1.0)
}

fn intersect2(
    p1: (f64, f64),
    v1: (f64, f64),
    p2: (f64, f64),
    v2: (f64, f64),
    zone: (f64, f64),
) -> bool {
    let mut ii = vec![];
    let mut jj = vec![];

    for ax in [zone.0, zone.1] {
        if (ax - p1.0).signum() == v1.0.signum() {
            let c1 = intersect_ax(p1, v1, ax);
            if c1 >= zone.0 && c1 <= zone.1 {
                ii.push((ax, c1));
            }
        }
        if (ax - p2.0).signum() == v2.0.signum() {
            let c2 = intersect_ax(p2, v2, ax);
            if c2 >= zone.0 && c2 <= zone.1 {
                jj.push((ax, c2));
            }
        }
        if (ax - p1.1).signum() == v1.1.signum() {
            let c1 = intersect_ax((p1.1, p1.0), (v1.1, v1.0), ax);
            if c1 >= zone.0 && c1 <= zone.1 {
                ii.push((c1, ax));
            }
        }
        if (ax - p2.1).signum() == v2.1.signum() {
            let c2 = intersect_ax((p2.1, p2.0), (v2.1, v2.0), ax);
            if c2 >= zone.0 && c2 <= zone.1 {
                jj.push((c2, ax));
            }
        }
    }
    if ii.len() < 2 {
        ii.push(p1);
    }
    if jj.len() < 2 {
        jj.push(p2);
    }
    if ii.len() < 2 || jj.len() < 2 {
        return false;
    }

    ccw(ii[0], ii[1], jj[1]).signum() != ccw(ii[0], ii[1], jj[0]).signum()
        && ccw(jj[0], jj[1], ii[0]).signum() != ccw(jj[0], jj[1], ii[1]).signum()
}

pub fn part_one_arg(input: &str, tx: i64, ty: i64) -> Option<u32> {
    let mut av = vec![];
    let mut ap = vec![];
    //let mut m: i64 = tx.max(ty);
    for line in input.lines() {
        if line.trim().is_empty() {
            continue;
        }
        let (p, v) = line.split_once(" @ ").unwrap();
        let (px, py, pz) = p
            .split(", ")
            .take(3)
            .map(|s| s.parse::<i64>().unwrap())
            .collect_tuple()
            .unwrap();
        let (vx, vy, vz) = v
            .split(", ")
            .take(3)
            .map(|s| s.parse::<i64>().unwrap())
            .collect_tuple()
            .unwrap();
        //m = m.max(px.max(py.max(pz)));
        ap.push((px, py, pz));
        av.push((vx, vy, vz));
    }
    let mut ans = 0;
    //let nf = 1e4 / m as f64;
    let n = ap.len();
    for i in 0..n {
        for j in i + 1..n {
            //if intersect((ap[i].0 as f64 * nf , ap[i].1 as f64 * nf, 0.0), (av[i].0 as f64, av[i].1 as f64, 0.0), (ap[j].0 as f64 * nf, ap[j].1 as f64 * nf, 0.0), (av[j].0 as f64, av[j].1 as f64, 0.0), (tx as f64 * nf, ty as f64 * nf)){
            //println!("{} {}", i, j);
            if intersect2(
                (ap[i].0 as f64, ap[i].1 as f64),
                (av[i].0 as f64, av[i].1 as f64),
                (ap[j].0 as f64, ap[j].1 as f64),
                (av[j].0 as f64, av[j].1 as f64),
                (tx as f64, ty as f64),
            ) {
                ans += 1;
            }
        }
    }

    Some(ans)
}

pub fn part_one(input: &str) -> Option<u32> {
    part_one_arg(input, 200000000000000, 400000000000000)
}

fn solve(dv1: (i64, i64), dv2: (i64, i64), dp: (i64, i64)) -> Option<(bool, i128)> {
    let det = dv1.0 as i128 * dv2.1 as i128 - dv1.1 as i128 * dv2.0 as i128;
    let pp = dp.0 as i128 * dv2.1 as i128 - dp.1 as i128 * dv2.0 as i128;
    if det == 0 {
        if pp == 0 {
            None
        } else {
            Some((false, 0))
        }
    } else if pp % det != 0 {
        Some((false, 0))
    } else {
        let t1 = pp / det;
        Some((true, t1))
    }
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut av = vec![];
    let mut ap = vec![];
    for line in input.lines() {
        if line.trim().is_empty() {
            continue;
        }
        let (p, v) = line.split_once(" @ ").unwrap();
        let (px, py, pz) = p
            .split(", ")
            .take(3)
            .map(|s| s.parse::<i64>().unwrap())
            .collect_tuple()
            .unwrap();
        let (vx, vy, vz) = v
            .split(", ")
            .take(3)
            .map(|s| s.parse::<i64>().unwrap())
            .collect_tuple()
            .unwrap();
        ap.push((px, py, pz));
        av.push((vx, vy, vz));
    }
    let n = ap.len();
    let mut ans = 0;
    let top = 350;
    let mut rdone = false;
    for vx in -top..top {
        for vy in -top..top {
            let mut found = false;
            let mut done = false;
            let mut cp0 = (0, 0);
            for i in 0..n {
                for j in i + 1..n {
                    let dvi = (vx - av[i].0, vy - av[i].1);
                    let dvj = (vx - av[j].0, vy - av[j].1);
                    let dp = (ap[i].0 - ap[j].0, ap[i].1 - ap[j].1);
                    let s = solve(dvi, dvj, dp);
                    if s.is_some() {
                        let (good, ti) = s.unwrap();
                        if good {
                            cp0 = (
                                ap[i].0 as i128 - dvi.0 as i128 * ti,
                                ap[i].1 as i128 - dvi.1 as i128 * ti,
                            );
                            found = true;
                        }
                        done = true;
                    }
                    if done {
                        break;
                    }
                }
                if done {
                    break;
                }
            }
            if found {
                // check dp0

                let mut ok = true;
                let mut t = vec![];
                for i in 0..n {
                    let dp = (cp0.0 - ap[i].0 as i128, cp0.1 - ap[i].1 as i128);
                    let dv = (vx - av[i].0, vy - av[i].1);
                    let mut ct = 0;
                    if dp.0 == 0 && dv.0 == 0 {
                        if dp.1 == 0 && dv.1 == 0 {
                            ct = 0;
                        } else if dv.1 != 0 && dp.1 % dv.1 as i128 == 0 {
                            ct = -dp.1 / dv.1 as i128;
                        } else {
                            ok = false;
                        }
                    } else if dv.0 != 0 && dp.0 % dv.0 as i128 == 0 {
                        ct = -dp.0 / dv.0 as i128;
                    } else {
                        ok = false;
                    }

                    if ok && dp.1 == -ct * dv.1 as i128 {
                        t.push(ct);
                    }

                    if !ok {
                        break;
                    }
                }
                if ok {
                    let mut cpz = 0;
                    let mut vz = 0;
                    let mut done = false;
                    let mut found = false;
                    for i in 0..n {
                        for j in i + 1..n {
                            if t[i] != t[j] {
                                let pp = t[i] * t[j] * (av[i].2 as i128 - av[j].2 as i128)
                                    + ap[i].2 as i128 * t[j]
                                    - ap[j].2 as i128 * t[i];
                                if pp % (t[i] - t[j]) == 0 {
                                    cpz = -pp / (t[i] - t[j]);
                                    if (cpz - ap[i].2 as i128) % t[i] == 0 {
                                        vz = -(cpz - ap[i].2 as i128 - t[i] * av[i].2 as i128)
                                            / t[i];
                                        found = true;
                                    }
                                }
                                done = true;
                            }
                            if done {
                                break;
                            }
                        }
                        if done {
                            break;
                        }
                    }
                    if found {
                        let mut good = true;
                        for i in 0..n {
                            if cpz + t[i] * vz != ap[i].2 as i128 + t[i] * av[i].2 as i128 {
                                good = false;
                                break;
                            }
                        }
                        if good {
                            ans = cp0.0 + cp0.1 + cpz;
                            rdone = true;
                        }
                    }
                }
            }
            if rdone {
                break;
            }
        }
        if rdone {
            break;
        }
    }

    Some(ans as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one_arg(&advent_of_code::template::read_file("examples", DAY), 7, 27);
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(47));
    }
}
