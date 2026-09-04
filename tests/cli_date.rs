// date_nongli 的 CLI/格式集成测试：直接运行二进制比对输出。
use std::process::Command;

fn run(args: &[&str]) -> String {
    let out = Command::new(env!("CARGO_BIN_EXE_date_nongli"))
        .args(args)
        .output()
        .expect("运行 date_nongli 失败");
    assert!(out.status.success(), "exit={:?} stderr={:?}", out.status.code(), out.stderr);
    String::from_utf8_lossy(&out.stdout).into_owned()
}

#[test]
fn default_output_no_format() {
    let s = run(&["-d", "2026-09-04"]);
    assert_eq!(
        s,
        "公历：2026年9月4日 星期五\n\
         农历：丙午年七月廿三\n\
         干支：丙午年 丙申月 辛巳日\n\
         生肖：马\n"
    );
}

#[test]
fn default_output_has_term_line() {
    // 2026-09-07 白露，默认会多一行 节气
    let s = run(&["-d", "2026-09-07"]);
    assert!(s.contains("节气：白露"));
    assert!(s.contains("农历：丙午年七月廿六"));
}

#[test]
fn format_solar() {
    // %A 只给星期几单字，前缀自理
    assert_eq!(run(&["-f", "%Y年%m月%d日 星期%A", "-d", "2026-09-04"]), "2026年9月4日 星期五\n");
    assert_eq!(run(&["-f", "周%A", "-d", "2026-09-04"]), "周五\n");
    assert_eq!(run(&["-f", "%A", "-d", "2026-09-04"]), "五\n");
}

#[test]
fn format_lunar() {
    assert_eq!(run(&["-f", "%G年%M%N", "-d", "2026-09-04"]), "丙午年七月廿三\n");
    // 闰月
    assert_eq!(run(&["-f", "%G年%M%N", "-d", "2023-03-22"]), "癸卯年闰二月初一\n");
}

#[test]
fn format_number_lunar_day() {
    assert_eq!(run(&["-f", "%G%M%n", "-d", "2026-09-04"]), "丙午七月23\n");
}

#[test]
fn format_ganzhi() {
    assert_eq!(run(&["-f", "%G年%H月%D日", "-d", "2026-09-04"]), "丙午年丙申月辛巳日\n");
    // 跨农历年的月柱：2026-01-15 属乙巳年，冬月=戊子月
    assert_eq!(run(&["-f", "%G年%H月%D日", "-d", "2026-01-15"]), "乙巳年戊子月己丑日\n");
}

#[test]
fn format_term_and_zodiac() {
    assert_eq!(run(&["-f", "节气%Q", "-d", "2026-09-07"]), "节气白露\n");
    // 非节气日 %Q 为空
    assert_eq!(run(&["-f", "[%Q]", "-d", "2026-09-04"]), "[]\n");
    assert_eq!(run(&["-f", "%S", "-d", "2026-09-04"]), "马\n");
}

#[test]
fn format_escape_and_percent() {
    // %% -> 字面 %, \n -> 换行, \t -> 制表符
    assert_eq!(run(&["-f", "100%%", "-d", "2026-09-04"]), "100%\n");
    assert_eq!(run(&["-f", "a\\nb\\tc", "-d", "2026-09-04"]), "a\nb\tc\n");
}

#[test]
fn numeric_positional_equals_date() {
    let a = run(&["-f", "%Y-%m-%d", "2026", "9", "4"]);
    let b = run(&["-f", "%Y-%m-%d", "-d", "2026-09-04"]);
    assert_eq!(a, b);
    assert_eq!(a, "2026-9-4\n");
}
