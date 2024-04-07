use regex::Regex;
use std::env::args;

fn main() {
    let args = args().collect::<Vec<_>>();
    if args.len() != 3 {
        panic!(
            "there should be 2 arguments but you have given me {}",
            args.len() - 1
        );
    }
    if !is_hex(&args[1]) {
        panic!("{} is not a valid color", args[1]);
    }
    if !is_hex(&args[2]) {
        panic!("{} is not a valid color", args[2]);
    }
    let fg = expand(&args[1]);
    let bg = expand(&args[2]);
    let score = contrast(&fg, &bg).unwrap_or(0.);
    println!(
        "contrast ratio: {}",
        Regex::new("[^0-9.tolw -]")
            .unwrap()
            .replace_all(&format!("{:?}", contrast(&fg, &bg)), "")
    );
    for x in [15., 30., 45., 60., 75., 90.]
        .iter()
        .filter(|x| **x > score.abs())
    {
        let mut better = vec![];
        for i in 0..=0xffffff {
            if contrast(&format!("{i:06x}"), &bg).unwrap_or(0.).abs() >= *x {
                better.push(format!("{i:06x}"));
            }
        }
        let dists = better.iter().map(|b| dist(b, &fg)).collect::<Vec<_>>();
        let mut better = dists.iter().zip(better.iter()).collect::<Vec<_>>();
        better.sort_by(|(a, _), (b, _)| a.partial_cmp(b).unwrap());
        if better.is_empty() {
            println!("changing the text color will not result in a score above ±{x} :(");
            break;
        } else {
            println!("closest text color with score above ±{x}: {}", better[0].1);
        }
    }
    for x in [15., 30., 45., 60., 75., 90.]
        .iter()
        .filter(|x| **x > score.abs())
    {
        let mut better = vec![];
        for i in 0..=0xffffff {
            if contrast(&fg, &format!("{i:06x}")).unwrap_or(0.).abs() >= *x {
                better.push(format!("{i:06x}"));
            }
        }
        let dists = better.iter().map(|b| dist(b, &bg)).collect::<Vec<_>>();
        let mut better = dists.iter().zip(better.iter()).collect::<Vec<_>>();
        better.sort_by(|(a, _), (b, _)| a.partial_cmp(b).unwrap());
        if better.is_empty() {
            println!("changing the background color will not result in a score above ±{x} :(");
            break;
        } else {
            println!("closest background color with score above ±{x}: {}", better[0].1);
        }
    }
}
fn is_hex(c: &str) -> bool {
    Regex::new("^#?[0-9a-f]{1,6}$").unwrap().is_match(c)
}
fn expand(c: &str) -> String {
    if c.len() == 3 {
        c.chars()
            .map(|x| x.to_string().repeat(2))
            .collect::<String>()
    } else {
        c.repeat((6. / c.len() as f32).ceil() as usize)
    }
}
struct Rgb {
    r: u8,
    g: u8,
    b: u8,
}
fn to_rgb(c: &str) -> Rgb {
    let c = u32::from_str_radix(c, 16).unwrap();
    Rgb {
        r: (c >> 16) as u8,
        g: (c >> 8) as u8,
        b: c as u8,
    }
}
fn y(c: Rgb) -> f32 {
    0.2126729 * (c.r as f32 / 255.).powf(2.4)
        + 0.715122 * (c.g as f32 / 255.).powf(2.4)
        + 0.072175 * (c.b as f32 / 255.).powf(2.4)
}
fn contrast(fg: &str, bg: &str) -> Result<f32, String> {
    let mut y_fg = y(to_rgb(fg));
    let mut y_bg = y(to_rgb(bg));
    if y_fg < 0.022 {
        y_fg = (0.022 - y_fg).powf(1.414) + y_fg;
    }
    if y_bg < 0.022 {
        y_bg = (0.022 - y_bg).powf(1.414) + y_bg;
    }
    let mut lc = 1.14
        * if y_fg > y_bg {
            y_bg.powf(0.56) - y_fg.powf(0.57)
        } else {
            y_bg.powf(0.65) - y_fg.powf(0.62)
        };
    if lc.abs() < 0.1 {
        return Err("too low".to_string());
    }
    lc = (lc + if lc > 0. { -1. } else { 1. } * 0.027) * 100.;
    Ok(lc)
}
fn dist(a: &str, b: &str) -> f32 {
    let a = to_rgb(a);
    let b = to_rgb(b);
    let a = (a.r as i32, a.g as i32, a.b as i32);
    let b = (b.r as i32, b.g as i32, b.b as i32);
    (((b.0 - a.0).pow(2) + (b.1 - a.1).pow(2) + (b.2 - a.2).pow(2)) as f32).sqrt()
}
