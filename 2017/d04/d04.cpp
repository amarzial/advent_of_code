#include <fstream>
#include <iostream>
#include <sstream>
#include <unordered_set>
#include <algorithm>

int main() {
  std::ifstream f("input.txt");
  std::string line;
  int part1 = 0;
  int part2 = 0;
  while (std::getline(f, line)) {
    std::unordered_set<std::string> words;
    std::unordered_set<std::string> words2;
    std::stringstream ss;
    ss << line;
    auto found = false;
    auto found2 = false;
    while (not ss.eof()) {
      std::string str;
      ss >> str;
      if (words.find(str) != words.end()) {
        found = true;
      }
      words.insert(str);
      std::sort(str.begin(), str.end());
      if (words2.find(str) != words2.end()) {
        found2 = true;
      }
      words2.insert(str);
    }
    part1 += found ? 0 : 1;
    part2 += found2 ? 0 : 1;
  }
  std::cout << "Part1: " << part1 << "\n";
  std::cout << "Part2: " << part2 << "\n";
  return 0;
}
