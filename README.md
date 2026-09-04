# cal_nongli

> English: [README.en.md](README.en.md)

中国农历命令行工具，一个 crate 提供两个命令，共享一套自包含的核心算法（公历与农历互转、干支、二十四节气、传统节日），无 daemon、无运行时依赖，纯 Rust + clap。

## 编译

需要 [Rust 工具链](https://www.rust-lang.org/)（cargo）。

```sh
cd cal_nongli
cargo build --release
# 产物在 target/release/{date_nongli, cal_nongli}
```

## 安装

任选其一：

- 用 `cargo install`（会装到 `~/.cargo/bin`，若已在 PATH 中可直接用）：
  ```sh
  cargo install --path .
  ```
- 手动拷贝到本地 bin 目录：
  ```sh
  cargo build --release
  install -Dm755 target/release/date_nongli ~/.local/bin/
  install -Dm755 target/release/cal_nongli ~/.local/bin/
  ```

版本号由 git 决定：打了 tag 显示 `vX.Y`，否则显示 commit 短 hash（有未提交改动追加 `-dirty`）。`-V` 查看。

## 用法

### date_nongli —— 仿 date，查看某一天的农历档案

```sh
date_nongli                     # 今天（公历/星期/农历/干支/生肖/节气）
date_nongli -d 2026-09-04      # 指定日期（date 风格）
date_nongli 2026 9 4            # 或 年 月 日
date_nongli -f '格式串'         # 自定义排版
```

`-f` 令牌（`date_nongli --help-format` 查看）：

| 令牌 | 含义 | 令牌 | 含义 |
|---|---|---|---|
| `%Y` | 公历年 | `%G` | 农历年干支(丙午) |
| `%m` | 公历月 | `%M` | 农历月汉字(七月/闰六月/腊月) |
| `%d` | 公历日 | `%N` | 农历日汉字(初一) |
| `%A` | 星期(星期五) | `%n` | 农历日数字 |
| `%S` | 生肖(马) | `%H` | 干支月(丙申) |
| `%Q` | 节气(无则空) | `%D` | 干支日(辛巳) |

支持 `\n` `\t` 与 `%%`。例：
```sh
date_nongli -f '%G年%M%N，星期%A' -d 2026-09-07
# 丙午年七月廿六，星期一
```

### cal_nongli —— 仿 cal，公历月叠加农历，或显示农历月

```sh
cal_nongli                    # 当前公历月（每格上=公历日，下=农历日/节气/节日）
cal_nongli 2026               # 公历整年
cal_nongli 2026 9             # 指定公历年月
cal_nongli -L                 # 当前农历月（用法与不带 -L 平行）
cal_nongli -L 2026            # 农历整年（自动含闰月）
cal_nongli -L 2026 7          # 指定农历月；加 -R 选闰月（如 -L 2020 4 -R）
```

常用选项：
- `-s` / `-m`：周日 / 周一为一周首列（默认周一）
- `-y`：当前公历整年；`-3` 前后三个月；`-n N` 连续 N 个月
- `--number`：农历日用数字（默认汉字）
- `--no-month-name`：关闭“初一显示为月份名”（默认 `正月/二月/…/腊月`）
- `--no-festival`：关闭节日覆盖（默认显示 除夕/春节/元宵/清明/端午/七夕/中元/中秋/重阳）

格子优先级：**节日 > 初一显示月份名 > 节气 > 农历日**。

## 示例

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

## 数据与范围

- 公历/农历换算：1900–2100（内嵌农历表）
- 二十四节气：1901–2100（香港天文台公布，向量压缩）
- 干支：年/月/日（与天文历对齐）
- 若需自验，可用系统里的 `lunar-date` 交叉比对（注意其部分旧版节气值有偏差）。

## 作者

nth233 · mrnothing233@gmail.com
