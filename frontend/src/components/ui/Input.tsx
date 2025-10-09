import { forwardRef } from 'react';
import type { InputHTMLAttributes } from 'react';
import { clsx } from 'clsx';

interface InputProps extends InputHTMLAttributes<HTMLInputElement> {
  label?: string;
  error?: string;
  helperText?: string;
}

export const Input = forwardRef<HTMLInputElement, InputProps>(
  ({ className, label, error, helperText, ...props }, ref) => {
    return (
      <div className="w-full">
        {label && (
          <label className="block text-sm font-medium text-slate-700 dark:text-slate-300 mb-1.5">
            {label}
          </label>
        )}

        <input
          ref={ref}
          className={clsx(
            'w-full px-3 py-2 rounded-lg transition-all duration-200',
            'focus:outline-none focus:ring-2 focus:ring-offset-0',
            'disabled:opacity-50 disabled:cursor-not-allowed',
            'placeholder:text-slate-400 dark:placeholder:text-slate-500',
            error
              ? 'border-2 border-red-500 focus:border-red-500 focus:ring-red-500/30 bg-red-50 dark:bg-red-900/10 text-red-900 dark:text-red-100'
              : 'border border-slate-200 dark:border-slate-800 focus:border-primary-500 focus:ring-primary-500/20 bg-white dark:bg-slate-900 text-slate-900 dark:text-slate-100',
            className
          )}
          {...props}
        />

        {/* Helper text or error message */}
        {(helperText || error) && (
          <p className={clsx(
            'mt-1.5 text-xs',
            error
              ? 'text-red-600 dark:text-red-400'
              : 'text-slate-500 dark:text-slate-400'
          )}>
            {error || helperText}
          </p>
        )}
      </div>
    );
  }
);

Input.displayName = 'Input';
