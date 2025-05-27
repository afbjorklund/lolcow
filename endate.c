#include <stdio.h>
#include <stdlib.h>
#include <time.h>

void print_date() {
  char outstr[200];
  time_t t;
  struct tm *tmp;

  t = time(NULL);
  tmp = localtime(&t);
  if (tmp == NULL) {
    exit(EXIT_FAILURE);
  }

  if (strftime(outstr, sizeof(outstr), "%a %b %d %T %Z %Y", tmp) == 0) {
    exit(EXIT_FAILURE);
  }

  puts(outstr);
}

int main() {
  print_date();
  exit(EXIT_SUCCESS);
}
