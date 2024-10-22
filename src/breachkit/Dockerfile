# Use an official Python image from the Docker Hub
FROM python:3.12-slim

# Set the working directory
WORKDIR /app

# Copy project files to the working directory
COPY . .

# Install dependencies
RUN pip install --no-cache-dir -r /app/requirements.txt

# Expose the application port
EXPOSE 8000

# Set the entry point to the ASGI server
CMD ["litestar", "run", "asgi.py", "--port", "8000", "--host", "127.0.0.1"]
