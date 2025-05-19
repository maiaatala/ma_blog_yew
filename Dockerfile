# Build stage: compile the Yew app
FROM rust:1.87 as builder

RUN rustup target add wasm32-unknown-unknown

# Install trunk from binary
ADD https://github.com/thedodd/trunk/releases/download/v0.21.14/trunk-x86_64-unknown-linux-gnu.tar.gz /tmp/trunk.tar.gz
RUN cd /tmp && tar xf trunk.tar.gz && chmod +x trunk && mv trunk /usr/local/bin/trunk

WORKDIR /app
COPY . .
RUN trunk build --release

# Runtime stage: serve the output
FROM node:18-alpine

# Install `serve` (same tool used for React apps!)
RUN npm install -g serve

COPY --from=builder /app/dist /app

ENV PORT=8080
EXPOSE 8080
CMD ["serve", "-s", "/app", "-l", "8080"]

