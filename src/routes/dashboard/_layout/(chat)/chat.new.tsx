import { commands } from '@/commands';
import { ChatForm, toChatTabConfig, type ChatFormValues } from '@/components/chat/form';
import { PageTitle } from '@/components/page-title';
import { createFileRoute, useRouter } from '@tanstack/react-router';
import { useChatStore } from '@/stores/chat.store';

export const Route = createFileRoute('/dashboard/_layout/(chat)/chat/new')({
  component: ChatNew,
});

function ChatNew() {
  const router = useRouter();
  const setTab = useChatStore((state) => state.setTab);
  const tabs = useChatStore((state) => state.tabs);

  const onSubmit = async (values: ChatFormValues) => {
    const config = toChatTabConfig(values, Object.keys(tabs).length);

    const tabId = await commands.createChatTab(config);
    setTab(tabId, config);
    router.navigate({ to: '/dashboard/chat/$tab_id', params: { tab_id: tabId } });
  };

  return (
    <div className="flex flex-col gap-6 w-full">
      <PageTitle title="Nouveau groupe" description="Créer un nouveau groupe de discussion" />
      <div className="px-6 flex justify-between items-center h-full pb-4">
        <ChatForm onSubmit={onSubmit} submitText="Créer le groupe" />
      </div>
    </div>
  );
}
