#include <vector>
#include <iostream>

using namespace std;

int main() {
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