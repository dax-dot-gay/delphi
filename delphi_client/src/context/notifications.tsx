import { notifications } from "@mantine/notifications";
import { useTranslation } from "react-i18next";
import { TbCircleCheckFilled, TbCircleXFilled } from "react-icons/tb";

export function useNotifications() {
    const { t } = useTranslation();

    return {
        success: (message: string) =>
            notifications.show({
                icon: <TbCircleCheckFilled />,
                color: "lime",
                title: t("common.notif.success"),
                message: message,
            }),
        error: (message: string) =>
            notifications.show({
                icon: <TbCircleXFilled />,
                color: "red",
                title: t("common.notif.error"),
                message: message,
            }),
    };
}
