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

 
#include <iostream> 
#include <limits>
#include <string>

int main()
{
	int x = 0;

	int n;
	std::cin >> n;

	std::cin.ignore(std::numeric_limits<std::streamsize>::max(), '\n');

	for (int i = 0; i < n; ++i) {
		std::string statement;
		std::getline(std::cin, statement, '\n');

		if (statement.find("++") != std::string::npos) {
			++x;
		} else {
			--x;
		}
	}

	std::cout << std::to_string(x) << '\n';
}
