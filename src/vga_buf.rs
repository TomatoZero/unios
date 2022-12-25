const BUF_ADDR: u32 = 0xb8000;
const BUF_HEIGHT: u32 = 25;
const BUF_WIDTH: u32 = 80;

const COLOR_LIGHT_GREEN: u8 = 0xa;
const COLOR_BLACK: u8 = 0x0;

pub const DEFAULT_COLOR: u8 = (COLOR_BLACK << 4) | COLOR_LIGHT_GREEN;

pub struct AsciiChar {
    pub char_byte: u8,
    pub color_byte: u8
}

#[derive(Copy, Clone)]
#[derive(PartialEq)]
pub enum Alignment {
    Left = 0,
    Right = 80,
    Center = 40
}

#[derive(Copy, Clone)]
pub enum Color{
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    LightGray = 7,
    DarkGray = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    Pink = 13,
    Yellow = 14,
    White = 15,
}

#[derive(Copy, Clone)]
pub struct ColorCode{
    color_code: u8
}

impl ColorCode{
    pub fn new(foreground: Color, background: Color) -> ColorCode {
        return ColorCode {
            color_code: (((background as u8) << 4) + (foreground as u8)),
        }
    }
}

pub struct Screen {
    buffer: *mut u8,
    color: ColorCode,
    align: Alignment,
    row: u32,
    column: u32
}

impl core::fmt::Write for Screen {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        self.print(s);
        Ok(())
    }
}

impl Screen {
    
    pub fn new(color: ColorCode, align: Alignment) -> Screen {
        return Screen{
            buffer: BUF_ADDR as *mut u8,
            color,
            align,
            row:0,
            column:0
        }
    }

    // pub fn print_hello_world(&mut self) {
    //     let mut i = 1;
    //     for byte in "Hello world!".bytes() {
    //         self.write_char(i, AsciiChar{char_byte: byte, color_byte: Color::Blue as u8});
    //         i += 1;
    //     }
    // }

    pub fn print(&mut self, s: &str) {

        if self.row >= BUF_HEIGHT - 1 {
            self.row -= 1;

            for i in 1..BUF_HEIGHT {
                for j in 0..BUF_WIDTH {
                    let mut char = self.read_char(i * BUF_WIDTH + j);
                    self.write_char((i - 1) * BUF_WIDTH + j, char);
                }
            }
        }

        let mut size = s.chars().count() as u32;
        let mut start_offset = 0;
        let mut bytes = s.as_bytes()[0];

        if self.align == Alignment::Left {}
        else if self.align == Alignment::Center {
            if bytes >= 0x30 && bytes <= 0x39 {}
            else {
                start_offset = BUF_WIDTH / 2 - size / 2;
            }
        } else if self.align == Alignment::Right {
            if bytes >= 0x30 && bytes <= 0x39{
                let mut copy_offset = 0;
                for i in 0..BUF_WIDTH{
                    let mut char = self.read_char(self.row * BUF_WIDTH + i);
                    if char.char_byte >= 0x21 && char.char_byte <= 0x7b {
                        break;
                    }
                    else {
                        copy_offset += 1;
                    }
                }

                for i in copy_offset..BUF_WIDTH {
                    let mut char = self.read_char(self.row * BUF_WIDTH + i);
                    self.write_char(self.row * BUF_WIDTH + i - size, char);
                }

                self.column -= size;
            }
            else {
                start_offset = BUF_WIDTH - size;
            }
        }

        let mut i = self.row * BUF_WIDTH + self.column + start_offset;
        self.column += start_offset;
        for byte in s.bytes() {
            if byte == b'\n'{
                self.row += 1;
                i = self.row * BUF_WIDTH;
                self.column = 0;
                continue;
            }

            self.write_char(i, AsciiChar{char_byte: byte, color_byte: self.color.color_code});
            i += 1;
            self.column += 1;
        }
    }

    pub fn write_char(&self, offset: u32, char: AsciiChar) {
        unsafe {
            *self.buffer.offset(offset as isize * 2) = char.char_byte;
            *self.buffer.offset(offset as isize * 2 + 1) = char.color_byte;
        }
    }

    pub fn read_char(&self, offset: u32) -> AsciiChar {
        unsafe {
            return AsciiChar{
                char_byte: *self.buffer.offset(offset as isize * 2),
                color_byte: *self.buffer.offset(offset as isize * 2 + 1)
            }
        }
    }
}
