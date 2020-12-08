#include <fstream>
#include <iostream>
#include <sstream>
#include <vector>

int main() {
  std::ifstream f("input.txt");
  std::string data;
  int part1 = 0;
  int part2 = 0;
  while (std::getline(f, data)) {
    std::stringstream ss;
    ss << data;
    int max = 0;
    int min = 0x7fffffff;
    int val = 0;
    std::vector<int> numbers;
    while (not ss.eof()) {
      ss >> val;
      numbers.push_back(val);
      max = (max < val) ? val : max;
      min = (min > val) ? val : min;
    }
    part1 += max - min;
    for (auto v : numbers) {
      auto found = false;
      for (auto w : numbers) {
        if (w != v and (v % w) == 0) {
          found = true;
          part2 += (v / w);
          break;
        }
      }
      if (found) break;
    }
  }
  std::cout << "Part 1: " << part1 << "\n";
  std::cout << "Part 2: " << part2 << "\n";
  return 0;
}
