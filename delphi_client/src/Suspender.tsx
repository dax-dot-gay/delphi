import { RouterProvider } from "react-router";
import { router } from "./routes";
import { Center, Group, Loader, MantineProvider, Title } from "@mantine/core";
import { shadcnCssVariableResolver } from "./theme/cssVariableResolver";
import { shadcnTheme } from "./theme/theme";
import { Suspense } from "react";
import { AuthProvider, getStatus } from "./context/auth";
import { useTranslation } from "react-i18next";

export function Suspender() {
    const { t } = useTranslation();
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
                                {t("app.name")}
                            </Title>
                        </Group>
                    </Center>
                }
            >
                <AuthProvider promise={statusPromise}>
                    <RouterProvider router={router} />
                </AuthProvider>
            </Suspense>
        </MantineProvider>
    );
}
