```text
Undefined symbols for architecture arm64:
  "std::__1::basic_string<char, std::__1::char_traits<char>, std::__1::allocator<char>>::compare(unsigned long, unsigned long, char const*, unsigned long) const", referenced from:
      bool std::__1::operator==[abi:ue170006]<char, std::__1::char_traits<char>, std::__1::allocator<char>>(std::__1::basic_string<char, std::__1::char_traits<char>, std::__1::allocator<char>> const&, char const*) in librust_lib_wlist_ui.a[arm64][2037](506d48707ec1d76d-simple_tokenizer.o)
      std::__1::basic_string<char, std::__1::char_traits<char>, std::__1::allocator<char>>::compare[abi:ue170006](unsigned long, unsigned long, std::__1::basic_string<char, std::__1::char_traits<char>, std::__1::allocator<char>> const&) const in librust_lib_wlist_ui.a[arm64][2037](506d48707ec1d76d-simple_tokenizer.o)
  "std::__1::locale::has_facet(std::__1::locale::id&) const", referenced from:
      bool std::__1::has_facet[abi:ue170006]<std::__1::codecvt<char, char, __mbstate_t>>(std::__1::locale const&) in librust_lib_wlist_ui.a[arm64][2037](506d48707ec1d76d-simple_tokenizer.o)
  "std::__1::locale::use_facet(std::__1::locale::id&) const", referenced from:
      std::__1::ctype<char> const& std::__1::use_facet[abi:ue170006]<std::__1::ctype<char>>(std::__1::locale const&) in librust_lib_wlist_ui.a[arm64][2035](506d48707ec1d76d-pinyin.o)
      std::__1::codecvt<char, char, __mbstate_t> const& std::__1::use_facet[abi:ue170006]<std::__1::codecvt<char, char, __mbstate_t>>(std::__1::locale const&) in librust_lib_wlist_ui.a[arm64][2037](506d48707ec1d76d-simple_tokenizer.o)
  "std::__1::ios_base::getloc() const", referenced from:
      std::__1::basic_ios<char, std::__1::char_traits<char>>::widen[abi:ue170006](char) const in librust_lib_wlist_ui.a[arm64][2035](506d48707ec1d76d-pinyin.o)
  "std::logic_error::logic_error(char const*)", referenced from:
      std::length_error::length_error[abi:ue170006](char const*) in librust_lib_wlist_ui.a[arm64][2035](506d48707ec1d76d-pinyin.o)
  "std::length_error::~length_error()", referenced from:
      std::__1::__throw_length_error[abi:ue170006](char const*) in librust_lib_wlist_ui.a[arm64][2035](506d48707ec1d76d-pinyin.o)
  "std::runtime_error::runtime_error(char const*)", referenced from:
      simple_tokenizer::PinYin::codepoint(std::__1::basic_string<char, std::__1::char_traits<char>, std::__1::allocator<char>> const&) in librust_lib_wlist_ui.a[arm64][2035](506d48707ec1d76d-pinyin.o)
  "std::runtime_error::~runtime_error()", referenced from:
      simple_tokenizer::PinYin::codepoint(std::__1::basic_string<char, std::__1::char_traits<char>, std::__1::allocator<char>> const&) in librust_lib_wlist_ui.a[arm64][2035](506d48707ec1d76d-pinyin.o)
  "std::bad_array_new_length::bad_array_new_length()", referenced from:
      std::__throw_bad_array_new_length[abi:ue170006]() in librust_lib_wlist_ui.a[arm64][2035](506d48707ec1d76d-pinyin.o)
  "std::bad_array_new_length::~bad_array_new_length()", referenced from:
      std::__throw_bad_array_new_length[abi:ue170006]() in librust_lib_wlist_ui.a[arm64][2035](506d48707ec1d76d-pinyin.o)
  "std::__1::__next_prime(unsigned long)", referenced from:
      void std::__1::__hash_table<unsigned int, std::__1::hash<unsigned int>, std::__1::equal_to<unsigned int>, std::__1::allocator<unsigned int>>::__rehash<true>(unsigned long) in librust_lib_wlist_ui.a[arm64][2037](506d48707ec1d76d-simple_tokenizer.o)
      void std::__1::__hash_table<unsigned int, std::__1::hash<unsigned int>, std::__1::equal_to<unsigned int>, std::__1::allocator<unsigned int>>::__rehash<true>(unsigned long) in librust_lib_wlist_ui.a[arm64][2037](506d48707ec1d76d-simple_tokenizer.o)
      void std::__1::__hash_table<std::__1::__hash_value_type<unsigned int, cppjieba::TrieNode*>, std::__1::__unordered_map_hasher<unsigned int, std::__1::__hash_value_type<unsigned int, cppjieba::TrieNode*>, std::__1::hash<unsigned int>, std::__1::equal_to<unsigned int>, true>, std::__1::__unordered_map_equal<unsigned int, std::__1::__hash_value_type<unsigned int, cppjieba::TrieNode*>, std::__1::equal_to<unsigned int>, std::__1::hash<unsigned int>, true>, std::__1::allocator<std::__1::__hash_value_type<unsigned int, cppjieba::TrieNode*>>>::__rehash<true>(unsigned long) in librust_lib_wlist_ui.a[arm64][2037](506d48707ec1d76d-simple_tokenizer.o)
      void std::__1::__hash_table<std::__1::__hash_value_type<unsigned int, cppjieba::TrieNode*>, std::__1::__unordered_map_hasher<unsigned int, std::__1::__hash_value_type<unsigned int, cppjieba::TrieNode*>, std::__1::hash<unsigned int>, std::__1::equal_to<unsigned int>, true>, std::__1::__unordered_map_equal<unsigned int, std::__1::__hash_value_type<unsigned int, cppjieba::TrieNode*>, std::__1::equal_to<unsigned int>, std::__1::hash<unsigned int>, true>, std::__1::allocator<std::__1::__hash_value_type<unsigned int, cppjieba::TrieNode*>>>::__rehash<true>(unsigned long) in librust_lib_wlist_ui.a[arm64][2037](506d48707ec1d76d-simple_tokenizer.o)
      void std::__1::__hash_table<std::__1::__hash_value_type<unsigned int, double>, std::__1::__unordered_map_hasher<unsigned int, std::__1::__hash_value_type<unsigned int, double>, std::__1::hash<unsigned int>, std::__1::equal_to<unsigned int>, true>, std::__1::__unordered_map_equal<unsigned int, std::__1::__hash_value_type<unsigned int, double>, std::__1::equal_to<unsigned int>, std::__1::hash<unsigned int>, true>, std::__1::allocator<std::__1::__hash_value_type<unsigned int, double>>>::__rehash<true>(unsigned long) in librust_lib_wlist_ui.a[arm64][2037](506d48707ec1d76d-simple_tokenizer.o)
      void std::__1::__hash_table<std::__1::__hash_value_type<unsigned int, double>, std::__1::__unordered_map_hasher<unsigned int, std::__1::__hash_value_type<unsigned int, double>, std::__1::hash<unsigned int>, std::__1::equal_to<unsigned int>, true>, std::__1::__unordered_map_equal<unsigned int, std::__1::__hash_value_type<unsigned int, double>, std::__1::equal_to<unsigned int>, std::__1::hash<unsigned int>, true>, std::__1::allocator<std::__1::__hash_value_type<unsigned int, double>>>::__rehash<true>(unsigned long) in librust_lib_wlist_ui.a[arm64][2037](506d48707ec1d76d-simple_tokenizer.o)
      void std::__1::__hash_table<std::__1::__hash_value_type<std::__1::basic_string<char, std::__1::char_traits<char>, std::__1::allocator<char>>, double>, std::__1::__unordered_map_hasher<std::__1::basic_string<char, std::__1::char_traits<char>, std::__1::allocator<char>>, std::__1::__hash_value_type<std::__1::basic_string<char, std::__1::char_traits<char>, std::__1::allocator<char>>, double>, std::__1::hash<std::__1::basic_string<char, std::__1::char_traits<char>, std::__1::allocator<char>>>, std::__1::equal_to<std::__1::basic_string<char, std::__1::char_traits<char>, std::__1::allocator<char>>>, true>, std::__1::__unordered_map_equal<std::__1::basic_string<char, std::__1::char_traits<char>, std::__1::allocator<char>>, std::__1::__hash_value_type<std::__1::basic_string<char, std::__1::char_traits<char>, std::__1::allocator<char>>, double>, std::__1::equal_to<std::__1::basic_string<char, std::__1::char_traits<char>, std::__1::allocator<char>>>, std::__1::hash<std::__1::basic_string<char, std::__1::char_traits<char>, std::__1::allocator<char>>>, true>, std::__1::allocator<std::__1::__hash_value_type<std::__1::basic_string<char, std::__1::char_traits<char>, std::__1::allocator<char>>, double>>>::__rehash<true>(unsigned long) in librust_lib_wlist_ui.a[arm64][2037](506d48707ec1d76d-simple_tokenizer.o)
      void std::__1::__hash_table<std::__1::__hash_value_type<std::__1::basic_string<char, std::__1::char_traits<char>, std::__1::allocator<char>>, double>, std::__1::__unordered_map_hasher<std::__1::basic_string<char, std::__1::char_traits<char>, std::__1::allocator<char>>, std::__1::__hash_value_type<std::__1::basic_string<char, std::__1::char_traits<char>, std::__1::allocator<char>>, double>, std::__1::hash<std::__1::basic_string<char, std::__1::char_traits<char>, std::__1::allocator<char>>>, std::__1::equal_to<std::__1::basic_string<char, std::__1::char_traits<char>, std::__1::allocator<char>>>, true>, std::__1::__unordered_map_equal<std::__1::basic_string<char, std::__1::char_traits<char>, std::__1::allocator<char>>, std::__1::__hash_value_type<std::__1::basic_string<char, std::__1::char_traits<char>, std::__1::allocator<char>>, double>, std::__1::equal_to<std::__1::basic_string<char, std::__1::char_traits<char>, std::__1::allocator<char>>>, std::__1::hash<std::__1::basic_string<char, std::__1::char_traits<char>, std::__1::allocator<char>>>, true>, std::__1::allocator<std::__1::__hash_value_type<std::__1::basic_string<char, std::__1::char_traits<char>, std::__1::allocator<char>>, double>>>::__rehash<true>(unsigned long) in librust_lib_wlist_ui.a[arm64][2037](506d48707ec1d76d-simple_tokenizer.o)
      ...
  "std::__1::basic_string<char, std::__1::char_traits<char>, std::__1::allocator<char>>::erase(unsigned long, unsigned long)", referenced from:
      std::__1::basic_string<char, std::__1::char_traits<char>, std::__1::allocator<char>>::erase[abi:ue170006](std::__1::__wrap_iter<char const*>) in librust_lib_wlist_ui.a[arm64][2035](506d48707ec1d76d-pinyin.o)
      std::__1::basic_string<char, std::__1::char_traits<char>, std::__1::allocator<char>>::erase[abi:ue170006](std::__1::__wrap_iter<char const*>, std::__1::__wrap_iter<char const*>) in librust_lib_wlist_ui.a[arm64][2037](506d48707ec1d76d-simple_tokenizer.o)
  "std::__1::basic_string<char, std::__1::char_traits<char>, std::__1::allocator<char>>::__init(char const*, unsigned long)", referenced from:
      std::__1::basic_string<char, std::__1::char_traits<char>, std::__1::allocator<char>>::basic_string[abi:ue170006]<0>(char const*) in librust_lib_wlist_ui.a[arm64][2034](506d48707ec1d76d-entry.o)
  "std::__1::basic_string<char, std::__1::char_traits<char>, std::__1::allocator<char>>::__init(unsigned long, char)", referenced from:
      std::__1::basic_string<char, std::__1::char_traits<char>, std::__1::allocator<char>>::basic_string[abi:ue170006](unsigned long, char) in librust_lib_wlist_ui.a[arm64][2037](506d48707ec1d76d-simple_tokenizer.o)
  "std::__1::basic_string<char, std::__1::char_traits<char>, std::__1::allocator<char>>::append(char const*)", referenced from:
      simple_tokenizer::SimpleTokenizer::append_result(std::__1::basic_string<char, std::__1::char_traits<char>, std::__1::allocator<char>>&, std::__1::basic_string<char, std::__1::char_traits<char>, std::__1::allocator<char>>, simple_tokenizer::TokenCategory, int, int) in librust_lib_wlist_ui.a[arm64][2037](506d48707ec1d76d-simple_tokenizer.o)
      simple_tokenizer::SimpleTokenizer::append_result(std::__1::basic_string<char, std::__1::char_traits<char>, std::__1::allocator<char>>&, std::__1::basic_string<char, std::__1::char_traits<char>, std::__1::allocator<char>>, simple_tokenizer::TokenCategory, int, int) in librust_lib_wlist_ui.a[arm64][2037](506d48707ec1d76d-simple_tokenizer.o)
      simple_tokenizer::SimpleTokenizer::append_result(std::__1::basic_string<char, std::__1::char_traits<char>, std::__1::allocator<char>>&, std::__1::basic_string<char, std::__1::char_traits<char>, std::__1::allocator<char>>, simple_tokenizer::TokenCategory, int, int) in librust_lib_wlist_ui.a[arm64][2037](506d48707ec1d76d-simple_tokenizer.o)
      simple_tokenizer::SimpleTokenizer::append_result(std::__1::basic_string<char, std::__1::char_traits<char>, std::__1::allocator<char>>&, std::__1::basic_string<char, std::__1::char_traits<char>, std::__1::allocator<char>>, simple_tokenizer::TokenCategory, int, int) in librust_lib_wlist_ui.a[arm64][2037](506d48707ec1d76d-simple_tokenizer.o)
      simple_tokenizer::SimpleTokenizer::append_result(std::__1::basic_string<char, std::__1::char_traits<char>, std::__1::allocator<char>>&, std::__1::basic_string<char, std::__1::char_traits<char>, std::__1::allocator<char>>, simple_tokenizer::TokenCategory, int, int) in librust_lib_wlist_ui.a[arm64][2037](506d48707ec1d76d-simple_tokenizer.o)
      simple_tokenizer::SimpleTokenizer::append_result(std::__1::basic_string<char, std::__1::char_traits<char>, std::__1::allocator<char>>&, std::__1::basic_string<char, std::__1::char_traits<char>, std::__1::allocator<char>>, simple_tokenizer::TokenCategory, int, int) in librust_lib_wlist_ui.a[arm64][2037](506d48707ec1d76d-simple_tokenizer.o)
      simple_tokenizer::SimpleTokenizer::append_result(std::__1::basic_string<char, std::__1::char_traits<char>, std::__1::allocator<char>>&, std::__1::basic_string<char, std::__1::char_traits<char>, std::__1::allocator<char>>, simple_tokenizer::TokenCategory, int, int) in librust_lib_wlist_ui.a[arm64][2037](506d48707ec1d76d-simple_tokenizer.o)
      ...
  "std::__1::basic_string<char, std::__1::char_traits<char>, std::__1::allocator<char>>::append(char const*, unsigned long)", referenced from:
      std::__1::basic_string<char, std::__1::char_traits<char>, std::__1::allocator<char>>::append[abi:ue170006](std::__1::basic_string<char, std::__1::char_traits<char>, std::__1::allocator<char>> const&) in librust_lib_wlist_ui.a[arm64][2035](506d48707ec1d76d-pinyin.o)
  "std::__1::basic_string<char, std::__1::char_traits<char>, std::__1::allocator<char>>::resize(unsigned long, char)", referenced from:
      std::__1::basic_string<char, std::__1::char_traits<char>, std::__1::allocator<char>>::resize[abi:ue170006](unsigned long) in librust_lib_wlist_ui.a[arm64][2035](506d48707ec1d76d-pinyin.o)
  "std::__1::basic_string<char, std::__1::char_traits<char>, std::__1::allocator<char>>::push_back(char)", referenced from:
      std::__1::basic_string<char, std::__1::char_traits<char>, std::__1::allocator<char>>::operator+=[abi:ue170006](char) in librust_lib_wlist_ui.a[arm64][2034](506d48707ec1d76d-entry.o)
      simple_tokenizer::PinYin::to_plain(std::__1::basic_string<char, std::__1::char_traits<char>, std::__1::allocator<char>> const&) in librust_lib_wlist_ui.a[arm64][2035](506d48707ec1d76d-pinyin.o)
      simple_tokenizer::PinYin::to_plain(std::__1::basic_string<char, std::__1::char_traits<char>, std::__1::allocator<char>> const&) in librust_lib_wlist_ui.a[arm64][2035](506d48707ec1d76d-pinyin.o)
      simple_tokenizer::PinYin::to_plain(std::__1::basic_string<char, std::__1::char_traits<char>, std::__1::allocator<char>> const&) in librust_lib_wlist_ui.a[arm64][2035](506d48707ec1d76d-pinyin.o)
      std::__1::basic_istream<char, std::__1::char_traits<char>>& std::__1::getline[abi:ue170006]<char, std::__1::char_traits<char>, std::__1::allocator<char>>(std::__1::basic_istream<char, std::__1::char_traits<char>>&, std::__1::basic_string<char, std::__1::char_traits<char>, std::__1::allocator<char>>&, char) in librust_lib_wlist_ui.a[arm64][2035](506d48707ec1d76d-pinyin.o)
      simple_tokenizer::PinYin::split_pinyin(std::__1::basic_string<char, std::__1::char_traits<char>, std::__1::allocator<char>> const&) in librust_lib_wlist_ui.a[arm64][2035](506d48707ec1d76d-pinyin.o)
      simple_tokenizer::PinYin::split_pinyin(std::__1::basic_string<char, std::__1::char_traits<char>, std::__1::allocator<char>> const&) in librust_lib_wlist_ui.a[arm64][2035](506d48707ec1d76d-pinyin.o)
      ...
  "std::__1::basic_string<char, std::__1::char_traits<char>, std::__1::allocator<char>>::basic_string(std::__1::basic_string<char, std::__1::char_traits<char>, std::__1::allocator<char>> const&)", referenced from:
      cmrc::embedded_filesystem::open(std::__1::basic_string<char, std::__1::char_traits<char>, std::__1::allocator<char>> const&) const in librust_lib_wlist_ui.a[arm64][2035](506d48707ec1d76d-pinyin.o)
      simple_tokenizer::PinYin::split_pinyin(std::__1::basic_string<char, std::__1::char_traits<char>, std::__1::allocator<char>> const&) in librust_lib_wlist_ui.a[arm64][2035](506d48707ec1d76d-pinyin.o)
      simple_tokenizer::PinYin::split_pinyin(std::__1::basic_string<char, std::__1::char_traits<char>, std::__1::allocator<char>> const&) in librust_lib_wlist_ui.a[arm64][2035](506d48707ec1d76d-pinyin.o)
      simple_tokenizer::PinYin::split_pinyin(std::__1::basic_string<char, std::__1::char_traits<char>, std::__1::allocator<char>> const&) in librust_lib_wlist_ui.a[arm64][2035](506d48707ec1d76d-pinyin.o)
      cmrc::embedded_filesystem::_get(std::__1::basic_string<char, std::__1::char_traits<char>, std::__1::allocator<char>>) const in librust_lib_wlist_ui.a[arm64][2035](506d48707ec1d76d-pinyin.o)
      std::__1::pair<std::__1::basic_string<char, std::__1::char_traits<char>, std::__1::allocator<char>> const, char>::pair[abi:ue170006](std::__1::pair<std::__1::basic_string<char, std::__1::char_traits<char>, std::__1::allocator<char>> const, char> const&) in librust_lib_wlist_ui.a[arm64][2035](506d48707ec1d76d-pinyin.o)
      void std::__1::allocator<std::__1::__tree_node<std::__1::basic_string<char, std::__1::char_traits<char>, std::__1::allocator<char>>, void*>>::construct[abi:ue170006]<std::__1::basic_string<char, std::__1::char_traits<char>, std::__1::allocator<char>>, std::__1::basic_string<char, std::__1::char_traits<char>, std::__1::allocator<char>> const&>(std::__1::basic_string<char, std::__1::char_traits<char>, std::__1::allocator<char>>*, std::__1::basic_string<char, std::__1::char_traits<char>, std::__1::allocator<char>> const&) in librust_lib_wlist_ui.a[arm64][2035](506d48707ec1d76d-pinyin.o)
      ...
  "std::__1::basic_string<char, std::__1::char_traits<char>, std::__1::allocator<char>>::basic_string(std::__1::basic_string<char, std::__1::char_traits<char>, std::__1::allocator<char>> const&, unsigned long, unsigned long, std::__1::allocator<char> const&)", referenced from:
      std::__1::basic_string<char, std::__1::char_traits<char>, std::__1::allocator<char>>::substr[abi:ue170006](unsigned long, unsigned long) const in librust_lib_wlist_ui.a[arm64][2035](506d48707ec1d76d-pinyin.o)
  "std::__1::basic_string<char, std::__1::char_traits<char>, std::__1::allocator<char>>::~basic_string()", referenced from:
      simple_query(sqlite3_context*, int, sqlite3_value**) in librust_lib_wlist_ui.a[arm64][2034](506d48707ec1d76d-entry.o)
      simple_query(sqlite3_context*, int, sqlite3_value**) in librust_lib_wlist_ui.a[arm64][2034](506d48707ec1d76d-entry.o)
      jieba_query(sqlite3_context*, int, sqlite3_value**) in librust_lib_wlist_ui.a[arm64][2034](506d48707ec1d76d-entry.o)
      jieba_query(sqlite3_context*, int, sqlite3_value**) in librust_lib_wlist_ui.a[arm64][2034](506d48707ec1d76d-entry.o)
      jieba_dict(sqlite3_context*, int, sqlite3_value**) in librust_lib_wlist_ui.a[arm64][2034](506d48707ec1d76d-entry.o)
      jieba_dict(sqlite3_context*, int, sqlite3_value**) in librust_lib_wlist_ui.a[arm64][2034](506d48707ec1d76d-entry.o)
      simple_tokenizer::PinYin::PinYin() in librust_lib_wlist_ui.a[arm64][2035](506d48707ec1d76d-pinyin.o)
      simple_tokenizer::PinYin::PinYin() in librust_lib_wlist_ui.a[arm64][2035](506d48707ec1d76d-pinyin.o)
      simple_tokenizer::PinYin::PinYin() in librust_lib_wlist_ui.a[arm64][2035](506d48707ec1d76d-pinyin.o)
      simple_tokenizer::PinYin::PinYin() in librust_lib_wlist_ui.a[arm64][2035](506d48707ec1d76d-pinyin.o)
      simple_tokenizer::PinYin::PinYin() in librust_lib_wlist_ui.a[arm64][2035](506d48707ec1d76d-pinyin.o)
      simple_tokenizer::PinYin::PinYin() in librust_lib_wlist_ui.a[arm64][2035](506d48707ec1d76d-pinyin.o)
      ...
  "std::__1::basic_string<char, std::__1::char_traits<char>, std::__1::allocator<char>>::operator=(std::__1::basic_string<char, std::__1::char_traits<char>, std::__1::allocator<char>> const&)", referenced from:
      jieba_dict(sqlite3_context*, int, sqlite3_value**) in librust_lib_wlist_ui.a[arm64][2034](506d48707ec1d76d-entry.o)
      std::__1::basic_stringbuf<char, std::__1::char_traits<char>, std::__1::allocator<char>>::str(std::__1::basic_string<char, std::__1::char_traits<char>, std::__1::allocator<char>> const&) in librust_lib_wlist_ui.a[arm64][2035](506d48707ec1d76d-pinyin.o)
      std::__1::pair<std::__1::__tree_const_iterator<std::__1::basic_string<char, std::__1::char_traits<char>, std::__1::allocator<char>>, std::__1::__tree_node<std::__1::basic_string<char, std::__1::char_traits<char>, std::__1::allocator<char>>, void*>*, long>, std::__1::basic_string<char, std::__1::char_traits<char>, std::__1::allocator<char>>*> std::__1::__copy_loop<std::__1::_ClassicAlgPolicy>::operator()[abi:ue170006]<std::__1::__tree_const_iterator<std::__1::basic_string<char, std::__1::char_traits<char>, std::__1::allocator<char>>, std::__1::__tree_node<std::__1::basic_string<char, std::__1::char_traits<char>, std::__1::allocator<char>>, void*>*, long>, std::__1::__tree_const_iterator<std::__1::basic_string<char, std::__1::char_traits<char>, std::__1::allocator<char>>, std::__1::__tree_node<std::__1::basic_string<char, std::__1::char_traits<char>, std::__1::allocator<char>>, void*>*, long>, std::__1::basic_string<char, std::__1::char_traits<char>, std::__1::allocator<char>>*>(std::__1::__tree_const_iterator<std::__1::basic_string<char, std::__1::char_traits<char>, std::__1::allocator<char>>, std::__1::__tree_node<std::__1::basic_string<char, std::__1::char_traits<char>, std::__1::allocator<char>>, void*>*, long>, std::__1::__tree_const_iterator<std::__1::basic_string<char, std::__1::char_traits<char>, std::__1::allocator<char>>, std::__1::__tree_node<std::__1::basic_string<char, std::__1::char_traits<char>, std::__1::allocator<char>>, void*>*, long>, std::__1::basic_string<char, std::__1::char_traits<char>, std::__1::allocator<char>>*) const in librust_lib_wlist_ui.a[arm64][2035](506d48707ec1d76d-pinyin.o)
      std::__1::pair<std::__1::basic_string<char, std::__1::char_traits<char>, std::__1::allocator<char>>*, std::__1::basic_string<char, std::__1::char_traits<char>, std::__1::allocator<char>>*> std::__1::__copy_loop<std::__1::_ClassicAlgPolicy>::operator()[abi:ue170006]<std::__1::basic_string<char, std::__1::char_traits<char>, std::__1::allocator<char>>*, std::__1::basic_string<char, std::__1::char_traits<char>, std::__1::allocator<char>>*, std::__1::basic_string<char, std::__1::char_traits<char>, std::__1::allocator<char>>*>(std::__1::basic_string<char, std::__1::char_traits<char>, std::__1::allocator<char>>*, std::__1::basic_string<char, std::__1::char_traits<char>, std::__1::allocator<char>>*, std::__1::basic_string<char, std::__1::char_traits<char>, std::__1::allocator<char>>*) const in librust_lib_wlist_ui.a[arm64][2035](506d48707ec1d76d-pinyin.o)
      cppjieba::DictTrie::MakeNodeInfo(cppjieba::DictUnit&, std::__1::basic_string<char, std::__1::char_traits<char>, std::__1::allocator<char>> const&, double, std::__1::basic_string<char, std::__1::char_traits<char>, std::__1::allocator<char>> const&) in librust_lib_wlist_ui.a[arm64][2037](506d48707ec1d76d-simple_tokenizer.o)
      cppjieba::GetStringsFromWords(std::__1::vector<cppjieba::Word, std::__1::allocator<cppjieba::Word>> const&, std::__1::vector<std::__1::basic_string<char, std::__1::char_traits<char>, std::__1::allocator<char>>, std::__1::allocator<std::__1::basic_string<char, std::__1::char_traits<char>, std::__1::allocator<char>>>>&) in librust_lib_wlist_ui.a[arm64][2037](506d48707ec1d76d-simple_tokenizer.o)
  "std::__1::system_error::system_error(std::__1::error_code, std::__1::basic_string<char, std::__1::char_traits<char>, std::__1::allocator<char>> const&)", referenced from:
      cmrc::embedded_filesystem::open(std::__1::basic_string<char, std::__1::char_traits<char>, std::__1::allocator<char>> const&) const in librust_lib_wlist_ui.a[arm64][2035](506d48707ec1d76d-pinyin.o)
  "std::__1::system_error::~system_error()", referenced from:
      cmrc::embedded_filesystem::open(std::__1::basic_string<char, std::__1::char_traits<char>, std::__1::allocator<char>> const&) const in librust_lib_wlist_ui.a[arm64][2035](506d48707ec1d76d-pinyin.o)
  "std::__1::basic_istream<char, std::__1::char_traits<char>>::sentry::sentry(std::__1::basic_istream<char, std::__1::char_traits<char>>&, bool)", referenced from:
      std::__1::basic_istream<char, std::__1::char_traits<char>>& std::__1::getline[abi:ue170006]<char, std::__1::char_traits<char>, std::__1::allocator<char>>(std::__1::basic_istream<char, std::__1::char_traits<char>>&, std::__1::basic_string<char, std::__1::char_traits<char>, std::__1::allocator<char>>&, char) in librust_lib_wlist_ui.a[arm64][2035](506d48707ec1d76d-pinyin.o)
  "std::__1::basic_istream<char, std::__1::char_traits<char>>::~basic_istream()", referenced from:
      construction vtable for std::__1::basic_istream<char, std::__1::char_traits<char>>-in-std::__1::basic_stringstream<char, std::__1::char_traits<char>, std::__1::allocator<char>> in librust_lib_wlist_ui.a[arm64][2035](506d48707ec1d76d-pinyin.o)
      construction vtable for std::__1::basic_istream<char, std::__1::char_traits<char>>-in-std::__1::basic_istringstream<char, std::__1::char_traits<char>, std::__1::allocator<char>> in librust_lib_wlist_ui.a[arm64][2035](506d48707ec1d76d-pinyin.o)
      construction vtable for std::__1::basic_istream<char, std::__1::char_traits<char>>-in-std::__1::basic_ifstream<char, std::__1::char_traits<char>> in librust_lib_wlist_ui.a[arm64][2037](506d48707ec1d76d-simple_tokenizer.o)
  "std::__1::basic_istream<char, std::__1::char_traits<char>>::~basic_istream()", referenced from:
      construction vtable for std::__1::basic_istream<char, std::__1::char_traits<char>>-in-std::__1::basic_stringstream<char, std::__1::char_traits<char>, std::__1::allocator<char>> in librust_lib_wlist_ui.a[arm64][2035](506d48707ec1d76d-pinyin.o)
      construction vtable for std::__1::basic_istream<char, std::__1::char_traits<char>>-in-std::__1::basic_istringstream<char, std::__1::char_traits<char>, std::__1::allocator<char>> in librust_lib_wlist_ui.a[arm64][2035](506d48707ec1d76d-pinyin.o)
      construction vtable for std::__1::basic_istream<char, std::__1::char_traits<char>>-in-std::__1::basic_ifstream<char, std::__1::char_traits<char>> in librust_lib_wlist_ui.a[arm64][2037](506d48707ec1d76d-simple_tokenizer.o)
  "std::__1::basic_istream<char, std::__1::char_traits<char>>::~basic_istream()", referenced from:
      std::__1::basic_istringstream<char, std::__1::char_traits<char>, std::__1::allocator<char>>::basic_istringstream[abi:ue170006](std::__1::basic_string<char, std::__1::char_traits<char>, std::__1::allocator<char>> const&, unsigned int) in librust_lib_wlist_ui.a[arm64][2035](506d48707ec1d76d-pinyin.o)
      std::__1::basic_istringstream<char, std::__1::char_traits<char>, std::__1::allocator<char>>::~basic_istringstream() in librust_lib_wlist_ui.a[arm64][2035](506d48707ec1d76d-pinyin.o)
      std::__1::basic_iostream<char, std::__1::char_traits<char>>::basic_iostream[abi:ue170006](std::__1::basic_streambuf<char, std::__1::char_traits<char>>*) in librust_lib_wlist_ui.a[arm64][2035](506d48707ec1d76d-pinyin.o)
      std::__1::basic_ifstream<char, std::__1::char_traits<char>>::basic_ifstream(char const*, unsigned int) in librust_lib_wlist_ui.a[arm64][2037](506d48707ec1d76d-simple_tokenizer.o)
      std::__1::basic_ifstream<char, std::__1::char_traits<char>>::~basic_ifstream() in librust_lib_wlist_ui.a[arm64][2037](506d48707ec1d76d-simple_tokenizer.o)
  "std::__1::basic_ostream<char, std::__1::char_traits<char>>::put(char)", referenced from:
      std::__1::basic_ostream<char, std::__1::char_traits<char>>& std::__1::endl[abi:ue170006]<char, std::__1::char_traits<char>>(std::__1::basic_ostream<char, std::__1::char_traits<char>>&) in librust_lib_wlist_ui.a[arm64][2037](506d48707ec1d76d-simple_tokenizer.o)
  "std::__1::basic_ostream<char, std::__1::char_traits<char>>::flush()", referenced from:
      std::__1::basic_ostream<char, std::__1::char_traits<char>>& std::__1::endl[abi:ue170006]<char, std::__1::char_traits<char>>(std::__1::basic_ostream<char, std::__1::char_traits<char>>&) in librust_lib_wlist_ui.a[arm64][2037](506d48707ec1d76d-simple_tokenizer.o)
  "std::__1::basic_ostream<char, std::__1::char_traits<char>>::sentry::sentry(std::__1::basic_ostream<char, std::__1::char_traits<char>>&)", referenced from:
      std::__1::basic_ostream<char, std::__1::char_traits<char>>& std::__1::__put_character_sequence[abi:ue170006]<char, std::__1::char_traits<char>>(std::__1::basic_ostream<char, std::__1::char_traits<char>>&, char const*, unsigned long) in librust_lib_wlist_ui.a[arm64][2037](506d48707ec1d76d-simple_tokenizer.o)
  "std::__1::basic_ostream<char, std::__1::char_traits<char>>::sentry::~sentry()", referenced from:
      std::__1::basic_ostream<char, std::__1::char_traits<char>>& std::__1::__put_character_sequence[abi:ue170006]<char, std::__1::char_traits<char>>(std::__1::basic_ostream<char, std::__1::char_traits<char>>&, char const*, unsigned long) in librust_lib_wlist_ui.a[arm64][2037](506d48707ec1d76d-simple_tokenizer.o)
      std::__1::basic_ostream<char, std::__1::char_traits<char>>& std::__1::__put_character_sequence[abi:ue170006]<char, std::__1::char_traits<char>>(std::__1::basic_ostream<char, std::__1::char_traits<char>>&, char const*, unsigned long) in librust_lib_wlist_ui.a[arm64][2037](506d48707ec1d76d-simple_tokenizer.o)
  "std::__1::basic_ostream<char, std::__1::char_traits<char>>::~basic_ostream()", referenced from:
      construction vtable for std::__1::basic_ostream<char, std::__1::char_traits<char>>-in-std::__1::basic_stringstream<char, std::__1::char_traits<char>, std::__1::allocator<char>> in librust_lib_wlist_ui.a[arm64][2035](506d48707ec1d76d-pinyin.o)
      construction vtable for std::__1::basic_ostream<char, std::__1::char_traits<char>>-in-std::__1::basic_ostringstream<char, std::__1::char_traits<char>, std::__1::allocator<char>> in librust_lib_wlist_ui.a[arm64][2037](506d48707ec1d76d-simple_tokenizer.o)
  "std::__1::basic_ostream<char, std::__1::char_traits<char>>::~basic_ostream()", referenced from:
      construction vtable for std::__1::basic_ostream<char, std::__1::char_traits<char>>-in-std::__1::basic_stringstream<char, std::__1::char_traits<char>, std::__1::allocator<char>> in librust_lib_wlist_ui.a[arm64][2035](506d48707ec1d76d-pinyin.o)
      construction vtable for std::__1::basic_ostream<char, std::__1::char_traits<char>>-in-std::__1::basic_ostringstream<char, std::__1::char_traits<char>, std::__1::allocator<char>> in librust_lib_wlist_ui.a[arm64][2037](506d48707ec1d76d-simple_tokenizer.o)
  "std::__1::basic_ostream<char, std::__1::char_traits<char>>::~basic_ostream()", referenced from:
      std::__1::basic_ostringstream<char, std::__1::char_traits<char>, std::__1::allocator<char>>::basic_ostringstream[abi:ue170006]() in librust_lib_wlist_ui.a[arm64][2037](506d48707ec1d76d-simple_tokenizer.o)
      std::__1::basic_ostringstream<char, std::__1::char_traits<char>, std::__1::allocator<char>>::~basic_ostringstream() in librust_lib_wlist_ui.a[arm64][2037](506d48707ec1d76d-simple_tokenizer.o)
  "std::__1::basic_ostream<char, std::__1::char_traits<char>>::operator<<(int)", referenced from:
      limonp::Logger::Logger(unsigned long, char const*, int) in librust_lib_wlist_ui.a[arm64][2037](506d48707ec1d76d-simple_tokenizer.o)
  "std::__1::basic_ostream<char, std::__1::char_traits<char>>::operator<<(unsigned long)", referenced from:
      cppjieba::KeywordExtractor::LoadIdfDict(std::__1::basic_string<char, std::__1::char_traits<char>, std::__1::allocator<char>> const&) in librust_lib_wlist_ui.a[arm64][2037](506d48707ec1d76d-simple_tokenizer.o)
      cppjieba::KeywordExtractor::LoadIdfDict(std::__1::basic_string<char, std::__1::char_traits<char>, std::__1::allocator<char>> const&) in librust_lib_wlist_ui.a[arm64][2037](506d48707ec1d76d-simple_tokenizer.o)
  "std::__1::basic_iostream<char, std::__1::char_traits<char>>::~basic_iostream()", referenced from:
      construction vtable for std::__1::basic_iostream<char, std::__1::char_traits<char>>-in-std::__1::basic_stringstream<char, std::__1::char_traits<char>, std::__1::allocator<char>> in librust_lib_wlist_ui.a[arm64][2035](506d48707ec1d76d-pinyin.o)
  "std::__1::basic_iostream<char, std::__1::char_traits<char>>::~basic_iostream()", referenced from:
      construction vtable for std::__1::basic_iostream<char, std::__1::char_traits<char>>-in-std::__1::basic_stringstream<char, std::__1::char_traits<char>, std::__1::allocator<char>> in librust_lib_wlist_ui.a[arm64][2035](506d48707ec1d76d-pinyin.o)
  "std::__1::basic_iostream<char, std::__1::char_traits<char>>::~basic_iostream()", referenced from:
      std::__1::basic_stringstream<char, std::__1::char_traits<char>, std::__1::allocator<char>>::basic_stringstream[abi:ue170006](std::__1::basic_string<char, std::__1::char_traits<char>, std::__1::allocator<char>> const&, unsigned int) in librust_lib_wlist_ui.a[arm64][2035](506d48707ec1d76d-pinyin.o)
      std::__1::basic_stringstream<char, std::__1::char_traits<char>, std::__1::allocator<char>>::~basic_stringstream() in librust_lib_wlist_ui.a[arm64][2035](506d48707ec1d76d-pinyin.o)
  "std::__1::basic_streambuf<char, std::__1::char_traits<char>>::sync()", referenced from:
      vtable for std::__1::basic_stringbuf<char, std::__1::char_traits<char>, std::__1::allocator<char>> in librust_lib_wlist_ui.a[arm64][2035](506d48707ec1d76d-pinyin.o)
  "std::__1::basic_streambuf<char, std::__1::char_traits<char>>::imbue(std::__1::locale const&)", referenced from:
      vtable for std::__1::basic_stringbuf<char, std::__1::char_traits<char>, std::__1::allocator<char>> in librust_lib_wlist_ui.a[arm64][2035](506d48707ec1d76d-pinyin.o)
  "std::__1::basic_streambuf<char, std::__1::char_traits<char>>::uflow()", referenced from:
      vtable for std::__1::basic_stringbuf<char, std::__1::char_traits<char>, std::__1::allocator<char>> in librust_lib_wlist_ui.a[arm64][2035](506d48707ec1d76d-pinyin.o)
      vtable for std::__1::basic_filebuf<char, std::__1::char_traits<char>> in librust_lib_wlist_ui.a[arm64][2037](506d48707ec1d76d-simple_tokenizer.o)
  "std::__1::basic_streambuf<char, std::__1::char_traits<char>>::setbuf(char*, long)", referenced from:
      vtable for std::__1::basic_stringbuf<char, std::__1::char_traits<char>, std::__1::allocator<char>> in librust_lib_wlist_ui.a[arm64][2035](506d48707ec1d76d-pinyin.o)
  "std::__1::basic_streambuf<char, std::__1::char_traits<char>>::xsgetn(char*, long)", referenced from:
      vtable for std::__1::basic_stringbuf<char, std::__1::char_traits<char>, std::__1::allocator<char>> in librust_lib_wlist_ui.a[arm64][2035](506d48707ec1d76d-pinyin.o)
      vtable for std::__1::basic_filebuf<char, std::__1::char_traits<char>> in librust_lib_wlist_ui.a[arm64][2037](506d48707ec1d76d-simple_tokenizer.o)
  "std::__1::basic_streambuf<char, std::__1::char_traits<char>>::xsputn(char const*, long)", referenced from:
      vtable for std::__1::basic_stringbuf<char, std::__1::char_traits<char>, std::__1::allocator<char>> in librust_lib_wlist_ui.a[arm64][2035](506d48707ec1d76d-pinyin.o)
      vtable for std::__1::basic_filebuf<char, std::__1::char_traits<char>> in librust_lib_wlist_ui.a[arm64][2037](506d48707ec1d76d-simple_tokenizer.o)
  "std::__1::basic_streambuf<char, std::__1::char_traits<char>>::showmanyc()", referenced from:
      vtable for std::__1::basic_stringbuf<char, std::__1::char_traits<char>, std::__1::allocator<char>> in librust_lib_wlist_ui.a[arm64][2035](506d48707ec1d76d-pinyin.o)
      vtable for std::__1::basic_filebuf<char, std::__1::char_traits<char>> in librust_lib_wlist_ui.a[arm64][2037](506d48707ec1d76d-simple_tokenizer.o)
  "std::__1::basic_streambuf<char, std::__1::char_traits<char>>::basic_streambuf()", referenced from:
      std::__1::basic_stringbuf<char, std::__1::char_traits<char>, std::__1::allocator<char>>::basic_stringbuf[abi:ue170006](std::__1::basic_string<char, std::__1::char_traits<char>, std::__1::allocator<char>> const&, unsigned int) in librust_lib_wlist_ui.a[arm64][2035](506d48707ec1d76d-pinyin.o)
      std::__1::basic_filebuf<char, std::__1::char_traits<char>>::basic_filebuf() in librust_lib_wlist_ui.a[arm64][2037](506d48707ec1d76d-simple_tokenizer.o)
      std::__1::basic_stringbuf<char, std::__1::char_traits<char>, std::__1::allocator<char>>::basic_stringbuf[abi:ue170006](unsigned int) in librust_lib_wlist_ui.a[arm64][2037](506d48707ec1d76d-simple_tokenizer.o)
  "std::__1::basic_streambuf<char, std::__1::char_traits<char>>::~basic_streambuf()", referenced from:
      std::__1::basic_stringbuf<char, std::__1::char_traits<char>, std::__1::allocator<char>>::~basic_stringbuf() in librust_lib_wlist_ui.a[arm64][2035](506d48707ec1d76d-pinyin.o)
      std::__1::basic_stringbuf<char, std::__1::char_traits<char>, std::__1::allocator<char>>::basic_stringbuf[abi:ue170006](std::__1::basic_string<char, std::__1::char_traits<char>, std::__1::allocator<char>> const&, unsigned int) in librust_lib_wlist_ui.a[arm64][2035](506d48707ec1d76d-pinyin.o)
      std::__1::basic_filebuf<char, std::__1::char_traits<char>>::basic_filebuf() in librust_lib_wlist_ui.a[arm64][2037](506d48707ec1d76d-simple_tokenizer.o)
      std::__1::basic_filebuf<char, std::__1::char_traits<char>>::~basic_filebuf() in librust_lib_wlist_ui.a[arm64][2037](506d48707ec1d76d-simple_tokenizer.o)
  "std::__1::generic_category()", referenced from:
      std::__1::make_error_code[abi:ue170006](std::__1::errc) in librust_lib_wlist_ui.a[arm64][2035](506d48707ec1d76d-pinyin.o)
  "std::__1::cerr", referenced from:
      limonp::Logger::~Logger() in librust_lib_wlist_ui.a[arm64][2037](506d48707ec1d76d-simple_tokenizer.o)
  "std::__1::ctype<char>::id", referenced from:
      std::__1::ctype<char> const& std::__1::use_facet[abi:ue170006]<std::__1::ctype<char>>(std::__1::locale const&) in librust_lib_wlist_ui.a[arm64][2035](506d48707ec1d76d-pinyin.o)
  "std::__1::stoul(std::__1::basic_string<char, std::__1::char_traits<char>, std::__1::allocator<char>> const&, unsigned long*, int)", referenced from:
      simple_tokenizer::PinYin::build_pinyin_map() in librust_lib_wlist_ui.a[arm64][2035](506d48707ec1d76d-pinyin.o)
  "std::__1::locale::locale(std::__1::locale const&)", referenced from:
      std::__1::basic_streambuf<char, std::__1::char_traits<char>>::getloc[abi:ue170006]() const in librust_lib_wlist_ui.a[arm64][2037](506d48707ec1d76d-simple_tokenizer.o)
  "std::__1::locale::~locale()", referenced from:
      std::__1::basic_ios<char, std::__1::char_traits<char>>::widen[abi:ue170006](char) const in librust_lib_wlist_ui.a[arm64][2035](506d48707ec1d76d-pinyin.o)
      std::__1::basic_ios<char, std::__1::char_traits<char>>::widen[abi:ue170006](char) const in librust_lib_wlist_ui.a[arm64][2035](506d48707ec1d76d-pinyin.o)
      std::__1::basic_filebuf<char, std::__1::char_traits<char>>::basic_filebuf() in librust_lib_wlist_ui.a[arm64][2037](506d48707ec1d76d-simple_tokenizer.o)
      std::__1::basic_filebuf<char, std::__1::char_traits<char>>::basic_filebuf() in librust_lib_wlist_ui.a[arm64][2037](506d48707ec1d76d-simple_tokenizer.o)
      std::__1::basic_filebuf<char, std::__1::char_traits<char>>::basic_filebuf() in librust_lib_wlist_ui.a[arm64][2037](506d48707ec1d76d-simple_tokenizer.o)
  "std::__1::codecvt<char, char, __mbstate_t>::id", referenced from:
      bool std::__1::has_facet[abi:ue170006]<std::__1::codecvt<char, char, __mbstate_t>>(std::__1::locale const&) in librust_lib_wlist_ui.a[arm64][2037](506d48707ec1d76d-simple_tokenizer.o)
      std::__1::codecvt<char, char, __mbstate_t> const& std::__1::use_facet[abi:ue170006]<std::__1::codecvt<char, char, __mbstate_t>>(std::__1::locale const&) in librust_lib_wlist_ui.a[arm64][2037](506d48707ec1d76d-simple_tokenizer.o)
  "std::__1::ios_base::__set_badbit_and_consider_rethrow()", referenced from:
      std::__1::basic_ostream<char, std::__1::char_traits<char>>& std::__1::__put_character_sequence[abi:ue170006]<char, std::__1::char_traits<char>>(std::__1::basic_ostream<char, std::__1::char_traits<char>>&, char const*, unsigned long) in librust_lib_wlist_ui.a[arm64][2037](506d48707ec1d76d-simple_tokenizer.o)
  "std::__1::ios_base::init(void*)", referenced from:
      std::__1::basic_ios<char, std::__1::char_traits<char>>::init[abi:ue170006](std::__1::basic_streambuf<char, std::__1::char_traits<char>>*) in librust_lib_wlist_ui.a[arm64][2035](506d48707ec1d76d-pinyin.o)
  "std::__1::ios_base::clear(unsigned int)", referenced from:
      std::__1::ios_base::setstate[abi:ue170006](unsigned int) in librust_lib_wlist_ui.a[arm64][2035](506d48707ec1d76d-pinyin.o)
  "std::__1::basic_ios<char, std::__1::char_traits<char>>::~basic_ios()", referenced from:
      std::__1::basic_istringstream<char, std::__1::char_traits<char>, std::__1::allocator<char>>::basic_istringstream[abi:ue170006](std::__1::basic_string<char, std::__1::char_traits<char>, std::__1::allocator<char>> const&, unsigned int) in librust_lib_wlist_ui.a[arm64][2035](506d48707ec1d76d-pinyin.o)
      std::__1::basic_stringstream<char, std::__1::char_traits<char>, std::__1::allocator<char>>::basic_stringstream[abi:ue170006](std::__1::basic_string<char, std::__1::char_traits<char>, std::__1::allocator<char>> const&, unsigned int) in librust_lib_wlist_ui.a[arm64][2035](506d48707ec1d76d-pinyin.o)
      std::__1::basic_stringstream<char, std::__1::char_traits<char>, std::__1::allocator<char>>::~basic_stringstream() in librust_lib_wlist_ui.a[arm64][2035](506d48707ec1d76d-pinyin.o)
      std::__1::basic_istringstream<char, std::__1::char_traits<char>, std::__1::allocator<char>>::~basic_istringstream() in librust_lib_wlist_ui.a[arm64][2035](506d48707ec1d76d-pinyin.o)
      std::__1::basic_ifstream<char, std::__1::char_traits<char>>::basic_ifstream(char const*, unsigned int) in librust_lib_wlist_ui.a[arm64][2037](506d48707ec1d76d-simple_tokenizer.o)
      std::__1::basic_ifstream<char, std::__1::char_traits<char>>::~basic_ifstream() in librust_lib_wlist_ui.a[arm64][2037](506d48707ec1d76d-simple_tokenizer.o)
      std::__1::basic_ostringstream<char, std::__1::char_traits<char>, std::__1::allocator<char>>::basic_ostringstream[abi:ue170006]() in librust_lib_wlist_ui.a[arm64][2037](506d48707ec1d76d-simple_tokenizer.o)
      ...
  "std::bad_cast::bad_cast()", referenced from:
      std::__1::__throw_bad_cast[abi:ue170006]() in librust_lib_wlist_ui.a[arm64][2037](506d48707ec1d76d-simple_tokenizer.o)
  "std::bad_cast::~bad_cast()", referenced from:
      std::__1::__throw_bad_cast[abi:ue170006]() in librust_lib_wlist_ui.a[arm64][2037](506d48707ec1d76d-simple_tokenizer.o)
  "std::terminate()", referenced from:
      ___clang_call_terminate in librust_lib_wlist_ui.a[arm64][2034](506d48707ec1d76d-entry.o)
  "typeinfo for std::__1::system_error", referenced from:
      cmrc::embedded_filesystem::open(std::__1::basic_string<char, std::__1::char_traits<char>, std::__1::allocator<char>> const&) const in librust_lib_wlist_ui.a[arm64][2035](506d48707ec1d76d-pinyin.o)
  "typeinfo for std::__1::basic_istream<char, std::__1::char_traits<char>>", referenced from:
      construction vtable for std::__1::basic_istream<char, std::__1::char_traits<char>>-in-std::__1::basic_stringstream<char, std::__1::char_traits<char>, std::__1::allocator<char>> in librust_lib_wlist_ui.a[arm64][2035](506d48707ec1d76d-pinyin.o)
      construction vtable for std::__1::basic_istream<char, std::__1::char_traits<char>>-in-std::__1::basic_stringstream<char, std::__1::char_traits<char>, std::__1::allocator<char>> in librust_lib_wlist_ui.a[arm64][2035](506d48707ec1d76d-pinyin.o)
      construction vtable for std::__1::basic_istream<char, std::__1::char_traits<char>>-in-std::__1::basic_istringstream<char, std::__1::char_traits<char>, std::__1::allocator<char>> in librust_lib_wlist_ui.a[arm64][2035](506d48707ec1d76d-pinyin.o)
      construction vtable for std::__1::basic_istream<char, std::__1::char_traits<char>>-in-std::__1::basic_istringstream<char, std::__1::char_traits<char>, std::__1::allocator<char>> in librust_lib_wlist_ui.a[arm64][2035](506d48707ec1d76d-pinyin.o)
      typeinfo for std::__1::basic_istringstream<char, std::__1::char_traits<char>, std::__1::allocator<char>> in librust_lib_wlist_ui.a[arm64][2035](506d48707ec1d76d-pinyin.o)
      construction vtable for std::__1::basic_istream<char, std::__1::char_traits<char>>-in-std::__1::basic_ifstream<char, std::__1::char_traits<char>> in librust_lib_wlist_ui.a[arm64][2037](506d48707ec1d76d-simple_tokenizer.o)
      construction vtable for std::__1::basic_istream<char, std::__1::char_traits<char>>-in-std::__1::basic_ifstream<char, std::__1::char_traits<char>> in librust_lib_wlist_ui.a[arm64][2037](506d48707ec1d76d-simple_tokenizer.o)
      ...
  "typeinfo for std::__1::basic_ostream<char, std::__1::char_traits<char>>", referenced from:
      construction vtable for std::__1::basic_ostream<char, std::__1::char_traits<char>>-in-std::__1::basic_stringstream<char, std::__1::char_traits<char>, std::__1::allocator<char>> in librust_lib_wlist_ui.a[arm64][2035](506d48707ec1d76d-pinyin.o)
      construction vtable for std::__1::basic_ostream<char, std::__1::char_traits<char>>-in-std::__1::basic_stringstream<char, std::__1::char_traits<char>, std::__1::allocator<char>> in librust_lib_wlist_ui.a[arm64][2035](506d48707ec1d76d-pinyin.o)
      construction vtable for std::__1::basic_ostream<char, std::__1::char_traits<char>>-in-std::__1::basic_ostringstream<char, std::__1::char_traits<char>, std::__1::allocator<char>> in librust_lib_wlist_ui.a[arm64][2037](506d48707ec1d76d-simple_tokenizer.o)
      construction vtable for std::__1::basic_ostream<char, std::__1::char_traits<char>>-in-std::__1::basic_ostringstream<char, std::__1::char_traits<char>, std::__1::allocator<char>> in librust_lib_wlist_ui.a[arm64][2037](506d48707ec1d76d-simple_tokenizer.o)
      typeinfo for std::__1::basic_ostringstream<char, std::__1::char_traits<char>, std::__1::allocator<char>> in librust_lib_wlist_ui.a[arm64][2037](506d48707ec1d76d-simple_tokenizer.o)
  "typeinfo for std::__1::basic_iostream<char, std::__1::char_traits<char>>", referenced from:
      construction vtable for std::__1::basic_iostream<char, std::__1::char_traits<char>>-in-std::__1::basic_stringstream<char, std::__1::char_traits<char>, std::__1::allocator<char>> in librust_lib_wlist_ui.a[arm64][2035](506d48707ec1d76d-pinyin.o)
      construction vtable for std::__1::basic_iostream<char, std::__1::char_traits<char>>-in-std::__1::basic_stringstream<char, std::__1::char_traits<char>, std::__1::allocator<char>> in librust_lib_wlist_ui.a[arm64][2035](506d48707ec1d76d-pinyin.o)
      construction vtable for std::__1::basic_iostream<char, std::__1::char_traits<char>>-in-std::__1::basic_stringstream<char, std::__1::char_traits<char>, std::__1::allocator<char>> in librust_lib_wlist_ui.a[arm64][2035](506d48707ec1d76d-pinyin.o)
      typeinfo for std::__1::basic_stringstream<char, std::__1::char_traits<char>, std::__1::allocator<char>> in librust_lib_wlist_ui.a[arm64][2035](506d48707ec1d76d-pinyin.o)
  "typeinfo for std::__1::basic_streambuf<char, std::__1::char_traits<char>>", referenced from:
      typeinfo for std::__1::basic_stringbuf<char, std::__1::char_traits<char>, std::__1::allocator<char>> in librust_lib_wlist_ui.a[arm64][2035](506d48707ec1d76d-pinyin.o)
      typeinfo for std::__1::basic_filebuf<char, std::__1::char_traits<char>> in librust_lib_wlist_ui.a[arm64][2037](506d48707ec1d76d-simple_tokenizer.o)
  "typeinfo for std::length_error", referenced from:
      std::__1::__throw_length_error[abi:ue170006](char const*) in librust_lib_wlist_ui.a[arm64][2035](506d48707ec1d76d-pinyin.o)
  "typeinfo for std::runtime_error", referenced from:
      simple_tokenizer::PinYin::codepoint(std::__1::basic_string<char, std::__1::char_traits<char>, std::__1::allocator<char>> const&) in librust_lib_wlist_ui.a[arm64][2035](506d48707ec1d76d-pinyin.o)
  "typeinfo for std::bad_array_new_length", referenced from:
      std::__throw_bad_array_new_length[abi:ue170006]() in librust_lib_wlist_ui.a[arm64][2035](506d48707ec1d76d-pinyin.o)
  "typeinfo for std::bad_cast", referenced from:
      std::__1::__throw_bad_cast[abi:ue170006]() in librust_lib_wlist_ui.a[arm64][2037](506d48707ec1d76d-simple_tokenizer.o)
  "vtable for __cxxabiv1::__class_type_info", referenced from:
      typeinfo for cppjieba::SegmentBase in librust_lib_wlist_ui.a[arm64][2037](506d48707ec1d76d-simple_tokenizer.o)
   NOTE: a missing vtable usually means the first non-inline virtual member function has no definition.
  "vtable for __cxxabiv1::__si_class_type_info", referenced from:
      typeinfo for std::__1::basic_stringstream<char, std::__1::char_traits<char>, std::__1::allocator<char>> in librust_lib_wlist_ui.a[arm64][2035](506d48707ec1d76d-pinyin.o)
      typeinfo for std::__1::basic_stringbuf<char, std::__1::char_traits<char>, std::__1::allocator<char>> in librust_lib_wlist_ui.a[arm64][2035](506d48707ec1d76d-pinyin.o)
      typeinfo for std::__1::basic_istringstream<char, std::__1::char_traits<char>, std::__1::allocator<char>> in librust_lib_wlist_ui.a[arm64][2035](506d48707ec1d76d-pinyin.o)
      typeinfo for std::__1::basic_ifstream<char, std::__1::char_traits<char>> in librust_lib_wlist_ui.a[arm64][2037](506d48707ec1d76d-simple_tokenizer.o)
      typeinfo for std::__1::basic_filebuf<char, std::__1::char_traits<char>> in librust_lib_wlist_ui.a[arm64][2037](506d48707ec1d76d-simple_tokenizer.o)
      typeinfo for std::__1::basic_ostringstream<char, std::__1::char_traits<char>, std::__1::allocator<char>> in librust_lib_wlist_ui.a[arm64][2037](506d48707ec1d76d-simple_tokenizer.o)
      typeinfo for cppjieba::SegmentTagged in librust_lib_wlist_ui.a[arm64][2037](506d48707ec1d76d-simple_tokenizer.o)
      ...
   NOTE: a missing vtable usually means the first non-inline virtual member function has no definition.
  "vtable for std::__1::ios_base", referenced from:
      std::__1::ios_base::ios_base[abi:ue170006]() in librust_lib_wlist_ui.a[arm64][2035](506d48707ec1d76d-pinyin.o)
   NOTE: a missing vtable usually means the first non-inline virtual member function has no definition.
  "vtable for std::__1::basic_ios<char, std::__1::char_traits<char>>", referenced from:
      std::__1::basic_ios<char, std::__1::char_traits<char>>::basic_ios[abi:ue170006]() in librust_lib_wlist_ui.a[arm64][2035](506d48707ec1d76d-pinyin.o)
   NOTE: a missing vtable usually means the first non-inline virtual member function has no definition.
  "vtable for std::length_error", referenced from:
      std::length_error::length_error[abi:ue170006](char const*) in librust_lib_wlist_ui.a[arm64][2035](506d48707ec1d76d-pinyin.o)
   NOTE: a missing vtable usually means the first non-inline virtual member function has no definition.
  "non-virtual thunk to std::__1::basic_iostream<char, std::__1::char_traits<char>>::~basic_iostream()", referenced from:
      construction vtable for std::__1::basic_iostream<char, std::__1::char_traits<char>>-in-std::__1::basic_stringstream<char, std::__1::char_traits<char>, std::__1::allocator<char>> in librust_lib_wlist_ui.a[arm64][2035](506d48707ec1d76d-pinyin.o)
  "non-virtual thunk to std::__1::basic_iostream<char, std::__1::char_traits<char>>::~basic_iostream()", referenced from:
      construction vtable for std::__1::basic_iostream<char, std::__1::char_traits<char>>-in-std::__1::basic_stringstream<char, std::__1::char_traits<char>, std::__1::allocator<char>> in librust_lib_wlist_ui.a[arm64][2035](506d48707ec1d76d-pinyin.o)
  "virtual thunk to std::__1::basic_istream<char, std::__1::char_traits<char>>::~basic_istream()", referenced from:
      construction vtable for std::__1::basic_istream<char, std::__1::char_traits<char>>-in-std::__1::basic_stringstream<char, std::__1::char_traits<char>, std::__1::allocator<char>> in librust_lib_wlist_ui.a[arm64][2035](506d48707ec1d76d-pinyin.o)
      construction vtable for std::__1::basic_istream<char, std::__1::char_traits<char>>-in-std::__1::basic_istringstream<char, std::__1::char_traits<char>, std::__1::allocator<char>> in librust_lib_wlist_ui.a[arm64][2035](506d48707ec1d76d-pinyin.o)
      construction vtable for std::__1::basic_istream<char, std::__1::char_traits<char>>-in-std::__1::basic_ifstream<char, std::__1::char_traits<char>> in librust_lib_wlist_ui.a[arm64][2037](506d48707ec1d76d-simple_tokenizer.o)
  "virtual thunk to std::__1::basic_istream<char, std::__1::char_traits<char>>::~basic_istream()", referenced from:
      construction vtable for std::__1::basic_istream<char, std::__1::char_traits<char>>-in-std::__1::basic_stringstream<char, std::__1::char_traits<char>, std::__1::allocator<char>> in librust_lib_wlist_ui.a[arm64][2035](506d48707ec1d76d-pinyin.o)
      construction vtable for std::__1::basic_istream<char, std::__1::char_traits<char>>-in-std::__1::basic_istringstream<char, std::__1::char_traits<char>, std::__1::allocator<char>> in librust_lib_wlist_ui.a[arm64][2035](506d48707ec1d76d-pinyin.o)
      construction vtable for std::__1::basic_istream<char, std::__1::char_traits<char>>-in-std::__1::basic_ifstream<char, std::__1::char_traits<char>> in librust_lib_wlist_ui.a[arm64][2037](506d48707ec1d76d-simple_tokenizer.o)
  "virtual thunk to std::__1::basic_ostream<char, std::__1::char_traits<char>>::~basic_ostream()", referenced from:
      construction vtable for std::__1::basic_ostream<char, std::__1::char_traits<char>>-in-std::__1::basic_stringstream<char, std::__1::char_traits<char>, std::__1::allocator<char>> in librust_lib_wlist_ui.a[arm64][2035](506d48707ec1d76d-pinyin.o)
      construction vtable for std::__1::basic_ostream<char, std::__1::char_traits<char>>-in-std::__1::basic_ostringstream<char, std::__1::char_traits<char>, std::__1::allocator<char>> in librust_lib_wlist_ui.a[arm64][2037](506d48707ec1d76d-simple_tokenizer.o)
  "virtual thunk to std::__1::basic_ostream<char, std::__1::char_traits<char>>::~basic_ostream()", referenced from:
      construction vtable for std::__1::basic_ostream<char, std::__1::char_traits<char>>-in-std::__1::basic_stringstream<char, std::__1::char_traits<char>, std::__1::allocator<char>> in librust_lib_wlist_ui.a[arm64][2035](506d48707ec1d76d-pinyin.o)
      construction vtable for std::__1::basic_ostream<char, std::__1::char_traits<char>>-in-std::__1::basic_ostringstream<char, std::__1::char_traits<char>, std::__1::allocator<char>> in librust_lib_wlist_ui.a[arm64][2037](506d48707ec1d76d-simple_tokenizer.o)
  "virtual thunk to std::__1::basic_iostream<char, std::__1::char_traits<char>>::~basic_iostream()", referenced from:
      construction vtable for std::__1::basic_iostream<char, std::__1::char_traits<char>>-in-std::__1::basic_stringstream<char, std::__1::char_traits<char>, std::__1::allocator<char>> in librust_lib_wlist_ui.a[arm64][2035](506d48707ec1d76d-pinyin.o)
  "virtual thunk to std::__1::basic_iostream<char, std::__1::char_traits<char>>::~basic_iostream()", referenced from:
      construction vtable for std::__1::basic_iostream<char, std::__1::char_traits<char>>-in-std::__1::basic_stringstream<char, std::__1::char_traits<char>, std::__1::allocator<char>> in librust_lib_wlist_ui.a[arm64][2035](506d48707ec1d76d-pinyin.o)
  "operator delete[](void*)", referenced from:
      std::__1::basic_filebuf<char, std::__1::char_traits<char>>::imbue(std::__1::locale const&) in librust_lib_wlist_ui.a[arm64][2037](506d48707ec1d76d-simple_tokenizer.o)
      std::__1::basic_filebuf<char, std::__1::char_traits<char>>::setbuf(char*, long) in librust_lib_wlist_ui.a[arm64][2037](506d48707ec1d76d-simple_tokenizer.o)
      std::__1::basic_filebuf<char, std::__1::char_traits<char>>::setbuf(char*, long) in librust_lib_wlist_ui.a[arm64][2037](506d48707ec1d76d-simple_tokenizer.o)
      std::__1::basic_filebuf<char, std::__1::char_traits<char>>::~basic_filebuf() in librust_lib_wlist_ui.a[arm64][2037](506d48707ec1d76d-simple_tokenizer.o)
      std::__1::basic_filebuf<char, std::__1::char_traits<char>>::~basic_filebuf() in librust_lib_wlist_ui.a[arm64][2037](506d48707ec1d76d-simple_tokenizer.o)
  "operator delete(void*)", referenced from:
      _fts5_simple_xCreate in librust_lib_wlist_ui.a[arm64][2034](506d48707ec1d76d-entry.o)
      _fts5_simple_xDelete in librust_lib_wlist_ui.a[arm64][2034](506d48707ec1d76d-entry.o)
      std::__1::basic_stringstream<char, std::__1::char_traits<char>, std::__1::allocator<char>>::~basic_stringstream() in librust_lib_wlist_ui.a[arm64][2035](506d48707ec1d76d-pinyin.o)
      std::__1::basic_stringbuf<char, std::__1::char_traits<char>, std::__1::allocator<char>>::~basic_stringbuf() in librust_lib_wlist_ui.a[arm64][2035](506d48707ec1d76d-pinyin.o)
      std::__1::basic_istringstream<char, std::__1::char_traits<char>, std::__1::allocator<char>>::~basic_istringstream() in librust_lib_wlist_ui.a[arm64][2035](506d48707ec1d76d-pinyin.o)
      void std::__1::__libcpp_operator_delete[abi:ue170006]<void*>(void*) in librust_lib_wlist_ui.a[arm64][2035](506d48707ec1d76d-pinyin.o)
      simple_tokenizer::SimpleTokenizer::get_pinyin() in librust_lib_wlist_ui.a[arm64][2037](506d48707ec1d76d-simple_tokenizer.o)
      ...
  "operator new[](unsigned long)", referenced from:
      std::__1::basic_filebuf<char, std::__1::char_traits<char>>::imbue(std::__1::locale const&) in librust_lib_wlist_ui.a[arm64][2037](506d48707ec1d76d-simple_tokenizer.o)
      std::__1::basic_filebuf<char, std::__1::char_traits<char>>::imbue(std::__1::locale const&) in librust_lib_wlist_ui.a[arm64][2037](506d48707ec1d76d-simple_tokenizer.o)
      std::__1::basic_filebuf<char, std::__1::char_traits<char>>::setbuf(char*, long) in librust_lib_wlist_ui.a[arm64][2037](506d48707ec1d76d-simple_tokenizer.o)
      std::__1::basic_filebuf<char, std::__1::char_traits<char>>::setbuf(char*, long) in librust_lib_wlist_ui.a[arm64][2037](506d48707ec1d76d-simple_tokenizer.o)
  "operator new(unsigned long)", referenced from:
      _fts5_simple_xCreate in librust_lib_wlist_ui.a[arm64][2034](506d48707ec1d76d-entry.o)
      void* std::__1::__libcpp_operator_new[abi:ue170006]<unsigned long>(unsigned long) in librust_lib_wlist_ui.a[arm64][2035](506d48707ec1d76d-pinyin.o)
      simple_tokenizer::SimpleTokenizer::get_pinyin() in librust_lib_wlist_ui.a[arm64][2037](506d48707ec1d76d-simple_tokenizer.o)
      cppjieba::DictTrie::CreateTrie(std::__1::vector<cppjieba::DictUnit, std::__1::allocator<cppjieba::DictUnit>> const&) in librust_lib_wlist_ui.a[arm64][2037](506d48707ec1d76d-simple_tokenizer.o)
      cppjieba::Trie::Trie(std::__1::vector<limonp::LocalVector<unsigned int>, std::__1::allocator<limonp::LocalVector<unsigned int>>> const&, std::__1::vector<cppjieba::DictUnit const*, std::__1::allocator<cppjieba::DictUnit const*>> const&) in librust_lib_wlist_ui.a[arm64][2037](506d48707ec1d76d-simple_tokenizer.o)
      cppjieba::Trie::InsertNode(limonp::LocalVector<unsigned int> const&, cppjieba::DictUnit const*) in librust_lib_wlist_ui.a[arm64][2037](506d48707ec1d76d-simple_tokenizer.o)
      cppjieba::Trie::InsertNode(limonp::LocalVector<unsigned int> const&, cppjieba::DictUnit const*) in librust_lib_wlist_ui.a[arm64][2037](506d48707ec1d76d-simple_tokenizer.o)
      ...
  "___cxa_allocate_exception", referenced from:
      cmrc::embedded_filesystem::open(std::__1::basic_string<char, std::__1::char_traits<char>, std::__1::allocator<char>> const&) const in librust_lib_wlist_ui.a[arm64][2035](506d48707ec1d76d-pinyin.o)
      simple_tokenizer::PinYin::codepoint(std::__1::basic_string<char, std::__1::char_traits<char>, std::__1::allocator<char>> const&) in librust_lib_wlist_ui.a[arm64][2035](506d48707ec1d76d-pinyin.o)
      std::__throw_bad_array_new_length[abi:ue170006]() in librust_lib_wlist_ui.a[arm64][2035](506d48707ec1d76d-pinyin.o)
      std::__1::__throw_length_error[abi:ue170006](char const*) in librust_lib_wlist_ui.a[arm64][2035](506d48707ec1d76d-pinyin.o)
      std::__1::__throw_bad_cast[abi:ue170006]() in librust_lib_wlist_ui.a[arm64][2037](506d48707ec1d76d-simple_tokenizer.o)
  "___cxa_begin_catch", referenced from:
      ___clang_call_terminate in librust_lib_wlist_ui.a[arm64][2034](506d48707ec1d76d-entry.o)
      std::__1::basic_istream<char, std::__1::char_traits<char>>& std::__1::getline[abi:ue170006]<char, std::__1::char_traits<char>, std::__1::allocator<char>>(std::__1::basic_istream<char, std::__1::char_traits<char>>&, std::__1::basic_string<char, std::__1::char_traits<char>, std::__1::allocator<char>>&, char) in librust_lib_wlist_ui.a[arm64][2035](506d48707ec1d76d-pinyin.o)
      std::__1::basic_stringbuf<char, std::__1::char_traits<char>, std::__1::allocator<char>>::overflow(int) in librust_lib_wlist_ui.a[arm64][2035](506d48707ec1d76d-pinyin.o)
      std::__1::basic_filebuf<char, std::__1::char_traits<char>>::~basic_filebuf() in librust_lib_wlist_ui.a[arm64][2037](506d48707ec1d76d-simple_tokenizer.o)
      std::__1::basic_ostream<char, std::__1::char_traits<char>>& std::__1::__put_character_sequence[abi:ue170006]<char, std::__1::char_traits<char>>(std::__1::basic_ostream<char, std::__1::char_traits<char>>&, char const*, unsigned long) in librust_lib_wlist_ui.a[arm64][2037](506d48707ec1d76d-simple_tokenizer.o)
  "___cxa_end_catch", referenced from:
      std::__1::basic_istream<char, std::__1::char_traits<char>>& std::__1::getline[abi:ue170006]<char, std::__1::char_traits<char>, std::__1::allocator<char>>(std::__1::basic_istream<char, std::__1::char_traits<char>>&, std::__1::basic_string<char, std::__1::char_traits<char>, std::__1::allocator<char>>&, char) in librust_lib_wlist_ui.a[arm64][2035](506d48707ec1d76d-pinyin.o)
      std::__1::basic_istream<char, std::__1::char_traits<char>>& std::__1::getline[abi:ue170006]<char, std::__1::char_traits<char>, std::__1::allocator<char>>(std::__1::basic_istream<char, std::__1::char_traits<char>>&, std::__1::basic_string<char, std::__1::char_traits<char>, std::__1::allocator<char>>&, char) in librust_lib_wlist_ui.a[arm64][2035](506d48707ec1d76d-pinyin.o)
      std::__1::basic_stringbuf<char, std::__1::char_traits<char>, std::__1::allocator<char>>::overflow(int) in librust_lib_wlist_ui.a[arm64][2035](506d48707ec1d76d-pinyin.o)
      std::__1::basic_filebuf<char, std::__1::char_traits<char>>::~basic_filebuf() in librust_lib_wlist_ui.a[arm64][2037](506d48707ec1d76d-simple_tokenizer.o)
      std::__1::basic_ostream<char, std::__1::char_traits<char>>& std::__1::__put_character_sequence[abi:ue170006]<char, std::__1::char_traits<char>>(std::__1::basic_ostream<char, std::__1::char_traits<char>>&, char const*, unsigned long) in librust_lib_wlist_ui.a[arm64][2037](506d48707ec1d76d-simple_tokenizer.o)
      std::__1::basic_ostream<char, std::__1::char_traits<char>>& std::__1::__put_character_sequence[abi:ue170006]<char, std::__1::char_traits<char>>(std::__1::basic_ostream<char, std::__1::char_traits<char>>&, char const*, unsigned long) in librust_lib_wlist_ui.a[arm64][2037](506d48707ec1d76d-simple_tokenizer.o)
  "___cxa_free_exception", referenced from:
      cmrc::embedded_filesystem::open(std::__1::basic_string<char, std::__1::char_traits<char>, std::__1::allocator<char>> const&) const in librust_lib_wlist_ui.a[arm64][2035](506d48707ec1d76d-pinyin.o)
      simple_tokenizer::PinYin::codepoint(std::__1::basic_string<char, std::__1::char_traits<char>, std::__1::allocator<char>> const&) in librust_lib_wlist_ui.a[arm64][2035](506d48707ec1d76d-pinyin.o)
      std::__1::__throw_length_error[abi:ue170006](char const*) in librust_lib_wlist_ui.a[arm64][2035](506d48707ec1d76d-pinyin.o)
  "___cxa_guard_abort", referenced from:
      simple_tokenizer::SimpleTokenizer::get_pinyin() in librust_lib_wlist_ui.a[arm64][2037](506d48707ec1d76d-simple_tokenizer.o)
      simple_tokenizer::SimpleTokenizer::tokenize_jieba_query(char const*, int, int) in librust_lib_wlist_ui.a[arm64][2037](506d48707ec1d76d-simple_tokenizer.o)
      cmrc::pinyin_text::get_filesystem() in librust_lib_wlist_ui.a[arm64][2038](17e99f3a56cf8303-lib.o)
      cmrc::pinyin_text::(anonymous namespace)::get_root_index() in librust_lib_wlist_ui.a[arm64][2038](17e99f3a56cf8303-lib.o)
      cmrc::pinyin_text::(anonymous namespace)::get_root_index() in librust_lib_wlist_ui.a[arm64][2038](17e99f3a56cf8303-lib.o)
  "___cxa_guard_acquire", referenced from:
      simple_tokenizer::SimpleTokenizer::get_pinyin() in librust_lib_wlist_ui.a[arm64][2037](506d48707ec1d76d-simple_tokenizer.o)
      simple_tokenizer::SimpleTokenizer::tokenize_jieba_query(char const*, int, int) in librust_lib_wlist_ui.a[arm64][2037](506d48707ec1d76d-simple_tokenizer.o)
      cmrc::pinyin_text::get_filesystem() in librust_lib_wlist_ui.a[arm64][2038](17e99f3a56cf8303-lib.o)
      cmrc::pinyin_text::(anonymous namespace)::get_root_index() in librust_lib_wlist_ui.a[arm64][2038](17e99f3a56cf8303-lib.o)
      cmrc::pinyin_text::(anonymous namespace)::get_root_index() in librust_lib_wlist_ui.a[arm64][2038](17e99f3a56cf8303-lib.o)
      cmrc::pinyin_text::(anonymous namespace)::get_root_index() in librust_lib_wlist_ui.a[arm64][2038](17e99f3a56cf8303-lib.o)
      cmrc::pinyin_text::(anonymous namespace)::get_root_index() in librust_lib_wlist_ui.a[arm64][2038](17e99f3a56cf8303-lib.o)
      ...
  "___cxa_guard_release", referenced from:
      simple_tokenizer::SimpleTokenizer::get_pinyin() in librust_lib_wlist_ui.a[arm64][2037](506d48707ec1d76d-simple_tokenizer.o)
      simple_tokenizer::SimpleTokenizer::tokenize_jieba_query(char const*, int, int) in librust_lib_wlist_ui.a[arm64][2037](506d48707ec1d76d-simple_tokenizer.o)
      cmrc::pinyin_text::get_filesystem() in librust_lib_wlist_ui.a[arm64][2038](17e99f3a56cf8303-lib.o)
      cmrc::pinyin_text::(anonymous namespace)::get_root_index() in librust_lib_wlist_ui.a[arm64][2038](17e99f3a56cf8303-lib.o)
      cmrc::pinyin_text::(anonymous namespace)::get_root_index() in librust_lib_wlist_ui.a[arm64][2038](17e99f3a56cf8303-lib.o)
      cmrc::pinyin_text::(anonymous namespace)::get_root_index() in librust_lib_wlist_ui.a[arm64][2038](17e99f3a56cf8303-lib.o)
      cmrc::pinyin_text::(anonymous namespace)::get_root_index() in librust_lib_wlist_ui.a[arm64][2038](17e99f3a56cf8303-lib.o)
      ...
  "___cxa_pure_virtual", referenced from:
      vtable for cppjieba::SegmentTagged in librust_lib_wlist_ui.a[arm64][2037](506d48707ec1d76d-simple_tokenizer.o)
      vtable for cppjieba::SegmentTagged in librust_lib_wlist_ui.a[arm64][2037](506d48707ec1d76d-simple_tokenizer.o)
      vtable for cppjieba::SegmentTagged in librust_lib_wlist_ui.a[arm64][2037](506d48707ec1d76d-simple_tokenizer.o)
      vtable for cppjieba::SegmentBase in librust_lib_wlist_ui.a[arm64][2037](506d48707ec1d76d-simple_tokenizer.o)
  "___cxa_rethrow", referenced from:
      std::__1::basic_istream<char, std::__1::char_traits<char>>& std::__1::getline[abi:ue170006]<char, std::__1::char_traits<char>, std::__1::allocator<char>>(std::__1::basic_istream<char, std::__1::char_traits<char>>&, std::__1::basic_string<char, std::__1::char_traits<char>, std::__1::allocator<char>>&, char) in librust_lib_wlist_ui.a[arm64][2035](506d48707ec1d76d-pinyin.o)
  "___cxa_throw", referenced from:
      cmrc::embedded_filesystem::open(std::__1::basic_string<char, std::__1::char_traits<char>, std::__1::allocator<char>> const&) const in librust_lib_wlist_ui.a[arm64][2035](506d48707ec1d76d-pinyin.o)
      simple_tokenizer::PinYin::codepoint(std::__1::basic_string<char, std::__1::char_traits<char>, std::__1::allocator<char>> const&) in librust_lib_wlist_ui.a[arm64][2035](506d48707ec1d76d-pinyin.o)
      std::__throw_bad_array_new_length[abi:ue170006]() in librust_lib_wlist_ui.a[arm64][2035](506d48707ec1d76d-pinyin.o)
      std::__1::__throw_length_error[abi:ue170006](char const*) in librust_lib_wlist_ui.a[arm64][2035](506d48707ec1d76d-pinyin.o)
      std::__1::__throw_bad_cast[abi:ue170006]() in librust_lib_wlist_ui.a[arm64][2037](506d48707ec1d76d-simple_tokenizer.o)
  "___gxx_personality_v0", referenced from:
      /Users/xuxiaocheng/Rust/wlist_ui/build/macos/Build/Products/Debug/rust_lib_wlist_ui/librust_lib_wlist_ui.a[arm64][2034](506d48707ec1d76d-entry.o)
ld: symbol(s) not found for architecture arm64
clang: error: linker command failed with exit code 1 (use -v to see invocation)
** BUILD FAILED **
```
