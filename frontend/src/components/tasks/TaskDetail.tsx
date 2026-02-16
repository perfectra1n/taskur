import { useState } from 'react';
import { useMutation, useQueryClient, useQuery } from '@tanstack/react-query';
import { api } from '../../services/api';
import type { Task, TaskStatus, TaskPriority, Reminder } from '../../types';
import { Button } from '../ui/Button';
import { useSuccessToast, useErrorToast } from '../ui/Toast';
import { Input } from '../ui/Input';
import { DateTimePicker } from '../ui/DateTimePicker';
import { MultiSelect, type MultiSelectOption } from '../ui/MultiSelect';
import { X, AlertCircle, Trash2, MessageSquare, User, Image as ImageIcon } from 'lucide-react';
import { TiptapEditor } from '../editor/TiptapEditor';
import { ReminderConfig } from './ReminderConfig';
import { HeroImageUpload } from './HeroImageUpload';
import { CommentSection } from './CommentSection';
import { format } from 'date-fns';

interface TaskDetailProps {
  task: Task;
  onClose: () => void;
}

export function TaskDetail({ task, onClose }: TaskDetailProps) {
  const [title, setTitle] = useState(task.title);
  const [description, setDescription] = useState(task.description ?? '');
  const [status, setStatus] = useState<TaskStatus>(task.status);
  const [priority, setPriority] = useState<TaskPriority>(task.priority);
  const [dueDate, setDueDate] = useState(
    task.due_date ? format(new Date(task.due_date), 'yyyy-MM-dd') : ''
  );
  const [startDate, setStartDate] = useState(
    task.start_date ? format(new Date(task.start_date), 'yyyy-MM-dd') : ''
  );
  const [endDate, setEndDate] = useState(
    task.end_date ? format(new Date(task.end_date), 'yyyy-MM-dd') : ''
  );
  const [heroImageId, setHeroImageId] = useState(task.hero_image_id ?? undefined);
  const [assignedTo, setAssignedTo] = useState<string[]>(task.assigned_to || []);
  const [reminders, setReminders] = useState<Reminder[]>(task.reminders || []);

  const queryClient = useQueryClient();
  const showSuccess = useSuccessToast();
  const showError = useErrorToast();

  const { data: teamMembers = [] } = useQuery({
    queryKey: ['team-members'],
    queryFn: () => api.getTeamMembers()
  });

  const updateMutation = useMutation({
    mutationFn: () =>
      api.updateTask(task.id, {
        title,
        description: description || null,
        status,
        priority,
        due_date: dueDate ? new Date(dueDate).toISOString() : null,
        start_date: startDate ? new Date(startDate).toISOString() : null,
        end_date: endDate ? new Date(endDate).toISOString() : null,
        hero_image_id: heroImageId ?? null,
        assigned_to: assignedTo.length > 0 ? assignedTo : undefined,
        reminders: reminders.length > 0 ? reminders : undefined,
      }),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['tasks'] });
      showSuccess('Task saved');
    },
    onError: () => {
      showError('Failed to save task');
    },
  });

  const deleteMutation = useMutation({
    mutationFn: () => api.deleteTask(task.id),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['tasks'] });
      showSuccess('Task deleted');
      onClose();
    },
    onError: () => {
      showError('Failed to delete task');
    },
  });

  const { data: comments = [] } = useQuery({
    queryKey: ['comments', task.id],
    queryFn: () => api.getComments(task.id),
  });

  const { data: attachments = [] } = useQuery({
    queryKey: ['attachments', task.id],
    queryFn: () => api.getAttachments(task.id),
  });

  const handleSave = () => {
    updateMutation.mutate();
  };

  const handleDelete = () => {
    if (confirm('Are you sure you want to delete this task?')) {
      deleteMutation.mutate();
    }
  };

  const teamMemberOptions: MultiSelectOption[] = teamMembers.map(member => ({
    value: member.id,
    label: member.name,
    subLabel: member.email || member.role || undefined,
    avatar: member.avatar_url || undefined
  }));

  const heroImageUrl = heroImageId
    ? attachments.find(a => a.id === heroImageId)?.download_url
    : null;

  return (
    <div className="w-[700px] bg-white dark:bg-gray-800 border-l border-gray-200 dark:border-gray-700 flex flex-col h-full">
      {/* Header */}
      <div className="relative">
        <button
          onClick={onClose}
          className="absolute top-4 right-4 z-10 p-2 bg-white/90 dark:bg-gray-800/90 rounded-full shadow-lg text-gray-700 dark:text-gray-300 hover:bg-white dark:hover:bg-gray-800 transition-colors"
        >
          <X className="w-5 h-5" />
        </button>

        {/* Hero Image */}
        {heroImageUrl && (
          <div className="w-full h-64 overflow-hidden bg-gray-100 dark:bg-gray-800">
            <img
              src={heroImageUrl}
              alt="Task hero"
              className="w-full h-full object-cover"
            />
          </div>
        )}

        <div className="p-6 border-b border-gray-200 dark:border-gray-700">
          <Input
            value={title}
            onChange={(e) => setTitle(e.target.value)}
            className="text-xl font-semibold border-none shadow-none focus:ring-0 px-0"
            placeholder="Task title"
          />
        </div>
      </div>

      {/* Content */}
      <div className="flex-1 overflow-y-auto p-6 space-y-6 custom-scrollbar">
        {/* Status */}
        <div>
          <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">
            Status
          </label>
          <select
            value={status}
            onChange={(e) => setStatus(e.target.value as TaskStatus)}
            className="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-700 text-gray-900 dark:text-white focus:outline-none focus:ring-2 focus:ring-primary-500"
          >
            <option value="todo">To Do</option>
            <option value="inprogress">In Progress</option>
            <option value="completed">Completed</option>
          </select>
        </div>

        {/* Priority */}
        <div>
          <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2 flex items-center gap-2">
            <AlertCircle className="w-4 h-4" />
            Priority
          </label>
          <select
            value={priority}
            onChange={(e) => setPriority(e.target.value as TaskPriority)}
            className="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-700 text-gray-900 dark:text-white focus:outline-none focus:ring-2 focus:ring-primary-500"
          >
            <option value="low">Low</option>
            <option value="medium">Medium</option>
            <option value="high">High</option>
            <option value="urgent">Urgent</option>
          </select>
        </div>

        {/* Date Fields */}
        <div className="grid grid-cols-1 gap-4">
          <DateTimePicker
            label="Start Date"
            type="date"
            value={startDate}
            onChange={setStartDate}
          />
          <DateTimePicker
            label="Due Date"
            type="date"
            value={dueDate}
            onChange={setDueDate}
            minDate={startDate}
          />
          <DateTimePicker
            label="End Date"
            type="date"
            value={endDate}
            onChange={setEndDate}
            minDate={dueDate || startDate}
          />
        </div>

        {/* Team Assignment */}
        {teamMembers.length > 0 && (
          <div>
            <MultiSelect
              label={
                <div className="flex items-center gap-2">
                  <User className="w-4 h-4" />
                  <span>Assigned To</span>
                </div>
              }
              options={teamMemberOptions}
              value={assignedTo}
              onChange={setAssignedTo}
              placeholder="Select team members..."
            />
          </div>
        )}

        {/* Hero Image Upload */}
        <div>
          <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2 flex items-center gap-2">
            <ImageIcon className="w-4 h-4" />
            Hero Image
          </label>
          <HeroImageUpload
            taskId={task.id}
            currentImageId={heroImageId}
            currentImageUrl={heroImageUrl}
            onImageUploaded={(id) => setHeroImageId(id)}
            onImageRemoved={() => setHeroImageId(undefined)}
          />
        </div>

        {/* Description */}
        <div>
          <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">
            Description
          </label>
          <div className="border border-gray-300 dark:border-gray-600 rounded-lg overflow-hidden">
            <TiptapEditor
              content={description}
              onChange={setDescription}
              placeholder="Add a description..."
            />
          </div>
        </div>

        {/* Reminders */}
        <div>
          <ReminderConfig
            reminders={reminders}
            onChange={setReminders}
          />
        </div>

        {/* Comments */}
        <div>
          <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-3 flex items-center gap-2">
            <MessageSquare className="w-4 h-4" />
            Comments ({comments.length})
          </label>
          <CommentSection
            taskId={task.id}
            comments={comments}
            attachments={attachments}
          />
        </div>
      </div>

      {/* Footer */}
      <div className="p-6 border-t border-gray-200 dark:border-gray-700 flex items-center justify-between">
        <Button
          variant="danger"
          onClick={handleDelete}
          isLoading={deleteMutation.isPending}
        >
          <Trash2 className="w-4 h-4 mr-2" />
          Delete
        </Button>
        <div className="flex gap-2">
          <Button variant="secondary" onClick={onClose}>
            Cancel
          </Button>
          <Button onClick={handleSave} isLoading={updateMutation.isPending}>
            Save Changes
          </Button>
        </div>
      </div>
    </div>
  );
}
