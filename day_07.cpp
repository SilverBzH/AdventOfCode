#include "solve.h"

typedef map<string, set<string>> bag_map;
const string SHINY_GOLD = "shiny gold ";

void parse(bag_map& bags);
bool explore_bags(bag_map& bags, string main_bag);

int main () {
    bag_map bags;
    set<string> counter;
    parse(bags);
    for (auto& main_bag : bags) {
        for (auto& aux_bag : main_bag.second) {
            if (counter.find(main_bag.first) == counter.end() && explore_bags(bags, aux_bag)) {
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
            if (aux_bag == SHINY_GOLD)
                return true;
        }
        for (auto& aux_bag : bags[main_bag]) {
            if(explore_bags(bags, aux_bag))
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
        regex e("((?:[a-z]+ )+)(?:bags?)");
        vector<string> parsed_bags;
        while (regex_search(item, m, e)) {
            parsed_bags.emplace_back(string(m[1]));
            item = m.suffix();
        }
        string main_bag = parsed_bags[0];
        set<string> aux_bags;
        for (int i=1 ; i < parsed_bags.size() ; i++) {
            aux_bags.emplace(parsed_bags[i]);
        }
        bags.emplace(main_bag, aux_bags);
    }
}