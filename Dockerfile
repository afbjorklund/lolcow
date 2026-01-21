FROM busybox:glibc
ADD lolcow.tar /bin/
ENV LC_ALL=C
ENTRYPOINT ["sh", "-c", "date | cowsay | lolcat"]
