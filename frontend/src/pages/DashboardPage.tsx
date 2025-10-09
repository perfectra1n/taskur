import { useState, useEffect } from 'react';
import { useQuery } from '@tanstack/react-query';
import { api } from '../services/api';
import { useAuthStore } from '../stores/authStore';
import { Button } from '../components/ui/Button';
import { ThemeToggle } from '../components/ui/ThemeToggle';
import { FloatingActionButton } from '../components/ui/FloatingActionButton';
import { TaskList } from '../components/tasks/TaskList';
import { TaskDetail } from '../components/tasks/TaskDetail';
import { CreateTaskModal } from '../components/tasks/CreateTaskModal';
import { LogOut, Plus, Search, Sparkles, LayoutDashboard, Filter, SortDesc } from 'lucide-react';
import type { Task } from '../types';

export function DashboardPage() {
  const [selectedTask, setSelectedTask] = useState<Task | null>(null);
  const [isCreateModalOpen, setIsCreateModalOpen] = useState(false);
  const [searchQuery, setSearchQuery] = useState('');

  const logout = useAuthStore((state) => state.logout);
  const user = useAuthStore((state) => state.user);

  const { data: tasks = [], isLoading } = useQuery({
    queryKey: ['tasks', searchQuery],
    queryFn: () => api.getTasks(searchQuery ? { search: searchQuery } : undefined),
  });

  const handleLogout = () => {
    logout();
    window.location.href = '/login';
  };

  // Keyboard shortcuts
  useEffect(() => {
    const handleKeyPress = (e: KeyboardEvent) => {
      // Ctrl/Cmd + K: Open create task modal
      if ((e.ctrlKey || e.metaKey) && e.key === 'k') {
        e.preventDefault();
        setIsCreateModalOpen(true);
      }
      // Escape: Close task detail panel or modal
      if (e.key === 'Escape') {
        if (isCreateModalOpen) {
          setIsCreateModalOpen(false);
        } else if (selectedTask) {
          setSelectedTask(null);
        }
      }
    };

    window.addEventListener('keydown', handleKeyPress);
    return () => window.removeEventListener('keydown', handleKeyPress);
  }, [selectedTask, isCreateModalOpen]);

  return (
    <div className="h-screen flex flex-col bg-[#fafbfc] dark:bg-black">
      {/* Floating Header */}
      <header className="sticky top-0 z-20 px-6 py-4">
        <div className="glass-card-strong rounded-2xl px-6 py-4 shadow-elevation-2">
          <div className="flex items-center justify-between">
            {/* Left side - Logo and Search */}
            <div className="flex items-center gap-6">
              {/* Logo */}
              <div className="flex items-center gap-3">
                <div className="relative">
                  <div className="w-10 h-10 rounded-xl bg-gradient-primary flex items-center justify-center shadow-elevation-2">
                    <Sparkles className="w-5 h-5 text-white" strokeWidth={2.5} />
                  </div>
                  <div className="absolute -top-1 -right-1 w-3 h-3 bg-accent-cyan rounded-full border-2 border-white dark:border-black animate-pulse" />
                </div>
                <div>
                  <h1 className="text-xl font-bold gradient-text">
                    Taskur
                  </h1>
                  <p className="text-xs text-slate-500 dark:text-slate-400">
                    Task Management
                  </p>
                </div>
              </div>

              {/* Search Bar */}
              <div className="relative w-96">
                <Search className="absolute left-4 top-1/2 transform -translate-y-1/2 w-4 h-4 text-slate-400 dark:text-slate-500" />
                <input
                  type="text"
                  placeholder="Search tasks... (⌘K)"
                  value={searchQuery}
                  onChange={(e) => setSearchQuery(e.target.value)}
                  className="w-full pl-11 pr-4 py-3 glass-input rounded-xl text-slate-900 dark:text-slate-100 placeholder:text-slate-400 dark:placeholder:text-slate-500 focus:outline-none focus:ring-2 focus:ring-primary-500/30 transition-all text-sm"
                />
              </div>
            </div>

            {/* Right side - User and Actions */}
            <div className="flex items-center gap-4">
              {/* User info */}
              <div className="glass-card px-4 py-2 rounded-xl">
                <div className="flex items-center gap-3">
                  <div className="w-8 h-8 rounded-lg bg-gradient-primary flex items-center justify-center text-white font-semibold text-sm">
                    {user?.email?.charAt(0).toUpperCase()}
                  </div>
                  <span className="text-sm font-medium text-slate-700 dark:text-slate-300">
                    {user?.email}
                  </span>
                </div>
              </div>

              {/* Theme toggle */}
              <ThemeToggle />

              {/* Logout */}
              <Button variant="ghost" size="sm" onClick={handleLogout}>
                <LogOut className="w-4 h-4" />
              </Button>
            </div>
          </div>
        </div>
      </header>

      {/* Main Content */}
      <div className="flex-1 flex overflow-hidden px-6 pb-6">
        <div className="flex-1 flex gap-6 overflow-hidden">
          {/* Sidebar */}
          <aside className="w-64 flex-shrink-0">
            <div className="glass-card rounded-2xl p-4 h-full">
              <nav className="space-y-2">
                {/* All Tasks */}
                <button className="w-full flex items-center gap-3 px-4 py-3 text-sm font-semibold text-white bg-gradient-primary rounded-xl shadow-elevation-2 hover:shadow-glow-primary transition-all hover:scale-105 active:scale-95">
                  <LayoutDashboard className="w-4 h-4" strokeWidth={2.5} />
                  All Tasks
                  <span className="ml-auto px-2 py-0.5 bg-white/20 rounded-lg text-xs">
                    {tasks.length}
                  </span>
                </button>

                {/* Filter options */}
                <div className="pt-4 pb-2">
                  <p className="px-2 text-xs font-semibold text-slate-500 dark:text-slate-400 uppercase tracking-wider">
                    Quick Filters
                  </p>
                </div>

                <button className="w-full flex items-center gap-3 px-4 py-2.5 text-sm font-medium text-slate-700 dark:text-slate-300 hover:bg-slate-100/80 dark:hover:bg-slate-800/80 rounded-xl transition-all hover:scale-105 active:scale-95">
                  <Filter className="w-4 h-4" />
                  Active
                  <span className="ml-auto px-1.5 py-0.5 bg-blue-100 dark:bg-blue-900/30 text-blue-700 dark:text-blue-300 rounded text-xs font-semibold">
                    {tasks.filter(t => t.status !== 'completed').length}
                  </span>
                </button>

                <button className="w-full flex items-center gap-3 px-4 py-2.5 text-sm font-medium text-slate-700 dark:text-slate-300 hover:bg-slate-100/80 dark:hover:bg-slate-800/80 rounded-xl transition-all hover:scale-105 active:scale-95">
                  <SortDesc className="w-4 h-4" />
                  Completed
                  <span className="ml-auto px-1.5 py-0.5 bg-emerald-100 dark:bg-emerald-900/30 text-emerald-700 dark:text-emerald-300 rounded text-xs font-semibold">
                    {tasks.filter(t => t.status === 'completed').length}
                  </span>
                </button>
              </nav>
            </div>
          </aside>

          {/* Task List Area */}
          <main className="flex-1 flex flex-col overflow-hidden">
            {/* Header */}
            <div className="flex items-center justify-between mb-6">
              <div>
                <h2 className="text-3xl font-bold text-slate-900 dark:text-slate-100">
                  Tasks
                </h2>
                <p className="text-sm text-slate-500 dark:text-slate-400 mt-1">
                  {tasks.length} {tasks.length === 1 ? 'task' : 'tasks'} total
                </p>
              </div>

              <Button variant="gradient" onClick={() => setIsCreateModalOpen(true)}>
                <Plus className="w-5 h-5" strokeWidth={2.5} />
                New Task
              </Button>
            </div>

            {/* Task List */}
            <div className="flex-1 overflow-hidden">
              {isLoading ? (
                <div className="flex items-center justify-center h-full">
                  <div className="flex flex-col items-center gap-4">
                    <div className="w-12 h-12 rounded-2xl bg-gradient-primary animate-spin" style={{
                      clipPath: 'polygon(50% 0%, 100% 50%, 50% 100%, 0% 50%)'
                    }} />
                    <p className="text-slate-500 dark:text-slate-400 font-medium">Loading tasks...</p>
                  </div>
                </div>
              ) : (
                <TaskList
                  tasks={tasks}
                  onTaskClick={setSelectedTask}
                  selectedTaskId={selectedTask?.id}
                />
              )}
            </div>
          </main>

          {/* Task Detail Panel - Slide in from right */}
          {selectedTask && (
            <div className="animate-slide-left">
              <TaskDetail
                task={selectedTask}
                onClose={() => setSelectedTask(null)}
              />
            </div>
          )}
        </div>
      </div>

      {/* Floating Action Button */}
      <FloatingActionButton
        onClick={() => setIsCreateModalOpen(true)}
        label="Create New Task (⌘K)"
      />

      {/* Create Task Modal */}
      <CreateTaskModal
        isOpen={isCreateModalOpen}
        onClose={() => setIsCreateModalOpen(false)}
      />
    </div>
  );
}
