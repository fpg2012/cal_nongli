// date_nongli —— 仿 date：输出某一天的农历档案；支持自定义格式。
use std::env;
use std::process::exit;
use clap::Parser;
use nongli::*;

#[derive(Parser)]
#[command(
    name = "date_nongli",
    version = env!("LONG_VERSION"),
    author = "nth233 <mrnothing233@gmail.com>",
    about = "date_nongli - 中国农历版 date",
    long_about = "date_nongli - 中国农历版 date\n\n输出某天的农历档案（公历/星期/农历/干支/生肖/节气），并支持自定义格式。\n用法仿 date：-d 指定日期；-f 传格式串自行排版。\n\n作者: nth233 <mrnothing233@gmail.com>"
)]
struct Args {
    /// 日期，date 风格，如 '2026-09-04'（缺省为今天）
    #[arg(short = 'd', long = "date")]
    date: Option<String>,

    /// 自定义输出格式（见令牌说明）
    #[arg(short = 'f', long = "format")]
    format: Option<String>,

    /// 位置参数：年 月 日（或只给字符串日期）
    #[arg(value_name = "年 月 日", num_args = 0..=3, allow_hyphen_values = true)]
    rest: Vec<String>,
}

fn today() -> (i32, u32, u32) {
    let out = std::process::Command::new("date").args(["+%Y %m %d"]).output().expect("date");
    let s = String::from_utf8_lossy(&out.stdout);
    let mut it = s.split_whitespace();
    let y = it.next().and_then(|x| x.parse().ok()).unwrap_or(2026);
    let m = it.next().and_then(|x| x.parse().ok()).unwrap_or(1);
    let d = it.next().and_then(|x| x.parse().ok()).unwrap_or(1);
    (y, m, d)
}

fn parse_ymd(s: &str) -> Option<(i32, u32, u32)> {
    let nums: Vec<&str> = s.split(['-', '/', '.', ' ']).filter(|t| !t.is_empty()).collect();
    if nums.len() == 3 {
        Some((nums[0].parse().ok()?, nums[1].parse().ok()?, nums[2].parse().ok()?))
    } else {
        None
    }
}

fn help_fmt() -> String {
    "格式令牌(在 -f/--format 中):\n\
        %Y 公历年   %m 公历月   %d 公历日   %A 星期几单字(一~日)\n\
        %G 农历年干支(丙午)  %M 农历月汉字(正月/闰六月/腊月)  %N 农历日汉字(初一)\n\
        %n 农历日数字(23)  %H 干支月(丙申)  %D 干支日(辛巳)\n\
        %S 生肖(马)  %Q 节气(当日无则空)  %% 字面%\n\
        %A 只给单字，前缀自理: 星期%A=星期一 / 周%A=周一 / 礼拜%A=礼拜一\n\
        \\n 换行  \\t 制表符\n\
        例: date_nongli -f '%G年%M%N，星期%A，%Q' -d 2026-09-07"
        .to_string()
}

fn main() {
    nongli::ignore_sigpipe();
    let a = Args::parse();
    if env::args().any(|x| x == "--help-format") {
        println!("{}", help_fmt());
        exit(0);
    }

    // 解析目标日期
    let (y, m, d) = if let Some(s) = &a.date {
        match parse_ymd(s) {
            Some(t) => t,
            None => {
                eprintln!("无法解析日期: {s}\n{}", help_fmt());
                exit(2);
            }
        }
    } else if !a.rest.is_empty() {
        let mut nums: Vec<&str> = Vec::new();
        for t in &a.rest {
            for part in t.split(['-', '/', '.']) {
                if !part.is_empty() {
                    nums.push(part);
                }
            }
        }
        if nums.len() == 3 {
            let ok = (nums[0].parse::<i32>().ok(), nums[1].parse::<u32>().ok(), nums[2].parse::<u32>().ok());
            match ok {
                (Some(yy), Some(mm), Some(dd)) => (yy, mm, dd),
                _ => {
                    eprintln!("无法解析日期参数\n{}", help_fmt());
                    exit(2);
                }
            }
        } else {
            eprintln!("无法解析日期参数\n{}", help_fmt());
            exit(2);
        }
    } else {
        today()
    };

    if !(1900..=2100).contains(&y) {
        eprintln!("年份需在 1900..2100");
        exit(1);
    }
    let gd = greg_month_days(y, m);
    if gd == 0 || !(1..=gd).contains(&d) {
        eprintln!("无效日期");
        exit(1);
    }

    let lun = to_lunar(y, m, d);
    let wd = weekday_index(y, m, d);
    // 只给星期几的单字(一~日)，供用户自加前缀：星期%A/周%A/礼拜%A 等。
    let wdname = WEEKDAY[wd].to_string();

    match &a.format {
        Some(fmt) => {
            print_expanded(&fmt, y, m, d, &wdname, &lun);
        }
        None => {
            let (gy, gz) = ganzhi_of_year(lun.year);
            let (mg, mz) = ganzhi_of_month(lun.year, lun.month);
            let (dg, dz) = ganzhi_of_day(serial(y, m, d));
            println!("公历：{}年{}月{}日 星期{}", y, m, d, WEEKDAY[wd]);
            println!(
                "农历：{}年{}{}",
                lunar_year_name(lun.year),
                lunar_month_name(lun.month, lun.is_leap),
                lunar_day_label(lun.day, false)
            );
            println!("干支：{}{}年 {}{}月 {}{}日", GAN[gy], ZHI[gz], GAN[mg], ZHI[mz], GAN[dg], ZHI[dz]);
            println!("生肖：{}", SHENGXIAO[ganzhi_of_year(lun.year).1]);
            if let Some(t) = jieqi_on(y, m, d) {
                println!("节气：{}", t);
            }
        }
    }
}

fn print_expanded(fmt: &str, y: i32, m: u32, d: u32, wdname: &str, lun: &Lunar) {
    let chars: Vec<char> = fmt.chars().collect();
    let mut i = 0;
    while i < chars.len() {
        let c = chars[i];
        if c == '\\' && i + 1 < chars.len() {
            i += 1;
            match chars[i] {
                'n' => print!("\n"),
                't' => print!("\t"),
                other => print!("\\{other}"),
            }
            i += 1;
            continue;
        }
        if c == '%' && i + 1 < chars.len() {
            i += 1;
            let t = chars[i];
            match t {
                '%' => print!("%"),
                'Y' => print!("{y}"),
                'm' => print!("{m}"),
                'd' => print!("{d}"),
                'A' => print!("{wdname}"),
                'G' => print!("{}", lunar_year_name(lun.year)),
                'M' => print!("{}", lunar_month_name(lun.month, lun.is_leap)),
                'N' => print!("{}", lunar_day_label(lun.day, false)),
                'n' => print!("{}", lun.day),
                'S' => print!("{}", SHENGXIAO[ganzhi_of_year(lun.year).1]),
                other => {
                    // 干支月/日、节气
                    match other {
                        'H' => {
                            let (mg, mz) = ganzhi_of_month(lun.year, lun.month);
                            print!("{}{}", GAN[mg], ZHI[mz]);
                        }
                        'D' => {
                            let (dg, dz) = ganzhi_of_day(serial(y, m, d));
                            print!("{}{}", GAN[dg], ZHI[dz]);
                        }
                        'Q' => {
                            if let Some(q) = jieqi_on(y, m, d) {
                                print!("{q}");
                            }
                        }
                        _ => {
                            print!("%{t}");
                        }
                    }
                }
            }
            i += 1;
            continue;
        }
        print!("{c}");
        i += 1;
    }
    println!();
}
