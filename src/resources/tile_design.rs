/*
    This file contains the logic for applying different colored backgrounds, stroke svg's, hover colors to each class tile
    on the class page.
*/

#[derive(Clone, Copy)]
pub enum TileDesign {
    SquiggleCrayonStroke,
    CircleCrayonStroke,
    ThickCrayonStroke,
    SwirlCrayonStroke,
}

impl TileDesign {
    // attaches the stroke backgrounds to the class tile on classes page.
    pub fn get_svg_path(&self) -> &'static str {
        match self {
            TileDesign::SquiggleCrayonStroke => {
                "/images/class_tile_designs/SquiggleCrayon_stroke.svg"
            }
            TileDesign::CircleCrayonStroke => "/images/class_tile_designs/CircleCrayon_stroke.svg",
            TileDesign::ThickCrayonStroke => "/images/class_tile_designs/ThickCrayon_stroke.svg",
            TileDesign::SwirlCrayonStroke => "/images/class_tile_designs/SwirlCrayon_stroke.svg",
        }
    }
    // sets background color for class tile on classes page.
    pub fn get_bg_color(&self) -> &'static str {
        match self {
            TileDesign::SquiggleCrayonStroke => "bg-classCardGreen",
            TileDesign::CircleCrayonStroke => "bg-classCardBrown",
            TileDesign::ThickCrayonStroke => "bg-classCardBlue",
            TileDesign::SwirlCrayonStroke => "bg-classCardPurple",
        }
    }
    // sets the hover color for the class tile on the classes page.
    pub fn get_hover_color(&self) -> &'static str {
        match self {
            TileDesign::SquiggleCrayonStroke => "hover:bg-classCardGreen-HOVER",
            TileDesign::CircleCrayonStroke => "hover:bg-classCardBrown-HOVER",
            TileDesign::ThickCrayonStroke => "hover:bg-classCardBlue-HOVER",
            TileDesign::SwirlCrayonStroke => "hover:bg-classCardPurple-HOVER",
        }
    }
}
