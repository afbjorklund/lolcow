// Copyright 2016 The Fuchsia Authors. All rights reserved.
//
// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions are
// met:
//    * Redistributions of source code must retain the above copyright
// notice, this list of conditions and the following disclaimer.
//    * Redistributions in binary form must reproduce the above
// copyright notice, this list of conditions and the following disclaimer
// in the documentation and/or other materials provided with the
// distribution.
// THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
// "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
// LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
// A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
// OWNER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
// SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
// LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
// DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
// THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
// (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
// OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.

const MAX_WIDTH: usize = 40;

pub fn cowsay_to_string(message: &str) -> String {
    let words: Vec<&str> = message.split_whitespace().collect();
    let bubble_width = longest_line_length(&words) + 1;

    let mut output = String::new();

    output.push_str(&format!(" {} \n", "_".repeat(bubble_width)));
    output.push_str(&print_message_to_string(&words, bubble_width - 1));
    output.push_str(&format!(" {} \n", "-".repeat(bubble_width)));

    output.push_str("         \\  ^__^\n");
    output.push_str("          \\ (oo)\\______\n");
    output.push_str("            (__)\\      )\\/\\\n");
    output.push_str("               ||----w |\n");
    output.push_str("               ||     ||\n");

    output
}

fn longest_line_length(words: &[&str]) -> usize {
    let mut max_len = 0;
    let mut cur_line = 0;

    for word in words {
        let word_len = word.len() + 1;

        if word_len >= MAX_WIDTH {
            return MAX_WIDTH;
        }

        if cur_line + word_len >= MAX_WIDTH {
            cur_line = word_len;
        } else {
            cur_line += word_len;
        }

        max_len = std::cmp::max(cur_line, max_len);
    }

    max_len
}

fn print_padded_break_to_string(pad: usize) -> String {
    format!("{:>width$}>\n", "", width = pad + 2)
}

fn print_message_to_string(words: &[&str], longest: usize) -> String {
    let mut output = String::new();
    let mut cur_line_len = 0;

    for (i, word) in words.iter().enumerate() {
        let word_len = word.len() + 1;

        if cur_line_len == 0 {
            output.push_str("< ");
        }

        if cur_line_len + word_len <= MAX_WIDTH {
            output.push_str(word);
            output.push(' ');

            if cur_line_len + word_len == MAX_WIDTH {
                output.push_str(&print_padded_break_to_string(
                    longest - cur_line_len - word_len,
                ));
                cur_line_len = 0;
                continue;
            }

            cur_line_len += word_len;

            if i == words.len() - 1 {
                output.push_str(&print_padded_break_to_string(longest - cur_line_len));
            }
        } else {
            if cur_line_len > 0 {
                output.push_str(&print_padded_break_to_string(longest - cur_line_len));
                output.push_str("> ");
            }

            if word_len > MAX_WIDTH {
                let mut str = *word;
                let mut processed = 0;
                for _j in 0..=word.len() / MAX_WIDTH {
                    let len = std::cmp::min(MAX_WIDTH, str.len());
                    output.push_str(&str[..len]);
                    output.push_str(&print_padded_break_to_string(longest - len));
                    str = &str[len..];
                    processed += len;
                    if processed >= word.len() - 1 {
                        break;
                    }
                    output.push_str("< ");
                }
                cur_line_len = 0;
            } else {
                output.push_str(word);
                output.push(' ');
                cur_line_len = word_len;
                if word_len == MAX_WIDTH || i == words.len() - 1 {
                    output.push_str(&print_padded_break_to_string(longest - cur_line_len));
                }
            }
        }
    }

    output
}
