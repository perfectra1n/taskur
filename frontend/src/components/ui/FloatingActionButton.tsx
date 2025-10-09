import { Plus, X } from 'lucide-react';
import { useState } from 'react';

interface FloatingActionButtonProps {
  onClick: () => void;
  icon?: React.ReactNode;
  label?: string;
  position?: 'bottom-right' | 'bottom-left';
}

export function FloatingActionButton({
  onClick,
  icon,
  label = 'New Task',
  position = 'bottom-right'
}: FloatingActionButtonProps) {
  const [isHovered, setIsHovered] = useState(false);

  const positionClasses = {
    'bottom-right': 'bottom-8 right-8',
    'bottom-left': 'bottom-8 left-8'
  };

  return (
    <div className={`fixed ${positionClasses[position]} z-50 flex items-center gap-3`}>
      {/* Label tooltip - appears on hover */}
      <div
        className={`transition-all duration-300 ${
          isHovered ? 'opacity-100 translate-x-0' : 'opacity-0 translate-x-2'
        }`}
      >
        <div className="glass-card-strong px-4 py-2 rounded-xl shadow-elevation-3 backdrop-blur-xl">
          <span className="text-sm font-semibold text-slate-900 dark:text-slate-100 whitespace-nowrap">
            {label}
          </span>
        </div>
      </div>

      {/* FAB Button */}
      <button
        onClick={onClick}
        onMouseEnter={() => setIsHovered(true)}
        onMouseLeave={() => setIsHovered(false)}
        className="group relative w-16 h-16 rounded-2xl bg-gradient-primary text-white shadow-elevation-3 hover:shadow-glow-primary transition-all duration-300 hover:scale-110 active:scale-95 focus:outline-none focus:ring-4 focus:ring-primary-500/30"
        aria-label={label}
      >
        {/* Background glow effect */}
        <div className="absolute inset-0 rounded-2xl bg-gradient-primary opacity-0 group-hover:opacity-100 blur-xl transition-opacity duration-300" />

        {/* Icon container */}
        <div className="relative z-10 w-full h-full flex items-center justify-center">
          <div className="transition-transform duration-300 group-hover:rotate-90">
            {icon || <Plus className="w-7 h-7" strokeWidth={2.5} />}
          </div>
        </div>

        {/* Ripple effect circle */}
        <div className="absolute inset-0 rounded-2xl">
          <div className="absolute inset-0 rounded-2xl bg-white/20 scale-0 group-hover:scale-100 group-active:scale-90 transition-transform duration-300" />
        </div>

        {/* Animated border */}
        <div className="absolute inset-0 rounded-2xl border-2 border-white/20 group-hover:border-white/40 transition-colors duration-300" />
      </button>
    </div>
  );
}

// Compact FAB variant for less prominent actions
export function CompactFAB({
  onClick,
  icon,
  tooltip
}: {
  onClick: () => void;
  icon: React.ReactNode;
  tooltip?: string;
}) {
  const [showTooltip, setShowTooltip] = useState(false);

  return (
    <div className="relative">
      {showTooltip && tooltip && (
        <div className="absolute bottom-full right-0 mb-2 px-3 py-1.5 glass-card-strong rounded-lg shadow-elevation-2 animate-slide-down">
          <span className="text-xs font-medium text-slate-900 dark:text-slate-100 whitespace-nowrap">
            {tooltip}
          </span>
        </div>
      )}

      <button
        onClick={onClick}
        onMouseEnter={() => setShowTooltip(true)}
        onMouseLeave={() => setShowTooltip(false)}
        className="w-12 h-12 rounded-xl bg-white/90 dark:bg-slate-800/90 backdrop-blur-lg border border-slate-200/60 dark:border-slate-700/60 shadow-elevation-2 hover:shadow-elevation-3 hover:scale-110 active:scale-95 transition-all duration-200 group focus:outline-none focus:ring-2 focus:ring-primary-500/30"
      >
        <div className="w-full h-full flex items-center justify-center text-slate-700 dark:text-slate-300 group-hover:text-primary-600 dark:group-hover:text-primary-400 transition-colors">
          {icon}
        </div>
      </button>
    </div>
  );
}
