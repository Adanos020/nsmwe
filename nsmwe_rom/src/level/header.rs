pub struct PrimaryLevelHeader {
    pub palette_bg: u8,
    pub palette_fg: u8,
    pub palette_sprite: u8,
    pub back_area_color: u8,
    pub sprite_gfx: u8,
    pub timer: u8,
    pub item_memory: u8,
    pub vertical_scroll: u8,
    pub fg_bg_gfx: u8,
    pub level_length: u8,
    pub level_mode: u8,
    pub layer3_priority: bool,
    pub music: u8,
}

pub struct SecondaryLevelHeader {
    pub layer2_scroll: u8,
    pub main_entrance_pos: (u8, u8),
    pub layer3: u8,
    pub main_entrance_mario_action: u8,
    pub main_entrance_screen: u8,
    pub midway_entrance_screen: u8,
    pub fg_initial_pos: u8,
    pub bg_initial_pos: u8,
    pub no_yoshi_level: bool,
    pub vertical_level: bool,

    // The fields below are added by Lunar Magic and not used in the original game.
    // Support for LM-modified ROMs will be added later.
    // slippery_level: Option<bool>,
    // water_level: Option<bool>,
    // use_extended_entrance_pos: Option<bool>,
    // smart_spawn: Option<bool>,
    // sprite_spawn_range: Option<u8>,
    // bg_relative_to_fg: Option<bool>,
    // fg_bg_relative_to_player: Option<bool>,
    // start_facing_left: Option<bool>,
    // bg_height: Option<u8>,
    // relative_bg_offset: Option<u8>,
    // elf_layer: Option<bool>,
    // elf_show_bottom_row: Option<bool>,
    // elf_horizontal_level_mode: Option<u8>,
}
