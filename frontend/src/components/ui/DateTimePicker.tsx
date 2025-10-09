import { forwardRef } from 'react';
import { clsx } from 'clsx';
import { Calendar, Clock } from 'lucide-react';

interface DateTimePickerProps {
  label?: string;
  value?: string;
  onChange: (value: string) => void;
  type?: 'date' | 'datetime-local' | 'time';
  placeholder?: string;
  error?: string;
  minDate?: string;
  maxDate?: string;
  className?: string;
  disabled?: boolean;
}

export const DateTimePicker = forwardRef<HTMLInputElement, DateTimePickerProps>(
  ({
    label,
    value,
    onChange,
    type = 'date',
    placeholder,
    error,
    minDate,
    maxDate,
    className,
    disabled
  }, ref) => {
    const Icon = type === 'time' ? Clock : Calendar;

    return (
      <div className={clsx('w-full', className)}>
        {label && (
          <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">
            {label}
          </label>
        )}
        <div className="relative">
          <div className="absolute left-3 top-1/2 -translate-y-1/2 text-gray-400 dark:text-gray-500 pointer-events-none">
            <Icon className="w-4 h-4" />
          </div>
          <input
            ref={ref}
            type={type}
            value={value || ''}
            onChange={(e) => onChange(e.target.value)}
            min={minDate}
            max={maxDate}
            placeholder={placeholder}
            disabled={disabled}
            className={clsx(
              'w-full pl-10 pr-3 py-2.5 border rounded-lg transition-all',
              'focus:outline-none focus:ring-2 focus:ring-primary-500 focus:border-transparent',
              'disabled:opacity-50 disabled:cursor-not-allowed',
              error
                ? 'border-red-500 focus:ring-red-500'
                : 'border-gray-300 dark:border-gray-600',
              'bg-white dark:bg-gray-800 text-gray-900 dark:text-gray-100',
              'placeholder:text-gray-400 dark:placeholder:text-gray-500',
              '[&::-webkit-calendar-picker-indicator]:cursor-pointer',
              '[&::-webkit-calendar-picker-indicator]:opacity-70',
              '[&::-webkit-calendar-picker-indicator]:hover:opacity-100',
              '[&::-webkit-calendar-picker-indicator]:dark:invert'
            )}
          />
        </div>
        {error && (
          <p className="mt-1.5 text-sm text-red-600 dark:text-red-400">{error}</p>
        )}
      </div>
    );
  }
);

DateTimePicker.displayName = 'DateTimePicker';
