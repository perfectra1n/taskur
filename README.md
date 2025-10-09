# Taskur - Modern Task Management Application

A beautiful, self-hosted task management application inspired by TickTick, built with Rust, PostgreSQL, React, and TypeScript.

## Features

### Backend (Rust + Actix-web + SQLx + PostgreSQL)
- **Authentication**: JWT-based authentication with bcrypt password hashing
- **Task Management**: Full CRUD operations with rich task metadata
  - Title, description (rich text/markdown support)
  - Status tracking (todo, in-progress, completed)
  - Priority levels (low, medium, high, urgent)
  - Due dates with timezone support
  - Tag-based organization
  - Drag-and-drop position ordering
- **Lists**: Organize tasks into custom lists with colors and icons
- **Tags**: Flexible tagging system with color coding
- **Comments**: Add threaded comments to tasks with markdown support
- **Attachments**: Upload and attach files/images to tasks and comments
  - Support for images, documents, and files up to 10MB
  - Secure file storage with access control
- **Search & Filtering**:
  - Full-text search across task titles and descriptions
  - Advanced filtering by status, priority, tags, and due dates
  - Real-time search results

### Frontend (React + TypeScript + Vite + TipTap + Tailwind)
- **Modern UI**: Clean, minimal interface with light/dark mode
- **Rich Text Editing**: TipTap editor for task descriptions and comments
  - Markdown support
  - Image embedding
  - Task lists within descriptions
  - Link handling
- **Multiple Views**:
  - List view with drag-and-drop reordering
  - Kanban board view
  - Filtered views (Today, Upcoming, Priority)
- **Real-time Updates**: Optimistic UI updates with TanStack Query
- **Responsive Design**: Mobile-first, works beautifully on all devices
- **Keyboard Shortcuts**: Quick access to common actions
- **File Management**: Drag-and-drop file uploads with preview

## Tech Stack

### Backend
- **Rust** - Systems programming language
- **Actix-web** - High-performance web framework
- **SQLx** - Async SQL toolkit with compile-time query verification
- **PostgreSQL** - Robust relational database
- **JWT** - Secure authentication
- **bcrypt** - Password hashing

### Frontend
- **React 19** - UI library
- **TypeScript** - Type-safe JavaScript
- **Vite** - Fast build tool and dev server
- **TipTap** - Headless rich text editor
- **Tailwind CSS** - Utility-first CSS framework
- **PostCSS** - CSS processing
- **TanStack Query** - Data fetching and caching
- **Zustand** - State management
- **React Router** - Client-side routing
- **@hello-pangea/dnd** - Drag-and-drop functionality
- **date-fns** - Date utilities
- **Lucide React** - Beautiful icon library

## Project Structure

```
taskur/
├── backend/                 # Rust backend
│   ├── src/
│   │   ├── handlers/       # API request handlers
│   │   ├── models/         # Data models
│   │   ├── middleware/     # Auth middleware
│   │   ├── utils/          # JWT, password utilities
│   │   ├── config.rs       # Configuration
│   │   ├── db.rs           # Database connection
│   │   ├── errors.rs       # Error handling
│   │   └── main.rs         # Application entry
│   ├── migrations/         # Database migrations
│   └── Cargo.toml          # Rust dependencies
├── frontend/               # React frontend
│   ├── src/
│   │   ├── components/     # React components
│   │   ├── pages/          # Page components
│   │   ├── hooks/          # Custom hooks
│   │   ├── services/       # API services
│   │   ├── types/          # TypeScript types
│   │   ├── utils/          # Utility functions
│   │   └── styles/         # CSS files
│   ├── public/             # Static assets
│   └── package.json        # Node dependencies
├── package.json            # Root monorepo config
├── pnpm-workspace.yaml     # pnpm workspace config
└── .env.example            # Environment variables template
```

## Setup Instructions

### Prerequisites
- **Rust** (1.70+): Install from [rustup.rs](https://rustup.rs/)
- **PostgreSQL** (14+): Install from [postgresql.org](https://www.postgresql.org/download/)
- **Node.js** (20+): Install from [nodejs.org](https://nodejs.org/)
- **pnpm**: Install with `npm install -g pnpm`
- **cargo-watch** (optional): Install with `cargo install cargo-watch`

### 1. Clone and Setup

```bash
cd taskur
cp .env.example .env
```

### 2. Configure Environment

Edit `.env` with your database credentials:

```env
DATABASE_URL=postgres://postgres:postgres@localhost:5432/taskur
JWT_SECRET=your-secret-key-change-this-in-production
RUST_LOG=info
HOST=127.0.0.1
PORT=8080
```

### 3. Setup Database

```bash
# Create PostgreSQL database
createdb taskur

# Run migrations (from root)
pnpm migrate
```

### 4. Install Dependencies

```bash
# Install all dependencies (backend and frontend)
pnpm install
```

### 5. Development

Run both backend and frontend in development mode:

```bash
pnpm dev
```

This will start:
- Backend API at `http://localhost:8080`
- Frontend dev server at `http://localhost:5173`

### Alternative: Run Separately

```bash
# Backend (in one terminal)
pnpm dev:backend

# Frontend (in another terminal)
pnpm dev:frontend
```

### 6. Production Build

```bash
# Build both backend and frontend
pnpm build

# Run backend in production
cd backend && cargo run --release
```

## API Endpoints

### Authentication
- `POST /api/auth/register` - Register new user
- `POST /api/auth/login` - Login user
- `GET /api/auth/me` - Get current user

### Tasks
- `GET /api/tasks` - List all tasks (with filtering)
- `POST /api/tasks` - Create new task
- `GET /api/tasks/:id` - Get task details
- `PUT /api/tasks/:id` - Update task
- `DELETE /api/tasks/:id` - Delete task

### Comments
- `GET /api/tasks/:task_id/comments` - List task comments
- `POST /api/tasks/:task_id/comments` - Add comment
- `PUT /api/tasks/:task_id/comments/:comment_id` - Update comment
- `DELETE /api/tasks/:task_id/comments/:comment_id` - Delete comment

### Attachments
- `GET /api/tasks/:task_id/attachments` - List task attachments
- `POST /api/tasks/:task_id/attachments` - Upload attachment
- `GET /api/attachments/:id` - Download attachment
- `DELETE /api/attachments/:id` - Delete attachment

### Lists
- `GET /api/lists` - List all lists
- `POST /api/lists` - Create list
- `GET /api/lists/:id` - Get list details
- `PUT /api/lists/:id` - Update list
- `DELETE /api/lists/:id` - Delete list

### Tags
- `GET /api/tags` - List all tags
- `POST /api/tags` - Create tag
- `DELETE /api/tags/:id` - Delete tag

## Database Schema

### Users
- `id` (UUID, PK)
- `email` (VARCHAR, UNIQUE)
- `password_hash` (VARCHAR)
- `created_at`, `updated_at` (TIMESTAMPTZ)

### Tasks
- `id` (UUID, PK)
- `user_id` (UUID, FK → users)
- `title` (VARCHAR)
- `description` (TEXT)
- `status` (ENUM: todo, inprogress, completed)
- `priority` (ENUM: low, medium, high, urgent)
- `due_date` (TIMESTAMPTZ, nullable)
- `tags` (TEXT[])
- `position` (INTEGER)
- `created_at`, `updated_at` (TIMESTAMPTZ)

### Lists
- `id` (UUID, PK)
- `user_id` (UUID, FK → users)
- `name` (VARCHAR)
- `color`, `icon` (VARCHAR, nullable)
- `position` (INTEGER)
- `created_at`, `updated_at` (TIMESTAMPTZ)

### Tags
- `id` (UUID, PK)
- `user_id` (UUID, FK → users)
- `name` (VARCHAR)
- `color` (VARCHAR, nullable)
- `created_at` (TIMESTAMPTZ)

### Comments
- `id` (UUID, PK)
- `task_id` (UUID, FK → tasks)
- `user_id` (UUID, FK → users)
- `content` (TEXT)
- `created_at`, `updated_at` (TIMESTAMPTZ)

### Attachments
- `id` (UUID, PK)
- `task_id` or `comment_id` (UUID, FK, mutually exclusive)
- `user_id` (UUID, FK → users)
- `filename`, `original_filename` (VARCHAR)
- `file_path` (VARCHAR)
- `file_size` (BIGINT)
- `mime_type` (VARCHAR)
- `created_at` (TIMESTAMPTZ)

## Development Workflow

The project uses **pnpm** as the monorepo tool:

```bash
# Install dependencies
pnpm install

# Development (runs both backend and frontend)
pnpm dev

# Build production
pnpm build

# Run migrations
pnpm migrate

# Create new migration
pnpm migrate:create <migration_name>
```

## Features Roadmap

### Completed
- ✅ Backend API with Rust + Actix-web
- ✅ PostgreSQL database with migrations
- ✅ JWT authentication
- ✅ Task CRUD operations
- ✅ Lists and tags
- ✅ Comments system
- ✅ File attachments
- ✅ Search and filtering
- ✅ Frontend React setup with TypeScript
- ✅ Tailwind CSS + PostCSS

### In Progress
- 🔄 Authentication UI
- 🔄 Task list view with drag-drop
- 🔄 TipTap rich text editor integration
- 🔄 List/tag management UI
- 🔄 Kanban board view

### Planned
- ⏳ Natural language date parsing
- ⏳ Keyboard shortcuts
- ⏳ Dark mode toggle
- ⏳ Mobile responsiveness
- ⏳ Real-time collaboration (WebSocket)
- ⏳ Recurring tasks
- ⏳ Task templates
- ⏳ Data export/import
- ⏳ Docker deployment setup

## Contributing

This is a self-hosted project designed for personal use. Feel free to fork and customize!

## License

MIT License - feel free to use this project for personal or commercial purposes.

## Acknowledgments

- Inspired by [TickTick](https://ticktick.com/)
- Built with amazing open-source technologies
- Special thanks to the Rust and React communities
