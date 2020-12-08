unsigned int puzzle_input = 1350;

bool isWall(unsigned int x, unsigned int y) {
  unsigned int val = x * x + 3 * x + 2 * x * y + y + y * y + puzzle_input;
  int count = 0;
  for (auto i = 0u; i < sizeof(unsigned int) * 8; ++i) {
    if (val & 0x1) ++count;
    val >>= 0x1;
  }
  return val % 2;
}
