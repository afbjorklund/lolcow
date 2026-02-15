FROM busybox:glibc
ADD lolcow.tar /bin/
ENV LC_ALL=C
CMD ["sh", "-c", "date | cowsay | lolcat"]
