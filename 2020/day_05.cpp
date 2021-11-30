#include "solve.h"

void parse(string& seat) {
    std::replace(seat.begin(), seat.end(), 'F', '0');
    std::replace(seat.begin(), seat.end(), 'B', '1');
    std::replace(seat.begin(), seat.end(), 'R', '1');
    std::replace(seat.begin(), seat.end(), 'L', '0');
}

int main() {
    int highest_seat_id = 0;
    vector<int> seat_ids;
    for (auto& seat : input_day_05) {
        parse(seat);
        int row = (int)(bitset<7>(seat)).to_ulong();
        int column = (int)(bitset<3>(seat.substr(7,seat.size()))).to_ulong();
        int seat_id = row * 8 + column;
        seat_ids.push_back(seat_id);
        if (seat_id > highest_seat_id)
            highest_seat_id = seat_id;
    }
    sort(seat_ids.begin(), seat_ids.end());
    for (int i = 0 ; i < seat_ids.size() ; i++) {
        if (seat_ids[i] != seat_ids[i+1]-1) {
            cout << seat_ids[i]+1 << endl;
        }
    }
    cout << highest_seat_id << endl;
}