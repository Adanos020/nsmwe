use crate::{
    addr::{AddrPc, AddrSnes},
    graphics::gfx_file::TileFormat,
    internal_header::MapMode,
};

use nom::{
    Err as NomErr,
    error::{
        Error as NomError,
        ErrorKind,
    },
};
use std::{
    error::Error,
    fmt,
};

// -------------------------------------------------------------------------------------------------

#[derive(Debug)]
pub enum RomParseError {
    BadAddress(usize),
    BadSize(usize),
    GfxFile(TileFormat, usize, usize),
    InternalHeader,
    IoError,
    Level(usize),
    PaletteGlobal,
    PaletteSetLevel(usize),
}

#[derive(Debug)]
pub enum AddressConversionError {
    PcToSnes(AddrPc),
    SnesToPc(AddrSnes, MapMode),
}

#[derive(Debug)]
pub struct DecompressionError(pub &'static str);

// -------------------------------------------------------------------------------------------------

impl fmt::Display for RomParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use RomParseError::*;
        let msg = match self {
            BadAddress(addr) =>
                format!("ROM doesn't contain PC address {}", addr),
            BadSize(size) =>
                format!("Invalid ROM size: {}", size),
            InternalHeader =>
                String::from("Parsing internal header failed"),
            IoError =>
                String::from("File IO Error"),
            Level(level_num) =>
                format!("Invalid level: {:#X}", level_num),
            PaletteGlobal =>
                String::from("Could not parse global level color palette"),
            PaletteSetLevel(level_num) =>
                format!("Invalid color palette in level {:#X}", level_num),
            GfxFile(tile_format, num, size_bytes) =>
                format!("Invalid GFX file - tile format: {}, file num: {:X}, size: {}B",
                        tile_format, num, size_bytes),
        };
        f.write_str(msg.as_str())
    }
}

impl Error for RomParseError {}

impl fmt::Display for AddressConversionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use AddressConversionError::*;
        let msg = match self {
            PcToSnes(addr) => format!("PC address {:#x} is too big for LoROM.", addr),
            SnesToPc(addr, map_mode) =>
                format!("Invalid SNES {} address: ${:x}", map_mode, addr),
        };
        f.write_str(msg.as_str())
    }
}

impl Error for AddressConversionError {}

impl fmt::Display for DecompressionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("Decompressing data failed:")?;
        f.write_str(self.0)
    }
}

impl Error for DecompressionError {}

// -------------------------------------------------------------------------------------------------

pub fn nom_error(input: &[u8], kind: ErrorKind) -> NomErr<NomError<&[u8]>> {
    NomErr::Error(NomError::new(input, kind))
}
