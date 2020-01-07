### Amoeba web server

It is a lightweight asynchronous web server.

Amoeba created as a development tool for serving static files only.
You can use it "as is" as you want but WITHOUT ANY WARRANTY.

##### HOW TO USE

Launch in Docker container:

```shell script
docker run --rm -it -p 8000:8000 -v $PWD:/app/public logix/amoeba:latest
```

Docker compose:

```yaml
amoeba_static:
    image: logix/amoeba:latest

    environment:
      - AMOEBA_PORT=8000
      - AMOEBA_HOST=0.0.0.0
      - AMOEBA_INDEX_FILE=index.html
      - AMOEBA_PUBLIC_DIR=public

    volumes:
      - <directory-with-static-files>:/app/public

    ports:
      - "8000:8000"

```
