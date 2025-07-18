import { PasswordInput, PasswordInputProps } from "@mantine/core";
import { TbEye, TbEyeClosed } from "react-icons/tb";

export function PasswordField(
    props: Omit<PasswordInputProps, "visibilityToggleIcon">
) {
    return (
        <PasswordInput
            {...props}
            visibilityToggleIcon={({ reveal }) =>
                reveal ? <TbEyeClosed size={16} /> : <TbEye size={16} />
            }
        />
    );
}
