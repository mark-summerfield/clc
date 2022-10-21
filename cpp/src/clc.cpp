// Copyright © 2022 Mark Summerfield. All rights reserved.
// License: GPLv3

#include <iostream>
#include "config.hpp"

using namespace std;


int main() {
    cout << "clc C++\n";
    Config config;
    try {
        config.update_from_command_line();
    } catch (exception& err) {
        cerr << "error: " << err.what() << '\n';
        return 1;
    }
}
