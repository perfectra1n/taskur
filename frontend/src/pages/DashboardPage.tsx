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
import { LogOut, Plus, Search, LayoutDashboard, Filter, SortDesc } from 'lucide-react';
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
    <div className="h-screen flex flex-col bg-slate-50 dark:bg-slate-950">
      {/* Header */}
      <header className="border-b border-slate-200 dark:border-slate-800 bg-white dark:bg-slate-900">
        <div className="px-4 py-3">
          <div className="flex items-center justify-between gap-4">
            {/* Logo */}
            <div className="flex items-center gap-2">
              <div className="w-8 h-8 rounded-lg bg-primary-600 dark:bg-primary-500 flex items-center justify-center">
                <LayoutDashboard className="w-4 h-4 text-white" />
              </div>
              <h1 className="text-lg font-bold text-slate-900 dark:text-slate-100">
                Taskur
              </h1>
            </div>

            {/* Search Bar - Hidden on mobile */}
            <div className="hidden md:flex flex-1 max-w-md">
              <div className="relative w-full">
                <Search className="absolute left-3 top-1/2 transform -translate-y-1/2 w-4 h-4 text-slate-400" />
                <input
                  type="text"
                  placeholder="Search tasks..."
                  value={searchQuery}
                  onChange={(e) => setSearchQuery(e.target.value)}
                  className="w-full pl-9 pr-3 py-2 border border-slate-200 dark:border-slate-800 rounded-lg bg-slate-50 dark:bg-slate-950 text-slate-900 dark:text-slate-100 placeholder:text-slate-400 focus:outline-none focus:ring-2 focus:ring-primary-500/20 focus:border-primary-500 text-sm"
                />
              </div>
            </div>

            {/* Right side actions */}
            <div className="flex items-center gap-2">
              <ThemeToggle />
              <Button variant="ghost" size="sm" onClick={handleLogout}>
                <LogOut className="w-4 h-4" />
              </Button>
            </div>
          </div>

          {/* Mobile Search */}
          <div className="md:hidden mt-3">
            <div className="relative">
              <Search className="absolute left-3 top-1/2 transform -translate-y-1/2 w-4 h-4 text-slate-400" />
              <input
                type="text"
                placeholder="Search tasks..."
                value={searchQuery}
                onChange={(e) => setSearchQuery(e.target.value)}
                className="w-full pl-9 pr-3 py-2 border border-slate-200 dark:border-slate-800 rounded-lg bg-slate-50 dark:bg-slate-950 text-slate-900 dark:text-slate-100 placeholder:text-slate-400 focus:outline-none focus:ring-2 focus:ring-primary-500/20 focus:border-primary-500 text-sm"
              />
            </div>
          </div>
        </div>
      </header>

      {/* Main Content */}
      <div className="flex-1 flex overflow-hidden">
        <div className="flex-1 flex overflow-hidden">
          {/* Sidebar - Hidden on mobile */}
          <aside className="hidden lg:flex w-56 border-r border-slate-200 dark:border-slate-800 bg-white dark:bg-slate-900">
            <div className="p-4 w-full">
              <nav className="space-y-1">
                <button className="w-full flex items-center gap-2 px-3 py-2 text-sm font-medium text-white bg-primary-600 hover:bg-primary-700 rounded-lg transition-colors">
                  <LayoutDashboard className="w-4 h-4" />
                  All Tasks
                  <span className="ml-auto text-xs">
                    {tasks.length}
                  </span>
                </button>

                <div className="pt-4 pb-2">
                  <p className="px-2 text-xs font-medium text-slate-500 dark:text-slate-400">
                    Filters
                  </p>
                </div>

                <button className="w-full flex items-center gap-2 px-3 py-2 text-sm font-medium text-slate-700 dark:text-slate-300 hover:bg-slate-100 dark:hover:bg-slate-800 rounded-lg transition-colors">
                  <Filter className="w-4 h-4" />
                  Active
                  <span className="ml-auto px-1.5 py-0.5 bg-blue-100 dark:bg-blue-900/30 text-blue-700 dark:text-blue-300 rounded text-xs">
                    {tasks.filter(t => t.status !== 'completed').length}
                  </span>
                </button>

                <button className="w-full flex items-center gap-2 px-3 py-2 text-sm font-medium text-slate-700 dark:text-slate-300 hover:bg-slate-100 dark:hover:bg-slate-800 rounded-lg transition-colors">
                  <SortDesc className="w-4 h-4" />
                  Completed
                  <span className="ml-auto px-1.5 py-0.5 bg-emerald-100 dark:bg-emerald-900/30 text-emerald-700 dark:text-emerald-300 rounded text-xs">
                    {tasks.filter(t => t.status === 'completed').length}
                  </span>
                </button>
              </nav>
            </div>
          </aside>

          {/* Task List Area */}
          <main className="flex-1 flex flex-col overflow-hidden">
            {/* Header */}
            <div className="flex items-center justify-between px-4 py-4 border-b border-slate-200 dark:border-slate-800 bg-white dark:bg-slate-900">
              <div>
                <h2 className="text-xl font-bold text-slate-900 dark:text-slate-100">
                  Tasks
                </h2>
                <p className="text-xs text-slate-500 dark:text-slate-400 mt-0.5">
                  {tasks.length} {tasks.length === 1 ? 'task' : 'tasks'}
                </p>
              </div>

              <Button size="sm" onClick={() => setIsCreateModalOpen(true)}>
                <Plus className="w-4 h-4" />
                <span className="hidden sm:inline ml-1.5">New Task</span>
              </Button>
            </div>

            {/* Task List */}
            <div className="flex-1 overflow-auto bg-slate-50 dark:bg-slate-950">
              {isLoading ? (
                <div className="flex items-center justify-center h-full">
                  <div className="flex flex-col items-center gap-3">
                    <div className="w-8 h-8 border-2 border-primary-600 border-t-transparent rounded-full animate-spin" />
                    <p className="text-sm text-slate-500 dark:text-slate-400">Loading tasks...</p>
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
            <div className="hidden xl:block">
              <TaskDetail
                task={selectedTask}
                onClose={() => setSelectedTask(null)}
              />
            </div>
          )}
        </div>
      </div>

      {/* Floating Action Button - Mobile only */}
      <div className="lg:hidden">
        <FloatingActionButton
          onClick={() => setIsCreateModalOpen(true)}
          label="New Task"
        />
      </div>

      {/* Create Task Modal */}
      <CreateTaskModal
        isOpen={isCreateModalOpen}
        onClose={() => setIsCreateModalOpen(false)}
      />

      {/* Mobile Task Detail Modal */}
      {selectedTask && (
        <div className="xl:hidden fixed inset-0 z-50 bg-black/50" onClick={() => setSelectedTask(null)}>
          <div className="absolute inset-y-0 right-0 w-full sm:w-96" onClick={(e) => e.stopPropagation()}>
            <TaskDetail
              task={selectedTask}
              onClose={() => setSelectedTask(null)}
            />
          </div>
        </div>
      )}
    </div>
  );
}
