#include <iostream>
#include <regex>

int main() {
	std::string s = "1,4,67,2,6";
	std::regex re("(\\d+),?");
	
	for (std::sregex_iterator it(s.begin(), s.end(), re), last{}; it != last; it++) {
		std::cout << (*it)[1] << std::endl;
	}
	
}