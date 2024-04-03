import { PageTitle } from '@/components/page-title';
import { Button } from '@/components/ui/button';
import { Link, createFileRoute, useRouter } from '@tanstack/react-router';
import { useChatStore } from '@/stores/chat.store';
import {
  ChatForm,
  type ChatFormValues,
  fromChatTabConfig,
  toChatTabConfig,
} from '@/components/chat/form';
import { commands } from '@/commands';
import { useMemo } from 'react';

export const Route = createFileRoute('/dashboard/_layout/(chat)/chat/$tab_id/edit')({
  component: EditTabComponent,
});

function EditTabComponent() {
  const { tab_id } = Route.useParams();
  const tabs = useChatStore((state) => state.tabs);
  const setTab = useChatStore((state) => state.setTab);
  const tab = tabs[tab_id];

  const router = useRouter();

  if (!tab) {
    {
      /* TODO: handle error */
    }
    return <div>Chat not found</div>;
  }

  const initalValues = useMemo(() => fromChatTabConfig(tab), [tab]);

  const onEdit = async (values: ChatFormValues) => {
    const config = toChatTabConfig(values, tab.order);
    await commands.updateChatTabConfig(tab_id, config);
    router.navigate({ to: '/dashboard/chat/$tab_id', params: { tab_id } });
    setTab(tab_id, config);
  };

  return (
    <div className="flex flex-col w-full pb-2 gap-6">
      <PageTitle title="Editer le groupe" description="Modifier les paramètres du groupe">
        <div className="flex gap-2">
          <Button asChild variant="filled-neutral">
            <Link to="/dashboard/chat/$tab_id" params={{ tab_id }}>
              Retour
            </Link>
          </Button>
        </div>
      </PageTitle>
      <div className="px-6 flex justify-between items-center h-full pb-4">
        <ChatForm onSubmit={onEdit} submitText="Modifier le groupe" initialValues={initalValues} />
      </div>
    </div>
  );
}
