#include "solve.h"

void part_1() {
    set<char> counter;
    int nb_yes = 0;
    for (auto& line : input_day_06) {
        if (line == "") {
            nb_yes += counter.size();
            counter.clear();
            continue;
        }
        smatch letter_match;
        while(regex_search(line, letter_match, regex("[a-z]"))) {
            char letter = *letter_match.str(0).c_str();
            if (counter.find(letter) == counter.end()) {
                counter.emplace(letter);
            }
            line = letter_match.suffix();
        }
    }
    nb_yes += counter.size();
    cout << nb_yes << endl;
}

void part_2() {
    map<char,int> counter;
    int nb_yes = 0;
    int nb_people = 0;
    for (auto& line : input_day_06) {
        if (line == "") {
            for (auto& x : counter) {
                if (x.second == nb_people)
                    nb_yes++;
            }
            nb_people = 0;
            counter.clear();
            continue;
        }
        nb_people++;
        smatch letter_match;
        while(regex_search(line, letter_match, regex("[a-z]"))) {
            char letter = *letter_match.str(0).c_str();
            if (counter.find(letter) == counter.end())
                counter.emplace(letter, 1);
            else
                counter.at(letter)++;
            line = letter_match.suffix();
        }
    }
    for (auto& x : counter) {
        if (x.second == nb_people)
            nb_yes++;
    }
    cout << nb_yes << endl;
}

int main() {
    part_2();
}