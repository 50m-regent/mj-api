#include <vector>

namespace mahjong {

class State {
private:
    const Wind &round_wind;
    const uint8_t &round_count;
    const uint8_t &round_points;
    const uint8_t &stack_points;

    const std::vector<Player> &players;

    const Action &action;
};

class Round {
private:
    const Mahjong &game;

    uint8_t turn;

public:
    inline explicit Round(
        const Mahjong &game,
        const uint8_t parent
    ) : game(game), turn(parent) {

    }

    void run() const noexcept {
        while (!this->is_over()) {
            
        }
    }
};

class Mahjong {
private:
    
public:
    const std::vector<Player> &players;

    std::vector<State> states;

    inline explicit Mahjong(
        const std::vector<Player> &players
    ) : players(players) {

    }

    void run() const noexcept {
        while (!this->is_over()) {
            this->create_round().run();
        }
    }
};

}

int main() {
    const mahjong::Mahjong game;

    game.run();
}