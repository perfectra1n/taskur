import { useEffect, useRef } from 'react';
import { createPortal } from 'react-dom';
import { clsx } from 'clsx';
import { X } from 'lucide-react';

interface ModalProps {
  isOpen: boolean;
  onClose: () => void;
  title?: string;
  children: React.ReactNode;
  size?: 'sm' | 'md' | 'lg' | 'xl' | 'full';
  showCloseButton?: boolean;
  className?: string;
}

export function Modal({
  isOpen,
  onClose,
  title,
  children,
  size = 'md',
  showCloseButton = true,
  className
}: ModalProps) {
  const modalRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    if (!isOpen) return;

    const handleEscape = (e: KeyboardEvent) => {
      if (e.key === 'Escape') onClose();
    };

    const handleClickOutside = (e: MouseEvent) => {
      if (modalRef.current && !modalRef.current.contains(e.target as Node)) {
        onClose();
      }
    };

    document.addEventListener('keydown', handleEscape);
    document.addEventListener('mousedown', handleClickOutside);
    document.body.style.overflow = 'hidden';

    return () => {
      document.removeEventListener('keydown', handleEscape);
      document.removeEventListener('mousedown', handleClickOutside);
      document.body.style.overflow = 'unset';
    };
  }, [isOpen, onClose]);

  if (!isOpen) return null;

  const sizeClasses = {
    sm: 'max-w-md',
    md: 'max-w-2xl',
    lg: 'max-w-4xl',
    xl: 'max-w-6xl',
    full: 'max-w-full mx-4'
  };

  return createPortal(
    <div className="fixed inset-0 z-50 flex items-center justify-center p-4 animate-fade-in">
      {/* Backdrop with blur */}
      <div className="absolute inset-0 backdrop-overlay animate-fade-in" />

      {/* Modal */}
      <div
        ref={modalRef}
        className={clsx(
          'relative w-full glass-card-strong rounded-3xl shadow-elevation-4',
          'animate-slide-up',
          'flex flex-col max-h-[90vh]',
          sizeClasses[size],
          className
        )}
      >
        {/* Header */}
        {(title || showCloseButton) && (
          <div className="flex items-center justify-between px-8 py-6 border-b border-slate-200/60 dark:border-slate-800/60">
            {title && (
              <h2 className="text-2xl font-bold text-slate-900 dark:text-slate-100">
                {title}
              </h2>
            )}
            <div className="flex-1" />
            {showCloseButton && (
              <button
                onClick={onClose}
                className="p-2 rounded-xl text-slate-400 hover:text-slate-600 dark:hover:text-slate-300 transition-all hover:bg-slate-100/80 dark:hover:bg-slate-800/80 hover:rotate-90 focus:outline-none focus:ring-2 focus:ring-primary-500/30 group"
                aria-label="Close modal"
              >
                <X className="w-5 h-5 transition-transform duration-300" />
              </button>
            )}
          </div>
        )}

        {/* Content */}
        <div className="flex-1 overflow-y-auto custom-scrollbar">
          {children}
        </div>

        {/* Decorative gradient border */}
        <div className="absolute inset-0 rounded-3xl pointer-events-none">
          <div className="absolute inset-0 rounded-3xl border border-transparent bg-gradient-to-br from-primary-500/20 via-transparent to-secondary-500/20 bg-clip-border opacity-50" />
        </div>
      </div>
    </div>,
    document.body
  );
}
