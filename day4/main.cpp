#include <cstddef>
#include <print>
#include <vector>
#include <fstream>
#include <string>
#include <cstdio>
#include <array>
#include <algorithm>

static size_t	get_answer(const std::vector<std::string>& input)
{
	size_t	answer{0};

	for (const std::string& line : input)
	{
		answer += std::ranges::count(line, 'x');
	}
	return answer;
}

static void	check_if_at_can_be_replaced(std::vector<std::string>& input, size_t row, size_t columns, size_t size_rows, size_t size_columns)
{
	std::array<std::array<int, 2>, 8>	templates{{{0, -1}, {0, 1}, {-1, -1}, {-1, 0}, {-1, 1}, {1, -1}, {1, 0}, {1, 1}}};
	size_t								count{0};
	char								character{'\0'};

	for(const std::array<int, 2>& coordinates : templates)
	{
		if ((row == 0 && coordinates.at(0) == -1) ||(columns == 0 && coordinates.at(1) == -1) ||
			((row + 1 == size_rows) && coordinates.at(0) == 1) || ((columns + 1 == size_columns) &&
			coordinates.at(1) == 1))
		{
			continue;
		}
		character = input[row + coordinates.at(0)][columns + coordinates.at(1)];
		if (character == '@' || character  == 'x')
		{
			++count;
		}
	}
	if (count < 4)
	{
		input[row][columns] = 'x';
	}
}

static void	replace_at_by_x(std::vector<std::string>& input)
{
	size_t	size_lines{input.size()};
	size_t	size_columns{input.at(0).size()};

	for (size_t row{0}; row < size_lines; ++row)
	{
		for (size_t columns{0}; columns < size_columns; ++columns)
		{
			if (input.at(row).at(columns) != '@')
			{
				continue;
			}
			check_if_at_can_be_replaced(input, row, columns, size_lines, size_columns);
		}
	}
}

static bool are_lines_symetric(const std::vector<std::string>& input_to_validate)
{
	size_t	length_string{input_to_validate.at(0).size()};

	for (const std::string& line : input_to_validate)
	{
		if (length_string != line.size())
		{
			return false;
		}
	}
	return true;
}

static std::vector<std::string>	get_input_lines(std::ifstream& file)
{
	std::vector<std::string>	input_per_line{};
	std::string					token{};

	while (std::getline(file, token))
	{
		input_per_line.emplace_back(token);
	}
	return input_per_line;
}

static bool	is_validate_amount_parameters_valid(int argc)
{
	if (argc == 1)
	{
		std::println(stderr, "No input file passed as parameter");
		return false;
	}
	else if (argc > 2)
	{
		std::println(stderr, "Too many parameters passed as input");
		return false;
	}
	return true;
}

static void	clear_x_from_input(std::vector<std::string>& input)
{
	for (std::string& line : input)
	{
		std::ranges::replace(line, 'x', '.');
	}
}

static size_t	get_part_two_answer(std::vector<std::string>& input)
{
	size_t	answer{0};

	clear_x_from_input(input);
	replace_at_by_x(input);
	answer = get_answer(input);
	if (answer != 0)
	{
		answer += get_part_two_answer(input);
		return answer;
	}
	return 0;
}

int main(int argc, char **argv)
{
	std::vector<std::string>	input_as_vector{};
	std::ifstream				file{};
	size_t						answer{0};

	if (!is_validate_amount_parameters_valid(argc))
	{
		return 1;
	}
	file.open(argv[1]);
	if (file.fail())
	{
		std::println(stderr, "Couldn't open the file: {0}", argv[1]);
		return 1;
	}
	input_as_vector = std::move(get_input_lines(file));
	file.close();
	if (input_as_vector.empty())
	{
		std::println(stderr, "The file provided as input was empty: {0}", argv[1]);
		return 1;
	}
	if (!are_lines_symetric(input_as_vector))
	{
		std::println(stderr, "Invalid map in the file: {0}", argv[1]);
		return 1;
	}
	replace_at_by_x(input_as_vector);
	answer = get_answer(input_as_vector);
	std::println("The answer for part 1 is {0}", answer);
	answer += get_part_two_answer(input_as_vector);
	std::println("The answer for part 2 is {0}", answer);
	return 0;
}
