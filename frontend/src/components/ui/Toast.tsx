import { createContext, useContext, useState, useCallback, ReactNode } from 'react';
import { createPortal } from 'react-dom';
import { CheckCircle2, XCircle, AlertCircle, Info, X } from 'lucide-react';

type ToastType = 'success' | 'error' | 'warning' | 'info';

interface Toast {
  id: string;
  type: ToastType;
  title: string;
  message?: string;
  duration?: number;
}

interface ToastContextValue {
  showToast: (toast: Omit<Toast, 'id'>) => void;
  hideToast: (id: string) => void;
}

const ToastContext = createContext<ToastContextValue | undefined>(undefined);

export function useToast() {
  const context = useContext(ToastContext);
  if (!context) {
    throw new Error('useToast must be used within ToastProvider');
  }
  return context;
}

export function ToastProvider({ children }: { children: ReactNode }) {
  const [toasts, setToasts] = useState<Toast[]>([]);

  const showToast = useCallback((toast: Omit<Toast, 'id'>) => {
    const id = Math.random().toString(36).substring(2, 9);
    const newToast: Toast = {
      ...toast,
      id,
      duration: toast.duration ?? 5000
    };

    setToasts((prev) => [...prev, newToast]);

    // Auto dismiss
    if (newToast.duration) {
      setTimeout(() => {
        hideToast(id);
      }, newToast.duration);
    }
  }, []);

  const hideToast = useCallback((id: string) => {
    setToasts((prev) => prev.filter((toast) => toast.id !== id));
  }, []);

  return (
    <ToastContext.Provider value={{ showToast, hideToast }}>
      {children}
      {createPortal(<ToastContainer toasts={toasts} onDismiss={hideToast} />, document.body)}
    </ToastContext.Provider>
  );
}

function ToastContainer({
  toasts,
  onDismiss
}: {
  toasts: Toast[];
  onDismiss: (id: string) => void;
}) {
  return (
    <div className="fixed top-0 right-0 z-50 p-6 flex flex-col gap-3 pointer-events-none max-w-md">
      {toasts.map((toast) => (
        <ToastItem key={toast.id} toast={toast} onDismiss={onDismiss} />
      ))}
    </div>
  );
}

function ToastItem({ toast, onDismiss }: { toast: Toast; onDismiss: (id: string) => void }) {
  const icons = {
    success: CheckCircle2,
    error: XCircle,
    warning: AlertCircle,
    info: Info
  };

  const Icon = icons[toast.type];

  const colorClasses = {
    success: {
      bg: 'bg-emerald-50/95 dark:bg-emerald-900/30',
      border: 'border-emerald-200/60 dark:border-emerald-800/60',
      icon: 'text-emerald-600 dark:text-emerald-400',
      text: 'text-emerald-900 dark:text-emerald-100',
      glow: 'shadow-glow-cyan'
    },
    error: {
      bg: 'bg-red-50/95 dark:bg-red-900/30',
      border: 'border-red-200/60 dark:border-red-800/60',
      icon: 'text-red-600 dark:text-red-400',
      text: 'text-red-900 dark:text-red-100',
      glow: 'shadow-glow-pink'
    },
    warning: {
      bg: 'bg-amber-50/95 dark:bg-amber-900/30',
      border: 'border-amber-200/60 dark:border-amber-800/60',
      icon: 'text-amber-600 dark:text-amber-400',
      text: 'text-amber-900 dark:text-amber-100',
      glow: 'shadow-[0_0_30px_rgba(245,158,11,0.5)]'
    },
    info: {
      bg: 'bg-blue-50/95 dark:bg-blue-900/30',
      border: 'border-blue-200/60 dark:border-blue-800/60',
      icon: 'text-blue-600 dark:text-blue-400',
      text: 'text-blue-900 dark:text-blue-100',
      glow: 'shadow-glow-secondary'
    }
  };

  const colors = colorClasses[toast.type];

  return (
    <div
      className={`pointer-events-auto backdrop-blur-xl border rounded-2xl shadow-elevation-3 hover:${colors.glow} transition-all duration-300 animate-slide-left ${colors.bg} ${colors.border}`}
    >
      <div className="p-4 flex items-start gap-3">
        {/* Icon */}
        <div className={`flex-shrink-0 ${colors.icon}`}>
          <Icon className="w-5 h-5" strokeWidth={2} />
        </div>

        {/* Content */}
        <div className="flex-1 min-w-0">
          <h4 className={`font-semibold text-sm ${colors.text}`}>
            {toast.title}
          </h4>
          {toast.message && (
            <p className={`mt-1 text-sm opacity-90 ${colors.text}`}>
              {toast.message}
            </p>
          )}
        </div>

        {/* Close button */}
        <button
          onClick={() => onDismiss(toast.id)}
          className={`flex-shrink-0 p-1 rounded-lg hover:bg-black/10 dark:hover:bg-white/10 transition-colors ${colors.icon}`}
          aria-label="Dismiss notification"
        >
          <X className="w-4 h-4" />
        </button>
      </div>

      {/* Progress bar */}
      {toast.duration && (
        <div className="h-1 bg-black/10 dark:bg-white/10 rounded-b-2xl overflow-hidden">
          <div
            className={`h-full ${colors.icon} opacity-50 animate-shrink-width`}
            style={{
              animation: `shrinkWidth ${toast.duration}ms linear`
            }}
          />
        </div>
      )}

      <style>{`
        @keyframes shrinkWidth {
          from { width: 100%; }
          to { width: 0%; }
        }
        .animate-shrink-width {
          animation: shrinkWidth linear;
        }
      `}</style>
    </div>
  );
}

// Helper hooks for common toast types
export function useSuccessToast() {
  const { showToast } = useToast();
  return useCallback(
    (title: string, message?: string) => {
      showToast({ type: 'success', title, message });
    },
    [showToast]
  );
}

export function useErrorToast() {
  const { showToast } = useToast();
  return useCallback(
    (title: string, message?: string) => {
      showToast({ type: 'error', title, message });
    },
    [showToast]
  );
}

export function useWarningToast() {
  const { showToast } = useToast();
  return useCallback(
    (title: string, message?: string) => {
      showToast({ type: 'warning', title, message });
    },
    [showToast]
  );
}

export function useInfoToast() {
  const { showToast } = useToast();
  return useCallback(
    (title: string, message?: string) => {
      showToast({ type: 'info', title, message });
    },
    [showToast]
  );
}
