#include "solve.h"

typedef map<string, map<string, int>> bag_map;
const string SHINY_GOLD = "shiny gold ";

/// part 01
void part_01(bag_map& bags);
void parse(bag_map& bags);
bool explore_bags(bag_map& bags, string main_bag);

/// part 02

int main () {
    bag_map bags;
    parse(bags);
    part_01(bags);
}

//////////////////
//// PART 02 /////
//////////////////


//////////////////
//// PART 01 /////
//////////////////
void part_01(bag_map& bags) {
    set<string> counter;
    for (auto& main_bag : bags) {
        for (auto& aux_bag : main_bag.second) {
            if (counter.find(main_bag.first) == counter.end() && explore_bags(bags, aux_bag.first)) {
                counter.emplace(main_bag.first);
            }
        }
    }
    cout << "answer: " << counter.size() << endl;
}

bool explore_bags(bag_map& bags, string main_bag) {
    if (main_bag == SHINY_GOLD)
        return true;
    if (bags.find(main_bag) == bags.end()) {
        return false;
    } else {
        for (auto& aux_bag : bags[main_bag]) {
            if (aux_bag.first == SHINY_GOLD)
                return true;
        }
        for (auto& aux_bag : bags[main_bag]) {
            if(explore_bags(bags, aux_bag.first))
                return true;
        }
    }
    return false;
}

void parse(bag_map& bags) {
    for (auto& item : input_day_07) {
        smatch m;
        if (regex_match(item, regex(".*no other.*")))
            continue;
        regex e("(?:([0-9]+)* ?((?:[a-z]+ )+)bags?)");
        string main_bag;
        map<string, int> aux_bags;
        while (regex_search(item, m, e)) {
            // for (auto& match : m) cout << match << endl;
            if (main_bag.empty()) {
                main_bag = string(m[2]);
            } else {
                aux_bags.emplace(string(m[2]), stoi(m[1]));
            }
            item = m.suffix();
        }
        bags.emplace(main_bag, aux_bags);
    }
}