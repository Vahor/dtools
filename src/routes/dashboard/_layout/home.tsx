import DofusItemIcon from '@/components/dofus/dofus-item-icon';
import { createFileRoute } from '@tanstack/react-router';

export const Route = createFileRoute('/dashboard/_layout/home')({
  component: Home,
});


function Home() {
  return (
    <div>
      <DofusItemIcon iconId={23003} />
    </div>
  )
}
