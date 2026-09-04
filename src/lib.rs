//! 农历核心库：自包含、零依赖。
//! 提供公历<->农历换算、干支（年/月/日）、星期、汉字化等，全部内嵌数据表。

pub const GAN: [&str; 10] = ["甲", "乙", "丙", "丁", "戊", "己", "庚", "辛", "壬", "癸"];
pub const ZHI: [&str; 12] = ["子", "丑", "寅", "卯", "辰", "巳", "午", "未", "申", "酉", "戌", "亥"];
pub const SHENGXIAO: [&str; 12] = ["鼠", "牛", "虎", "兔", "龙", "蛇", "马", "羊", "猴", "鸡", "狗", "猪"];
// 周名（周一为首列），index 0 = 星期一
pub const WEEKDAY: [&str; 7] = ["一", "二", "三", "四", "五", "六", "日"];

pub const MONTH_NAME: [&str; 12] = [
    "正月", "二月", "三月", "四月", "五月", "六月", "七月", "八月", "九月", "十月", "冬月", "腊月",
];

/// 农历日汉字：初一..三十（各为两个汉字）
pub const DAY_HANZI: [&str; 30] = [
    "初一", "初二", "初三", "初四", "初五", "初六", "初七", "初八", "初九", "初十", "十一", "十二",
    "十三", "十四", "十五", "十六", "十七", "十八", "十九", "二十", "廿一", "廿二", "廿三", "廿四",
    "廿五", "廿六", "廿七", "廿八", "廿九", "三十",
];

// 农历数据表：1900..=2100，共 201 项。
// 0x10000=闰月30天; bits15..4=正月..十二月是否30天; 低4位=闰月号(0=无闰)。
const LUNAR_INFO: [u32; 201] = [
    0x04bd8, 0x04ae0, 0x0a570, 0x054d5, 0x0d260, 0x0d950, 0x16554, 0x056a0, 0x09ad0, 0x055d2, // 1900-1909
    0x04ae0, 0x0a5b6, 0x0a4d0, 0x0d250, 0x1d255, 0x0b540, 0x0d6a0, 0x0ada2, 0x095b0, 0x14977, // 1910-1919
    0x04970, 0x0a4b0, 0x0b4b5, 0x06a50, 0x06d40, 0x1ab54, 0x02b60, 0x09570, 0x052f2, 0x04970, // 1920-1929
    0x06566, 0x0d4a0, 0x0ea50, 0x16a95, 0x05ad0, 0x02b60, 0x186e3, 0x092e0, 0x1c8d7, 0x0c950, // 1930-1939
    0x0d4a0, 0x1d8a6, 0x0b550, 0x056a0, 0x1a5b4, 0x025d0, 0x092d0, 0x0d2b2, 0x0a950, 0x0b557, // 1940-1949
    0x06ca0, 0x0b550, 0x15355, 0x04da0, 0x0a5d0, 0x14573, 0x052d0, 0x0a9a8, 0x0e950, 0x06aa0, // 1950-1959
    0x0aea6, 0x0ab50, 0x04b60, 0x0aae4, 0x0a570, 0x05260, 0x0f263, 0x0d950, 0x05b57, 0x056a0, // 1960-1969
    0x096d0, 0x04dd5, 0x04ad0, 0x0a4d0, 0x0d4d4, 0x0d250, 0x0d558, 0x0b540, 0x0b5a0, 0x195a6, // 1970-1979
    0x095b0, 0x049b0, 0x0a974, 0x0a4b0, 0x0b27a, 0x06a50, 0x06d40, 0x0af46, 0x0ab60, 0x09570, // 1980-1989
    0x04af5, 0x04970, 0x064b0, 0x074a3, 0x0ea50, 0x06b58, 0x055c0, 0x0ab60, 0x096d5, 0x092e0, // 1990-1999
    0x0c960, 0x0d954, 0x0d4a0, 0x0da50, 0x07552, 0x056a0, 0x0abb7, 0x025d0, 0x092d0, 0x0cab5, // 2000-2009
    0x0a950, 0x0b4a0, 0x0baa4, 0x0ad50, 0x055d9, 0x04ba0, 0x0a5b0, 0x15176, 0x052b0, 0x0a930, // 2010-2019
    0x07954, 0x06aa0, 0x0ad50, 0x05b52, 0x04b60, 0x0a6e6, 0x0a4e0, 0x0d260, 0x0ea65, 0x0d530, // 2020-2029
    0x05aa0, 0x076a3, 0x096d0, 0x04afb, 0x04ad0, 0x0a4d0, 0x1d0b6, 0x0d250, 0x0d520, 0x0dd45, // 2030-2039
    0x0b5a0, 0x056d0, 0x055b2, 0x049b0, 0x0a577, 0x0a4b0, 0x0aa50, 0x1b255, 0x06d20, 0x0ada0, // 2040-2049
    0x14b63, 0x09370, 0x049f8, 0x04970, 0x064b0, 0x168a6, 0x0ea50, 0x06b20, 0x1a6c4, 0x0aae0, // 2050-2059
    0x092e0, 0x0d2e3, 0x0c960, 0x0d557, 0x0d4a0, 0x0da50, 0x05d55, 0x056a0, 0x0a6d0, 0x055d4, // 2060-2069
    0x052d0, 0x0a9b8, 0x0a950, 0x0b4a0, 0x0b6a6, 0x0ad50, 0x055a0, 0x0aba4, 0x0a5b0, 0x052b0, // 2070-2079
    0x0b273, 0x06930, 0x07337, 0x06aa0, 0x0ad50, 0x14b55, 0x04b60, 0x0a570, 0x054e4, 0x0d160, // 2080-2089
    0x0e968, 0x0d520, 0x0daa0, 0x16aa6, 0x056d0, 0x04ae0, 0x0a9d4, 0x0a2d0, 0x0d150, 0x0f252, // 2090-2099
    0x0d520, // 2100
];

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Lunar {
    pub year: i32,     // 农历年号 (1900..=2100)
    pub month: u32,    // 1..=12
    pub day: u32,      // 1..=30
    pub is_leap: bool, // 是否闰月
}

// ---------- 农历表 ----------
pub fn leap_month(y: i32) -> u32 {
    LUNAR_INFO[(y - 1900) as usize] & 0xf
}
pub fn leap_days(y: i32) -> u32 {
    let lm = leap_month(y);
    if lm == 0 {
        0
    } else if (LUNAR_INFO[(y - 1900) as usize] & 0x10000) != 0 {
        30
    } else {
        29
    }
}
pub fn month_days(y: i32, m: u32) -> u32 {
    if (LUNAR_INFO[(y - 1900) as usize] & (0x10000 >> m)) != 0 {
        30
    } else {
        29
    }
}
/// 农历年按序排列的月份槽：(月号, 是否闰月, 天数)。闰月插在其同名月之后。
pub fn lunar_year_slots(y: i32) -> Vec<(u32, bool, u32)> {
    let mut v: Vec<(u32, bool, u32)> = (1..=12).map(|m| (m, false, month_days(y, m))).collect();
    let lm = leap_month(y);
    if lm > 0 {
        let idx = lm as usize; // 插到同名月(0-based lm-1)之后 => 0-based 位置 lm
        v.insert(idx, (lm, true, leap_days(y)));
    }
    v
}

// ---------- 公历 ----------
pub fn is_leap_year(y: i32) -> bool {
    (y % 4 == 0 && y % 100 != 0) || y % 400 == 0
}
pub fn greg_month_days(y: i32, m: u32) -> u32 {
    match m {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 => if is_leap_year(y) { 29 } else { 28 },
        _ => 0,
    }
}
/// 自 1900-01-01(=0) 起的天数。
pub fn serial(y: i32, m: u32, d: u32) -> i64 {
    let mdays = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
    let mut t = 0i64;
    for yy in 1900..y {
        t += if is_leap_year(yy) { 366 } else { 365 };
    }
    let mut t2 = (d - 1) as i64;
    for i in 0..(m as i32 - 1) {
        t2 += mdays[i as usize] as i64;
        if i == 1 && is_leap_year(y) {
            t2 += 1;
        }
    }
    t + t2
}
/// 天数(自1900-1-1) -> 公历 (y,m,d)
pub fn from_serial(s: i64) -> (i32, u32, u32) {
    let mut y = 1900i32;
    let mut left = s;
    loop {
        let len = if is_leap_year(y) { 366 } else { 365 };
        if left < len {
            break;
        }
        left -= len;
        y += 1;
    }
    let mdays = [31, if is_leap_year(y) { 29 } else { 28 }, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
    let mut m = 0usize;
    while left >= mdays[m] {
        left -= mdays[m];
        m += 1;
    }
    (y, (m + 1) as u32, (left + 1) as u32)
}

// ---------- 公历 -> 农历 ----------
pub fn to_lunar(y: i32, m: u32, d: u32) -> Lunar {
    let epoch = serial(1900, 1, 31); // 1900 农历正月初一
    let mut offset = serial(y, m, d) - epoch;

    let mut ly = 1900i32;
    while ly <= 2100 {
        let days: i64 = lunar_year_slots(ly).iter().map(|s| s.2 as i64).sum();
        if offset < days {
            break;
        }
        offset -= days;
        ly += 1;
    }
    if ly > 2100 {
        panic!("日期超出 1900-2100 支持范围");
    }
    let slots = lunar_year_slots(ly);
    let mut idx = 0usize;
    while idx < slots.len() - 1 && offset >= slots[idx].2 as i64 {
        offset -= slots[idx].2 as i64;
        idx += 1;
    }
    let (mnum, is_leap) = (slots[idx].0, slots[idx].1);
    Lunar { year: ly, month: mnum, day: (offset + 1) as u32, is_leap }
}

/// 农历年 `y` 的正月初一对应的公历 (y,m,d)。(农历年 y 的正月初一总落在公历年 y 的 1~2 月)
pub fn lunar_new_year_solar(y: i32) -> (i32, u32, u32) {
    // 在公历年 y 的头 70 天里找第一个 农历(y,1,1)
    for d in 1..=70 {
        if to_lunar(y, 1, d).year == y {
            return (y, 1, d);
        }
    }
    (y, 2, 1)
}

// ---------- 干支 ----------
/// 公历/农历年份 -> 干支年 (stem,branch)。以农历年号为准。
pub fn ganzhi_of_year(y: i32) -> (usize, usize) {
    (((y - 4).rem_euclid(10)) as usize, ((y - 4).rem_euclid(12)) as usize)
}
/// 月干支：农历年 + 农历月号 (1..=12，闰月取同名)。正月=寅支，天干按五虎遁。
pub fn ganzhi_of_month(lunar_year: i32, month: u32) -> (usize, usize) {
    let g = ganzhi_of_year(lunar_year).0; // 年干
    // 五虎遁: 甲己->丙寅(2), 乙庚->戊寅(14), 丙辛->庚寅(26), 丁壬->壬寅(38), 戊癸->甲寅(50)
    let first = [2, 14, 26, 38, 50][g % 5] as usize;
    let idx = (first + (month - 1) as usize) % 60;
    (idx % 10, idx % 12)
}
/// 日干支：由天数序列定。idx = (serial + K) % 60，K 校准自 2026-09-04=辛巳。
pub fn ganzhi_of_day(s: i64) -> (usize, usize) {
    let k = 17i64 - serial(2026, 9, 4);
    let idx = (s + k).rem_euclid(60) as usize;
    (idx % 10, idx % 12)
}
/// 星期序号：0=星期一。1900-01-01 为星期一。
pub fn weekday_index(y: i32, m: u32, d: u32) -> usize {
    (serial(y, m, d).rem_euclid(7)) as usize
}

// ---------- 汉字化 ----------
/// 农历月汉字名（含闰前缀），如 (7,false)->七月, (7,true)->闰七月
pub fn lunar_month_name(month: u32, is_leap: bool) -> String {
    let base = MONTH_NAME[(month - 1) as usize];
    if is_leap {
        format!("闰{}", base)
    } else {
        base.to_string()
    }
}
/// 农历日：汉字(初一..) 或数字(1..)。numeric=true 时输出数字。
pub fn lunar_day_label(day: u32, numeric: bool) -> String {
    if numeric {
        day.to_string()
    } else {
        DAY_HANZI[(day - 1) as usize].to_string()
    }
}
/// 农历年名（干支），如 2026 -> "丙午"
pub fn lunar_year_name(y: i32) -> String {
    let (g, z) = ganzhi_of_year(y);
    format!("{}{}", GAN[g], ZHI[z])
}

/// 终端显示宽度：CJK(非 ascii) 记 2 列。
pub fn disp_width(s: &str) -> usize {
    s.chars().map(|c| if (c as u32) >= 0x80 { 2 } else { 1 }).sum()
}

/// 忽略 SIGPIPE，避免被 `| head` 截断时 panic（标准 CLI 行为）。
pub fn ignore_sigpipe() {
    unsafe {
        extern "C" {
            fn signal(signum: i32, handler: usize) -> usize;
        }
        signal(13, 0); // SIGPIPE=13, SIG_DFL=0
    }
}

// ================= 二十四节气 =================
// 数据来源：香港天文台公布 1901~2100 节气，向量压缩法。下标 0 为 1901 年。
pub const TERM_NAMES: [&str; 24] = [
    "小寒", "大寒", "立春", "雨水", "惊蛰", "春分", "清明", "谷雨", "立夏", "小满", "芒种", "夏至",
    "小暑", "大暑", "立秋", "处暑", "白露", "秋分", "寒露", "霜降", "立冬", "小雪", "大雪", "冬至",
];
// 每个节气所在月的最早可能日期（小寒第0个起，两两一组同月）
const ENC_VECTOR: [u32; 24] = [
    4, 19, 3, 18, 4, 19, 4, 19, 4, 20, 4, 20, 6, 22, 6, 22, 6, 22, 7, 22, 6, 21, 6, 21,
];

/// 某年某节气(0..23，0=小寒)的公历(月,日)。支持 1901..=2100。
pub fn term_date(year: i32, n: usize) -> Option<(u32, u32)> {
    if !(1901..=2100).contains(&year) || n >= 24 {
        return None;
    }
    let data = TERMS_DATA[(year - 1901) as usize];
    let x = data >> (2 * n as u32);
    let month = (n / 2 + 1) as u32;
    let day = ENC_VECTOR[n] + (x & 3) as u32;
    Some((month, day))
}

/// 公历某天若为节气，返回其名。
pub fn jieqi_on(y: i32, m: u32, d: u32) -> Option<&'static str> {
    for n in 0..24 {
        if let Some((tm, td)) = term_date(y, n) {
            if tm == m && td == d {
                return Some(TERM_NAMES[n]);
            }
        }
    }
    None
}

/// 公历某月内的节气：(日, 名)。
pub fn jieqi_in_month(y: i32, m: u32) -> Vec<(u32, &'static str)> {
    let mut out = Vec::new();
    for n in 0..24 {
        if let Some((tm, td)) = term_date(y, n) {
            if tm == m {
                out.push((td, TERM_NAMES[n]));
            }
        }
    }
    out.sort();
    out
}

pub static TERMS_DATA: [u64; 200] = [
    0x6aaaa6aa9a5a, 0xaaaaaabaaa6a, 0xaaabbabbafaa, 0x5aa665a65aab, 0x6aaaa6aa9a5a,
    0xaaaaaaaaaa6a, 0xaaabbabbafaa, 0x5aa665a65aab, 0x6aaaa6aa9a5a, 0xaaaaaaaaaa6a,
    0xaaabbabbafaa, 0x5aa665a65aab, 0x6aaaa6aa9a56, 0xaaaaaaaa9a5a, 0xaaabaabaaeaa,
    0x569665a65aaa, 0x5aa6a6a69a56, 0x6aaaaaaa9a5a, 0xaaabaabaaeaa, 0x569665a65aaa,
    0x5aa6a6a65a56, 0x6aaaaaaa9a5a, 0xaaabaabaaa6a, 0x569665a65aaa, 0x5aa6a6a65a56,
    0x6aaaa6aa9a5a, 0xaaaaaabaaa6a, 0x555665665aaa, 0x5aa665a65a56, 0x6aaaa6aa9a5a,
    0xaaaaaabaaa6a, 0x555665665aaa, 0x5aa665a65a56, 0x6aaaa6aa9a5a, 0xaaaaaaaaaa6a,
    0x555665665aaa, 0x5aa665a65a56, 0x6aaaa6aa9a5a, 0xaaaaaaaaaa6a, 0x555665665aaa,
    0x5aa665a65a56, 0x6aaaa6aa9a5a, 0xaaaaaaaaaa6a, 0x555665655aaa, 0x569665a65a56,
    0x6aa6a6aa9a56, 0xaaaaaaaa9a5a, 0x5556556559aa, 0x569665a65a55, 0x6aa6a6a65a56,
    0xaaaaaaaa9a5a, 0x5556556559aa, 0x569665a65a55, 0x5aa6a6a65a56, 0x6aaaa6aa9a5a,
    0x5556556555aa, 0x569665a65a55, 0x5aa665a65a56, 0x6aaaa6aa9a5a, 0x55555565556a,
    0x555665665a55, 0x5aa665a65a56, 0x6aaaa6aa9a5a, 0x55555565556a, 0x555665665a55,
    0x5aa665a65a56, 0x6aaaa6aa9a5a, 0x55555555556a, 0x555665665a55, 0x5aa665a65a56,
    0x6aaaa6aa9a5a, 0x55555555556a, 0x555665655a55, 0x5aa665a65a56, 0x6aa6a6aa9a5a,
    0x55555555456a, 0x555655655a55, 0x5a9665a65a56, 0x6aa6a6a69a5a, 0x55555555456a,
    0x555655655a55, 0x569665a65a56, 0x6aa6a6a65a56, 0x55555155455a, 0x555655655955,
    0x569665a65a55, 0x5aa6a5a65a56, 0x15555155455a, 0x555555655555, 0x569665665a55,
    0x5aa665a65a56, 0x15555155455a, 0x555555655515, 0x555665665a55, 0x5aa665a65a56,
    0x15555155455a, 0x555555555515, 0x555665665a55, 0x5aa665a65a56, 0x15555155455a,
    0x555555555515, 0x555665665a55, 0x5aa665a65a56, 0x15555155455a, 0x555555555515,
    0x555655655a55, 0x5aa665a65a56, 0x15515155455a, 0x555555554515, 0x555655655a55,
    0x5a9665a65a56, 0x15515151455a, 0x555551554515, 0x555655655a55, 0x569665a65a56,
    0x155151510556, 0x555551554505, 0x555655655955, 0x569665665a55, 0x155110510556,
    0x155551554505, 0x555555655555, 0x569665665a55, 0x055110510556, 0x155551554505,
    0x555555555515, 0x555665665a55, 0x055110510556, 0x155551554505, 0x555555555515,
    0x555665665a55, 0x055110510556, 0x155551554505, 0x555555555515, 0x555655655a55,
    0x055110510556, 0x155551554505, 0x555555555515, 0x555655655a55, 0x055110510556,
    0x155151514505, 0x555555554515, 0x555655655a55, 0x054110510556, 0x155151510505,
    0x555551554515, 0x555655655a55, 0x014110110556, 0x155110510501, 0x555551554505,
    0x555555655555, 0x014110110555, 0x155110510501, 0x555551554505, 0x555555555555,
    0x014110110555, 0x055110510501, 0x155551554505, 0x555555555555, 0x000110110555,
    0x055110510501, 0x155551554505, 0x555555555515, 0x000110110555, 0x055110510501,
    0x155551554505, 0x555555555515, 0x000100100555, 0x055110510501, 0x155151514505,
    0x555555555515, 0x000100100555, 0x054110510501, 0x155151514505, 0x555551554515,
    0x000100100555, 0x054110510501, 0x155150510505, 0x555551554515, 0x000100100555,
    0x014110110501, 0x155110510505, 0x555551554505, 0x000000100055, 0x014110110500,
    0x155110510501, 0x555551554505, 0x000000000055, 0x014110110500, 0x055110510501,
    0x155551554505, 0x000000000055, 0x000110110500, 0x055110510501, 0x155551554505,
    0x000000000015, 0x000100110500, 0x055110510501, 0x155551554505, 0x555555555515
];

// ================= 节日 =================
// 节日 -> 公历某日。清明按节气(太阳)日，其余按农历固定日。
/// 公历某日若为预设传统节日，返回名称。
pub fn festival_on(y: i32, m: u32, d: u32) -> Option<&'static str> {
    let lun = to_lunar(y, m, d);
    // 除夕 = 春节前一天（即：次日为农历正月初一）
    let (ny, nm, nd) = from_serial(serial(y, m, d) + 1);
    let tom = to_lunar(ny, nm, nd);
    if tom.month == 1 && tom.day == 1 && !tom.is_leap {
        return Some("除夕");
    }
    // 清明（节气日）
    if jieqi_on(y, m, d) == Some("清明") {
        return Some("清明");
    }
    // 农历固定节日（闰月不计）
    if !lun.is_leap {
        let f = match (lun.month, lun.day) {
            (1, 1) => Some("春节"),
            (1, 15) => Some("元宵"),
            (5, 5) => Some("端午"),
            (7, 7) => Some("七夕"),
            (7, 15) => Some("中元"),
            (8, 15) => Some("中秋"),
            (9, 9) => Some("重阳"),
            _ => None,
        };
        if f.is_some() {
            return f;
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    // ---------- 公历 -> 农历 ----------
    #[test]
    fn solar_to_lunar_known() {
        let l = to_lunar(2026, 9, 4);
        assert_eq!((l.year, l.month, l.day), (2026, 7, 23));
        assert!(!l.is_leap);

        let l = to_lunar(2026, 1, 15); // 落在农历 2025(乙巳) 冬月
        assert_eq!((l.year, l.month, l.day), (2025, 11, 27));

        let l = to_lunar(2024, 2, 10);
        assert_eq!((l.year, l.month, l.day), (2024, 1, 1));

        let l = to_lunar(2000, 2, 5);
        assert_eq!((l.year, l.month, l.day), (2000, 1, 1));

        let l = to_lunar(1900, 1, 31); // 表起点 = 农历1900正月初一
        assert_eq!((l.year, l.month, l.day), (1900, 1, 1));
    }

    #[test]
    fn leap_month_cases() {
        // 闰月定位
        let l = to_lunar(2023, 3, 22);
        assert_eq!((l.year, l.month, l.day), (2023, 2, 1));
        assert!(l.is_leap);

        let l = to_lunar(2020, 5, 23);
        assert_eq!((l.year, l.month, l.day), (2020, 4, 1));
        assert!(l.is_leap);

        let l = to_lunar(2025, 7, 25);
        assert_eq!((l.year, l.month, l.day), (2025, 6, 1));
        assert!(l.is_leap);

        // 闰月号
        assert_eq!(leap_month(2025), 6);
        assert_eq!(leap_month(2023), 2);
        assert_eq!(leap_month(2020), 4);
        assert_eq!(leap_month(2026), 0);
    }

    #[test]
    fn lunar_month_slots_order() {
        // 2025(闰六月): 1..6, 闰6, 7..12 => 13 个槽
        let slots = lunar_year_slots(2025);
        assert_eq!(slots.len(), 13);
        assert_eq!(slots[6], (6, true, leap_days(2025)));
        // 无闰年的 2026 = 12 槽
        assert_eq!(lunar_year_slots(2026).len(), 12);
    }

    // ---------- serial 往返 ----------
    #[test]
    fn serial_roundtrip() {
        for (y, m, d) in [
            (1900, 2, 1), (1950, 6, 15), (2000, 2, 29), (2026, 9, 4), (2100, 1, 1),
        ] {
            assert_eq!(from_serial(serial(y, m, d)), (y, m, d));
        }
    }

    #[test]
    fn weekday_known() {
        // 1900-01-01 为星期一(索引0)；2026-09-04 为星期五(索引4)
        assert_eq!(weekday_index(1900, 1, 1), 0);
        assert_eq!(weekday_index(2026, 9, 4), 4);
        assert_eq!(weekday_index(2024, 2, 10), 5); // 星期六
    }

    // ---------- 干支 ----------
    fn gzname(g: usize, z: usize) -> String {
        format!("{}{}", GAN[g], ZHI[z])
    }

    #[test]
    fn ganzhi_year() {
        assert_eq!(gzname(ganzhi_of_year(2026).0, ganzhi_of_year(2026).1), "丙午");
        assert_eq!(gzname(ganzhi_of_year(2025).0, ganzhi_of_year(2025).1), "乙巳");
        assert_eq!(gzname(ganzhi_of_year(2024).0, ganzhi_of_year(2024).1), "甲辰");
        assert_eq!(gzname(ganzhi_of_year(2000).0, ganzhi_of_year(2000).1), "庚辰");
    }

    #[test]
    fn ganzhi_month() {
        // 五虎遁：丙年(2026)正月=庚寅，第7月=丙申
        let (g, z) = ganzhi_of_month(2026, 1);
        assert_eq!(gzname(g, z), "庚寅");
        let (g, z) = ganzhi_of_month(2026, 7);
        assert_eq!(gzname(g, z), "丙申");
        // 乙年(2025) 冬月(11月)=戊子
        let (g, z) = ganzhi_of_month(2025, 11);
        assert_eq!(gzname(g, z), "戊子");
    }

    #[test]
    fn ganzhi_day() {
        let (g, z) = ganzhi_of_day(serial(2026, 9, 4));
        assert_eq!(gzname(g, z), "辛巳");
        let (g, z) = ganzhi_of_day(serial(2026, 1, 15));
        assert_eq!(gzname(g, z), "己丑");
        let (g, z) = ganzhi_of_day(serial(2020, 5, 23));
        assert_eq!(gzname(g, z), "丙寅");
    }

    // ---------- 节气 ----------
    #[test]
    fn solar_terms_known() {
        // 小寒/立春/白露/秋分/冬至 (2026)
        assert_eq!(term_date(2026, 0), Some((1, 5)));
        assert_eq!(term_date(2026, 2), Some((2, 4)));
        assert_eq!(term_date(2026, 16), Some((9, 7)));
        assert_eq!(term_date(2026, 17), Some((9, 23)));
        assert_eq!(term_date(2026, 23), Some((12, 22)));
        // 权威: 2020 小暑 = 7/6 (部分旧版 lunar-date 误为 7/7)
        assert_eq!(term_date(2020, 12), Some((7, 6)));
        assert_eq!(jieqi_on(2026, 4, 5), Some("清明"));
    }

    // ---------- 节日 ----------
    #[test]
    fn festivals_known() {
        // 春节/除夕
        assert_eq!(festival_on(2024, 2, 10), Some("春节"));
        assert_eq!(festival_on(2026, 2, 17), Some("春节"));
        assert_eq!(festival_on(2026, 2, 16), Some("除夕"));
        // 2026 元宵/端午/七夕/中元/中秋/重阳
        assert_eq!(festival_on(2026, 3, 3), Some("元宵"));
        assert_eq!(festival_on(2026, 6, 19), Some("端午"));
        assert_eq!(festival_on(2026, 8, 19), Some("七夕"));
        assert_eq!(festival_on(2026, 8, 27), Some("中元"));
        assert_eq!(festival_on(2026, 9, 25), Some("中秋"));
        assert_eq!(festival_on(2026, 10, 18), Some("重阳"));
        // 端午公历锚点(法定假日)
        assert_eq!(festival_on(2024, 6, 10), Some("端午"));
        // 普通日无节日
        assert_eq!(festival_on(2026, 9, 4), None);
    }
}
