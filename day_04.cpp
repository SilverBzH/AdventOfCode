#include "solve.h"

#define NB_ID_NEEDED 7

int main() {
    int nb_passport_valid = 0;
    int nb_valid_id = 0;

    for (auto& line : input_day_04) {
        stringstream ss(line);
        string tmp;
        vector<string> id;
        if (line == "") {
            if (nb_valid_id == NB_ID_NEEDED)
                nb_passport_valid++;
            nb_valid_id = 0;
            continue;
        }
        while(getline(ss, tmp, ' ')) {
            stringstream tmp_ss(tmp);
            string id;
            getline(tmp_ss, id, ':');
            if (id == "byr" || id == "iyr" || id == "eyr" ||
                id == "hgt" || id == "hcl" || id == "ecl" || 
                id == "pid") {
                    nb_valid_id++;
            }
        }
    }
    if (nb_valid_id == NB_ID_NEEDED)
                nb_passport_valid++;
    cout << nb_passport_valid << endl;
    return 0;
}



    // byr (Birth Year)
    // iyr (Issue Year)
    // eyr (Expiration Year)
    // hgt (Height)
    // hcl (Hair Color)
    // ecl (Eye Color)
    // pid (Passport ID)
    // cid (Country ID)
