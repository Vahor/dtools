import { createFileRoute } from '@tanstack/react-router'

export const Route = createFileRoute('/dashboard/_layout/(chat)/chat/')({
  component: () => <div>Hello /dashboard/_layout/(chat)/chat/!</div>
})