#include <fstream>
#include <iostream>

int main() {
  std::ifstream f("input.txt");
  std::string data;
  std::getline(f, data);
  int part1 = 0;
  int part2 = 0;
  size_t idx = 0;
  while (idx < data.size()) {
    if (data[idx] == data[(idx + 1) % data.size()]) part1 += data[idx] - '0';
    if (data[idx] == data[(idx + (data.size() / 2)) % data.size()])
      part2 += data[idx] - '0';
    ++idx;
  }
  std::cout << "Part 1: " << part1 << "\nPart 2: " << part2 << "\n";
  return 0;
}
