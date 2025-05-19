APP_NAME = yew-hello
IMAGE_NAME = yew-hello-app
CONTAINER_NAME = yew-hello-container
PORT = 8080

.PHONY: all build docker-build run clean

dev:
	trunk serve

# Local trunk build
build:
	trunk build --release

# Build Docker image
docker-build:
	docker build -t $(IMAGE_NAME) .

# Run the Docker container locally
docker-run:
	docker run --rm -p $(PORT):$(PORT) --name $(CONTAINER_NAME) -e PORT=$(PORT) $(IMAGE_NAME)

# Stop container manually (if needed)
stop:
	docker stop $(CONTAINER_NAME) || true

# Clean trunk build output
clean:
	rm -rf dist

