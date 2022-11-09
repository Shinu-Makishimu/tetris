use sdl2::pixels::Color as SdlColor;
use crate::engine::Color as SemanticColor;

pub trait ScreenColor {
    fn screen_color(&self) -> SdlColor;
}

impl ScreenColor for SemanticColor{
    fn screen_color(&self) -> SdlColor {
        match self {
            SemanticColor::Yellow => SdlColor::RGB(0xc4,0xa0,0x00),
            SemanticColor::Cyan   => SdlColor::RGB(0x20,0x4a,0x87),
            SemanticColor::Purple => SdlColor::RGB(0x75,0x50,0x7b),
            SemanticColor::Orange => SdlColor::RGB(0xf5,0x79,0x00),
            SemanticColor::Blue   => SdlColor::RGB(0x72,0x9f,0xcf),
            SemanticColor::Green  => SdlColor::RGB(0x4e,0x9a,0x06),
            SemanticColor::Red    => SdlColor::RGB(0xcc,0x00,0x00),
   
        }
    }

}