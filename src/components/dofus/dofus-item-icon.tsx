interface DofusItemIconProps extends Omit<React.HTMLAttributes<HTMLImageElement>, | "src"> {
  iconId: number;
}

export default function DofusItemIcon({ iconId, ...props }: DofusItemIconProps) {
  return (
    <img src={`https://static.vahor.fr/ankama/dofus/items/${iconId}.webp`} {...props} />
  );
}
