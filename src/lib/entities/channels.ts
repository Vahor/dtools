import { translate } from './translations';
import { readEntitiesFile } from './utils';

interface RawChatChannel {
  id: number;
  nameId: number;
}

interface ChatChannel {
  id: number;
  name: string;
}

let channels: ChatChannel[] | undefined = undefined;

const loadChannels = async () => {
  const rawChannels = (await readEntitiesFile('ChatChannels.json')) as { data: RawChatChannel[] };
  channels = rawChannels.data.map((rawChannel) => ({
    id: rawChannel.id,
    name: translate(rawChannel.nameId),
  }));
};

const getChannels = () => {
  if (channels === undefined) {
    throw new Error('Channels not loaded');
  }
  return channels;
};

export const chatManager: ResourceManager<ChatChannel[]> = {
  load: loadChannels,
  get: getChannels,
};
