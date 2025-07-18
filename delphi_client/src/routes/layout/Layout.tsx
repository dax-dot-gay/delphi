import {
    TbCrystalBall,
    TbLogout,
    TbLogout2,
    TbUser,
    TbUserShield,
} from "react-icons/tb";
import { useDisclosure } from "@mantine/hooks";
import "./layout.scss";
import {
    AppShell,
    AppShellHeader,
    AppShellMain,
    AppShellNavbar,
    Box,
    Burger,
    Button,
    Divider,
    Group,
    ScrollArea,
    Skeleton,
    Stack,
    Text,
    ThemeIcon,
    useMatches,
} from "@mantine/core";
import { Outlet, useLocation, useNavigate } from "react-router";
import { useTranslation } from "react-i18next";
import { useAuthRefresh, useAuthUser } from "../../context/auth";
import { useEffect } from "react";
import { Root } from "../../api";

export function Layout() {
    const isMobile = useMatches({
        base: true,
        md: false,
    });

    const [collapsed, { toggle: toggleCollapsed }] = useDisclosure(true);
    const { t } = useTranslation();

    const nav = useNavigate();
    const location = useLocation();
    const authUser = useAuthUser();
    const refresh = useAuthRefresh();

    useEffect(() => {
        if (location.pathname !== "/auth" && !authUser) {
            nav("/auth");
        }
    }, [nav, location.pathname, authUser?.id]);

    return (
        <AppShell
            header={{ height: 48 }}
            navbar={{
                width: "256",
                breakpoint: "md",
                collapsed: { desktop: false, mobile: collapsed },
            }}
            className={`layout root ${
                location.pathname === "/auth" ? "disabled" : ""
            }`}
            disabled={location.pathname === "/auth"}
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
                m="xs"
                mr={0}
                className={`layout nav ${
                    collapsed && isMobile ? "collapsed" : ""
                } ${isMobile ? "mobile" : ""}`}
            >
                {authUser && (
                    <>
                        <AppShell.Section
                            grow
                            my="md"
                            component={ScrollArea}
                            p="xs"
                        >
                            <Stack gap="sm"></Stack>
                        </AppShell.Section>
                        <AppShell.Section>
                            <Stack gap={0} mt="sm">
                                <Divider />
                                <Group gap="sm" justify="space-between" p="xs">
                                    <ThemeIcon
                                        color="primary"
                                        variant="light"
                                        size="lg"
                                    >
                                        {authUser.is_admin ? (
                                            <TbUserShield size={20} />
                                        ) : (
                                            <TbUser size={20} />
                                        )}
                                    </ThemeIcon>
                                    <Button
                                        leftSection={<TbLogout size={20} />}
                                        variant="subtle"
                                        onClick={() => {
                                            Root.logout().then(refresh);
                                        }}
                                    >
                                        {t("layout.logout")}
                                    </Button>
                                </Group>
                            </Stack>
                        </AppShell.Section>
                    </>
                )}
            </AppShellNavbar>
            <AppShellMain className="layout content">
                <Box id="app-content">
                    <Outlet />
                </Box>
            </AppShellMain>
        </AppShell>
    );
}
