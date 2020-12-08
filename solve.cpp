#include "solve.h"

#include <string>
#include <vector>
#include <iostream>
#include <sstream>

using namespace std;

static void day_01();
static void day_02();

int main() {
    ios::sync_with_stdio(false);
    // day_01();
    day_02();
    return 0;
}

void day_01() {
    vector<int> input;
    int tmp;
    while (cin >> tmp) {
        input.push_back(tmp);
    }
    
    for (int i = 0 ; i < input.size() ; i++) {
        for (int j = 0 ; j < input.size() ; j++) {
            int result = input[i] * input[j];
            cout << result << endl;
            return;
        }
    }
}

void day_02() {
    for (auto& password : input_day_2) {
        std::stringstream ss(password);
        vector<string> item;
        while (ss.good()) {
            string tmp;
            getline(ss, tmp, ',');
            item.push_back(tmp);
        }
        int low = stoi(item[0]);
        int high = stoi(item[1]);
        string letter = item[2];
        string pwd = item[3];
        cout << "GG" << endl;
        cout << low << " " << high << " " << letter << " " << pwd << endl;
    }
}