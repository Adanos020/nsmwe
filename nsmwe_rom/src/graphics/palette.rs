use crate::{
    addr::{
        AddrPc,
        AddrSnes,
    },
    graphics::color::{
        Bgr16,
        BGR16_SIZE,
    },
    level::primary_header::PrimaryHeader,
};
use self::constants::*;

use nom::{
    count,
    do_parse,
    map,
    named,
    number::complete::le_u16,
    preceded,
    take,
    IResult,
};

use std::convert::{TryInto, TryFrom};

mod constants {
    use crate::graphics::color::BGR16_SIZE;

    pub mod addr {
        use crate::addr::AddrSnes;
        pub const BACK_AREA_COLORS: AddrSnes = AddrSnes(0x00B0A0);
        pub const BG_PALETTES:      AddrSnes = AddrSnes(0x00B0B0);
        pub const FG_PALETTES:      AddrSnes = AddrSnes(0x00B190);
        pub const SPRITE_PALETTES:  AddrSnes = AddrSnes(0x00B318);
        pub const WTF_PALETTES:     AddrSnes = AddrSnes(0x00B250);
        pub const PLAYER_PALETTES:  AddrSnes = AddrSnes(0x00B2C8);
        pub const LAYER3_PALETTES:  AddrSnes = AddrSnes(0x00B170);
        pub const BERRY_PALETTES:   AddrSnes = AddrSnes(0x00B674);
        pub const ANIMATED_COLOR:   AddrSnes = AddrSnes(0x00B60C);
    }

    pub const PALETTE_BG_SIZE:       usize = 0x18;
    pub const PALETTE_FG_SIZE:       usize = 0x18;
    pub const PALETTE_SPRITE_SIZE:   usize = 0x18;
    pub const PALETTE_WTF_SIZE:      usize = 11 * 0x0C;
    pub const PALETTE_PLAYER_SIZE:   usize = 4 * 0x14;
    pub const PALETTE_LAYER3_SIZE:   usize = 0x20;
    pub const PALETTE_BERRY_SIZE:    usize = 3 * 0x0E;
    pub const PALETTE_ANIMATED_SIZE: usize = 8 * BGR16_SIZE;

    pub const PALETTE_LENGTH:          usize = 16 * 16;
    pub const PALETTE_BG_LENGTH:       usize = PALETTE_BG_SIZE / BGR16_SIZE;
    pub const PALETTE_FG_LENGTH:       usize = PALETTE_FG_SIZE / BGR16_SIZE;
    pub const PALETTE_SPRITE_LENGTH:   usize = PALETTE_SPRITE_SIZE / BGR16_SIZE;
    pub const PALETTE_WTF_LENGTH:      usize = PALETTE_WTF_SIZE / BGR16_SIZE;
    pub const PALETTE_PLAYER_LENGTH:   usize = PALETTE_PLAYER_SIZE / BGR16_SIZE;
    pub const PALETTE_LAYER3_LENGTH:   usize = PALETTE_LAYER3_SIZE / BGR16_SIZE;
    pub const PALETTE_BERRY_LENGTH:    usize = PALETTE_BERRY_SIZE / BGR16_SIZE;
    pub const PALETTE_ANIMATED_LENGTH: usize = PALETTE_ANIMATED_SIZE / BGR16_SIZE;
}

// -------------------------------------------------------------------------------------------------

pub struct ColorPalette {
    _back_area_color: Bgr16,
    colors: [Bgr16; PALETTE_LENGTH],
}

// -------------------------------------------------------------------------------------------------

named!(le_bgr16<Bgr16>, map!(le_u16, Bgr16));

impl ColorPalette {
    pub fn parse_level_palette<'a>(rom_data: &'a [u8], addr: AddrSnes, header: &PrimaryHeader)
        -> IResult<&'a [u8], ColorPalette>
    {
        if addr == AddrSnes(0x0) || addr == AddrSnes(0xFFFFFF) {
            ColorPalette::parse_vanilla_level_palette(rom_data, header)
        } else {
            ColorPalette::parse_custom_level_palette(rom_data)
        }
    }

    named!(pub parse_custom_level_palette<&[u8], Self>, do_parse!(
        back_area_color: le_bgr16 >>
        colors: count!(le_bgr16, PALETTE_LENGTH) >>
        (ColorPalette {
            _back_area_color: back_area_color,
            colors: colors.try_into().unwrap(),
        })
    ));

    pub fn parse_vanilla_level_palette<'a>(rom_data: &'a [u8], header: &PrimaryHeader)
        -> IResult<&'a [u8], ColorPalette>
    {
        let parse_colors = |pos, n| {
            let pos: usize = AddrPc::try_from(pos).unwrap().into();
            preceded!(rom_data, take!(pos), count!(le_bgr16, n))
        };

        let (_, back_area_color) = parse_colors(
            addr::BACK_AREA_COLORS + (BGR16_SIZE * header.back_area_color as usize), 1)?;
        let (_, bg) = parse_colors(
            addr::BG_PALETTES + (PALETTE_BG_SIZE * header.palette_bg as usize),
            PALETTE_BG_LENGTH)?;
        let (_, fg) = parse_colors(
            addr::FG_PALETTES + (PALETTE_FG_SIZE * header.palette_fg as usize),
            PALETTE_FG_LENGTH)?;
        let (_, sprite) = parse_colors(
            addr::SPRITE_PALETTES + (PALETTE_SPRITE_SIZE * header.palette_sprite as usize),
            PALETTE_SPRITE_LENGTH)?;

        let (_, wtf)      = parse_colors(addr::WTF_PALETTES,    PALETTE_WTF_LENGTH)?;
        let (_, players)  = parse_colors(addr::PLAYER_PALETTES, PALETTE_PLAYER_LENGTH)?;
        let (_, layer3)   = parse_colors(addr::LAYER3_PALETTES, PALETTE_LAYER3_LENGTH)?;
        let (_, berry)    = parse_colors(addr::BERRY_PALETTES,  PALETTE_BERRY_LENGTH)?;
        let (_, animated) = parse_colors(addr::ANIMATED_COLOR,  PALETTE_ANIMATED_LENGTH)?;

        let mut palette = ColorPalette {
            _back_area_color: back_area_color[0],
            colors: [Bgr16(0); PALETTE_LENGTH],
        };

        palette.set_colors(&bg,      |i| 0x0 + (i / 6), |i| 0x2 + (i % 6)); // rows: 0-1, cols: 2-7
        palette.set_colors(&fg,      |i| 0x2 + (i / 6), |i| 0x2 + (i % 6)); // rows: 2-3, cols: 2-7
        palette.set_colors(&sprite,  |i| 0xE + (i / 6), |i| 0x2 + (i % 6)); // rows: E-F, cols: 2-7
        palette.set_colors(&wtf,     |i| 0x4 + (i / 6), |i| 0x2 + (i % 6)); // rows: 4-D, cols: 2-7
        palette.set_colors(&players, |_| 0x8,           |i| 0x6 + i);       // rows: 8-8, cols: 6-F
        palette.set_colors(&layer3,  |i| 0x0 + (i / 8), |i| 0x8 + (i % 8)); // rows: 0-1, cols: 8-F
        palette.set_colors(&berry,   |i| 0x2 + (i / 7), |i| 0x2 + (i % 7)); // rows: 2-4, cols: 9-F
        palette.set_colors(&berry,   |i| 0x9 + (i / 7), |i| 0x2 + (i % 7)); // rows: 9-B, cols: 9-F
        palette.set_color_at(0x6, 0x4, animated[0]);

        Ok((rom_data, palette))
    }

    pub fn get_color_at(&self, x: usize, y: usize) -> Option<&Bgr16> {
        let idx = Self::get_index_at(x, y);
        self.colors.get(idx)
    }

    pub fn set_color_at(&mut self, x: usize, y: usize, col: Bgr16) {
        self.colors[Self::get_index_at(x, y)] = col;
    }

    fn set_colors<Fx, Fy>(&mut self, subpal: &[Bgr16], calc_x: Fx, calc_y: Fy)
        where Fx: Fn(usize) -> usize,
              Fy: Fn(usize) -> usize,
    {
        for (idx, &col) in subpal.iter().enumerate() {
            let x = calc_x(idx);
            let y = calc_y(idx);
            self.set_color_at(x, y, col);
        }
    }

    fn get_index_at(x: usize, y: usize) -> usize {
        x + y * 16
    }
}