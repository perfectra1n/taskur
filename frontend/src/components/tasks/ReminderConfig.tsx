import { useState } from 'react';
import { clsx } from 'clsx';
import { Plus, Trash2, Bell, Mail } from 'lucide-react';
import type { Reminder } from '../../types';
import { DateTimePicker } from '../ui/DateTimePicker';
import { Button } from '../ui/Button';

interface ReminderConfigProps {
  reminders: Reminder[];
  onChange: (reminders: Reminder[]) => void;
  className?: string;
}

export function ReminderConfig({ reminders, onChange, className }: ReminderConfigProps) {
  const [isExpanded, setIsExpanded] = useState(reminders.length > 0);

  const addReminder = () => {
    const newReminder: Reminder = {
      id: crypto.randomUUID(),
      datetime: '',
      type: 'notification',
      message: ''
    };
    onChange([...reminders, newReminder]);
    setIsExpanded(true);
  };

  const removeReminder = (id: string) => {
    onChange(reminders.filter(r => r.id !== id));
  };

  const updateReminder = (id: string, updates: Partial<Reminder>) => {
    onChange(reminders.map(r => r.id === id ? { ...r, ...updates } : r));
  };

  return (
    <div className={clsx('space-y-3', className)}>
      <div className="flex items-center justify-between">
        <button
          type="button"
          onClick={() => setIsExpanded(!isExpanded)}
          className="flex items-center gap-2 text-sm font-medium text-gray-700 dark:text-gray-300 hover:text-gray-900 dark:hover:text-gray-100 transition-colors"
        >
          <Bell className="w-4 h-4" />
          <span>Reminders</span>
          {reminders.length > 0 && (
            <span className="px-2 py-0.5 bg-primary-100 dark:bg-primary-900/30 text-primary-700 dark:text-primary-300 rounded-full text-xs font-medium">
              {reminders.length}
            </span>
          )}
        </button>
        <button
          type="button"
          onClick={addReminder}
          className="text-primary-600 dark:text-primary-400 hover:text-primary-700 dark:hover:text-primary-300 transition-colors"
        >
          <Plus className="w-5 h-5" />
        </button>
      </div>

      {isExpanded && reminders.length > 0 && (
        <div className="space-y-3 pl-1">
          {reminders.map((reminder) => (
            <div
              key={reminder.id}
              className="p-3 bg-gray-50 dark:bg-gray-800/50 rounded-lg border border-gray-200 dark:border-gray-700 space-y-3"
            >
              <div className="flex items-start gap-3">
                <div className="flex-1 space-y-3">
                  {/* Date and time */}
                  <DateTimePicker
                    type="datetime-local"
                    value={reminder.datetime}
                    onChange={(value) => updateReminder(reminder.id, { datetime: value })}
                    placeholder="Select date and time"
                  />

                  {/* Type selector */}
                  <div className="flex gap-2">
                    <button
                      type="button"
                      onClick={() => updateReminder(reminder.id, { type: 'notification' })}
                      className={clsx(
                        'flex-1 flex items-center justify-center gap-2 px-3 py-2 rounded-lg border transition-all',
                        reminder.type === 'notification'
                          ? 'bg-primary-50 dark:bg-primary-900/20 border-primary-500 text-primary-700 dark:text-primary-300'
                          : 'bg-white dark:bg-gray-800 border-gray-300 dark:border-gray-600 text-gray-700 dark:text-gray-300 hover:border-primary-400'
                      )}
                    >
                      <Bell className="w-4 h-4" />
                      <span className="text-sm font-medium">Notification</span>
                    </button>
                    <button
                      type="button"
                      onClick={() => updateReminder(reminder.id, { type: 'email' })}
                      className={clsx(
                        'flex-1 flex items-center justify-center gap-2 px-3 py-2 rounded-lg border transition-all',
                        reminder.type === 'email'
                          ? 'bg-primary-50 dark:bg-primary-900/20 border-primary-500 text-primary-700 dark:text-primary-300'
                          : 'bg-white dark:bg-gray-800 border-gray-300 dark:border-gray-600 text-gray-700 dark:text-gray-300 hover:border-primary-400'
                      )}
                    >
                      <Mail className="w-4 h-4" />
                      <span className="text-sm font-medium">Email</span>
                    </button>
                  </div>

                  {/* Optional message */}
                  <input
                    type="text"
                    value={reminder.message || ''}
                    onChange={(e) => updateReminder(reminder.id, { message: e.target.value })}
                    placeholder="Custom message (optional)"
                    className="w-full px-3 py-2 text-sm border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-800 text-gray-900 dark:text-gray-100 placeholder:text-gray-400 focus:outline-none focus:ring-2 focus:ring-primary-500"
                  />
                </div>

                <button
                  type="button"
                  onClick={() => removeReminder(reminder.id)}
                  className="p-2 text-gray-400 hover:text-red-600 dark:hover:text-red-400 transition-colors"
                >
                  <Trash2 className="w-4 h-4" />
                </button>
              </div>
            </div>
          ))}
        </div>
      )}

      {isExpanded && reminders.length === 0 && (
        <div className="text-center py-6 text-sm text-gray-500 dark:text-gray-400">
          No reminders set. Click the + button to add one.
        </div>
      )}
    </div>
  );
}
