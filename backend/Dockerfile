FROM scratch

# Expose api port
EXPOSE 80/tcp

# Copy default env file
COPY ./env.default ./.env

# Copy executable
COPY ./musl/backend ./backend

# Start
ENTRYPOINT [ "./backend" ]
