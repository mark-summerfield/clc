// Copyright © 2022 Mark Summerfield. All rights reserved.
// License: GPLv3

#include <boost/program_options.hpp>
#include <string>
#include "config.hpp"

using namespace std;
namespace po = boost::program_options;

Config::Config() :
    languages(unordered_set<string>()),
    skiplanguages(unordered_set<string>()),
    includes(unordered_set<string>()),
    maxwidth(0),
    sortbylines(false),
    summary(false),
    files(unordered_set<string>()) {
    excludes = unordered_set<string>({"__pycache__", "build", "build.rs",
                                      "CVS", "dist", "setup.py", "target"});
}

void Config::update_from_command_line() {
    // TODO insert the list of supported language names
    po::options_description desc("Counts the lines in the code files for "
                                 "the languages processed (ignoring . folders).\n"
                                 "Supported language names: ");
}
