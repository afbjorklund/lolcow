
all: endate cowsay lolcat lolcow
	./endate | ./cowsay | ./lolcat
	PATH=.:$$PATH ./lolcow

SCRATCH = false

ifeq ($(SCRATCH),false)
CC = gcc
CFLAGS = -O2
LDFLAGS = -O2
else
CC = musl-gcc
CFLAGS = -Os -static
LDFLAGS = -Os -static
endif

TAR = tar
DOCKER = docker
IMAGE = lolcow:latest
DOCKERFILE = Dockerfile
APPTAINERFILE = Apptainer

ifeq ($(SCRATCH),true)
IMAGE = lolcow:scratch
DOCKERFILE = Dockerfile.scratch
APPTAINERFILE = Apptainer.scratch
endif

lolcow: lolcow.o

endate: endate.o

cowsay: cowsay.o

lolcat: LDLIBS = -lm
lolcat: lolcat.o

lolcow.tar: lolcow endate cowsay lolcat
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
	$(RM) lolcow endate cowsay lolcat
	$(RM) *.o
