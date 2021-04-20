#include <algorithm>
#include <array>
#include <cmath>
#include <iostream>

constexpr std::array<std::uint16_t, 14> lucky_numbers = {
	4, 7,
	44, 47, 74, 77,
	444, 447, 474, 477, 744, 747, 774, 777
};

std::uint8_t get_number_of_digits(std::uint16_t num)
{
	std::uint8_t ret = 0;

	for (; num > 0; num /= 10)
		++ret;

	return ret;
}

bool is_lucky(const std::uint16_t num)
{
	return std::find(lucky_numbers.cbegin(), lucky_numbers.cend(), num) != lucky_numbers.cend();
}

bool is_almost_lucky(const std::uint16_t num)
{
	std::uint8_t n = 2;

	for (std::uint8_t i = 1; i < get_number_of_digits(num); ++i)
		n += std::pow(2, i);

	for (auto i = lucky_numbers.cbegin(); i < lucky_numbers.cbegin() + n; ++i) {
		if (num % *i == 0)
			return true;
	}

	return false;
}

int main()
{
	std::uint16_t n;
	std::cin >> n;

	std::cout << ((is_lucky(n) || is_almost_lucky(n)) ? "YES" : "NO") << '\n';
}
