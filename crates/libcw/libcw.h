#pragma once

#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

/**
 * Represents a set of supported encodings for a [`Parser`](crate::Parser).
 */
typedef enum Encoding {
  /**
   * UTF-8 encoded text, the default setting for [`Parser`](crate::Parser)
   */
  UTF8,
  /**
   * UTF-16 encoded text
   */
  UTF16,
} Encoding;

/**
 * Represents a set of supported line breaks for [`Parser`](crate::Parser).
 */
typedef enum LineBreak {
  /**
   * Carriage return. Often used on old Macintosh systems
   * - Unicode code: U+000D
   * - Default: No
   */
  CR,
  /**
   * Line feed. Most common implementation of new line used on all POSIX
   * systems such as macOS, Linux and FreeBSD
   * - Unicode code: U+000A
   * - Default: Yes
   */
  LF,
} LineBreak;

/**
 * Parser is libcw's main component. It provides abstractions over the
 * different counters contained inside this crate. To learn more how to use
 * `Parser` read [`Parser::process`](crate::Parser::process)
 *
 * The default `Parser` configuration will count **lines**, **words** and **characters**
 */
typedef struct Parser Parser;

/**
 * ABI representation of [Stats](crate::Stats)
 *
 * # Example
 *
 * ```c
 * Stats * stats = new_stats();
 * printf("%i",stats -> lines);
 * ```
 */
typedef struct Stats {
  unsigned long lines;
  unsigned long words;
  unsigned long characters;
  unsigned long bytes;
  unsigned long length;
} Stats;

/**
 * Creates a new Stats instance and returns its pointer
 *
 * ```c
 * Stats * stats = new_stats();
 * printf("%i",stats -> lines);
 * ```
 *
 * # Unsafe
 *
 * This method uses unsafe pointers that leak memory (required by the ABI
 * interface). To free up a Stats instance, use
 * [destroy_stats](crate::c::destroy_stats)
 */
struct Stats *new_stats(void);

/**
 * Destroys a Stats instance and frees its memory
 *
 * ```c
 * Stats * stats = new_stats();
 * destroy_stats(&stats);
 * // stats == NULL
 * ```
 *
 * # Warning
 *
 * The received pointer will point to `NULL`
 */
void destroy_stats(struct Stats **stats);

/**
 * Creates a new Parser instance and returns its pointer. For more information
 * read [Parser::new](crate::Parser::new)
 *
 * ```c
 * Parser * parser = new_parser(UTF8,LF,true,true,true,true,true);
 * ```
 *
 * # Unsafe
 *
 * This method uses unsafe pointers that leak memory (required by the ABI
 * interface). To free up a Stats instance, use
 * [destroy_parser](crate::c::destroy_parser)
 */
struct Parser *new_parser(enum Encoding encoding,
                          enum LineBreak linebreak,
                          bool lines,
                          bool words,
                          bool chars,
                          bool bytes,
                          bool max_length);

/**
 * Destroys a Parser instance and frees its memory
 *
 * ```c
 * Parser * parser = new_parser(UTF8,LF,true,true,true,true,true);
 * destroy_stats(&parser);
 * // parser == NULL
 * ```
 *
 * # Warning
 *
 * The received pointer will point to `NULL`
 */
void destroy_parser(struct Parser **parser);

/**
 * Runs the parser over a file
 *
 * # Params
 * - parser: A valid parser instance
 * - path: A valid path to a file
 * - out: A pointer to a valid stats instance
 *
 * # Successful codes
 *
 * - Code 0: The file was correctly parsed
 *
 * # Error codes
 *
 * - Code -1: Parser is null
 * - Code -2: Stats is null
 * - Code -3: The received string is not a valid Rust String Slice (see [str](std::str))
 * - Code -4: The file couldn't be opened
 * - Code -5: The parser couldn't read the file
 */
char process_file(const struct Parser *parser,
                  const char *path,
                  struct Stats *out);

/**
 * Runs the parser over an array slice
 *
 * # Params
 * - parser: A valid parser instance
 * - path: A pointer to the where the array slice starts
 * - size: The size of the array slice
 * - out: A pointer to a valid stats instance
 *
 * # Successful codes
 *
 * - Code 0: The slice was correctly parsed
 *
 * # Error codes
 *
 * - Code -1: Parser is null
 * - Code -2: Stats is null
 * - Code -5: The parser couldn't read the slice
 */
char process_slice(const struct Parser *parser,
                   const unsigned char *ptr,
                   unsigned long size,
                   struct Stats *out);

/**
 * Runs the parser over a string
 *
 * # Params
 * - parser: A valid parser instance
 * - ptr: A valid string
 * - out: A pointer to a valid stats instance
 *
 * # Successful codes
 *
 * - Code 0: The file was correctly parsed
 *
 * # Error codes
 *
 * - Code -1: Parser is null
 * - Code -2: Stats is null
 * - Code -5: The parser couldn't read the string
 */
char process_string(const struct Parser *parser,
                    const char *ptr,
                    struct Stats *out);
