FROM scratch

# Expose frontend port
EXPOSE 80/tcp

# Copy default env file
COPY ./server/env.default ./.env

# Copy executable
COPY ./server/musl/frontend ./frontend

# Copy files
COPY ./empty_directory ./static
COPY ./static/background_images ./static/background_images
COPY ./static/css ./static/css
COPY ./static/js ./static/js
COPY ./static/snippets ./static/snippets
COPY ./static/index.html ./static/index.html
COPY ./static/tic-tac-toe.svg ./static/tic-tac-toe.svg
COPY ./static/wasm.js ./static/wasm.js
COPY ./static/wasm_bg.wasm ./static/wasm_bg.wasm

# Start
ENTRYPOINT [ "./frontend" ]
