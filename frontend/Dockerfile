# Build stage
FROM node:22-alpine as builder

WORKDIR /app

# Enable corepack and prepare pnpm
RUN corepack enable && corepack prepare pnpm@latest --activate

# Copy workspace and package files from root
COPY pnpm-lock.yaml pnpm-workspace.yaml ./
COPY frontend/package.json ./

# Install dependencies (no frozen lockfile due to workspace setup)
RUN pnpm install --no-frozen-lockfile

# Copy frontend source code (excluding node_modules which were just installed)
COPY frontend/src ./src
COPY frontend/public ./public
COPY frontend/*.ts frontend/*.js frontend/*.json frontend/*.html .

# Build the application (skip TypeScript checking in Docker)
RUN pnpm build:docker

# Production stage
FROM nginx:alpine

# Copy built assets from builder
COPY --from=builder /app/dist /usr/share/nginx/html

# Copy nginx configuration from frontend directory
COPY frontend/nginx.conf /etc/nginx/conf.d/default.conf

EXPOSE 80

CMD ["nginx", "-g", "daemon off;"]
