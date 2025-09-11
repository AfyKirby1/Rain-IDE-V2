import { create } from 'zustand';
import { devtools } from 'zustand/middleware';
import { immer } from 'zustand/middleware/immer';

interface AppSettings {
  theme: 'dark' | 'light';
  autoSave: boolean;
  fontSize: number;
  ai?: {
    auto_load_model?: boolean;
    preferred_models?: string[];
  };
}

interface RecentProject {
  id: string;
  name: string;
  path: string;
  lastOpened: Date;
}

interface AppState {
  // UI State
  theme: 'dark' | 'light';
  showWelcome: boolean;
  isLoading: boolean;
  
  // Settings
  settings: AppSettings | null;
  
  // Recent Projects
  recentProjects: RecentProject[];
  
  // Actions
  setTheme: (theme: 'dark' | 'light') => void;
  setShowWelcome: (show: boolean) => void;
  setLoading: (loading: boolean) => void;
  setSettings: (settings: AppSettings) => void;
  addRecentProject: (project: any) => void;
}

export const useAppStore = create<AppState>()(
  devtools(
    immer((set) => ({
      // Initial state
      theme: 'dark',
      showWelcome: true,
      isLoading: false,
      settings: null,
      recentProjects: [],

      // Actions
      setTheme: (theme) => set((state) => {
        state.theme = theme;
      }),

      setShowWelcome: (show) => set((state) => {
        state.showWelcome = show;
      }),

      setLoading: (loading) => set((state) => {
        state.isLoading = loading;
      }),

      setSettings: (settings) => set((state) => {
        state.settings = settings;
      }),

      addRecentProject: (project) => set((state) => {
        const existingIndex = state.recentProjects.findIndex(p => p.path === project.path);
        if (existingIndex >= 0) {
          state.recentProjects.splice(existingIndex, 1);
        }
        state.recentProjects.unshift({
          id: project.id,
          name: project.name,
          path: project.path,
          lastOpened: new Date(),
        });
        // Keep only 10 most recent
        if (state.recentProjects.length > 10) {
          state.recentProjects.splice(10);
        }
      }),
    })),
    { name: 'app-store' }
  )
);