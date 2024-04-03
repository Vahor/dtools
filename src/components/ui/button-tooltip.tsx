import type { TooltipContentProps } from '@radix-ui/react-tooltip';
import { Button, type ButtonProps } from './button';
import { Tooltip, TooltipContent, TooltipTrigger } from './tooltip';

interface ButtonTooltipProps extends ButtonProps {
  tooltip: string;
  children: React.ReactNode;
  side?: TooltipContentProps['side'];
  align?: TooltipContentProps['align'];
}

export const ButtonTooltip = ({
  tooltip,
  children,
  side = 'top',
  align = 'center',
  ...props
}: ButtonTooltipProps) => {
  return (
    <Tooltip disableHoverableContent>
      <TooltipTrigger asChild>
        <Button {...props}>{children}</Button>
      </TooltipTrigger>
      <TooltipContent side={side} align={align}>
        {tooltip}
      </TooltipContent>
    </Tooltip>
  );
};
