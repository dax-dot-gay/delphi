import { useTranslation } from "react-i18next";
import { useNavigate } from "react-router";
import { useAuthRefresh, useAuthUser } from "../../context/auth";
import { useEffect } from "react";
import {
    Button,
    Center,
    Divider,
    Group,
    Paper,
    Stack,
    Text,
    TextInput,
    Title,
} from "@mantine/core";
import { useForm } from "@mantine/form";
import "./auth.scss";
import {
    TbCrystalBall,
    TbLogin2,
    TbPassword,
    TbUserFilled,
} from "react-icons/tb";
import { PasswordField } from "../../components/PasswordField";
import { useNotifications } from "../../context/notifications";
import { Root } from "../../api";
import { useDisclosure } from "@mantine/hooks";

export function AuthView() {
    const { t } = useTranslation();
    const nav = useNavigate();
    const refreshAuth = useAuthRefresh();
    const authUser = useAuthUser();
    const loginForm = useForm({
        initialValues: {
            username: "",
            password: "",
        },
    });
    const { error, success } = useNotifications();
    const [loading, { open: startLoading, close: stopLoading }] =
        useDisclosure(false);

    useEffect(() => {
        if (authUser !== null) {
            nav("/");
        }
    }, [nav, authUser?.id]);

    return (
        <Center className="auth-container" h="100vh" w="100vw">
            <Stack gap="sm">
                <Paper withBorder p="sm" radius="xs">
                    <Group gap="sm" justify="space-between" w="100%">
                        <TbCrystalBall size={28} />
                        <Title order={3} ff="monospace" fw={500}>
                            {t("app.name")}
                        </Title>
                    </Group>
                </Paper>
                <Paper className="paper-light auth-box" p="sm" radius="xs">
                    <form
                        onSubmit={loginForm.onSubmit(
                            ({ username, password }) => {
                                startLoading();
                                Root.login({
                                    body: { username, password },
                                }).then((result) => {
                                    stopLoading();
                                    if (result.data) {
                                        success(
                                            t("views.auth.feedback.success")
                                        );
                                        refreshAuth();
                                    } else {
                                        if (
                                            result.error.code ===
                                            "invalid_login"
                                        ) {
                                            error(
                                                t("views.auth.feedback.unknown")
                                            );
                                        } else {
                                            console.log(result.error);
                                            error(
                                                t("common.errors.api.unhandled")
                                            );
                                        }
                                    }
                                });
                            }
                        )}
                    >
                        <Stack gap="sm">
                            <Group gap="sm" justify="space-between">
                                <TbLogin2 size={24} />
                                <Text size="lg" fw={500}>
                                    {t("views.auth.login")}
                                </Text>
                            </Group>
                            <Divider />
                            <TextInput
                                label={t("views.auth.username.label")}
                                leftSection={<TbUserFilled size={20} />}
                                {...loginForm.getInputProps("username")}
                            />
                            <PasswordField
                                label={t("views.auth.password.label")}
                                leftSection={<TbPassword size={20} />}
                                {...loginForm.getInputProps("password")}
                            />
                            <Group justify="end">
                                <Button
                                    loading={loading}
                                    type="submit"
                                    leftSection={<TbLogin2 size={20} />}
                                    disabled={
                                        loginForm.values.username.length ===
                                            0 ||
                                        loginForm.values.password.length === 0
                                    }
                                >
                                    {t("views.auth.login")}
                                </Button>
                            </Group>
                        </Stack>
                    </form>
                </Paper>
            </Stack>
        </Center>
    );
}
