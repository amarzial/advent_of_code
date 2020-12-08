#include <cstdint>
#include <fstream>
#include <iostream>
#include <queue>
#include <string>
#include <unordered_map>
#include <vector>

using byte = std::uint8_t;

class State {
  std::vector<byte> m_state;
  char m_elevator;

 public:
  State(std::vector<byte> &&arr, char floor)
      : m_state(std::move(arr)), m_elevator{floor} {}
  State(int elements) : m_state(elements), m_elevator{0} {}

  bool is_valid() {
    bool single_micro = false;
    bool generator = false;
    for (auto i = 0; i < 4; ++i) {
      auto idx = i * 2;
      for (auto e : m_state) {
        if ((e >> idx) & 0x1) generator = true;
        if (((e >> idx) & 0x3) == 0x2) single_micro = true;
        if (generator and single_micro) return false;
      }
      return true;
    }
  }

  bool is_solved() {
    for (auto e : m_state) {
      if ((e >> 6) != 0x3) return false;
    }
    return true;
  }

  bool prevoius_floor_empty() {
    if (m_elevator) {
      for (auto e : m_state) {
        if ((e >> (2 * m_elevator)) & 0x3) return false;
      }
    }
    return true;
  }

  void add_all_double_up(std::queue<State> &q) {
    bool couple = false;
    for (int i = 0; i < m_state.size(); ++i) {
      byte curr = (m_state[i] >> (2 * m_elevator)) & 0x3;
      if (curr == 0x3 and couple)
        continue;
      else
        couple = true;
      if (curr & 1) {

      }
    }
  }
};

void read(const std::string &filename, std::vector<byte> &floors) {
  std::ifstream f(filename);
  if (f.fail()) exit(1);
  std::string line;
  int count = 0;
  int floor_n = 0;
  std::unordered_map<std::string, int> elems;
  while (std::getline(f, line)) {
    size_t pos = 0;
    while ((pos = line.find(" a ", pos)) != std::string::npos) {
      pos += 3;
      size_t end = line.find_first_of(",.", pos);
      std::string item = line.substr(pos, end - (pos));
      std::string element = item.substr(0, (item.find_first_of("- ")));
      std::string type = item.substr((item.find_first_of("- ") + 1));
      int id;
      if (elems.find(element) == elems.end()) {
        elems[element] = count;
        ++count;
        floors.push_back(0);
      }
      id = elems[element];
      if (type.find("generator") == std::string::npos)
        floors[id] |= (0x2 << (2 * floor_n));
      else
        floors[id] |= (0x1 << (2 * floor_n));
    }
    ++floor_n;
  }
}

void run_queue(std::queue<State> &q) {
  while (q.size()) {
    auto &current = q.front();
    if (current.is_solved()) break;
    if (current.is_valid()) {
      current.add_all_double_up(q);
    }
    q.pop();
  }
}

int main(int argc, char **argv) {
  std::string filename;
  if (argc == 1) filename = "input.txt";
  if (argc > 2) return 1;
  filename = std::string(argv[1]);
  std::vector<byte> init;
  read(filename, init);
  for (auto i = 3; i >= 0; --i) {
    auto idx = i * 2;
    int elem = 1;
    for (auto e : init) {
      if ((e >> idx) & 0x1)
        std::cout << "G" << elem;
      else
        std::cout << "..";
      if ((e >> idx) & 0x2)
        std::cout << "M" << elem;
      else
        std::cout << "..";
      elem++;
    }
    std::cout << "\n";
  }
  std::queue<State> queue;
  queue.emplace(State(std::move(init), 0));
  if (queue.front().is_valid()) {
    run_queue(queue);
  } else {
    std::cout << "Initial state not vaild\n";
  }
  return 0;
}
