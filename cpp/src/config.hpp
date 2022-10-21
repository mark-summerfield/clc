#ifndef CONFIG_HPP
#define CONFIG_HPP
// Copyright © 2022 Mark Summerfield. All rights reserved.
// License: GPLv3

#include <unordered_set>
#include <string>

using namespace std;

struct Config {
    Config();
    void update_from_command_line();

    unordered_set<string> languages;
    unordered_set<string> skiplanguages;
    unordered_set<string> excludes;
    unordered_set<string> includes;
    size_t maxwidth;
    bool sortbylines;
    bool summary;
    unordered_set<string> files;
};

#endif // CONFIG_HPP
