#include <iostream>
#include <fstream>
#include <sstream>
#include <regex>
#include <vector>

class Intcode {
	std::vector<int> m_code;
	std::vector<int>::iterator m_position;
	std::string m_init;

public:
	Intcode(std::string str) {
		m_init = str;
		setup(12, 2);
	}

	void setup(int noun, int verb) {
		m_code.clear();
		std::regex re("(\\d+),?");
		for (std::sregex_iterator it(m_init.begin(), m_init.end(), re), last{}; it != last; it++) {
			m_code.push_back(std::stoi((*it)[1]));
		}
		m_code[1] = noun;
		m_code[2] = verb;
		m_position = m_code.begin();
	}

	bool step() {
		int opcode = *m_position;
		if (opcode == 99) return false;

		int offset1 = *(m_position + 1);
		int offset2 = *(m_position + 2);
		int offset3 = *(m_position + 3);

		switch (opcode) {
			case 1: {
				m_code[offset3] = m_code[offset1] + m_code[offset2];
			}
			break;
			case 2: {
				m_code[offset3] = m_code[offset1] * m_code[offset2];
			}
			break;
		}

		m_position += 4;
	}

	int first() {
		return m_code[0];
	}

	int run() {
		while (step());
		return first();
	}
};

int main() {
	std::ifstream file("input.txt");
	std::stringstream ss;
	ss << file.rdbuf();
	auto str = ss.str();
	Intcode ic(str);

	std::cout << "part 1: " << ic.run() << std::endl;

	int noun = 0, verb = 0;
	do {
		ic.setup(noun++,verb);
		if (noun > 99) { verb++; noun = 0; }
	} while (ic.run() != 19690720);
	noun--;
	std::cout << "part 2: " << 100 * noun + verb << std::endl;
}