pub struct Mahjong {

}

impl Mahjong {
    pub fn run(&self) {
        self.shuffle();
        self.deal();

        loop {
            if 0 == self.wall.len() {
                break;
            }

            
        }

        self.print();
    }
}