#include <algorithm>
#include <array>
#include <iostream>

std::array<char, 6> vowels = {'A', 'E', 'I', 'O', 'U', 'Y'};

bool is_vowel(const char c)
{
	return std::find(vowels.cbegin(), vowels.cend(), c) != vowels.cend();
}

int main()
{
	std::string str;
	std::cin >> str;

	str.reserve(str.capacity() * 2);

	for (auto i = str.begin(); i < str.end(); ++i) {
		if (is_vowel(std::toupper(*i))) {
			str.erase(i);
			--i;
		} else {
			if (std::isupper(*i))
				*i = std::tolower(*i);

			str.insert(i, '.');
			++i;
		}
	}

	std::cout << str << '\n';
}
