// Prototypes for executing builtin_random function.
#ifndef FISH_BUILTIN_RANDOM_H
#define FISH_BUILTIN_RANDOM_H

#include "../maybe.h"

class parser_t;
struct io_streams_t;

maybe_t<int> builtin_random(parser_t &parser, io_streams_t &streams, const wchar_t **argv);
#endif