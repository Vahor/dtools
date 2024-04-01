import { InnerSidebar } from '@/components/inner-sidebar';
import { Outlet, createFileRoute } from '@tanstack/react-router'

export const Route = createFileRoute('/dashboard/_layout/(chat)/chat')({
  component: ChatComponent,
})

function ChatComponent() {

  return (
    <>
      <InnerSidebar className="select-auto">
        <div>sidebar</div>
      </InnerSidebar>
      <Outlet />
    </>
  );
}

