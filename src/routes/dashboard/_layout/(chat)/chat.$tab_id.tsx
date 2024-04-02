import { createFileRoute } from '@tanstack/react-router'
import { useEffect, useRef, useState } from 'react';
import { useChatStore } from '@/stores/chat.store';
import { PageTitle } from '@/components/page-title';
import { ChatEvent, ChatTabConfig, commands, events } from '@/commands';
import { readChatHistory } from '@/lib/features/chat/history';
import { useVirtualizer } from '@tanstack/react-virtual';
import { create } from 'zustand';

export const Route = createFileRoute('/dashboard/_layout/(chat)/chat/$tab_id')({
  component: ChatComponent,
  errorComponent: () => <div>Chat not found</div>,
});

const useMessageStore = create<{ messages: ChatEvent[], setMessages: (messages: ChatEvent[]) => void, addMessage: (message: ChatEvent) => void, addBulkMessages: (messages: ChatEvent[]) => void }>((set) => ({
  messages: [],
  setMessages: (messages) => set({ messages }),
  addMessage: (message) => set((state) => ({ messages: [...state.messages, message] })),
  addBulkMessages: (messages) => set((state) => ({ messages: [...state.messages, ...messages] })),
}));

function ChatComponent() {
  const { tab_id } = Route.useParams();
  const tabs = useChatStore(state => state.tabs);
  const tab = tabs[tab_id];

  if (!tab) {
    {/* TODO: handle error */ }
    return <div>Chat not found</div>
  }


  return (
    <div className='flex flex-col  w-full select-auto' >
      <PageTitle title={tab.name} description={<TitleDescription />}>
        <div className='flex gap-2'>
          <span>scroll lock</span>
          <span>bell {tab.options.notify ? "on" : "off"}</span>
          <span>persistant {tab.options.keepHistory ? "on" : "off"}</span>
          <span>config</span>
        </div>
      </PageTitle>
      <ChatMessageList tab={tab} tab_id={tab_id} key={tab_id} />
    </div >
  )
}

const TitleDescription = () => {
  const count = useMessageStore(state => state.messages.length);

  return `${count} messages`;
}

const ChatMessageList = ({ tab, tab_id }: { tab: ChatTabConfig, tab_id: string }) => {
  const messages = useMessageStore(state => state.messages);
  const setMessages = useMessageStore(state => state.setMessages);
  const addMessage = useMessageStore(state => state.addMessage);


  const count = messages.length;

  const scrollParentRef = useRef<HTMLDivElement>(null);
  const rowVirtualizer = useVirtualizer({
    count: messages.length,
    getScrollElement: () => scrollParentRef.current,
    estimateSize: () => 80,
    overscan: 5,
    scrollPaddingEnd: 80
  });

  useEffect(() => {
    if (!tab) return;
    commands.setActiveChatTab(tab_id);


    const isPersistant = tab?.options.keepHistory;
    if (isPersistant) {
      readChatHistory(tab_id).then((history) => {
        setMessages(history);
      });
    } else {
      setMessages([]);
    }

    const unlisten = events.chatEvent.listen((event) => {
      addMessage(event.payload);
    })

    return () => {
      console.log('unmounting chat')
      unlisten.then(f => f());
      commands.setActiveChatTab(null);
    }
  }, [])

  const items = rowVirtualizer.getVirtualItems();

  useEffect(() => {
    if (count === 0) return;
    rowVirtualizer.scrollToIndex(count - 1);
  }, [count, rowVirtualizer])

  return (
    <div className='px-6 contain-strict justify-between h-full pb-8 overflow-y-auto'
      ref={scrollParentRef}
    >
      <WelcomeTo tab={tab} />
      <div
        style={{ height: rowVirtualizer.getTotalSize(), position: 'relative', width: '100%' }}
      >
        <ul
          style={{
            position: 'absolute',
            top: 0,
            left: 0,
            width: '100%',
            transform: `translateY(${items[0]?.start ?? 0}px)`,
          }}
        >
          {items.map(({ index, key }) => {
            const item = messages[index];
            return (
              <li
                key={key}
                data-index={index}
                className='items-center flex py-2 w-full border-t'
                ref={rowVirtualizer.measureElement}
              >
                <ChatMessage message={item} />
              </li>
            );
          })}
        </ul>
      </div>
    </div>

  );

}

const WelcomeTo = ({ tab }: { tab: ChatTabConfig }) => {
  return (
    <div className='flex flex-col pt-4 pb-2'>
      <h2 className='font-bold text-xl'>Bienvenue dans {tab.name} !</h2>
      <p className='text-soft text-sm'>Ce groupe est vide pour le moment. Des messages vont apparaitre ici.</p>
      <p className='text-soft text-sm'>Utilisez le mode persistant pour conserver l'historique des messages.</p>
    </div>
  )
}

const ChatMessage = ({ message }: { message: ChatEvent }) => {
  return (
    <div className='flex w-full'>
      {message.sender_name}: {message.content}
    </div>
  );
}
