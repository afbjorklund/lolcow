#include <stdio.h>
#include <stdlib.h>
#include <unistd.h>
#include <sys/wait.h>

int pid;
int pip1[2];
int pip2[2];

void exec1(void);
void exec2(void);
void exec3(void);

int main() {

  if (pipe(pip1) == -1) {
    perror("bad pipe1");
    exit(1);
  }

  if ((pid = fork()) == -1) {
    perror("bad fork1");
    exit(1);
  } else if (pid == 0) {
    // stdin --> ps --> pipe1
    exec1();
  }
  // parent

  if (pipe(pip2) == -1) {
    perror("bad pipe2");
    exit(1);
  }

  if ((pid = fork()) == -1) {
    perror("bad fork2");
    exit(1);
  } else if (pid == 0) {
    // pipe1 --> grep --> pipe2
    exec2();
  }
  // parent

  // close unused fds
  close(pip1[0]);
  close(pip1[1]);

  if ((pid = fork()) == -1) {
    perror("bad fork3");
    exit(1);
  } else if (pid == 0) {
    // pipe2 --> grep --> stdout
    exec3();
  }
  // parent

  usleep(1000); // 1ms
  waitpid(-1, NULL, 0);
  exit(0);
}


void exec1() {
  // input from stdin (already done)
  // output to pipe1
  dup2(pip1[1], 1);
  // close fds
  close(pip1[0]);
  close(pip1[1]);
  // exec
  execlp("endate", "endate", NULL);
  // exec didn't work, exit
  perror("bad exec endate");
  exit(1);
}

void exec2() {
  // input from pipe1
  dup2(pip1[0], 0);
  // output to pipe2
  dup2(pip2[1], 1);
  // close fds
  close(pip1[0]);
  close(pip1[1]);
  close(pip2[0]);
  close(pip2[1]);
  // exec
  execlp("cowsay", "cowsay", NULL);
  // exec didn't work, exit
  perror("bad exec cowsay");
  exit(1);
}

void exec3() {
  // input from pipe2
  dup2(pip2[0], 0);
  // output to stdout (already done)
  // close fds
  close(pip2[0]);
  close(pip2[1]);
  // exec
  execlp("lolcat", "lolcat", NULL);
  // exec didn't work, exit
  perror("bad exec lolcat");
  exit(1);
}
