use bracket_lib::prelude::*;

mod fov;
mod world;

use crate::world::{Offset, Pos, TileKind};

const FOV_RANGE: i32 = 8;

type NamedColor = (u8, u8, u8);

const DARK_BLACK: NamedColor = (0x1c, 0x1c, 0x1c);
const DARK_WHITE: NamedColor = (0x9c, 0x99, 0x8e);
const LIGHT_BLACK: NamedColor = (0x54, 0x50, 0x54);
const LIGHT_WHITE: NamedColor = (0xf8, 0xfc, 0xf8);

struct Ui {
    gs: world::GameState,
}

impl GameState for Ui {
    fn tick(&mut self, ctx: &mut BTerm) {
        player_input(self, ctx);
        ctx.cls();
        self.draw(ctx);
    }
}

struct TilePrintable {
    symbol: &'static str,
    fg: RGB,
    bg: RGB,
}

fn get_printable(tile_kind: TileKind, visible: bool) -> TilePrintable {
    let (symbol, fg, bg) = match (tile_kind, visible) {
        (TileKind::Wall, true) => ("#", DARK_BLACK, LIGHT_WHITE),
        (TileKind::Wall, false) => ("#", LIGHT_BLACK, DARK_WHITE),
        (TileKind::Floor, true) => (".", LIGHT_WHITE, LIGHT_BLACK),
        (TileKind::Floor, false) => (".", DARK_WHITE, DARK_BLACK),
        (TileKind::Unseen, _) => (" ", DARK_BLACK, DARK_BLACK),
    };
    TilePrintable {
        symbol,
        fg: RGB::named(fg),
        bg: RGB::named(bg),
    }
}

impl Ui {
    fn move_player(&mut self, off: world::Offset) {
        self.gs.move_player(off);
    }

    fn draw(&mut self, ctx: &mut BTerm) {
        let gs = &self.gs;
        let (w, h) = ctx.get_char_size();
        let offset_x = gs.player_pos.x - w as i32 / 2;
        let offset_y = gs.player_pos.y - h as i32 / 2;
        let seen = fov::calculate_fov(gs.player_pos, FOV_RANGE, &gs.world);
        for x in 0..w as i32 {
            for y in 0..h as i32 {
                let pos = Pos {
                    x: x + offset_x,
                    y: y + offset_y,
                };
                let world = if gs.debug_mode {
                    &gs.world
                } else {
                    &gs.player_memory
                };
                let printable = get_printable(world[pos].kind, seen.contains(&pos));

                ctx.print_color(
                    x,
                    y,
                    printable.fg.into(),
                    printable.bg.into(),
                    printable.symbol,
                );
            }
        }
        ctx.print_color(
            w as i32 / 2,
            h as i32 / 2,
            RGB::named(LIGHT_WHITE),
            get_printable(gs.world[gs.player_pos].kind, true).bg,
            "@",
        );
    }
}

fn player_input(ui: &mut Ui, ctx: &mut BTerm) {
    use VirtualKeyCode::*;
    // Player movement
    match ctx.key {
        None => {} // Nothing happened
        Some(key) => match key {
            Left | H => ui.move_player(world::Offset { x: -1, y: 0 }),
            Right | L => ui.move_player(world::Offset { x: 1, y: 0 }),
            Up | K => ui.move_player(world::Offset { x: 0, y: -1 }),
            Down | J => ui.move_player(world::Offset { x: 0, y: 1 }),
            _ => {}
        },
    }
}

fn main() {
    let context = BTermBuilder::simple80x50()
        .with_title("Roguelike Tutorial")
        .build();
    let mut gs = world::GameState::new();
    gs.generate_world(RandomNumberGenerator::new().rand());
    let ui = Ui { gs };
    main_loop(context, ui);
}
