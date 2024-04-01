import { PropsWithChildren } from "react";
import { InnerSidebarLayout } from "./layout";

export const InnerSidebar = (props: PropsWithChildren<{ className?: string }>) => {
  return (
    <InnerSidebarLayout className={props.className}>
      {props.children}
    </InnerSidebarLayout>
  );
}
