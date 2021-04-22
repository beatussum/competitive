#include <iostream>

int main()
{
	std::uint16_t a, b, c;
	std::cin >> a >> b >> c;

	auto max = a + b + c;

	if (const auto t = a + b * c; t > max)
		max = t;

	if (const auto t = a * ( b + c ); t > max)
		max = t;

	if (const auto t = a * b * c; t > max)
		max = t;

	if (const auto t = ( a + b ) * c; t > max)
		max = t;

	std::cout << max << '\n';
}
