#include "solve.h"

//answer1: 500

static void parse(string& password, int& low, int& high, char& letter, string& pwd);

int main() {
    int low = 0;
    int high = 0;
    int nb_pwd_valid = 0;
    char letter;
    string pwd;
    bool part1 = true;

    for (auto& password : input_day_02) {
        parse(password, low, high, letter, pwd);
        int trigger = 0;
        if (part1) {
            for (int i=0 ; i < pwd.size() ; i++) {
                if (pwd[i] == letter)
                    trigger++;
                    if (trigger > high || (trigger < low && (i-pwd.size()) < low))
                        break;
            }
            if (trigger >= low && trigger <= high) {
                nb_pwd_valid++;
            }
        } else {
            int letter_occurence = 0;
            if (pwd[low-1] == letter)
                letter_occurence++;
            if (pwd[high-1] == letter)
                letter_occurence++;
            if (letter_occurence == 1)
                nb_pwd_valid++;
        }
    }
    cout << nb_pwd_valid << endl;
}



void parse(string& password, int& low, int& high, char& letter, string& pwd) {
    std::stringstream ss(password);
    vector<string> item;
    while (ss.good()) {
        string tmp;
        getline(ss, tmp, ',');
        item.push_back(tmp);
    }
    low = stoi(item[0]);
    high = stoi(item[1]);
    const char* c = item[2].c_str();
    letter = *c;
    pwd = item[3];
}