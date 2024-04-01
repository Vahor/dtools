import { createFileRoute } from '@tanstack/react-router'

export const Route = createFileRoute('/dashboard/_layout/home')({
  component: () => <div>Hello /dashboard/_layout/home!</div>
})