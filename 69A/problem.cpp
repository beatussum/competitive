#include <iostream> 
#include <tuple>

using vec_t = std::tuple<int, int, int>;

bool is_null(const vec_t& vec) 
{
	return (std::get<0>(vec) == 0) 
		&& (std::get<1>(vec) == 0) 
		&& (std::get<2>(vec) == 0);
}

vec_t& operator+=(vec_t& l, const vec_t& r)
{
	l = {
		std::get<0>(l) + std::get<0>(r),
		std::get<1>(l) + std::get<1>(r),
		std::get<2>(l) + std::get<2>(r)
	};

	return l;
}

int main()
{
	int n;
	std::cin >> n;

	vec_t vec;
	for (int i = 0; i < n; ++i) {
		int x, y, z;
		std::cin >> x >> y >> z;

		vec += {x, y, z};
	}

	std::cout << (is_null(vec) ? "YES" : "NO") << '\n';
}
