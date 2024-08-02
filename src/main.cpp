#include <cctype>
#include <cstdlib>
#include <fstream>
#include <iostream>
#include <optional>
#include <sstream>
#include <vector>

enum class TokenType {
  _return,
  int_lit,
  semi,
};

struct Token {
  TokenType type;
  std::optional<std::string> value;
};

std::vector<Token> tokenize(const std::string &contents) {
  std::string buf = "";
  std::vector<Token> tokens;

  for (unsigned long i = 0; i < contents.length(); i++) {
    char c = contents.at(i);
    if (std::isalpha(c)) {
      buf.push_back(c);
      i++;

      while (std::isalnum(contents.at(i))) {
        buf.push_back(contents.at(i));
        /*std::cout << contents.at(i);*/
        i++;
      }
      i--;

      if (buf == "return") {
        tokens.push_back({.type = TokenType::_return, .value = buf});
        buf.clear();
      } else {
        std::cerr << "Unknown token: " << buf << std::endl;
        buf.clear();
        exit(EXIT_FAILURE);
      }
    } else if (std::isdigit(c)) {
      buf.push_back(c);
      i++;

      while (std::isdigit(contents.at(i))) {
        buf.push_back(contents.at(i));
        /*std::cout << contents.at(i);*/
        i++;
      }
      i--;
      tokens.push_back({.type = TokenType::int_lit, .value = buf});

      buf.clear();
    } else if (c == ';') {
      tokens.push_back({.type = TokenType::semi, .value = std::nullopt});
      buf.clear();
    }
  }

  return tokens;
}

int main(int argc, char *argv[]) {
  if (argc != 2) {
    std::cerr << "Usage: " << argv[0] << " <file>" << std::endl;
    exit(EXIT_FAILURE);
  }

  std::cout << argv[0] << std::endl;

  std::fstream input(argv[1], std::ios::in);

  std::string contents;
  {
    std::stringstream contents_stream;
    std::fstream input(argv[1], std::ios::in);
    contents_stream << input.rdbuf();
    contents = contents_stream.str();
  }

  /*std::cout << contents << std::endl;*/
  std::vector<Token> tokens = tokenize(contents);

  for (auto token : tokens) {
    std::cout << ": " << token.value.value_or("") << std::endl;
  }

  return 0;
}
