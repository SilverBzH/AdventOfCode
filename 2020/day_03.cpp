#include "solve.h"
//65
//54
//241 (too low)

static int count_tree(int step, int line_step);

int main() {
    int a = count_tree(1, 1);
    int b = count_tree(3, 1);
    int c = count_tree(5, 1);
    int d = count_tree(7, 1);
    int e = count_tree(1, 2);
    long sum = a * b * c * d * e;
    cout << sum << endl;

    return 0;
}

int count_tree(int x_step, int line_step) {
    int line = 0;
    int pos = 0;
    int tree = 0;
    int size = input_day_03.size();
    int period = input_day_03[0].size();
    while (line < size) {
        if (input_day_03[line][pos%period] == '#') {
            tree++;
        }
        pos += x_step;
        line += line_step;
    }
    return tree;
}

