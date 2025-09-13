import { create } from 'zustand';
import { devtools } from 'zustand/middleware';

export interface Project {
  id: string;
  name: string;
  path: string;
  project_type: string;
  last_opened: string;
}

interface ProjectState {
  currentProject: Project | null;
  projects: Project[];
  
  // Actions
  setCurrentProject: (project: Project) => void;
  setProjects: (projects: Project[]) => void;
  addProject: (project: Project) => void;
  removeProject: (projectId: string) => void;
}

export const useProjectStore = create<ProjectState>()(
  devtools(
    (set) => ({
      // Initial state
      currentProject: null,
      projects: [],

    // Actions
    setCurrentProject: (project) => {
      set({ currentProject: project });
    },

      setProjects: (projects) => set({ projects }),

      addProject: (project) => set((state) => {
        const existingIndex = state.projects.findIndex(p => p.id === project.id);
        if (existingIndex >= 0) {
          const newProjects = [...state.projects];
          newProjects[existingIndex] = project;
          return { projects: newProjects };
        } else {
          return { projects: [...state.projects, project] };
        }
      }),

      removeProject: (projectId) => set((state) => {
        const newProjects = state.projects.filter(p => p.id !== projectId);
        const newCurrentProject = state.currentProject?.id === projectId ? null : state.currentProject;
        return { 
          projects: newProjects, 
          currentProject: newCurrentProject 
        };
      }),
    }),
    { name: 'project-store' }
  )
);