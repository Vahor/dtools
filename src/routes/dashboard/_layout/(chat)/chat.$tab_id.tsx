import { createFileRoute } from '@tanstack/react-router'
import { useEffect, useRef, useState } from 'react';
import { useChatStore } from '@/stores/chat.store';
import { PageTitle } from '@/components/page-title';
import { ChatEvent, ChatTabConfig, commands, events } from '@/commands';
import { readChatHistory } from '@/lib/features/chat/history';
import { useVirtualizer } from '@tanstack/react-virtual';

export const Route = createFileRoute('/dashboard/_layout/(chat)/chat/$tab_id')({
  component: ChatComponent,
  errorComponent: () => <div>Chat not found</div>,
});

const itemHeight = (containerWidth: number, item: ChatEvent, baseHeight: number, lineHeight: number) => {
  const avgCharWidth = 10;
  const charsPerLine = Math.floor(containerWidth / avgCharWidth);
  const textLength = item.content.length;
  const linesOfText = Math.ceil(textLength / charsPerLine);

  const totalHeight = baseHeight + (linesOfText * lineHeight);
  return totalHeight;
}

function ChatComponent() {
  const { tab_id } = Route.useParams();
  const tabs = useChatStore(state => state.tabs);
  const tab = tabs[tab_id];


  const [chatMessages, setChatMessages] = useState<ChatEvent[]>([])

  const scrollParentRef = useRef<HTMLDivElement>(null);
  const rowVirtualizer = useVirtualizer({
    count: chatMessages.length,
    getScrollElement: () => scrollParentRef.current,
    estimateSize: (index) => itemHeight(scrollParentRef.current!.clientWidth, chatMessages[index], 30, 20),
    overscan: 5,
  });

  useEffect(() => {
    if (!tab) return;
    commands.setActiveChatTab(tab_id);
    setChatMessages([]);


    const isPersistant = tab?.options.keepHistory;
    if (isPersistant) {
      readChatHistory(tab_id).then((history) => {
        setChatMessages((prev) => [...prev, ...history]);
      });
    }

    const unlisten = events.chatEvent.listen((event) => {
      setChatMessages((prev) => [event.payload, ...prev]);
    })

    return () => {
      console.log('unmounting chat')
      unlisten.then(f => f());
      commands.setActiveChatTab(null);
    }
  }, [rowVirtualizer])




  if (!tab) {
    {/* TODO: handle error */ }
    return <div>Chat not found</div>
  }

  return (
    <div className='flex flex-col  w-full select-auto' >
      <PageTitle title={tab.name} description={`${chatMessages.length} messages`}>
        <div className='flex gap-2'>
          <span>mute</span>
          <span>persistant</span>
          <span>config</span>
        </div>
      </PageTitle>
      <div className='px-6 flex flex-col flex-1 justify-between h-full pb-8 overflow-auto'
        ref={scrollParentRef}
      >
        <WelcomeTo tab={tab} />
        <ul
          style={{ height: `${rowVirtualizer.getTotalSize()}px`, position: 'relative' }}
        >
          {rowVirtualizer.getVirtualItems().map(({ index, key, size, start }) => {
            const item = chatMessages[index];
            return (
              <div
                key={key}
                className='items-center flex py-2 w-full border-t'
                style={{
                  position: 'absolute',
                  top: 0,
                  left: 0,
                  height: `${size}px`,
                  width: '100%',
                  transform: `translateY(${start}px)`,
                }}
              >
                <ChatMessage message={item} />
              </div>
            );
          })}
        </ul>
      </div>
    </div >
  )
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
