import type { ChatEvent } from '@/commands';
import { readJsonlFile } from '@/lib/entities/utils';

export const readChatHistory = async (tab_id: string): Promise<ChatEvent[]> => {
  const content = await readJsonlFile(`features/chat/history/${tab_id}.jsonl`);
  return content as ChatEvent[];
};
