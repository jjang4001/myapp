# Getting Started

Make sure you have wasm-pack downloaded before running.

1. From the root directory, run
    ```
    npm install
    npm run build
    npm start
    ```

2. Docker
    ```
    docker build -t jji421/myapp-client:latest . -f Dockerfile(.dev)
    docker run -it jji421/myapp-client sh
    docker run -p 3000:3000 jji421/myapp-client
    ```

3. Publishing wasm package
Make sure to have wasm-pack downloaded. Then, from `/crate`, run `wasm-pack build`.
Then, navigate to the `/pkg` dir, and run `npm publish --access=public --tag beta`.