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
        while(regex_search(line, match_id, regex("([a-z]+):(#?[0-9a-z]+)"))) {
            string id = match_id[1];
            string data = match_id[2];
            if ((id == "byr" && stoi(data) >= 1920 && stoi(data) <= 2002) || 
                (id == "iyr" && stoi(data) >= 2010 && stoi(data) <= 2020) ||
                (id == "eyr" && stoi(data) >= 2020 && stoi(data) <= 2030) || 
                (id == "hcl" && regex_match(data, regex("^#[0-9a-f]{6}"))) ||
                (id == "ecl" && regex_match(data, regex("amb|blu|brn|gry|grn|hzl|oth"))) ||
                (id == "pid" && regex_match(data, regex("[0-9]{9}")))
                ) {
                    nb_valid_id++;
            } else if (id == "hgt" && regex_match(data, regex("(^[0-9]+)(cm|in)"))) {
                smatch metrics;
                regex_search(data, metrics, regex("(^[0-9]+)(cm|in)"));
                int value = stoi(metrics[1]);
                string metric = metrics[2];
                if ((metric == "cm" && value >= 150 && value <= 193) ||
                    (metric == "in" && value >= 59 && value <= 76)) {
                        nb_valid_id++;
                    }
            }
            line = match_id.suffix();
        }
    }
    if (nb_valid_id == NB_ID_NEEDED)
                nb_passport_valid++;
    cout << nb_passport_valid << endl;
    return 0;
}