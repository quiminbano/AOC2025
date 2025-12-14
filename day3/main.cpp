#include <cstddef>
#include <cstdint>
#include <print>
#include <string>
#include <fstream>
#include <cstdio>
#include <vector>
#include <algorithm>
#include <span>

static uint64_t getSumOfJoltsPartTwo(const std::vector<std::string>& inputToAnalyze)
{
	uint64_t	result{0};
	char		maxChar{0};
	char		minChar{0};
	size_t		cohortIndex{0};
	size_t		filterIndex{0};
	size_t		sizeStrings{0};
	std::string	bankOfChars{};

	for(auto& input : inputToAnalyze)
	{
		bankOfChars.clear();
		sizeStrings = input.size();
		cohortIndex = sizeStrings - 12;
		maxChar = *(std::ranges::max_element(input));
		minChar = *(std::ranges::min_element(input));
		filterIndex = 0;
		while (maxChar >= minChar)
		{
			while (maxChar >= input[cohortIndex] && input.find(maxChar, filterIndex) != std::string::npos &&
					input.find(maxChar, filterIndex) <= cohortIndex && bankOfChars.size() < 12)
			{
				bankOfChars.push_back(maxChar);
				filterIndex = (input.find(maxChar, filterIndex) + 1);
				++cohortIndex;
			}
			--maxChar;
			if (bankOfChars.size() == 12 || maxChar < input[cohortIndex])
			{
				break;
			}
		}
		if ((sizeStrings - cohortIndex) > 0 && (sizeStrings - cohortIndex) == (12 - bankOfChars.size()))
		{
			bankOfChars += input.substr(cohortIndex, sizeStrings);
		}
		result += std::stoull(bankOfChars);
	}
	return result;
}

static uint64_t getSumOfJoltsPartOne(const std::vector<std::string>& inputToAnalyze)
{
	uint64_t	result{0};
	char		firstChar{0};
	char		secondChar{0};

	for(auto& input : inputToAnalyze)
	{
		firstChar = *(std::ranges::max_element(input));
		size_t length = input.size();
		size_t position = input.find(firstChar);
		if (std::ranges::count(input, firstChar) >= 2)
		{
			secondChar = firstChar;
		}
		else if (position == length - 1)
		{
			secondChar = firstChar;
			std::span<const char> slice(input.begin(), input.end() - 1);
			firstChar = *(std::ranges::max_element(slice));
		}
		else if (position == 0)
		{
			std::span<const char> slice(input.begin() + 1, input.end());
			secondChar = *(std::ranges::max_element(slice));
		}
		else
		{
			std::span<const char> slice1(input.begin() + position + 1, input.end());
			secondChar = *(std::ranges::max_element(slice1));
			uint64_t number1{((firstChar - 48u) * 10u) + (secondChar - 48u)};
			std::span<const char> slice2(input.begin(), input.end() - (length - position));
			secondChar = firstChar;
			firstChar = *(std::ranges::max_element(slice2));
			uint64_t number2{((firstChar - 48u) * 10u) + (secondChar - 48u)};
			result += std::max(number1, number2);
			continue;
		}
		result += ((firstChar - 48) * 10) + (secondChar - 48);
	}
	return result;
}

static std::vector<std::string>	getLines(std::ifstream& file)
{
	std::vector<std::string>	result{};
	std::string					token{};

	while(std::getline(file, token))
	{
		result.emplace_back(token);
	}
	return result;
}

int main(int argc, char ** argv)
{
	if (argc != 2)
	{
		std::println(stderr, "Invalid amount of parameters passed to the program");
		return 1;
	}
	std::ifstream	file(argv[1]);

	if (file.fail())
	{
		std::println(stderr, "Failed to open the file");
		return (1);
	}
	std::vector<std::string>	fileLines{getLines(file)};
	uint64_t				 	sumOfJoltsPartOne{getSumOfJoltsPartOne(fileLines)};
	uint64_t					sumOfJoltsPartTwo{getSumOfJoltsPartTwo(fileLines)};
	std::println("The sum of jolts, in part one, is {0}", sumOfJoltsPartOne);
	std::println("The sum of jolts, in part two, is {0}", sumOfJoltsPartTwo);
	file.close();
	return 0;
}
