FROM busybox:glibc
ADD lolcow.tar /bin/
ENV LC_ALL=C
ENTRYPOINT date | cowsay | lolcat
