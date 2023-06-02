mod mahjong;

mod tiles;

fn main() {
    let game = mahjong::Mahjong {

    };

    game.run();

    println!("{}", tiles::Tile::M5 | tiles::Tile::IsRed);
}
