import { create } from 'zustand';
import { invoke } from '@tauri-apps/api/core';

export interface EditorTab {
  id: string;
  filePath: string;
  fileName: string;
  content: string;
  isDirty: boolean;
  language?: string;
  isUntitled?: boolean; // For scratch files
}

interface EditorState {
  // Editor state
  openTabs: EditorTab[];
  activeTabId: string | null;
  isEditorLoading: boolean;
  
  // Actions
  openFile: (filePath: string) => Promise<void>;
  closeTab: (tabId: string) => void;
  setActiveTab: (tabId: string) => void;
  updateTabContent: (tabId: string, content: string) => void;
  saveFile: (tabId: string) => Promise<void>;
  createFile: (filePath: string) => Promise<void>;
  deleteFile: (filePath: string) => Promise<void>;
  createUntitledFile: (language?: string) => void;
}

export const useEditorStore = create<EditorState>((set, get) => ({
  // Initial state
  openTabs: [],
  activeTabId: null,
  isEditorLoading: false,

  // Actions
  openFile: async (filePath: string) => {
    const state = get();
    
    console.log('EditorStore: Opening file:', filePath);
    
    // Check if file is already open
    const existingTab = state.openTabs.find(tab => tab.filePath === filePath);
    if (existingTab) {
      console.log('EditorStore: File already open, switching to tab:', existingTab.id);
      set({ activeTabId: existingTab.id });
      return;
    }

    console.log('EditorStore: Loading file content...');
    set({ isEditorLoading: true });
    
    try {
      // Open file in backend (this will read the file and open it in the editor)
      const fileContent = await invoke('open_file', { filePath: filePath }) as any;
      console.log('EditorStore: File opened in backend:', fileContent);
      
      const content = fileContent.content || '';
      const language = fileContent.language || 'text';
      console.log('EditorStore: File content loaded, length:', content.length);
      console.log('EditorStore: Language detected:', language);
      
      // Get file name from path
      const fileName = filePath.split('/').pop() || filePath.split('\\').pop() || 'Unknown';

      const newTab: EditorTab = {
        id: `tab_${Date.now()}`,
        filePath,
        fileName,
        content,
        isDirty: false,
        language,
      };

      set({
        openTabs: [...state.openTabs, newTab],
        activeTabId: newTab.id,
        isEditorLoading: false,
      });
      
      console.log('EditorStore: File opened successfully, new tab created:', newTab.id);
    } catch (error) {
      console.error('EditorStore: Failed to open file:', error);
      set({ isEditorLoading: false });
      throw error;
    }
  },

  closeTab: (tabId: string) => {
    const state = get();
    const tabIndex = state.openTabs.findIndex(tab => tab.id === tabId);
    
    if (tabIndex === -1) return;

    const newTabs = state.openTabs.filter(tab => tab.id !== tabId);
    
    // If we're closing the active tab, set a new active tab
    let newActiveTabId = state.activeTabId;
    if (state.activeTabId === tabId) {
      if (newTabs.length > 0) {
        // Set to the next tab, or previous if we closed the last one
        const nextIndex = tabIndex < newTabs.length ? tabIndex : tabIndex - 1;
        newActiveTabId = newTabs[nextIndex]?.id || null;
      } else {
        newActiveTabId = null;
      }
    }

    set({
      openTabs: newTabs,
      activeTabId: newActiveTabId,
    });
  },

  setActiveTab: (tabId: string) => {
    set({ activeTabId: tabId });
  },

  updateTabContent: (tabId: string, content: string) => {
    const state = get();
    const updatedTabs = state.openTabs.map(tab => 
      tab.id === tabId 
        ? { ...tab, content, isDirty: true }
        : tab
    );
    
    set({ openTabs: updatedTabs });
  },

  saveFile: async (tabId: string) => {
    const state = get();
    const tab = state.openTabs.find(tab => tab.id === tabId);
    
    if (!tab) return;

    try {
      let filePath = tab.filePath;
      
      // If it's an untitled file, prompt for filename
      if (tab.isUntitled) {
        const fileName = prompt('Save file as:', 'untitled.txt');
        if (!fileName) return; // User cancelled
        
        // Get current project path
        const { useProjectStore } = await import('../stores/projectStore');
        const projectStore = useProjectStore.getState();
        const currentProject = projectStore.currentProject;
        
        if (!currentProject) {
          throw new Error('No project open');
        }
        
        filePath = `${currentProject.path}/${fileName}`;
        
        // Create the file in backend
        await invoke('create_file', { filePath: filePath });
        await invoke('save_file', { 
          filePath: filePath, 
          content: tab.content 
        });
        
        // Update the tab to be a regular file
        const updatedTabs = state.openTabs.map(t => 
          t.id === tabId ? { 
            ...t, 
            filePath,
            fileName,
            isDirty: false,
            isUntitled: false
          } : t
        );
        
        set({ openTabs: updatedTabs });
      } else {
        // Regular file save
        await invoke('save_file', { 
          filePath: filePath, 
          content: tab.content 
        });
        
        // Mark as not dirty
        const updatedTabs = state.openTabs.map(t => 
          t.id === tabId ? { ...t, isDirty: false } : t
        );
        
        set({ openTabs: updatedTabs });
      }
    } catch (error) {
      console.error('Failed to save file:', error);
      throw error;
    }
  },

  createFile: async (filePath: string) => {
    try {
      await invoke('create_file', { filePath: filePath });
    } catch (error) {
      console.error('Failed to create file:', error);
      throw error;
    }
  },

  deleteFile: async (filePath: string) => {
    try {
      await invoke('delete_file', { filePath: filePath });
    } catch (error) {
      console.error('Failed to delete file:', error);
      throw error;
    }
  },

  createUntitledFile: (language: string = 'text') => {
    const state = get();
    const tabId = `untitled_${Date.now()}`;
    
    const newTab: EditorTab = {
      id: tabId,
      filePath: '', // No file path for untitled files
      fileName: 'Untitled',
      content: '',
      isDirty: false,
      language,
      isUntitled: true,
    };

    set({
      openTabs: [...state.openTabs, newTab],
      activeTabId: tabId,
    });
    
    console.log('EditorStore: Created untitled file:', tabId);
  },
}));
