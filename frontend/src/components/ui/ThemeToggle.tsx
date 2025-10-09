import { Moon, Sun } from 'lucide-react';
import { useTheme } from '../../contexts/ThemeContext';

export function ThemeToggle() {
  const { theme, toggleTheme } = useTheme();

  return (
    <button
      onClick={toggleTheme}
      className="relative w-16 h-9 rounded-2xl transition-all duration-500 shadow-elevation-2 hover:shadow-elevation-3 focus:outline-none focus:ring-2 focus:ring-primary-500/30 focus:ring-offset-2 focus:ring-offset-transparent active:scale-95 group"
      aria-label={`Switch to ${theme === 'light' ? 'dark' : 'light'} mode`}
      title={`Switch to ${theme === 'light' ? 'dark' : 'light'} mode`}
    >
      {/* Background gradient */}
      <div className={`absolute inset-0 rounded-2xl transition-all duration-500 ${
        theme === 'light'
          ? 'bg-gradient-to-r from-amber-400 via-orange-400 to-amber-500'
          : 'bg-gradient-to-r from-indigo-600 via-purple-600 to-indigo-700'
      }`}>
        {/* Animated stars for dark mode */}
        {theme === 'dark' && (
          <>
            <div className="absolute top-2 left-2 w-1 h-1 bg-white/80 rounded-full animate-pulse" style={{ animationDelay: '0s' }} />
            <div className="absolute top-3 right-4 w-0.5 h-0.5 bg-white/60 rounded-full animate-pulse" style={{ animationDelay: '0.3s' }} />
            <div className="absolute bottom-2 left-5 w-0.5 h-0.5 bg-white/70 rounded-full animate-pulse" style={{ animationDelay: '0.6s' }} />
          </>
        )}
      </div>

      {/* Sliding circle */}
      <div
        className={`absolute top-1 left-1 w-7 h-7 rounded-xl bg-white dark:bg-slate-900 shadow-lg transition-all duration-500 flex items-center justify-center ${
          theme === 'dark' ? 'translate-x-7' : 'translate-x-0'
        } group-hover:scale-110`}
      >
        {/* Icon with rotation animation */}
        <div className={`transition-all duration-500 ${theme === 'dark' ? 'rotate-180' : 'rotate-0'}`}>
          {theme === 'light' ? (
            <Sun className="w-4 h-4 text-amber-500" strokeWidth={2.5} />
          ) : (
            <Moon className="w-4 h-4 text-indigo-400" strokeWidth={2.5} />
          )}
        </div>
      </div>

      {/* Glow effect on hover */}
      <div className={`absolute inset-0 rounded-2xl transition-opacity duration-300 opacity-0 group-hover:opacity-100 ${
        theme === 'light'
          ? 'shadow-glow-secondary'
          : 'shadow-glow-primary'
      }`} />
    </button>
  );
}
