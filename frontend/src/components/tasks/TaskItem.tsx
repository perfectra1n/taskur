import { useMutation, useQueryClient } from '@tanstack/react-query';
import { api } from '../../services/api';
import type { Task, TaskStatus } from '../../types';
import { Circle, CheckCircle2, Clock, Calendar, Tag } from 'lucide-react';
import { clsx } from 'clsx';
import { format } from 'date-fns';

interface TaskItemProps {
  task: Task;
  onClick: () => void;
  isSelected: boolean;
}

const statusConfig = {
  todo: {
    icon: Circle,
    label: 'To Do',
    className: 'status-badge-todo'
  },
  inprogress: {
    icon: Clock,
    label: 'In Progress',
    className: 'status-badge-inprogress'
  },
  completed: {
    icon: CheckCircle2,
    label: 'Completed',
    className: 'status-badge-completed'
  },
};

const priorityConfig = {
  low: {
    label: 'Low',
    className: 'priority-badge-low',
    dotColor: 'bg-slate-400'
  },
  medium: {
    label: 'Medium',
    className: 'priority-badge-medium',
    dotColor: 'bg-blue-500'
  },
  high: {
    label: 'High',
    className: 'priority-badge-high',
    dotColor: 'bg-orange-500'
  },
  urgent: {
    label: 'Urgent',
    className: 'priority-badge-urgent',
    dotColor: 'bg-red-500 animate-pulse'
  },
};

export function TaskItem({ task, onClick, isSelected }: TaskItemProps) {
  const queryClient = useQueryClient();
  const statusInfo = statusConfig[task.status];
  const StatusIcon = statusInfo.icon;
  const priorityInfo = priorityConfig[task.priority];

  const updateStatusMutation = useMutation({
    mutationFn: (status: TaskStatus) => api.updateTask(task.id, { status }),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['tasks'] });
    },
  });

  const handleStatusToggle = (e: React.MouseEvent) => {
    e.stopPropagation();
    const nextStatus: TaskStatus = task.status === 'completed' ? 'todo' : 'completed';
    updateStatusMutation.mutate(nextStatus);
  };

  const isOverdue = task.due_date && new Date(task.due_date) < new Date() && task.status !== 'completed';

  return (
    <div
      onClick={onClick}
      className={clsx(
        'group relative p-5 rounded-2xl cursor-pointer transition-all duration-300',
        'card-lift-subtle',
        isSelected
          ? 'glass-card-strong border-2 border-primary-500/60 shadow-glow-primary'
          : 'glass-card hover:border-primary-300/60 dark:hover:border-primary-700/60'
      )}
    >
      {/* Gradient accent bar on the left */}
      <div className={clsx(
        'absolute left-0 top-4 bottom-4 w-1 rounded-r-full transition-all duration-300',
        task.status === 'completed'
          ? 'bg-gradient-to-b from-emerald-500 to-emerald-600'
          : task.status === 'inprogress'
          ? 'bg-gradient-to-b from-blue-500 to-blue-600'
          : 'bg-gradient-to-b from-slate-300 to-slate-400 dark:from-slate-600 dark:to-slate-700',
        isSelected && 'w-1.5'
      )} />

      <div className="flex items-start gap-4 pl-2">
        {/* Status checkbox */}
        <button
          onClick={handleStatusToggle}
          className={clsx(
            'mt-1 flex-shrink-0 transition-all duration-300',
            'hover:scale-110 active:scale-95',
            'focus:outline-none focus:ring-2 focus:ring-primary-500/30 rounded-lg p-0.5',
            task.status === 'completed'
              ? 'text-emerald-500 dark:text-emerald-400'
              : 'text-slate-400 dark:text-slate-500 hover:text-primary-500 dark:hover:text-primary-400'
          )}
          aria-label={'Mark task as ' + (task.status === 'completed' ? 'incomplete' : 'complete')}
        >
          <StatusIcon className={clsx(
            'w-6 h-6 transition-transform duration-300',
            task.status === 'completed' && 'scale-110'
          )} strokeWidth={2} />
        </button>

        {/* Task content */}
        <div className="flex-1 min-w-0">
          {/* Title and priority */}
          <div className="flex items-start gap-3 mb-3">
            <h3
              className={clsx(
                'flex-1 font-semibold text-base transition-all duration-200',
                task.status === 'completed'
                  ? 'line-through text-slate-500 dark:text-slate-500'
                  : 'text-slate-900 dark:text-slate-100 group-hover:text-primary-600 dark:group-hover:text-primary-400'
              )}
            >
              {task.title}
            </h3>

            {/* Priority indicator */}
            <div className={clsx(
              'flex items-center gap-1.5 px-2 py-1 rounded-lg text-xs font-semibold',
              priorityInfo.className
            )}>
              <div className={clsx('w-1.5 h-1.5 rounded-full', priorityInfo.dotColor)} />
              <span>{priorityInfo.label}</span>
            </div>
          </div>

          {/* Status and metadata */}
          <div className="flex flex-wrap items-center gap-2">
            {/* Status badge */}
            <span className={clsx('status-badge', statusInfo.className)}>
              <StatusIcon className="w-3.5 h-3.5" strokeWidth={2.5} />
              <span>{statusInfo.label}</span>
            </span>

            {/* Due date */}
            {task.due_date && (
              <span className={clsx(
                'status-badge',
                isOverdue
                  ? 'bg-red-100/80 dark:bg-red-900/30 text-red-700 dark:text-red-300 border-red-200/60 dark:border-red-800/60 animate-pulse'
                  : 'bg-slate-100/80 dark:bg-slate-800/80 text-slate-600 dark:text-slate-400 border-slate-200/60 dark:border-slate-700/60'
              )}>
                <Calendar className="w-3.5 h-3.5" strokeWidth={2.5} />
                <span>{format(new Date(task.due_date), 'MMM d')}</span>
              </span>
            )}

            {/* Tags */}
            {task.tags.length > 0 && (
              <div className="flex gap-1.5">
                {task.tags.slice(0, 2).map((tag) => (
                  <span
                    key={tag}
                    className="status-badge bg-primary-100/80 dark:bg-primary-900/30 text-primary-700 dark:text-primary-300 border-primary-200/60 dark:border-primary-800/60"
                  >
                    <Tag className="w-3 h-3" strokeWidth={2.5} />
                    <span>{tag}</span>
                  </span>
                ))}
                {task.tags.length > 2 && (
                  <span className="status-badge bg-slate-100/80 dark:bg-slate-800/80 text-slate-600 dark:text-slate-400 border-slate-200/60 dark:border-slate-700/60">
                    +{task.tags.length - 2}
                  </span>
                )}
              </div>
            )}
          </div>
        </div>

        {/* Hover indicator */}
        <div className={clsx(
          'flex-shrink-0 opacity-0 group-hover:opacity-100 transition-opacity duration-300',
          'text-primary-500 dark:text-primary-400'
        )}>
          <svg className="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M9 5l7 7-7 7" />
          </svg>
        </div>
      </div>

      {/* Selection highlight glow */}
      {isSelected && (
        <div className="absolute inset-0 rounded-2xl pointer-events-none">
          <div className="absolute inset-0 rounded-2xl bg-gradient-to-br from-primary-500/10 via-transparent to-secondary-500/10" />
        </div>
      )}
    </div>
  );
}
