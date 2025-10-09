/** @type {import('tailwindcss').Config} */
export default {
  content: [
    "./index.html",
    "./src/**/*.{js,ts,jsx,tsx}",
  ],
  theme: {
    extend: {
      colors: {
        // 2026 SF Tech Startup - Purple/Blue Gradient Primary
        primary: {
          50: '#f5f3ff',
          100: '#ede9fe',
          200: '#ddd6fe',
          300: '#c4b5fd',
          400: '#a78bfa',
          500: '#8b5cf6',
          600: '#7c3aed',
          700: '#6d28d9',
          800: '#5b21b6',
          900: '#4c1d95',
          950: '#2e1065',
        },
        // Secondary blue
        secondary: {
          50: '#eff6ff',
          100: '#dbeafe',
          200: '#bfdbfe',
          300: '#93c5fd',
          400: '#60a5fa',
          500: '#3b82f6',
          600: '#2563eb',
          700: '#1d4ed8',
          800: '#1e40af',
          900: '#1e3a8a',
          950: '#172554',
        },
        // Accent colors for 2026
        accent: {
          purple: '#a78bfa',
          violet: '#8b5cf6',
          indigo: '#6366f1',
          blue: '#3b82f6',
          cyan: '#06b6d4',
          teal: '#14b8a6',
          emerald: '#10b981',
          pink: '#ec4899',
          rose: '#f43f5e',
          amber: '#f59e0b',
        },
        // Glassmorphism surfaces
        surface: {
          light: 'rgba(255, 255, 255, 0.7)',
          'light-strong': 'rgba(255, 255, 255, 0.9)',
          dark: 'rgba(24, 24, 27, 0.7)',
          'dark-strong': 'rgba(24, 24, 27, 0.9)',
        },
      },
      backgroundImage: {
        'gradient-radial': 'radial-gradient(var(--tw-gradient-stops))',
        'gradient-conic': 'conic-gradient(from 180deg at 50% 50%, var(--tw-gradient-stops))',
        // 2026 Premium Gradients
        'gradient-primary': 'linear-gradient(135deg, #6366f1 0%, #8b5cf6 100%)',
        'gradient-aurora': 'linear-gradient(135deg, #667eea 0%, #764ba2 50%, #f093fb 100%)',
        'gradient-sunset': 'linear-gradient(135deg, #fa709a 0%, #fee140 100%)',
        'gradient-ocean': 'linear-gradient(135deg, #667eea 0%, #06b6d4 100%)',
        'gradient-forest': 'linear-gradient(135deg, #10b981 0%, #14b8a6 100%)',
        'gradient-candy': 'linear-gradient(135deg, #a8edea 0%, #fed6e3 100%)',
        'gradient-holographic': 'linear-gradient(135deg, #667eea 0%, #764ba2 25%, #f093fb 50%, #4facfe 75%, #00f2fe 100%)',
        // Mesh backgrounds
        'mesh-light': 'radial-gradient(at 0% 0%, #ede9fe 0%, transparent 50%), radial-gradient(at 100% 0%, #dbeafe 0%, transparent 50%), radial-gradient(at 100% 100%, #fce7f3 0%, transparent 50%), radial-gradient(at 0% 100%, #e0f2fe 0%, transparent 50%)',
        'mesh-dark': 'radial-gradient(at 0% 0%, #2e1065 0%, transparent 50%), radial-gradient(at 100% 0%, #172554 0%, transparent 50%), radial-gradient(at 100% 100%, #831843 0%, transparent 50%), radial-gradient(at 0% 100%, #0c4a6e 0%, transparent 50%)',
      },
      animation: {
        // Smooth animations for 2026
        'fade-in': 'fadeIn 0.4s cubic-bezier(0.16, 1, 0.3, 1)',
        'fade-out': 'fadeOut 0.3s cubic-bezier(0.16, 1, 0.3, 1)',
        'slide-up': 'slideUp 0.4s cubic-bezier(0.16, 1, 0.3, 1)',
        'slide-down': 'slideDown 0.4s cubic-bezier(0.16, 1, 0.3, 1)',
        'slide-left': 'slideLeft 0.4s cubic-bezier(0.16, 1, 0.3, 1)',
        'slide-right': 'slideRight 0.4s cubic-bezier(0.16, 1, 0.3, 1)',
        'scale-in': 'scaleIn 0.3s cubic-bezier(0.16, 1, 0.3, 1)',
        'scale-out': 'scaleOut 0.2s cubic-bezier(0.16, 1, 0.3, 1)',
        'shimmer': 'shimmer 2.5s linear infinite',
        'float': 'float 6s ease-in-out infinite',
        'pulse-slow': 'pulseSlow 3s cubic-bezier(0.4, 0, 0.6, 1) infinite',
        'glow': 'glow 2s ease-in-out infinite',
        'spin-slow': 'spin 3s linear infinite',
        'bounce-slow': 'bounce 2s infinite',
      },
      keyframes: {
        fadeIn: {
          '0%': { opacity: '0' },
          '100%': { opacity: '1' },
        },
        fadeOut: {
          '0%': { opacity: '1' },
          '100%': { opacity: '0' },
        },
        slideUp: {
          '0%': { transform: 'translateY(20px)', opacity: '0' },
          '100%': { transform: 'translateY(0)', opacity: '1' },
        },
        slideDown: {
          '0%': { transform: 'translateY(-20px)', opacity: '0' },
          '100%': { transform: 'translateY(0)', opacity: '1' },
        },
        slideLeft: {
          '0%': { transform: 'translateX(20px)', opacity: '0' },
          '100%': { transform: 'translateX(0)', opacity: '1' },
        },
        slideRight: {
          '0%': { transform: 'translateX(-20px)', opacity: '0' },
          '100%': { transform: 'translateX(0)', opacity: '1' },
        },
        scaleIn: {
          '0%': { transform: 'scale(0.9)', opacity: '0' },
          '100%': { transform: 'scale(1)', opacity: '1' },
        },
        scaleOut: {
          '0%': { transform: 'scale(1)', opacity: '1' },
          '100%': { transform: 'scale(0.9)', opacity: '0' },
        },
        shimmer: {
          '0%': { backgroundPosition: '-1000px 0' },
          '100%': { backgroundPosition: '1000px 0' },
        },
        float: {
          '0%, 100%': { transform: 'translateY(0) rotate(0deg)' },
          '50%': { transform: 'translateY(-20px) rotate(5deg)' },
        },
        pulseSlow: {
          '0%, 100%': { opacity: '1' },
          '50%': { opacity: '0.8' },
        },
        glow: {
          '0%, 100%': { boxShadow: '0 0 20px rgba(139, 92, 246, 0.5)' },
          '50%': { boxShadow: '0 0 40px rgba(139, 92, 246, 0.8)' },
        },
      },
      backdropBlur: {
        xs: '2px',
      },
      boxShadow: {
        // Neumorphism 2.0 shadows
        'neu-light': '8px 8px 16px rgba(163, 177, 198, 0.3), -8px -8px 16px rgba(255, 255, 255, 0.9)',
        'neu-dark': '8px 8px 16px rgba(0, 0, 0, 0.4), -8px -8px 16px rgba(255, 255, 255, 0.05)',
        'neu-inset-light': 'inset 4px 4px 8px rgba(163, 177, 198, 0.3), inset -4px -4px 8px rgba(255, 255, 255, 0.9)',
        'neu-inset-dark': 'inset 4px 4px 8px rgba(0, 0, 0, 0.4), inset -4px -4px 8px rgba(255, 255, 255, 0.05)',
        // Glassmorphism shadows
        'glass': '0 8px 32px 0 rgba(99, 102, 241, 0.1)',
        'glass-lg': '0 20px 60px 0 rgba(99, 102, 241, 0.15)',
        'glass-dark': '0 8px 32px 0 rgba(0, 0, 0, 0.4)',
        'glass-dark-lg': '0 20px 60px 0 rgba(0, 0, 0, 0.6)',
        // Elevation shadows
        'elevation-1': '0 2px 8px rgba(0, 0, 0, 0.04), 0 1px 2px rgba(0, 0, 0, 0.06)',
        'elevation-2': '0 4px 16px rgba(0, 0, 0, 0.06), 0 2px 4px rgba(0, 0, 0, 0.08)',
        'elevation-3': '0 8px 24px rgba(0, 0, 0, 0.08), 0 4px 8px rgba(0, 0, 0, 0.1)',
        'elevation-4': '0 16px 48px rgba(0, 0, 0, 0.1), 0 8px 16px rgba(0, 0, 0, 0.12)',
        // Glow effects
        'glow-primary': '0 0 30px rgba(139, 92, 246, 0.5), 0 0 60px rgba(139, 92, 246, 0.3)',
        'glow-secondary': '0 0 30px rgba(59, 130, 246, 0.5), 0 0 60px rgba(59, 130, 246, 0.3)',
        'glow-cyan': '0 0 30px rgba(6, 182, 212, 0.5), 0 0 60px rgba(6, 182, 212, 0.3)',
        'glow-pink': '0 0 30px rgba(236, 72, 153, 0.5), 0 0 60px rgba(236, 72, 153, 0.3)',
      },
      borderRadius: {
        '2xl': '1rem',
        '3xl': '1.5rem',
        '4xl': '2rem',
      },
    },
  },
  plugins: [
    require('@tailwindcss/typography'),
  ],
  darkMode: 'class',
}
