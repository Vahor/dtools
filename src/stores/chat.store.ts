import { ChatTabConfig } from '@/commands';
import { create } from 'zustand'

interface ChatStore {
  tabs: Record<string, ChatTabConfig>,

  setTabs: (channels: Record<string, ChatTabConfig>) => void,
  addTab: (id: string, channel: ChatTabConfig) => void,
}

export const useChatStore = create<ChatStore>((set) => ({
  tabs: {},

  setTabs: (channels) => set({ tabs: channels }),
  addTab: (id, channel) => set((state) => ({ tabs: { ...state.tabs, [id]: channel } })),
}))
