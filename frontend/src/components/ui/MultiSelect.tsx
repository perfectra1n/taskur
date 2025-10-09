import { useState, useRef, useEffect } from 'react';
import { clsx } from 'clsx';
import { X, ChevronDown, Check } from 'lucide-react';

export interface MultiSelectOption {
  value: string;
  label: string;
  avatar?: string;
  subLabel?: string;
}

interface MultiSelectProps {
  label?: React.ReactNode;
  options: MultiSelectOption[];
  value: string[];
  onChange: (value: string[]) => void;
  placeholder?: string;
  error?: string;
  className?: string;
  disabled?: boolean;
}

export function MultiSelect({
  label,
  options,
  value,
  onChange,
  placeholder = 'Select...',
  error,
  className,
  disabled
}: MultiSelectProps) {
  const [isOpen, setIsOpen] = useState(false);
  const [searchQuery, setSearchQuery] = useState('');
  const containerRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    const handleClickOutside = (event: MouseEvent) => {
      if (containerRef.current && !containerRef.current.contains(event.target as Node)) {
        setIsOpen(false);
        setSearchQuery('');
      }
    };

    document.addEventListener('mousedown', handleClickOutside);
    return () => document.removeEventListener('mousedown', handleClickOutside);
  }, []);

  const selectedOptions = options.filter(opt => value.includes(opt.value));
  const filteredOptions = options.filter(opt =>
    opt.label.toLowerCase().includes(searchQuery.toLowerCase()) ||
    opt.subLabel?.toLowerCase().includes(searchQuery.toLowerCase())
  );

  const toggleOption = (optionValue: string) => {
    if (value.includes(optionValue)) {
      onChange(value.filter(v => v !== optionValue));
    } else {
      onChange([...value, optionValue]);
    }
  };

  const removeOption = (optionValue: string, e: React.MouseEvent) => {
    e.stopPropagation();
    onChange(value.filter(v => v !== optionValue));
  };

  return (
    <div ref={containerRef} className={clsx('relative w-full', className)}>
      {label && (
        <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">
          {label}
        </label>
      )}

      {/* Selected values display */}
      <div
        onClick={() => !disabled && setIsOpen(!isOpen)}
        className={clsx(
          'min-h-[42px] w-full px-3 py-2 border rounded-lg transition-all cursor-pointer',
          'focus-within:ring-2 focus-within:ring-primary-500 focus-within:border-transparent',
          isOpen && 'ring-2 ring-primary-500 border-transparent',
          error
            ? 'border-red-500'
            : 'border-gray-300 dark:border-gray-600',
          'bg-white dark:bg-gray-800',
          disabled && 'opacity-50 cursor-not-allowed'
        )}
      >
        <div className="flex items-center justify-between gap-2">
          <div className="flex flex-wrap items-center gap-1.5 flex-1 min-w-0">
            {selectedOptions.length === 0 ? (
              <span className="text-gray-400 dark:text-gray-500 text-sm">
                {placeholder}
              </span>
            ) : (
              selectedOptions.map(option => (
                <div
                  key={option.value}
                  className="inline-flex items-center gap-1.5 px-2 py-1 bg-primary-100 dark:bg-primary-900/30 text-primary-800 dark:text-primary-200 rounded-md text-sm"
                >
                  {option.avatar && (
                    <img
                      src={option.avatar}
                      alt={option.label}
                      className="w-4 h-4 rounded-full object-cover"
                    />
                  )}
                  <span className="truncate max-w-[150px]">{option.label}</span>
                  <button
                    onClick={(e) => removeOption(option.value, e)}
                    className="hover:text-primary-900 dark:hover:text-primary-100 transition-colors"
                    type="button"
                  >
                    <X className="w-3.5 h-3.5" />
                  </button>
                </div>
              ))
            )}
          </div>
          <ChevronDown
            className={clsx(
              'w-4 h-4 text-gray-400 dark:text-gray-500 transition-transform flex-shrink-0',
              isOpen && 'rotate-180'
            )}
          />
        </div>
      </div>

      {/* Dropdown */}
      {isOpen && (
        <div className="absolute z-50 mt-2 w-full bg-white dark:bg-gray-800 border border-gray-200 dark:border-gray-700 rounded-lg shadow-lg max-h-[300px] flex flex-col">
          {/* Search input */}
          <div className="p-2 border-b border-gray-200 dark:border-gray-700">
            <input
              type="text"
              value={searchQuery}
              onChange={(e) => setSearchQuery(e.target.value)}
              placeholder="Search..."
              className="w-full px-3 py-2 text-sm border border-gray-300 dark:border-gray-600 rounded-md bg-white dark:bg-gray-800 text-gray-900 dark:text-gray-100 placeholder:text-gray-400 focus:outline-none focus:ring-2 focus:ring-primary-500"
              onClick={(e) => e.stopPropagation()}
            />
          </div>

          {/* Options list */}
          <div className="overflow-y-auto custom-scrollbar">
            {filteredOptions.length === 0 ? (
              <div className="px-3 py-6 text-center text-sm text-gray-500 dark:text-gray-400">
                No options found
              </div>
            ) : (
              filteredOptions.map(option => {
                const isSelected = value.includes(option.value);
                return (
                  <div
                    key={option.value}
                    onClick={() => toggleOption(option.value)}
                    className={clsx(
                      'flex items-center gap-3 px-3 py-2.5 cursor-pointer transition-colors',
                      'hover:bg-gray-100 dark:hover:bg-gray-700',
                      isSelected && 'bg-primary-50 dark:bg-primary-900/20'
                    )}
                  >
                    <div className="flex items-center gap-3 flex-1 min-w-0">
                      {option.avatar && (
                        <img
                          src={option.avatar}
                          alt={option.label}
                          className="w-8 h-8 rounded-full object-cover flex-shrink-0"
                        />
                      )}
                      <div className="flex-1 min-w-0">
                        <div className="text-sm font-medium text-gray-900 dark:text-gray-100 truncate">
                          {option.label}
                        </div>
                        {option.subLabel && (
                          <div className="text-xs text-gray-500 dark:text-gray-400 truncate">
                            {option.subLabel}
                          </div>
                        )}
                      </div>
                    </div>
                    {isSelected && (
                      <Check className="w-5 h-5 text-primary-600 dark:text-primary-400 flex-shrink-0" />
                    )}
                  </div>
                );
              })
            )}
          </div>
        </div>
      )}

      {error && (
        <p className="mt-1.5 text-sm text-red-600 dark:text-red-400">{error}</p>
      )}
    </div>
  );
}
