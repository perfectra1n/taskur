import { create } from 'zustand';
import { persist } from 'zustand/middleware';
import type { User } from '../types';
import { api } from '../services/api';

interface AuthState {
  user: User | null;
  token: string | null;
  isAuthenticated: boolean;
  login: (email: string, password: string) => Promise<void>;
  register: (email: string, password: string) => Promise<void>;
  logout: () => void;
  checkAuth: () => Promise<void>;
}

export const useAuthStore = create<AuthState>()(
  persist(
    (set) => ({
      user: null,
      token: null,
      isAuthenticated: false,

      login: async (email: string, password: string) => {
        const response = await api.login(email, password);
        localStorage.setItem('auth_token', response.token);
        set({
          user: response.user,
          token: response.token,
          isAuthenticated: true,
        });
      },

      register: async (email: string, password: string) => {
        const response = await api.register(email, password);
        localStorage.setItem('auth_token', response.token);
        set({
          user: response.user,
          token: response.token,
          isAuthenticated: true,
        });
      },

      logout: () => {
        localStorage.removeItem('auth_token');
        set({
          user: null,
          token: null,
          isAuthenticated: false,
        });
      },

      checkAuth: async () => {
        const token = localStorage.getItem('auth_token');
        if (!token) {
          set({ isAuthenticated: false, user: null, token: null });
          return;
        }

        try {
          const user = await api.getCurrentUser();
          set({
            user,
            token,
            isAuthenticated: true,
          });
        } catch (error) {
          localStorage.removeItem('auth_token');
          set({ isAuthenticated: false, user: null, token: null });
        }
      },
    }),
    {
      name: 'auth-storage',
      partialize: (state) => ({
        token: state.token,
        user: state.user,
        isAuthenticated: state.isAuthenticated,
      }),
    }
  )
);
