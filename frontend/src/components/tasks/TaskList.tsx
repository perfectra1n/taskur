import type { Task } from '../../types';
import { TaskItem } from './TaskItem';

interface TaskListProps {
  tasks: Task[];
  onTaskClick: (task: Task) => void;
  selectedTaskId?: string;
}

export function TaskList({ tasks, onTaskClick, selectedTaskId }: TaskListProps) {
  if (tasks.length === 0) {
    return (
      <div className="flex flex-col items-center justify-center h-64 text-gray-500 dark:text-gray-400">
        <p className="text-lg">No tasks yet</p>
        <p className="text-sm">Create your first task to get started</p>
      </div>
    );
  }

  return (
    <div className="space-y-2">
      {tasks.map((task) => (
        <TaskItem
          key={task.id}
          task={task}
          onClick={() => onTaskClick(task)}
          isSelected={task.id === selectedTaskId}
        />
      ))}
    </div>
  );
}
