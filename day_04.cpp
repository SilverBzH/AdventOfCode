#include "solve.h"

#define NB_ID_NEEDED 7

int main() {
    int nb_passport_valid = 0;
    int nb_valid_id = 0;

    for (auto& line : input_day_04) {
        stringstream ss(line);
        string tmp;
        if (line == "") {
            if (nb_valid_id == NB_ID_NEEDED)
                nb_passport_valid++;
            nb_valid_id = 0;
            continue;
        }
        smatch match_id;
        while(regex_search(line, match_id, regex("([a-z]+):#?[0-9a-z]+"))) {
            if (regex_match(string(match_id[1]), regex("byr|iyr|eyr|hgt|hcl|ecl|pid")))
                nb_valid_id++;
            line = match_id.suffix();
        }
    }
    if (nb_valid_id == NB_ID_NEEDED)
                nb_passport_valid++;
    cout << nb_passport_valid << endl;
    return 0;
}