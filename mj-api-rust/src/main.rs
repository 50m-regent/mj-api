mod mahjong;

mod tiles;

fn main() {
    let game: mahjong::Mahjong = mahjong::Mahjong {

    };

    game.run();

    println!("{}", tiles::Tile::MR);
}
