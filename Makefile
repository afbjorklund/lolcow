
all: endate cowsay lolcat
	./endate | ./cowsay | ./lolcat

CC = gcc
CFLAGS = -O2
LDFLAGS = -O2

TAR = tar
DOCKER = docker
IMAGE = lolcow:latest
DOCKERFILE = Dockerfile
APPTAINERFILE = Apptainer

endate: endate.o

cowsay: cowsay.o

lolcat: LDLIBS = -lm
lolcat: lolcat.o

lolcow.tar: endate cowsay lolcat
	$(TAR) cf $@ $^

lolcow.docker.tar: $(DOCKERFILE) lolcow.tar
	$(DOCKER) build -t $(IMAGE) -f $< .
	$(DOCKER) save $(IMAGE) >$@

lolcow.sif: lolcow.docker.tar
	apptainer build $@ $(APPTAINERFILE)

.PHONY: clean
clean:
	$(RM) lolcow.sif
	$(RM) lolcow.tar lolcow.docker.tar
	$(RM) endate cowsay lolcat
	$(RM) *.o
