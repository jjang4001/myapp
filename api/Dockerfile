FROM iron/go:dev
WORKDIR /app

ENV SRC_DIR=/go/src/myapp/api

# Add the source code:
ADD . $SRC_DIR

# Build it:
RUN go get -u github.com/kardianos/govendor
RUN cd $SRC_DIR; govendor sync; go build -o myapp; cp myapp /app/
ENTRYPOINT ["./myapp"]