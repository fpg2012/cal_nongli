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
| `%A`  | weekday (星期五)    | `%n`  | lunar day, number               |
| `%S`  | zodiac (马)         | `%H`  | ganzhi month (丙申)             |
| `%Q`  | solar term (or empty) | `%D` | ganzhi day (辛巳)               |

`\n`, `\t` and `%%` are supported. Example:

```sh
date_nongli -f '%G年%M%N，星期%A' -d 2026-09-07
# 丙午年七月廿六，星期一
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

## Example

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

## Data and range

- Gregorian/lunar conversion: 1900-2100 (embedded lunar table)
- 24 solar terms: 1901-2100 (Hong Kong Observatory data, vector-compressed)
- ganzhi (year/month/day): aligned with the astronomical calendar
- To cross-check, you can compare against a system `lunar-date` (note: some old versions
  have a one-day offset in a few solar terms).

## Author

nth233 - mrnothing233@gmail.com
