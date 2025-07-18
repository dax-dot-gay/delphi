import { RouterProvider } from "react-router";
import { router } from "./routes";
import { Center, Group, Loader, MantineProvider, Title } from "@mantine/core";
import { shadcnCssVariableResolver } from "./theme/cssVariableResolver";
import { shadcnTheme } from "./theme/theme";
import { Suspense } from "react";
import { AuthProvider, getStatus } from "./context/auth";
import { Localization } from "./lang/provider";
import { Notifications } from "@mantine/notifications";

export function Suspender() {
    const statusPromise = getStatus();
    return (
        <MantineProvider
            forceColorScheme="dark"
            theme={shadcnTheme}
            cssVariablesResolver={shadcnCssVariableResolver}
        >
            <Suspense
                fallback={
                    <Center w="100vw" h="100vh">
                        <Group gap="md">
                            <Loader type="dots" color="primary" size="lg" />
                            <Title order={2} ff="monospace">
                                Delphi
                            </Title>
                        </Group>
                    </Center>
                }
            >
                <AuthProvider promise={statusPromise}>
                    <Localization>
                        <Notifications />
                        <RouterProvider router={router} />
                    </Localization>
                </AuthProvider>
            </Suspense>
        </MantineProvider>
    );
}
