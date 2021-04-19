#include <iostream> 
#include <string>

int main()
{
	const std::string hello = "hello";

	std::string str;
	std::cin >> str;

	auto p_hello = hello.cbegin();

	for (auto p_str = str.cbegin(); (p_str < str.cend()) && (p_hello < hello.cend()); ++p_str) {
		if (*p_str == *p_hello)
			++p_hello;
	}

	std::cout << ((p_hello == hello.cend()) ? "YES" : "NO") << '\n';
}
