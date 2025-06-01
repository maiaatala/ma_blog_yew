include .env
export

# Defaults (can be overridden on the command line)
API_URL   ?= $(API_URL)
TEST_VAR  ?= $(TEST_VAR)

APP_NAME = yew-hello
IMAGE_NAME = yew-hello-app
IMAGE = yew-hello-app
CONTAINER = yew-hello-container
PORT = 8080

.PHONY: all build docker-build run clean

# ─── Local dev server ───────────────────────────────────────────────────
# Loads .env, then runs `trunk serve` on $PORT.
dev:
	@echo "📡 Running local dev with TEST_VAR=$(TEST_VAR)  API_URL=$(API_URL)"
	API_URL=$(API_URL) TEST_VAR=$(TEST_VAR) trunk serve --port $(PORT)

# ─── Local production build ─────────────────────────────────────────────
# Bakes env! into the WASM → you can serve dist/ however you want.
build:
	@echo "🛠  Building (release) with TEST_VAR=$(TEST_VAR)  API_URL=$(API_URL)"
	API_URL=$(API_URL) TEST_VAR=$(TEST_VAR) trunk build --release

# ─── Build Docker image for Railway (and local) ─────────────────────────
# Passes TEST_VAR and API_URL as --build-arg so the Dockerfile can see them.
docker-build:
	@echo "🐳 Building Docker image with TEST_VAR=$(TEST_VAR)  API_URL=$(API_URL)"
	docker build \
	  --build-arg API_URL=$(API_URL) \
	  --build-arg TEST_VAR=$(TEST_VAR) \
	  -t $(IMAGE) .

# ─── Run Docker container locally ────────────────────────────────────────
docker-run:
	@echo "▶️  Running Docker container locally on port $(PORT)"
	docker run --rm \
	  -p $(PORT):$(PORT) \
	  -e PORT=$(PORT) \
	  --name $(CONTAINER) \
	  $(IMAGE)

# ─── Clean trunk output ─────────────────────────────────────────────────
clean:
	trunk clean
	rm -rf dist
