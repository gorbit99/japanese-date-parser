use std::collections::HashMap;

use lazy_static::lazy_static;

lazy_static! {
    pub static ref DIGITS: HashMap<char, u32> = {
        let mut m = HashMap::new();
        m.insert('0', 0);
        m.insert('０', 0);
        m.insert('〇', 0);
        m.insert('1', 1);
        m.insert('１', 1);
        m.insert('2', 2);
        m.insert('２', 2);
        m.insert('3', 3);
        m.insert('３', 3);
        m.insert('4', 4);
        m.insert('４', 4);
        m.insert('5', 5);
        m.insert('５', 5);
        m.insert('6', 6);
        m.insert('６', 6);
        m.insert('7', 7);
        m.insert('７', 7);
        m.insert('8', 8);
        m.insert('８', 8);
        m.insert('9', 9);
        m.insert('９', 9);
        m
    };
}

pub const YEAR_MARKER: char = '年';
pub const MONTH_MARKER: char = '月';
pub const DAY_MARKER: char = '日';

pub const WEEKDAYS: [&str; 14] = [
    "月",
    "火",
    "水",
    "木",
    "金",
    "土",
    "日",
    "月曜日",
    "火曜日",
    "水曜日",
    "木曜日",
    "金曜日",
    "土曜日",
    "日曜日",
];

pub const ALTERNATE_SEPARATORS: [char; 3] = ['/', '.', '-'];

pub const OPEN_PARENS: [char; 2] = ['（', '('];
pub const CLOSE_PARENS: [char; 2] = ['）', ')'];

pub const SPACE: [char; 2] = [' ', '　'];
