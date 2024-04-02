import { NodeConfig } from '@/commands'
import { create } from 'zustand'

interface ConfigStore {
  config: NodeConfig | undefined,
  setConfig: (config: NodeConfig) => void
}

export const useConfigStore = create<ConfigStore>((set) => ({
  config: undefined,
  setConfig: (config) => set({ config }),
}))
