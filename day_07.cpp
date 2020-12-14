#include "solve.h"

void parse(vector<pair<string, map<string, int>>>& bags);

int main () {
    vector<pair<string, map<string, int>>> bags;
    parse(bags);
    for (auto& bag : bags) {
        cout << "main bag: " << bag.first << " contains: " << endl;
        for (auto& aux_bag : bag.second) cout << aux_bag.second << " " << aux_bag.first << endl;
        
    }
}

void parse(vector<pair<string, map<string, int>>>& bags) {
    for (auto& item : input_day_07_example) {
        smatch m;
        regex e("(([a-z]+ ){2})bags contain ([0-9]+) (([a-z]+ ){2})[a-z]+, ([0-9]+) (([a-z]+ ){2}).*?");
        regex_match(item, m, e);
        string main_bag = m[1];
        map<string, int> aux_bags;
        aux_bags.emplace(m[4], stoi(m[3]));
        aux_bags.emplace(m[7], stoi(m[6]));
        pair<string, map<string, int>> total_bag(main_bag, aux_bags);
        bags.emplace_back(total_bag);
        return;
    }
}