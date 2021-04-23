#include <iostream>

int main()
{
	std::uint64_t n, m;
	std::cin >> n >> m;
	
	std::uint64_t time = 0;

	for (std::uint64_t i = 0, a = 1, b; i < m; ++i) {
		std::cin >> b;

		const std::int64_t step = b - a;

		time += step;

		if (step < 0)
			time += n;

		a = b;
	}

	std::cout << time << '\n';
}
