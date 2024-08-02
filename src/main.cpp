#include <cctype>
#include <cstdlib>
#include <filesystem>
#include <format>
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

std::string tokens_to_asm(const std::vector<Token> &tokens) {
  std::stringstream output;
  output << "global _start\n_start:\n";
  for (unsigned long i = 0; i < tokens.size(); i++) {
    const Token &token = tokens.at(i);
    if (token.type == TokenType::_return) {
      if (i + 1 < tokens.size() &&
          tokens.at(i + 1).type == TokenType::int_lit) {
        if (i + 2 < tokens.size() && tokens.at(i + 2).type == TokenType::semi) {
          output << "    mov rax, 60\n";
          output << "    mov rdi, " << tokens.at(i + 1).value.value() << "\n";
          output << "    syscall";
        }
      }
    }
  }
  return output.str();
}

void make_executable(const std::string &build_dir) {
  int code;
  try {
    // compile asm
    code = std::system(std::format("nasm -f elf64 -o {}/output.o {}/output.asm",
                                   build_dir, build_dir)
                           .c_str());

    if (code != 0) {
      std::cerr << "ERROR: nasm failed" << std::endl;
      exit(EXIT_FAILURE);
    }

    // link
    code =
        std::system(std::format("ld -o output {}/output.o", build_dir).c_str());

    if (code != 0) {
      std::cerr << "ERROR: ld failed" << std::endl;
      exit(EXIT_FAILURE);
    }
  } catch (std::exception &e) {
    std::cerr << "ERROR: " << e.what() << std::endl;
    exit(EXIT_FAILURE);
  }
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

  // make asm build folder if it doesnt exist
  std::string build_dir = ".skibidi";
  if (!std::filesystem::exists(build_dir)) {
    std::filesystem::create_directory(build_dir);
  }

  std::string asm_code = tokens_to_asm(tokens);
  {
    std::fstream file(std::format("{}/output.asm", build_dir), std::ios::out);
    file << asm_code;
  }

  make_executable(build_dir);

  /*std::cout << asm_code << std::endl;*/

  return 0;
}
