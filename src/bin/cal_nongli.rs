// cal_nongli —— 仿 cal：公历月叠加农历日，或单独显示农历月。
// 特性：默认在“初一”显示月份名(正月..腊月)，默认以传统节日覆盖日期。
// 选项：--no-month-name / --no-festival / -s,-m 周起始 / -y 整年 / -3 三联 / -n N连月
use std::process::exit;
use clap::Parser;
use nongli::*;

const COLS: usize = 7;
const CW: usize = 7;

fn pad_right_disp(s: &str, w: usize) -> String {
    let mut out = s.to_string();
    while disp_width(&out) < w {
        out.push(' ');
    }
    out
}

#[derive(Clone, Copy)]
struct Opts {
    number: bool,      // 农历日用数字
    month_name: bool,  // 初一显示为月份名
    festival: bool,    // 节日覆盖
}

#[derive(Parser)]
#[command(
    name = "cal_nongli",
    version = env!("LONG_VERSION"),
    author = "nth233 <mrnothing233@gmail.com>",
    about = "cal_nongli - 中国农历版 cal",
    long_about = "cal_nongli - 中国农历版 cal\n\n在公历月内叠加农历日/节气/节日，或单独显示农历月。\n用法仿 cal：无参=当月，单参=整年，双参=指定年月；-L 切农历月。\n\n作者: nth233 <mrnothing233@gmail.com>"
)]
struct Args {
    /// 单独显示农历月
    #[arg(short = 'L', long = "lunar")]
    lunar: bool,
    /// (-L 时)选择闰月
    #[arg(short = 'R', long = "leap")]
    leap: bool,
    /// 周日作为一周第一天(默认周一)
    #[arg(short = 's', long = "sunday")]
    sunday: bool,
    /// 周一作为一周第一天
    #[arg(short = 'm', long = "monday")]
    monday: bool,
    /// 整年(缺省当前年)
    #[arg(short = 'y', long = "year")]
    year: bool,
    /// 显示连续 3 个月
    #[arg(short = '3', long = "three")]
    three: bool,
    /// 显示连续 N 个月
    #[arg(short = 'n', long = "months")]
    months: Option<i32>,
    /// 农历日用数字(默认汉字)
    #[arg(long = "number")]
    number: bool,
    /// 关闭“初一显示为月份名”
    #[arg(long = "no-month-name")]
    no_month_name: bool,
    /// 关闭节日覆盖
    #[arg(long = "no-festival")]
    no_festival: bool,
    /// 位置参数：公历年/公历月，或 -L 农历年/农历月
    #[arg(value_name = "年 [月]", num_args = 0..=2)]
    rest: Vec<i32>,
}

fn today_ymd() -> (i32, u32, u32) {
    let out = std::process::Command::new("date").args(["+%Y %m %d"]).output().expect("date");
    let s = String::from_utf8_lossy(&out.stdout);
    let mut it = s.split_whitespace();
    let y = it.next().and_then(|x| x.parse().ok()).unwrap_or(2026);
    let m = it.next().and_then(|x| x.parse().ok()).unwrap_or(1);
    let d = it.next().and_then(|x| x.parse().ok()).unwrap_or(1);
    (y, m, d)
}

/// col0: 第 0 列(周一)在 WEEKDAY 里的下标。默认周一=0，周日起始=6。
fn render(title: &str, days: &[(String, String)], col0: usize, first_wd: usize) {
    println!("{title}");
    let mut hdr = String::new();
    for k in 0..COLS {
        hdr.push_str(&pad_right_disp(WEEKDAY[(col0 + k) % 7], CW));
    }
    println!("{}", hdr.trim_end());
    let lead = (first_wd + 7 - col0) % 7;
    let mut cells: Vec<Option<(String, String)>> = vec![None; lead];
    for (t, b) in days {
        cells.push(Some((t.clone(), b.clone())));
    }
    while cells.len() % COLS != 0 {
        cells.push(None);
    }
    for row in cells.chunks(COLS) {
        let mut top = String::new();
        let mut bot = String::new();
        for c in row {
            match c {
                Some((t, b)) => {
                    top.push_str(&pad_right_disp(t, CW));
                    bot.push_str(&pad_right_disp(b, CW));
                }
                None => {
                    top.push_str(&" ".repeat(CW));
                    bot.push_str(&" ".repeat(CW));
                }
            }
        }
        println!("{}", top.trim_end());
        println!("{}", bot.trim_end());
    }
}

/// 公历视图某日底行：节日 > 初一显示月份 > 节气 > 农历日
fn greg_cell_label(y: i32, m: u32, d: u32, o: Opts) -> String {
    let lun = to_lunar(y, m, d);
    if o.festival {
        if let Some(f) = festival_on(y, m, d) {
            return f.to_string();
        }
    }
    if o.month_name && lun.day == 1 {
        return lunar_month_name(lun.month, lun.is_leap);
    }
    if let Some(t) = jieqi_on(y, m, d) {
        return t.to_string();
    }
    lunar_day_label(lun.day, o.number)
}

fn render_greg_month(y: i32, m: u32, col0: usize, o: Opts) {
    let nd = greg_month_days(y, m);
    let first_wd = weekday_index(y, m, 1);
    let mut days = Vec::new();
    for d in 1..=nd {
        days.push((format!("{d}"), greg_cell_label(y, m, d, o)));
    }
    render(&format!("      {}年{}月", y, m), &days, col0, first_wd);
}

fn lunar_cell_label(solar_y: i32, m: u32, d: u32, day: u32, o: Opts) -> String {
    if o.festival {
        if let Some(f) = festival_on(solar_y, m, d) {
            return f.to_string();
        }
    }
    lunar_day_label(day, o.number)
}

fn render_lunar_slot(ly: i32, slot_idx: usize, col0: usize, o: Opts) {
    let slots = lunar_year_slots(ly);
    let (month, is_leap, ndays) = slots[slot_idx];
    let (cny_y, cny_m, cny_d) = lunar_new_year_solar(ly);
    let mut start = serial(cny_y, cny_m, cny_d);
    for i in 0..slot_idx {
        start += slots[i].2 as i64;
    }
    let mut days = Vec::new();
    for j in 0..ndays {
        let (sy, sm, sd) = from_serial(start + j as i64);
        days.push((format!("{sm}/{sd}"), lunar_cell_label(sy, sm, sd, j + 1, o)));
    }
    let (fy, fm, fd) = from_serial(start);
    let first_wd = weekday_index(fy, fm, fd);
    let name = lunar_month_name(month, is_leap);
    render(&format!("     农历 {}年 {}", lunar_year_name(ly), name), &days, col0, first_wd);
}

fn add_months(y: i32, m: u32, k: i32) -> (i32, u32) {
    let total = (y as i64) * 12 + (m as i64 - 1) + (k as i64);
    let yy = total.div_euclid(12) as i32;
    let mm = total.rem_euclid(12) as u32 + 1;
    (yy, mm)
}

fn main() {
    nongli::ignore_sigpipe();
    let a = Args::parse();
    let o = Opts {
        number: a.number,
        month_name: !a.no_month_name,
        festival: !a.no_festival,
    };
    let col0 = if a.sunday { 6 } else { 0 }; // 默认周一(0)
    let today = today_ymd();

    if a.lunar {
        // 农历模式（不支持 -3/-n/-y，用位置参数控制）
        match a.rest.len() {
            0 => {
                let (fy, fm, fd) = (today.0, today.1, today.2);
                let cur = to_lunar(fy, fm, fd);
                let idx = cur.is_leap
                    .then(|| lunar_year_slots(cur.year).iter().position(|(mo, lp, _)| *mo == cur.month && *lp))
                    .flatten()
                    .or_else(|| {
                        if a.leap {
                            lunar_year_slots(cur.year).iter().position(|(mo, lp, _)| *mo == cur.month && *lp)
                        } else {
                            None
                        }
                    })
                    .unwrap_or_else(|| {
                        lunar_year_slots(cur.year).iter().position(|(mo, lp, _)| *mo == cur.month && !*lp).unwrap_or(0)
                    });
                render_lunar_slot(cur.year, idx, col0, o);
            }
            1 => {
                let ly = a.rest[0];
                check_ly(ly);
                let slots = lunar_year_slots(ly);
                for i in 0..slots.len() {
                    render_lunar_slot(ly, i, col0, o);
                    println!();
                }
            }
            2 => {
                let ly = a.rest[0];
                let lm = a.rest[1] as u32;
                check_ly(ly);
                if !(1..=12).contains(&lm) {
                    eprintln!("农历月需 1..12");
                    exit(1);
                }
                let slots = lunar_year_slots(ly);
                let idx = if a.leap {
                    match slots.iter().position(|(mo, lp, _)| *mo == lm && *lp) {
                        Some(i) => i,
                        None => {
                            eprintln!("农历 {}年没有闰{}月", lunar_year_name(ly), MONTH_NAME[(lm - 1) as usize]);
                            exit(1);
                        }
                    }
                } else {
                    slots.iter().position(|(mo, lp, _)| *mo == lm && !*lp).unwrap_or(0)
                };
                render_lunar_slot(ly, idx, col0, o);
            }
            _ => unreachable!(),
        }
        return;
    }

    // ============ 公历模式 ============
    let pos = &a.rest;
    // 确定“锚定月份”与起始年份/月
    let anchor = if pos.len() >= 2 {
        (pos[0], pos[1] as u32)
    } else {
        (today.0, today.1)
    };
    let n_months = if let Some(n) = a.months {
        if n < 1 {
            eprintln!("-n 需 >= 1");
            exit(1);
        }
        n
    } else if a.three {
        3
    } else {
        0
    };
    if n_months > 0 {
        let start_k = if a.three { -1 } else { 0 };
        for k in start_k..(start_k + n_months) {
            let (yy, mm) = add_months(anchor.0, anchor.1, k);
            render_greg_month(yy, mm, col0, o);
            println!();
        }
        return;
    }
    // 单个/整年
    let full_year = a.year || pos.len() == 1;
    let (y, m) = match pos.len() {
        0 => (if a.year { today.0 } else { today.0 }, if a.year { 0 } else { today.1 }),
        1 => (pos[0], 0),
        2 => (pos[0], pos[1] as u32),
        _ => unreachable!(),
    };
    check_gy(y);
    if full_year || m == 0 {
        for mm in 1..=12 {
            render_greg_month(y, mm, col0, o);
            println!();
        }
    } else {
        if !(1..=12).contains(&m) {
            eprintln!("月份需 1..12");
            exit(1);
        }
        render_greg_month(y, m, col0, o);
    }
}

fn check_ly(y: i32) {
    if !(1900..=2100).contains(&y) {
        eprintln!("农历年需 1900..2100");
        exit(1);
    }
}
fn check_gy(y: i32) {
    if !(1900..=2100).contains(&y) {
        eprintln!("年份需在 1900..2100");
        exit(1);
    }
}
