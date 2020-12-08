#include <iostream>
#include <fstream>

int calcFuel(int mass) {
	return std::max(mass / 3 - 2, 0);
}

int calcFuel2(int mass) {
	int result = calcFuel(mass);
	if (result == 0) return 0;
	return result + calcFuel2(result);
}

int main() {
	std::ifstream file;
	file.open("input.txt");
	int fuel = 0;
	int fuel2 = 0;
	std::string line;
	while (std::getline(file, line)) {
		fuel += calcFuel(std::stoi(line));
		fuel2 += calcFuel2(std::stoi(line));
	}
	std::cout << "Fuel part 1: " << fuel << std::endl;
	std::cout << "Fuel part 2: " << fuel2 << std::endl;
}