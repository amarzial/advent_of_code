#include <cmath>
#include <iostream>
#include <unordered_map>
#include <utility>
#include <vector>

template <class T>
inline void hash_combine(std::size_t& seed, const T& v) {
  std::hash<T> hasher;
  seed ^= hasher(v) + 0x9e3779b9 + (seed << 6) + (seed >> 2);
}

namespace std {
template <typename T1, typename T2>
struct hash<std::pair<T1, T2>> {
  std::size_t operator()(std::pair<T1, T2> const& p) const {
    std::size_t seed1(0);
    hash_combine(seed1, p.first);
    hash_combine(seed1, p.second);

    std::size_t seed2(0);
    hash_combine(seed2, p.second);
    hash_combine(seed2, p.first);

    return std::min(seed1, seed2);
  }
};
}

int make_sum(std::unordered_map<std::pair<int, int>, int>& grid,
             std::pair<int, int>& p) {
  int sum = 0;
  std::vector<std::pair<int, int>> vec;
  vec.reserve(8);
  vec.emplace_back(p.first + 1, p.second + 1);
  vec.emplace_back(p.first + 1, p.second);
  vec.emplace_back(p.first + 1, p.second - 1);
  vec.emplace_back(p.first, p.second + 1);
  vec.emplace_back(p.first, p.second - 1);
  vec.emplace_back(p.first - 1, p.second + 1);
  vec.emplace_back(p.first - 1, p.second);
  vec.emplace_back(p.first - 1, p.second - 1);
  for (auto& e : vec) {
    std::unordered_map<std::pair<int, int>, int>::iterator it;
    if ((it = grid.find(e)) != grid.end()) {
      sum += (*it).second;
    }
  }
  return sum;
}

int main(int argc, char** argv) {
  if (argc != 2) {
    std::cout << "Specify a number as argument\n";
    return 1;
  }
  auto target = std::stoi(std::string(argv[1]));
  auto part1 = 0;
  if (target > 1) {
    int layer = 0;
    int count = 1;
    while ((((layer * 2) * 4) + count) < target) {
      count += layer * 2 * 4;
      layer++;
    }
    part1 = layer;
    auto offset = ((target - (count + 1)) % (layer * 2)) -
                  ((layer > 1) ? (layer - 1) : 0);
    part1 += (offset > 0) ? offset : -offset;
  }
  auto part2 = 1;
  {
    std::unordered_map<std::pair<int, int>, int> grid;
    grid[std::make_pair(0, 0)] = part2;
    int current = 0;
    int layer = 1;
    std::pair<int, int> pos(0, 0);
    while (part2 < target) {
      if (current == layer * 8) {
        current = 0;
        ++layer;
      }

      if (current == 0 or current / (layer * 2) == 3) {
        pos = std::make_pair(pos.first + 1, pos.second);
      } else if (current / (layer * 2) == 0) {
        pos = std::make_pair(pos.first, pos.second + 1);
      } else if (current / (layer * 2) == 1) {
        pos = std::make_pair(pos.first - 1, pos.second);
      } else if (current / (layer * 2) == 2) {
        pos = std::make_pair(pos.first, pos.second - 1);
      }
      part2 = make_sum(grid, pos);
      grid[pos] = part2;
      ++current;
    }
  }
  std::cout << "Part1: " << part1 << "\n";
  std::cout << "Part2: " << part2 << "\n";
  return 0;
}
