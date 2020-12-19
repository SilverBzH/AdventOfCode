#include "solve.h"

void parse(vector<pair<string, map<string, int>>>& bags);

int main () {
    vector<pair<string, map<string, int>>> bags;
    parse(bags);
    for (auto& bag : bags) {
        cout << "------" << endl;
        cout << "main bag: " << bag.first << endl;
        for (auto& aux_bag : bag.second) cout << aux_bag.second << " " << aux_bag.first << endl;
    }
}

void parse(vector<pair<string, map<string, int>>>& bags) {
    for (auto& item : input_day_07_example) {
        smatch m;
        regex e("(([a-z]+ ){2})bags contain (no|[0-9]+) (([a-z]+ ){1,2})[a-z]+.( ([0-9]+) (([a-z]+ ){2}).*?)?");
        regex_search(item, m, e);
        string main_bag = m[1];
        map<string, int> aux_bags;
        if (regex_match(string(m[3]), regex("[0-9]")))
            aux_bags.emplace(m[4], stoi(m[3]));
        if (regex_match(string(m[7]), regex("[0-9]")))
            aux_bags.emplace(m[8], stoi(m[7]));
        pair<string, map<string, int>> total_bag(main_bag, aux_bags);
        bags.emplace_back(total_bag);
    }
}