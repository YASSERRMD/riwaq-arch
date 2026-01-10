# Docker Setup for Riwaq Arch

Run the Riwaq Arch server using Docker to avoid local dependency issues.

## Prerequisites

- Docker
- Docker Compose

## Quick Start

1. **Set your LLM API Key** (optional, but recommended for full features):
   ```bash
   export RIWAQ_LLM_API_KEY="sk-..."
   ```

2. **Start the Server**:
   ```bash
   # By default, this mounts the current directory into the container at /data
   # You can override this by setting PROJECTS_DIR
   
   PROJECTS_DIR=/path/to/my/projects docker-compose up -d
   ```

3. **Verify**:
   The server is now running at `http://localhost:9527`.
   
   You can verify it works:
   ```bash
   curl http://localhost:9527/health
   ```

## Usage with specific projects

When running in Docker, the server creates a volume mount mapping your host directory to `/data` inside the container.

**Important**: When asking Riwaq to analyze a path via the API or CLI, you must use the **container path** (`/data/...`), not your host path.

### Example:

If you mounted `~/Documents/Coding` to `/data`:

1. Your project on host: `~/Documents/Coding/my-app`
2. Path inside container: `/data/my-app`

**Analyzing via API**:
```bash
curl -X POST http://localhost:9527/analyze \
  -H "Content-Type: application/json" \
  -d '{
    "path": "/data/my-app",
    "max_commits": 100
  }'
```

## VS Code Extension Configuration

To use the Dockerized server with the VS Code extension:

1. Start the Docker container.
2. Open your VS Code settings.
3. Ensure `riwaq.serverUrl` is set to `http://127.0.0.1:9527`.
4. **Note:** The extension currently sends *local host paths* to the server. Since the server is in Docker, path mapping might fail if the paths don't match exactly.
   
   > [!IMPORTANT]
   > For the best experience with the VS Code extension, **we recommend running the server natively** (`riwaq serve`) on your host machine.
   >
   > If you must use Docker with VS Code, you need to ensure the volume mount path in `docker-compose.yml` exactly matches your local path (e.g., if project is at `/Users/me/proj`, you must mount it to `/Users/me/proj` in the container).

## Building Manually

```bash
docker build -t riwaq-arch .
docker run -p 9527:9527 -v $(pwd):/data -e RIWAQ_LLM_API_KEY=$RIWAQ_LLM_API_KEY riwaq-arch
```
