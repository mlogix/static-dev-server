FROM debian:buster-slim

LABEL maintainer="Mykhailo Odyniuk <m.odyniuk@gmail.com>"
LABEL version="0.0.1"

ENV AMOEBA_PORT     8000
ENV AMOEBA_HOST     0.0.0.0
ENV AMOEBA_INDEX_FILE   index.html
ENV AMOEBA_PUBLIC_DIR   public

EXPOSE 8000

RUN mkdir /app
RUN mkdir /app/public

WORKDIR /app

ADD bin/amoeba-x86_64-unknown-linux-gnu /usr/local/bin/amoeba

CMD ["amoeba"]