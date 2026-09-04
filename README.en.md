# cal_nongli

> 中文: [README.md](README.md)

Chinese lunar calendar command-line tools. One crate provides two commands backed by a
single self-contained core (Gregorian/lunar conversion, ganzhi (sexagenary), 24 solar terms,
traditional festivals). No daemon, no runtime dependencies, pure Rust with clap.

## Build

Requires the [Rust toolchain](https://www.rust-lang.org/) (cargo).

```sh
cd cal_nongli
cargo build --release
# binaries in target/release/{date_nongli, cal_nongli}
```

## Install

Either:

- Install via cargo (goes to `~/.cargo/bin`; usable once it is on your PATH):
  ```sh
  cargo install --path .
  ```
- Or copy the binaries manually:
  ```sh
  cargo build --release
  install -Dm755 target/release/date_nongli ~/.local/bin/
  install -Dm755 target/release/cal_nongli ~/.local/bin/
  ```

The version string comes from git: a tag shows as `vX.Y`, otherwise the short commit hash
(`-dirty` is appended when there are uncommitted changes). Run `-V` to view it.

## Usage

### date_nongli - a date-like tool: print the lunar profile of a day

```sh
date_nongli                    # today (solar/weekday/lunar/ganzhi/zodiac/solar term)
date_nongli -d 2026-09-04     # a specific date (date style)
date_nongli 2026 9 4           # or: year month day
date_nongli -f 'FORMAT'        # custom layout
```

Format tokens for `-f` (see also `date_nongli --help-format`):

| token | meaning            | token | meaning                        |
|-------|--------------------|-------|--------------------------------|
| `%Y`  | solar year         | `%G`  | lunar ganzhi year (丙午)        |
| `%m`  | solar month        | `%M`  | lunar month (七月/闰六月/腊月)  |
| `%d`  | solar day          | `%N`  | lunar day, hanzi (初一)         |
| `%A`  | weekday, single hanzi (五) | `%n` | lunar day, number             |
| `%S`  | zodiac (马)         | `%H`  | ganzhi month (丙申)             |
| `%Q`  | solar term (or empty) | `%D` | ganzhi day (辛巳)               |

`%A` is just the single hanzi for the weekday (一~日); you supply the prefix yourself, so any of `星期%A`, `周%A` or `礼拜%A` works. `\n`, `\t` and `%%` are supported. Examples:

```sh
date_nongli -f '%G年%M%N，星期%A' -d 2026-09-07
# 丙午年七月廿六，星期一

date_nongli -f '周%A' -d 2026-09-07
# 周一
```

### cal_nongli - a cal-like tool: Gregorian months with lunar days, or a lunar month

```sh
cal_nongli                    # current Gregorian month (each cell: solar day on top, lunar day/solar term/festival below)
cal_nongli 2026               # a full Gregorian year
cal_nongli 2026 9             # a specific Gregorian year/month
cal_nongli -L                 # current lunar month (same usage shape as without -L)
cal_nongli -L 2026            # a full lunar year (leap months included automatically)
cal_nongli -L 2026 7          # a specific lunar month; add -R for the leap month (e.g. -L 2020 4 -R)
```

Common options:
- `-s` / `-m`: Sunday / Monday as the first column (Monday is the default)
- `-y`: full current Gregorian year; `-3`: three consecutive months; `-n N`: N consecutive months
- `--number`: lunar days as numerals (hanzi by default)
- `--no-month-name`: disable showing the month name on the 1st lunar day (default: `正月/二月/.../腊月`)
- `--no-festival`: disable festival overlay (default shows 除夕/春节/元宵/清明/端午/七夕/中元/中秋/重阳)

Cell priority: festival > month-name on 1st lunar day > solar term > lunar day.

## Examples

### date_nongli

Print the lunar profile of a single day, including ganzhi and the solar term (if any):

```
$ date_nongli -d 2026-09-07
公历：2026年9月7日 星期一
农历：丙午年七月廿六
干支：丙午年 丙申月 甲申日
生肖：马
节气：白露
```

Leap lunar months are shown as "闰×月":

```
$ date_nongli -d 2020-05-23
公历：2020年5月23日 星期六
农历：庚子年闰四月初一
干支：庚子年 辛巳月 丙寅日
生肖：鼠
```

Chinese New Year's day (the first day of the lunar year):

```
$ date_nongli 2026 2 17
公历：2026年2月17日 星期二
农历：丙午年正月初一
干支：丙午年 庚寅月 壬戌日
生肖：马
```

Custom layout; tokens support `\n`, `\t` and `%%`:

```
$ date_nongli -f '%G年%M%N，星期%A\n干支日：%D' -d 2026-09-07
丙午年七月廿六，星期一
干支日：甲申

$ date_nongli -f '周%A 农历%M%N（日序 %n），生肖%S，节气：%Q' -d 2026-09-07
周一 农历七月廿六（日序 26），生肖马，节气：白露
```

### cal_nongli - Gregorian month overlaid with lunar days (Monday first by default)

```
$ cal_nongli 2026 9
      2026年9月
一     二     三     四     五     六     日
       1      2      3      4      5      6
       二十   廿一   廿二   廿三   廿四   廿五
7      8      9      10     11     12     13
白露   廿七   廿八   廿九   八月   初二   初三
14     15     16     17     18     19     20
初四   初五   初六   初七   初八   初九   初十
21     22     23     24     25     26     27
十一   十二   秋分   十四   中秋   十六   十七
28     29     30
十八   十九   二十
```

Here the 11th is the 1st of lunar month 八月 and, per the default "month-name on the 1st lunar day", shows 八月; the 7th is overlaid by the 白露 solar term (really 廿六), the 23rd by 秋分 (really 十三), and the 25th by the 中秋 festival (really 十五). Switch the first weekday column with `-s` (Sunday) or `-m` (Monday):

```
$ cal_nongli 2026 9 -s
      2026年9月
日     一     二     三     四     五     六
              1      2      3      4      5
              二十   廿一   廿二   廿三   廿四
6      7      8      9      10     11     12
廿五   白露   廿七   廿八   廿九   八月   初二
13     14     15     16     17     18     19
初三   初四   初五   初六   初七   初八   初九
20     21     22     23     24     25     26
初十   十一   十二   秋分   十四   中秋   十六
27     28     29     30
十七   十八   十九   二十
```

### cal_nongli - lunar month view (each cell: Gregorian month/day on top, lunar day/festival below)

Current lunar month (丙午年七月, showing 七夕 and 中元):

```
$ cal_nongli -L
     农历 丙午年 七月
一     二     三     四     五     六     日
                     8/13   8/14   8/15   8/16
                     初一   初二   初三   初四
8/17   8/18   8/19   8/20   8/21   8/22   8/23
初五   初六   七夕   初八   初九   初十   十一
8/24   8/25   8/26   8/27   8/28   8/29   8/30
十二   十三   十四   中元   十六   十七   十八
8/31   9/1    9/2    9/3    9/4    9/5    9/6
十九   二十   廿一   廿二   廿三   廿四   廿五
9/7    9/8    9/9    9/10
廿六   廿七   廿八   廿九
```

First lunar month of the year, showing 春节 and 元宵 laid out by Gregorian dates:

```
$ cal_nongli -L 2026 1
     农历 丙午年 正月
一     二     三     四     五     六     日
       2/17   2/18   2/19   2/20   2/21   2/22
       春节   初二   初三   初四   初五   初六
2/23   2/24   2/25   2/26   2/27   2/28   3/1
初七   初八   初九   初十   十一   十二   十三
3/2    3/3    3/4    3/5    3/6    3/7    3/8
十四   元宵   十六   十七   十八   十九   二十
3/9    3/10   3/11   3/12   3/13   3/14   3/15
廿一   廿二   廿三   廿四   廿五   廿六   廿七
3/16   3/17   3/18
廿八   廿九   三十
```

A whole lunar year chains every month together (leap months included); the header is that year's ganzhi:

```
$ cal_nongli -L 2026 | head -1
     农历 丙午年 正月
```

A leap month is shown separately with `-R`:

```
$ cal_nongli -L 2020 4 -R
     农历 庚子年 闰四月
一     二     三     四     五     六     日
                                   5/23   5/24
                                   初一   初二
5/25   5/26   5/27   5/28   5/29   5/30   5/31
初三   初四   初五   初六   初七   初八   初九
6/1    6/2    6/3    6/4    6/5    6/6    6/7
初十   十一   十二   十三   十四   十五   十六
6/8    6/9    6/10   6/11   6/12   6/13   6/14
十七   十八   十九   二十   廿一   廿二   廿三
6/15   6/16   6/17   6/18   6/19   6/20
廿四   廿五   廿六   廿七   廿八   廿九
```

Numeric lunar days and disabling the festival/month-name overlays (`--number`, `--no-festival`, `--no-month-name`) are handy for scripting or a quieter look.

The cell fill priority is: **festival > month-name on the 1st lunar day > solar term > lunar day**.

## Data and range

- Gregorian/lunar conversion: 1900-2100 (embedded lunar table)
- 24 solar terms: 1901-2100 (Hong Kong Observatory data, vector-compressed)
- ganzhi (year/month/day): aligned with the astronomical calendar
- To cross-check, you can compare against a system `lunar-date` (note: some old versions
  have a one-day offset in a few solar terms).

## Author

nth233 - mrnothing233@gmail.com
