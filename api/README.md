# Set up
1. Clone into ~/go/src directory
2. Run
```
govendor sync
```
3. To start the server, run
```
go run main.go
```
or from the root directory,
```
docker build -t ./ . -f Dockerfile.dev
docker run --rm -p 8080:3000 jji421/dockergo

docker run --rm -v "$PWD":/go/src/myapp/api -w /go/src/myapp/api iron/go:dev go build -o myapp
docker build -t jji421/dockergo:latest .
<!-- docker build -t jji421/dockergo . -->
docker run --rm -p 8080:3000 jji421/dockergo
```
and go to port [8080](localhost:8080)

To test the websocket connection, open socket.html, and click "Test socket".