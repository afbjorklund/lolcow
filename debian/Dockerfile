FROM debian:stable-slim
RUN apt-get update && apt-get install -y cowsay lolcat
ENV PATH=/usr/games:$PATH
ENV LC_ALL=C
CMD ["/bin/sh", "-c", "date | cowsay | lolcat"]
