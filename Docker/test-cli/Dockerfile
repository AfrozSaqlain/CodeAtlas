FROM python:3.11-slim

ENV PYTHONDONTWRITEBYTECODE=1
ENV PYTHONUNBUFFERED=1

# Install runtime dependencies for matplotlib (no gcc → smaller image)
RUN apt-get update && apt-get install -y \
    libglib2.0-0 \
    libsm6 \
    libxext6 \
    libxrender1 \
    && rm -rf /var/lib/apt/lists/*

# 👇 App lives here
WORKDIR /opt/app

# Copy project files
COPY pyproject.toml .
COPY src/ ./src/

# Install package
RUN pip install --no-cache-dir --upgrade pip && \
    pip install --no-cache-dir .

# 👇 Start shell in home directory instead of /opt/app
CMD ["bash", "-lc", "cd ~ && exec bash"]
