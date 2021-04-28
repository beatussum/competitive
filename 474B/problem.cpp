#include <iostream>
#include <vector>

int main()
{
	std::int64_t n;
	std::cin >> n;

	std::vector<std::int64_t> piles;
	piles.reserve(n);

	for (std::int64_t i = 0; i < n; ++i) {
		std::int64_t a;
		std::cin >> a;

		piles.push_back(a + ((i != 0) ? piles[i - 1] : 0));
	}

	std::int64_t m;
	std::cin >> m;

	std::vector<std::int64_t> jworms;
	jworms.reserve(m);

	for (std::int64_t i = 0; i < m; ++i) {
		std::int64_t j;
		std::cin >> j;

		jworms.push_back(j);
	}

	for (auto i = jworms.cbegin(); i < jworms.cend(); ++i) {
		for (std::int64_t a = 0, b = n, j;;) {
			j = a + (b - a) / 2;
			const auto prev = (j != 0) ? piles[j - 1] : 0;
			const auto diff = *i - prev - 1;				// first index is 1

			if (diff < 0) {
				b = j;
			} else if (diff >= ( piles[j] - prev )) {		// first index is 1
				a = j;
			} else {
				std::cout << (j + 1) << '\n';				// first index is 1
				break;
			}
		}
	}
}
