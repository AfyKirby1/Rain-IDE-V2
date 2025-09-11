import { create } from 'zustand';
import { devtools } from 'zustand/middleware';
import { immer } from 'zustand/middleware/immer';
import { invoke } from '@tauri-apps/api/core';

export interface ModelStatus {
  id: string;
  name: string;
  type: 'chat' | 'embedding';
  status: 'discovering' | 'ready' | 'loading' | 'loaded' | 'error' | 'not_found';
  progress?: number;
  error?: string;
  lastUpdated: Date;
}

interface ModelStatusState {
  // Model status tracking
  modelStatuses: Map<string, ModelStatus>;
  
  // Actions
  setModelStatus: (id: string, status: Partial<ModelStatus>) => void;
  updateModelProgress: (id: string, progress: number) => void;
  setModelError: (id: string, error: string) => void;
  clearModelStatus: (id: string) => void;
  initializeModels: () => Promise<void>;
  getModelStatus: (id: string) => ModelStatus | undefined;
  getAllModelStatuses: () => ModelStatus[];
}

export const useModelStatusStore = create<ModelStatusState>()(
  devtools(
    immer((set, get) => ({
      // Initial state
      modelStatuses: new Map(),

      // Actions
      setModelStatus: (id: string, status: Partial<ModelStatus>) => set((state) => {
        const existing = state.modelStatuses.get(id);
        const newStatus: ModelStatus = {
          id,
          name: status.name || existing?.name || id,
          type: status.type || existing?.type || 'chat',
          status: status.status || existing?.status || 'ready',
          progress: status.progress,
          error: status.error,
          lastUpdated: new Date(),
        };
        state.modelStatuses.set(id, newStatus);
      }),

      updateModelProgress: (id: string, progress: number) => set((state) => {
        const existing = state.modelStatuses.get(id);
        if (existing) {
          existing.progress = progress;
          existing.lastUpdated = new Date();
        }
      }),

      setModelError: (id: string, error: string) => set((state) => {
        const existing = state.modelStatuses.get(id);
        if (existing) {
          existing.status = 'error';
          existing.error = error;
          existing.lastUpdated = new Date();
        }
      }),

      clearModelStatus: (id: string) => set((state) => {
        state.modelStatuses.delete(id);
      }),

      initializeModels: async () => {
        try {
          // Discover chat models
          await invoke('discover_models');
          
          // Discover embedding models
          const embeddingModels = await invoke('discover_embedding_models') as any[];
          
          // Update status for discovered models
          set((state) => {
            // Mark embedding models as discovered
            embeddingModels.forEach((model) => {
              state.modelStatuses.set(model.id, {
                id: model.id,
                name: model.name,
                type: 'embedding',
                status: 'ready',
                lastUpdated: new Date(),
              });
            });
          });
          
        } catch (error) {
          console.error('Failed to initialize models:', error);
        }
      },

      getModelStatus: (id: string) => {
        return get().modelStatuses.get(id);
      },

      getAllModelStatuses: () => {
        return Array.from(get().modelStatuses.values());
      },
    })),
    { name: 'model-status-store' }
  )
);
