FROM alpine
RUN apk add perl ruby
RUN wget https://github.com/ilbers/cowsay/archive/refs/tags/debian/3.03+dfsg2-7.tar.gz && \
    tar xzf *.tar.gz && \
    cd cowsay-debian-* && \
    ./install.sh /usr && \
    cd - && \
    rm -rf cowsay-debian-* && \
    rm *.tar.gz && \
    gem install lolcat && \
    rm -rf ~/.cache
ENV LC_ALL=C
CMD ["/bin/sh", "-c", "date | cowsay | lolcat"]
