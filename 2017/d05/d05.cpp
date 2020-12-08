#include <fstream>
#include <iostream>
#include <sstream>
#include <vector>

int main() {
  std::ifstream f("input.txt");
  std::string line;
  int part1 = 0;
  int part2 = 0;
  std::vector<int> jumps;
  while (std::getline(f, line)) {
    std::stringstream ss;
    ss << line;
    int val;
    ss >> val;
    jumps.push_back(val);
  }
  auto jumps_2 = jumps;
  // part1
  size_t position = 0;
  while (position < jumps.size()) {
    auto prev = position;
    position += jumps[position];
    jumps[prev] += 1;
    ++part1;
  }
  // part2
  position = 0;
  while (position < jumps_2.size()) {
    auto prev = position;
    position += jumps_2[position];
    jumps_2[prev] += (jumps_2[prev] >= 3) ? -1 : 1;
    ++part2;
  }
  std::cout << "Part1: " << part1 << "\n";
  std::cout << "Part2: " << part2 << "\n";
  return 0;
}
