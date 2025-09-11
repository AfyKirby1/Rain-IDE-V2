import { create } from 'zustand';
import { devtools } from 'zustand/middleware';
import { immer } from 'zustand/middleware/immer';
import { invoke } from '@tauri-apps/api/core';

interface AIModel {
  id: string;
  name: string;
  description: string;
  format: string;
  path: string;
  size_mb: number;
  loaded: boolean;
  capabilities: string[];
}

interface ChatMessage {
  id: string;
  role: 'user' | 'assistant' | 'system';
  content: string;
  timestamp: Date;
}

interface ChatSession {
  id: string;
  name: string;
  messages: ChatMessage[];
  created_at: Date;
}

interface AIState {
  // Model management
  availableModels: AIModel[];
  currentModel: AIModel | null;
  isModelLoading: boolean;
  isDiscoveringModels: boolean;
  
  // Chat
  chatSessions: ChatSession[];
  currentChatSession: string | null;
  
  // Actions
  setAvailableModels: (models: AIModel[]) => void;
  setCurrentModel: (model: AIModel | null) => void;
  setModelLoading: (loading: boolean) => void;
  setDiscoveringModels: (discovering: boolean) => void;
  addChatSession: (session: ChatSession) => void;
  setCurrentChatSession: (sessionId: string | null) => void;
  addMessageToSession: (sessionId: string, message: ChatMessage) => void;
  discoverModels: () => Promise<void>;
  loadModel: (modelName: string) => Promise<boolean>;
  loadBestModel: () => Promise<boolean>;
  generateResponse: (message: string) => Promise<string>;
  getModelInfo: () => Promise<any>;
  clearConversation: () => Promise<void>;
}

export const useAIStore = create<AIState>()(
  devtools(
    immer((set) => ({
      // Initial state
      availableModels: [],
      currentModel: null,
      isModelLoading: false,
      isDiscoveringModels: false,
      chatSessions: [],
      currentChatSession: null,

      // Actions
      setAvailableModels: (models) => set((state) => {
        state.availableModels = models;
      }),

      setCurrentModel: (model) => set((state) => {
        state.currentModel = model;
      }),

      setModelLoading: (loading) => set((state) => {
        state.isModelLoading = loading;
      }),

      setDiscoveringModels: (discovering) => set((state) => {
        state.isDiscoveringModels = discovering;
      }),

      addChatSession: (session) => set((state) => {
        state.chatSessions.push(session);
      }),

      setCurrentChatSession: (sessionId) => set((state) => {
        state.currentChatSession = sessionId;
      }),

      addMessageToSession: (sessionId, message) => set((state) => {
        const session = state.chatSessions.find(s => s.id === sessionId);
        if (session) {
          session.messages.push(message);
        }
      }),

      // AI Operations
      discoverModels: async () => {
        try {
          const models: AIModel[] = await invoke('discover_models');
          set((state) => {
            state.availableModels = models;
          });
        } catch (error) {
          console.error('Failed to discover models:', error);
        }
      },

      loadModel: async (modelName: string) => {
        set((state) => {
          state.isModelLoading = true;
        });
        try {
          const success: boolean = await invoke('load_model_by_name', { modelName });
          
          if (success) {
            // Refresh the model list to get updated loaded status
            const models: AIModel[] = await invoke('discover_models');
            const loadedModel = models.find(m => m.name === modelName && m.loaded);
            
            set((state) => {
              state.currentModel = loadedModel || null;
              state.availableModels = models;
              state.isModelLoading = false;
            });
            return true;
          } else {
            set((state) => {
              state.isModelLoading = false;
            });
            return false;
          }
        } catch (error) {
          console.error('Failed to load model:', error);
          set((state) => {
            state.isModelLoading = false;
          });
          return false;
        }
      },

      loadBestModel: async () => {
        set((state) => {
          state.isModelLoading = true;
        });
        try {
          const success: boolean = await invoke('load_best_model');
          
          if (success) {
            const models: AIModel[] = await invoke('discover_models');
            const loadedModel = models.find(m => m.loaded);
            
            set((state) => {
              state.currentModel = loadedModel || null;
              state.availableModels = models;
              state.isModelLoading = false;
            });
          } else {
            set((state) => {
              state.isModelLoading = false;
            });
          }
          return success;
        } catch (error) {
          console.error('Failed to load best model:', error);
          set((state) => {
            state.isModelLoading = false;
          });
          return false;
        }
      },

      generateResponse: async (message: string) => {
        try {
          return await invoke('generate_response', { message });
        } catch (error) {
          console.error('Failed to generate response:', error);
          throw error;
        }
      },

      getModelInfo: async () => {
        try {
          return await invoke('get_model_info');
        } catch (error) {
          console.error('Failed to get model info:', error);
          return null;
        }
      },

      clearConversation: async () => {
        try {
          await invoke('clear_conversation');
        } catch (error) {
          console.error('Failed to clear conversation:', error);
        }
      },
    })),
    { name: 'ai-store' }
  )
);