import DofusItem from '@/components/dofus/dofus-item';
import { createFileRoute } from '@tanstack/react-router';

export const Route = createFileRoute('/dashboard/_layout/home')({
  component: Home,
});


function Home() {
  return (
    <div>
      <DofusItem iconId={23003} />
    </div>
  )
}
