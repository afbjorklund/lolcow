// Copyright 2016 The Fuchsia Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.
/*
Redistribution and use in source and binary forms, with or without
modification, are permitted provided that the following conditions are
met:
   * Redistributions of source code must retain the above copyright
notice, this list of conditions and the following disclaimer.
   * Redistributions in binary form must reproduce the above
copyright notice, this list of conditions and the following disclaimer
in the documentation and/or other materials provided with the
distribution.
THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
"AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
OWNER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
(INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
*/

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define MAX_WIDTH 40
#define MAX(a, b) ((a > b) ? a : b)
#define MIN(a, b) ((a < b) ? a : b)

// Returns the length of the longst line of the message.
static size_t LongestLineLength(int argc, char** argv) {
  size_t max_len = 0;
  size_t cur_line = 0;

  for (int i = 0; i < argc; i++) {
    size_t word_len = strlen(argv[i]) + 1;

    // If the word itself is too long to fit in a line, then
    // we return the maximum width.
    if (word_len >= MAX_WIDTH)
      return MAX_WIDTH;

    if (cur_line + word_len >= MAX_WIDTH) {
      cur_line = word_len;
    } else {
      cur_line += word_len;
    }

    max_len = MAX(cur_line, max_len);
  }

  return max_len;
}

static void PrintPaddedBreak(size_t pad) {
  for (size_t i = 0; i < pad; i++) {
    printf(" ");
  }
  printf(">\n");
}

// Prints the message
static void PrintMessage(int argc, char** argv, size_t longest) {
  size_t cur_line_len = 0;

  for (int i = 0; i < argc; i++) {
    size_t word_len = strlen(argv[i]) + 1;

    if (cur_line_len == 0)
      printf("< ");

    // If it all fits in the line, then print the word and move on.
    if (cur_line_len + word_len <= MAX_WIDTH) {
      printf("%s ", argv[i]);

      if (cur_line_len + word_len == MAX_WIDTH) {
        PrintPaddedBreak(longest - cur_line_len - word_len);
        cur_line_len = 0;
        continue;
      }

      cur_line_len += word_len;

      if (i == argc - 1)
        PrintPaddedBreak(longest - cur_line_len);

      continue;
    }

    // Create a line break if the current line is nonempty.
    if (cur_line_len > 0) {
      PrintPaddedBreak(longest - cur_line_len);
      printf("> ");
    }

    // If the word itself is too long, then we need to break it apart.
    // Otherwise, we print the current word and move on.
    if (word_len > MAX_WIDTH) {
      char* str = argv[i];
      size_t processed = 0;
      for (size_t j = 0; j <= word_len / MAX_WIDTH; j++) {
        size_t len = MIN(MAX_WIDTH, strlen(str));
        printf("%.*s", (int)len, str);
        PrintPaddedBreak(longest - len);
        str += len;
        processed += len;
        if (processed >= word_len - 1)
          break;
        printf("< ");
      }

      cur_line_len = 0;
    } else {
      printf("%s ", argv[i]);
      cur_line_len = word_len;
      if (word_len == MAX_WIDTH || i == argc - 1) {
        PrintPaddedBreak(longest - cur_line_len);
      }
    }
  }
}

static char *buf;
static size_t len;

void cowsay(int argc, char** argv) {
  // No wordwrap because I'm too lazy.
  size_t bubble_width = LongestLineLength(argc - 1, argv + 1) + 1;

  printf(" ");
  for (size_t i = 0; i < bubble_width; i++)
    printf("_");
  printf(" \n");

  PrintMessage(argc - 1, argv + 1, bubble_width - 1);

  printf(" ");
  for (size_t i = 0; i < bubble_width; i++)
    printf("-");
  printf(" \n");

  printf("         \\  ^__^\n");
  printf("          \\ (oo)\\______\n");
  printf("            (__)\\      )\\/\\\n");
  printf("               ||----w |\n");
  printf("               ||     ||\n");
}

int main(int argc, char** argv) {
  static char *arg[2];
  if (argc == 1) {
    //printf("Usage: cowsay [message]\n");
    //return 1;
    buf = malloc(4096);
    len = fread(buf, 1, 4095, stdin);
    if (len < 0)
        return 1;
    if (buf[len-1] == '\n')
	len -= 1;
    buf[len] = '\0';
    arg[0] = NULL;
    arg[1] = buf;

    argc = 2;
    argv = arg;
  }

  cowsay(argc, argv);
  return 0;
}
