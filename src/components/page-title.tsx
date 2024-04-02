
interface PageTitleProps {
  title: React.ReactNode;
  description?: React.ReactNode;
  children?: React.ReactNode;
}

export const PageTitle = ({ title, description, children }: PageTitleProps) => {
  return (
    <div className="flex items-center justify-between py-4 px-6 border-b w-full">
      <div className="flex flex-col gap-1">
        <h1 className="text-2xl font-bold text-strong">{title}</h1>
        <p className="text-soft text-sm">{description}&nbsp;</p>
      </div>
      {children}
    </div>
  )
}
