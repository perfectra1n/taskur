import { useState } from 'react';
import { useMutation, useQuery, useQueryClient } from '@tanstack/react-query';
import { clsx } from 'clsx';
import { User, Tag, Flag } from 'lucide-react';
import { api } from '../../services/api';
import type { CreateTaskRequest, TaskPriority, TaskStatus } from '../../types';
import { Modal } from '../ui/Modal';
import { Input } from '../ui/Input';
import { Button } from '../ui/Button';
import { DateTimePicker } from '../ui/DateTimePicker';
import { MultiSelect, type MultiSelectOption } from '../ui/MultiSelect';
import { useSuccessToast, useErrorToast } from '../ui/Toast';
import { TiptapEditor } from '../editor/TiptapEditor';
import { ReminderConfig } from './ReminderConfig';
import { HeroImageUpload } from './HeroImageUpload';

interface CreateTaskModalProps {
  isOpen: boolean;
  onClose: () => void;
}

export function CreateTaskModal({ isOpen, onClose }: CreateTaskModalProps) {
  const [formData, setFormData] = useState<CreateTaskRequest>({
    title: '',
    description: '',
    status: 'todo',
    priority: 'medium',
    due_date: '',
    start_date: '',
    end_date: '',
    hero_image_id: undefined,
    assigned_to: [],
    reminders: [],
    tags: [],
    list_ids: []
  });
  const [pendingHeroFile, setPendingHeroFile] = useState<File | null>(null);

  const queryClient = useQueryClient();
  const showSuccess = useSuccessToast();
  const showError = useErrorToast();

  const { data: teamMembers = [] } = useQuery({
    queryKey: ['team-members'],
    queryFn: () => api.getTeamMembers()
  });

  const createMutation = useMutation({
    mutationFn: (data: CreateTaskRequest) => api.createTask(data),
    onSuccess: async (newTask) => {
      // If there's a pending hero image file, upload it now
      if (pendingHeroFile) {
        try {
          const formData = new FormData();
          formData.append('file', pendingHeroFile);
          const dataTransfer = new DataTransfer();
          dataTransfer.items.add(pendingHeroFile);
          const attachments = await api.uploadAttachment(newTask.id, dataTransfer.files);
          if (attachments.length > 0) {
            await api.updateTask(newTask.id, { hero_image_id: attachments[0].id });
          }
        } catch {
          showError('Task created but hero image upload failed');
        }
      }
      queryClient.invalidateQueries({ queryKey: ['tasks'] });
      showSuccess('Task created');
      onClose();
      resetForm();
    },
    onError: () => {
      showError('Failed to create task');
    },
  });

  const resetForm = () => {
    setFormData({
      title: '',
      description: '',
      status: 'todo',
      priority: 'medium',
      due_date: '',
      start_date: '',
      end_date: '',
      hero_image_id: undefined,
      assigned_to: [],
      reminders: [],
      tags: [],
      list_ids: []
    });
    setPendingHeroFile(null);
  };

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    if (!formData.title.trim()) return;

    // Clean up empty fields and convert dates to ISO strings
    const cleanData: CreateTaskRequest = {
      ...formData,
      description: formData.description || undefined,
      due_date: formData.due_date ? new Date(formData.due_date).toISOString() : undefined,
      start_date: formData.start_date ? new Date(formData.start_date).toISOString() : undefined,
      end_date: formData.end_date ? new Date(formData.end_date).toISOString() : undefined,
      hero_image_id: undefined, // handled after creation
      assigned_to: formData.assigned_to?.length ? formData.assigned_to : undefined,
      reminders: formData.reminders?.length ? formData.reminders : undefined,
      tags: formData.tags?.length ? formData.tags : undefined
    };

    createMutation.mutate(cleanData);
  };

  const teamMemberOptions: MultiSelectOption[] = teamMembers.map(member => ({
    value: member.id,
    label: member.name,
    subLabel: member.email || member.role || undefined,
    avatar: member.avatar_url || undefined
  }));

  const priorities: { value: TaskPriority; label: string; color: string }[] = [
    { value: 'low', label: 'Low', color: 'text-blue-600 dark:text-blue-400' },
    { value: 'medium', label: 'Medium', color: 'text-yellow-600 dark:text-yellow-400' },
    { value: 'high', label: 'High', color: 'text-orange-600 dark:text-orange-400' },
    { value: 'urgent', label: 'Urgent', color: 'text-red-600 dark:text-red-400' }
  ];

  return (
    <Modal
      isOpen={isOpen}
      onClose={onClose}
      title="Create New Task"
      size="lg"
    >
      <form onSubmit={handleSubmit} className="flex flex-col h-full">
        <div className="flex-1 p-6 space-y-6">
          {/* Hero Image Section */}
          <div>
            <HeroImageUpload
              currentImageId={formData.hero_image_id}
              onImageUploaded={(id) => setFormData({ ...formData, hero_image_id: id })}
              onImageRemoved={() => { setFormData({ ...formData, hero_image_id: undefined }); setPendingHeroFile(null); }}
              onFileSelected={(file) => setPendingHeroFile(file)}
            />
          </div>

          {/* Title */}
          <div>
            <Input
              label="Task Title"
              value={formData.title}
              onChange={(e) => setFormData({ ...formData, title: e.target.value })}
              placeholder="Enter task title..."
              autoFocus
              className="text-lg font-medium"
            />
          </div>

          {/* Quick Actions Row */}
          <div className="grid grid-cols-2 gap-3">
            {/* Priority */}
            <div>
              <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">
                <div className="flex items-center gap-2">
                  <Flag className="w-4 h-4" />
                  Priority
                </div>
              </label>
              <div className="grid grid-cols-2 gap-2">
                {priorities.map((priority) => (
                  <button
                    key={priority.value}
                    type="button"
                    onClick={() => setFormData({ ...formData, priority: priority.value })}
                    className={clsx(
                      'px-3 py-2 text-sm font-medium rounded-lg border transition-all',
                      formData.priority === priority.value
                        ? 'bg-primary-50 dark:bg-primary-900/20 border-primary-500 text-primary-700 dark:text-primary-300'
                        : 'bg-white dark:bg-gray-800 border-gray-300 dark:border-gray-600 text-gray-700 dark:text-gray-300 hover:border-primary-400'
                    )}
                  >
                    {priority.label}
                  </button>
                ))}
              </div>
            </div>

            {/* Status */}
            <div>
              <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">
                Status
              </label>
              <select
                value={formData.status}
                onChange={(e) => setFormData({ ...formData, status: e.target.value as TaskStatus })}
                className="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-800 text-gray-900 dark:text-gray-100 focus:outline-none focus:ring-2 focus:ring-primary-500"
              >
                <option value="todo">To Do</option>
                <option value="inprogress">In Progress</option>
                <option value="completed">Completed</option>
              </select>
            </div>
          </div>

          {/* Date Fields */}
          <div className="grid grid-cols-3 gap-3">
            <DateTimePicker
              label="Start Date"
              value={formData.start_date}
              onChange={(value) => setFormData({ ...formData, start_date: value })}
              type="date"
            />
            <DateTimePicker
              label="Due Date"
              value={formData.due_date}
              onChange={(value) => setFormData({ ...formData, due_date: value })}
              type="date"
              minDate={formData.start_date}
            />
            <DateTimePicker
              label="End Date"
              value={formData.end_date}
              onChange={(value) => setFormData({ ...formData, end_date: value })}
              type="date"
              minDate={formData.due_date || formData.start_date}
            />
          </div>

          {/* Team Assignment */}
          {teamMembers.length > 0 && (
            <div>
              <MultiSelect
                label={
                  <div className="flex items-center gap-2">
                    <User className="w-4 h-4" />
                    <span>Assign To</span>
                  </div>
                }
                options={teamMemberOptions}
                value={formData.assigned_to || []}
                onChange={(value) => setFormData({ ...formData, assigned_to: value })}
                placeholder="Select team members..."
              />
            </div>
          )}

          {/* Description */}
          <div>
            <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">
              Description
            </label>
            <div className="border border-gray-300 dark:border-gray-600 rounded-lg overflow-hidden">
              <TiptapEditor
                content={formData.description || ''}
                onChange={(value) => setFormData({ ...formData, description: value })}
                placeholder="Add a detailed description..."
              />
            </div>
          </div>

          {/* Reminders */}
          <div>
            <ReminderConfig
              reminders={formData.reminders || []}
              onChange={(reminders) => setFormData({ ...formData, reminders })}
            />
          </div>

          {/* Tags */}
          <div>
            <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">
              <div className="flex items-center gap-2">
                <Tag className="w-4 h-4" />
                Tags
              </div>
            </label>
            <Input
              value={(formData.tags || []).join(', ')}
              onChange={(e) => {
                const tags = e.target.value.split(',').map(t => t.trim()).filter(Boolean);
                setFormData({ ...formData, tags });
              }}
              placeholder="Enter tags separated by commas..."
            />
            {formData.tags && formData.tags.length > 0 && (
              <div className="flex flex-wrap gap-2 mt-2">
                {formData.tags.map((tag, index) => (
                  <span
                    key={index}
                    className="px-2 py-1 bg-primary-100 dark:bg-primary-900/30 text-primary-800 dark:text-primary-200 rounded-md text-sm"
                  >
                    {tag}
                  </span>
                ))}
              </div>
            )}
          </div>
        </div>

        {/* Footer */}
        <div className="px-6 py-4 border-t border-gray-200 dark:border-gray-700 flex items-center justify-between bg-gray-50 dark:bg-gray-800/50">
          <div className="text-sm text-gray-500 dark:text-gray-400">
            All fields except title are optional
          </div>
          <div className="flex gap-3">
            <Button
              type="button"
              variant="secondary"
              onClick={onClose}
            >
              Cancel
            </Button>
            <Button
              type="submit"
              isLoading={createMutation.isPending}
              disabled={!formData.title.trim()}
            >
              Create Task
            </Button>
          </div>
        </div>
      </form>
    </Modal>
  );
}
