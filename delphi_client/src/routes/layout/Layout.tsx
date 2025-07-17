import { TbCrystalBall } from "react-icons/tb";
import { useDisclosure } from "@mantine/hooks";
import "./layout.scss";
import {
    AppShell,
    AppShellHeader,
    AppShellMain,
    AppShellNavbar,
    Box,
    Burger,
    Group,
    Text,
    useMatches,
} from "@mantine/core";
import { Outlet } from "react-router";
import { useTranslation } from "react-i18next";

export function Layout() {
    const isMobile = useMatches({
        base: true,
        md: false,
    });

    const [collapsed, { toggle: toggleCollapsed }] = useDisclosure(true);
    const { t } = useTranslation();

    return (
        <AppShell
            header={{ height: 48 }}
            navbar={{
                width: "256",
                breakpoint: "md",
                collapsed: { desktop: false, mobile: collapsed },
            }}
            className="layout root"
        >
            <AppShellHeader p={0} m="xs" mb={0} className="layout header">
                <Group
                    gap="sm"
                    w="100%"
                    h="100%"
                    justify="start"
                    align="center"
                    p={0}
                >
                    <span />
                    {isMobile ? (
                        <>
                            <Burger
                                opened={!collapsed}
                                onClick={toggleCollapsed}
                                size="sm"
                            />
                            <Text size="lg" ff="monospace" fw={500}>
                                {t("app.name")}
                            </Text>
                        </>
                    ) : (
                        <>
                            <TbCrystalBall size={24} />
                            <Text ff="monospace" size="lg" fw={500}>
                                {t("app.name")}
                            </Text>
                        </>
                    )}
                    <Box
                        id="app-header-content"
                        style={{ flexGrow: 1 }}
                        h="100%"
                    ></Box>
                </Group>
            </AppShellHeader>
            <AppShellNavbar
                p="xs"
                m="xs"
                mr={0}
                className={`layout nav ${
                    collapsed && isMobile ? "collapsed" : ""
                } ${isMobile ? "mobile" : ""}`}
            ></AppShellNavbar>
            <AppShellMain className="layout content">
                <Box id="app-content">
                    <Outlet />
                </Box>
            </AppShellMain>
        </AppShell>
    );
}
