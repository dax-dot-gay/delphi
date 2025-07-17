import { Portal } from "@mantine/core";
import { ReactNode } from "react";

export function HeaderContents({
    children,
}: {
    children?: ReactNode | ReactNode[];
}) {
    return <Portal target="#app-header-content">{children}</Portal>;
}
