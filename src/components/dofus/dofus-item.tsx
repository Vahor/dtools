interface DofusItemProps extends Omit<React.HTMLAttributes<HTMLImageElement>, | "src"> {
  iconId: number;
}

export default function DofusItem({ iconId, ...props }: DofusItemProps) {
  return (
    <img src={`https://static.vahor.fr/ankama/dofus/items/${iconId}.webp`} {...props} />
  );
}
