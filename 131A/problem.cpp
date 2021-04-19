/*
 * Copyright (C) 2021 Mattéo Rossillol‑‑Laruelle <beatussum@protonmail.com>
 *
 * This program is free software:  you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published  by
 * the Free Software Foundation, either version 3 of the License, or (at
 * your option) any later version.
 *
 * This program is distributed in the hope that it will be  useful,
 * but WITHOUT ANY WARRANTY;   without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU
 * General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License along
 * with this program. If not, see <https://www.gnu.org/licenses/>.
 */

 
#include <algorithm>
#include <cctype>
#include <iostream> 
#include <string>

int main()
{
	std::string str;
	std::cin >> str;

	if (std::all_of( str.cbegin() + 1, str.cend()
	               , [] (const auto c) { return std::isupper(c) != 0; })) 
	{
		if (auto& f = str.front(); std::isupper(f) != 0) {
			f = std::tolower(f);
		} else {
			f = std::toupper(f);
		}
		
		for (auto i = str.begin() + 1; i < str.cend(); ++i)
			*i = std::tolower(*i);
	}

	std::cout << str << '\n';
}
